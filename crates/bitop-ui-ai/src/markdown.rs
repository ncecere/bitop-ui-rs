//! Markdown for model output: closing a partial stream, rendering it to safe HTML block
//! by block, and finding the blocks that changed while it streams.
//!
//! Model output is untrusted. [`render_blocks`] shows raw HTML as text, keeps a link
//! or an image only when its URL is `http`, `https` (or `mailto` for links, and a
//! `data:image/` picture for images), and by default does not load images from other
//! hosts until the reader asks. The output is safe to insert into a page without a
//! separate sanitiser. [`response`](crate::components::response::response) renders it.
//!
//! Adapted from the MIT-licensed `oci-markdown` crate by the same author: the URL
//! check and the streaming diff come from there; the HTML writer is this crate's own,
//! so that code blocks and citations can be components.

use pulldown_cmark::{
    Alignment, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Options, Parser, Tag, TagEnd,
};

use crate::components::sources::hostname;
use crate::css::response as css;

/// How Markdown images load in a response.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResponseImages {
    /// Images in the page (`data:` URLs) load; an image from another host is a
    /// "Load image from example.com" button, fetched only when it is pressed.
    #[default]
    Click,
    /// Every image with a safe URL loads, lazily. Only for trusted content.
    Show,
    /// No image loads; "[Image: alt]" is shown instead.
    Alt,
}

/// What [`render_blocks`] needs besides the source: the options of
/// [`response`](crate::components::response::response) and its strings.
#[derive(Clone, Debug)]
pub struct MarkdownOptions<'a> {
    /// Levels added to Markdown headings: 2 renders `#` as `<h3>`.
    pub heading_offset: u8,
    /// Renders every heading at this level (1 to 6) instead, keeping its size.
    pub heading_level: Option<u8>,
    /// Drops raw HTML instead of showing it as text.
    pub skip_html: bool,
    /// How images load.
    pub images: ResponseImages,
    /// How many sources `[n]` markers may refer to; 0 leaves markers as text.
    pub citations: usize,
    /// The response's ID, from which citation IDs are made.
    pub id: &'a str,
    /// Read after a link that opens in a new tab.
    pub new_tab_label: &'a str,
    /// The text of an image's load button, with `{host}`.
    pub load_image_label: &'a str,
    /// The same, when the host cannot be read.
    pub load_image_unknown_label: &'a str,
    /// Shown for an image that is not loaded, with `{alt}`.
    pub image_alt_label: &'a str,
    /// Shown for an image without alt text that is not loaded.
    pub image_label: &'a str,
    /// Read before a done task.
    pub done_label: &'a str,
    /// Read before an open task.
    pub todo_label: &'a str,
}

impl Default for MarkdownOptions<'_> {
    fn default() -> Self {
        Self {
            heading_offset: 2,
            heading_level: None,
            skip_html: false,
            images: ResponseImages::Click,
            citations: 0,
            id: "response",
            new_tab_label: "(opens in a new tab)",
            load_image_label: "Load image from {host}",
            load_image_unknown_label: "Load image from another site",
            image_alt_label: "[Image: {alt}]",
            image_label: "[Image]",
            done_label: "Done: ",
            todo_label: "To do: ",
        }
    }
}

/// A piece of a rendered block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    /// Sanitised HTML.
    Html(String),
    /// A fenced or indented code block, for
    /// [`code_block`](crate::components::code_block::code_block). `block` is set when
    /// the code block is a top-level block itself.
    Code {
        language: String,
        code: String,
        block: Option<usize>,
    },
    /// A `[n]` marker, for
    /// [`inline_citation`](crate::components::inline_citation::inline_citation):
    /// its ID and its numbers, counted from 1.
    Citation { id: String, index: Vec<usize> },
}

/// One top-level block (a paragraph, a list, a table, a code block…) as segments.
/// Its first element carries `data-block` with its position.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Block {
    pub segments: Vec<Segment>,
}

impl Block {
    /// A string that changes whenever the block's output does.
    #[must_use]
    pub fn key(&self) -> String {
        let mut key = String::new();
        for segment in &self.segments {
            match segment {
                Segment::Html(html) => key.push_str(html),
                Segment::Code { language, code, .. } => {
                    key.push_str("\u{0}code:");
                    key.push_str(language);
                    key.push('\u{0}');
                    key.push_str(code);
                }
                Segment::Citation { id, index } => {
                    key.push_str("\u{0}cite:");
                    key.push_str(id);
                    key.push_str(&format!("{index:?}"));
                }
            }
        }
        key
    }

    /// The block's HTML when it holds no component, as for most paragraphs.
    #[must_use]
    pub fn html(&self) -> Option<&str> {
        match self.segments.as_slice() {
            [Segment::Html(html)] => Some(html),
            _ => None,
        }
    }
}

fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM
}

/// Escapes text for HTML content and attribute values.
#[must_use]
pub fn escape_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// What a URL may be used for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UrlUse {
    Link,
    Image,
}

/// The URL if it is safe to use: `http` and `https`; `mailto` for links; for images a
/// `data:` URL of a PNG, JPEG, GIF or WebP picture. Anything else, relative URLs
/// included, is refused: a model has no business pointing into the app itself.
///
/// ```
/// use bitop_ui_ai::markdown::{UrlUse, safe_url};
///
/// assert_eq!(safe_url("https://example.com", UrlUse::Link), Some("https://example.com"));
/// assert_eq!(safe_url("javascript:alert(1)", UrlUse::Link), None);
/// assert_eq!(safe_url("data:text/html,x", UrlUse::Image), None);
/// ```
#[must_use]
pub fn safe_url(url: &str, use_: UrlUse) -> Option<&str> {
    // Browsers ignore leading whitespace and control characters, and compare the scheme
    // without regard to case, so the check must do the same.
    let plain: String = url
        .chars()
        .filter(|c| !c.is_control() && !c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase();
    let allowed = plain.starts_with("https://")
        || plain.starts_with("http://")
        || (use_ == UrlUse::Link && plain.starts_with("mailto:"))
        || (use_ == UrlUse::Image
            && ["png", "jpeg", "gif", "webp"]
                .iter()
                .any(|kind| plain.starts_with(&format!("data:image/{kind};"))));
    allowed.then_some(url.trim())
}

fn is_identifier(c: Option<char>) -> bool {
    c.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The `[n]` and `[n, m]` citation markers in `text`: their byte ranges and numbers.
/// Brackets attached to an identifier (`a[3]`, `m[i][2]`, `[3]int`) or followed by `(`
/// are not markers, as upstream's rules say.
#[must_use]
pub fn citation_markers(text: &str) -> Vec<(std::ops::Range<usize>, Vec<usize>)> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut last_end: Option<usize> = None;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'[' {
            i += 1;
            continue;
        }
        // Digits (one to three) separated by commas and spaces, then "]".
        let mut j = i + 1;
        let mut numbers = Vec::new();
        let mut ok = false;
        loop {
            let start = j;
            while j < bytes.len() && bytes[j].is_ascii_digit() && j - start < 3 {
                j += 1;
            }
            if j == start || (j < bytes.len() && bytes[j].is_ascii_digit()) {
                break;
            }
            numbers.push(text[start..j].parse::<usize>().unwrap_or(0));
            let mut k = j;
            while k < bytes.len() && bytes[k] == b' ' {
                k += 1;
            }
            if k < bytes.len() && bytes[k] == b']' {
                j = k + 1;
                ok = true;
                break;
            }
            if k < bytes.len() && bytes[k] == b',' {
                k += 1;
                while k < bytes.len() && bytes[k] == b' ' {
                    k += 1;
                }
                j = k;
                continue;
            }
            break;
        }
        if !ok {
            i += 1;
            continue;
        }
        let before = text[..i].chars().next_back();
        let after = text[j..].chars().next();
        let attached = is_identifier(before)
            || (before == Some(']') && last_end != Some(i))
            || is_identifier(after)
            || after == Some('(');
        if !attached {
            found.push((i..j, numbers));
            last_end = Some(j);
        }
        i = j;
    }
    found
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LinkKind {
    External,
    Mail,
    Dropped,
}

struct Writer<'o> {
    options: &'o MarkdownOptions<'o>,
    blocks: Vec<Block>,
    segments: Vec<Segment>,
    out: String,
    depth: usize,
    citations: usize,
    pending_text: String,
    links: Vec<LinkKind>,
    image: Option<(String, String)>,
    code: Option<(String, String, Option<usize>)>,
    in_head: bool,
    head_start: Option<usize>,
    head_text: bool,
    body_open: bool,
    alignments: Vec<Alignment>,
    cell: usize,
    cell_start: Option<usize>,
    cell_text: bool,
    in_cell: bool,
    headings: Vec<String>,
}

impl<'o> Writer<'o> {
    fn new(options: &'o MarkdownOptions<'o>) -> Self {
        Self {
            options,
            blocks: Vec::new(),
            segments: Vec::new(),
            out: String::new(),
            depth: 0,
            citations: 0,
            pending_text: String::new(),
            links: Vec::new(),
            image: None,
            code: None,
            in_head: false,
            head_start: None,
            head_text: false,
            body_open: false,
            alignments: Vec::new(),
            cell: 0,
            cell_start: None,
            cell_text: false,
            in_cell: false,
            headings: Vec::new(),
        }
    }

    /// ` data-block="n"` for the element that opens a top-level block.
    fn block_attr(&self) -> String {
        if self.depth == 0 {
            format!(" data-block=\"{}\"", self.blocks.len())
        } else {
            String::new()
        }
    }

    fn flush_html(&mut self) {
        if !self.out.is_empty() {
            self.segments
                .push(Segment::Html(std::mem::take(&mut self.out)));
            // Positions in the old buffer no longer mean anything.
            self.head_start = None;
            self.cell_start = None;
        }
    }

    fn finish_block(&mut self) {
        self.flush_text();
        self.flush_html();
        if !self.segments.is_empty() {
            self.blocks.push(Block {
                segments: std::mem::take(&mut self.segments),
            });
        }
    }

    fn text_seen(&mut self, text: &str) {
        if !text.trim().is_empty() {
            if self.in_head {
                self.head_text = true;
            }
            if self.in_cell {
                self.cell_text = true;
            }
        }
    }

    /// Writes text gathered from consecutive text events, turning `[n]` markers into
    /// citations. Markdown splits text at brackets, so markers are found only once the
    /// pieces are joined.
    fn flush_text(&mut self) {
        if self.pending_text.is_empty() {
            return;
        }
        let text = std::mem::take(&mut self.pending_text);
        let markers = if self.options.citations > 0 && self.links.is_empty() {
            citation_markers(&text)
        } else {
            Vec::new()
        };
        let mut last = 0;
        for (range, numbers) in markers {
            if numbers
                .iter()
                .any(|&n| n == 0 || n > self.options.citations)
            {
                // A number with no source: the marker stays text.
                continue;
            }
            self.out.push_str(&escape_html(&text[last..range.start]));
            self.flush_html();
            self.citations += 1;
            self.segments.push(Segment::Citation {
                id: format!("{}-cite-{}", self.options.id, self.citations),
                index: numbers,
            });
            last = range.end;
        }
        self.out.push_str(&escape_html(&text[last..]));
    }

    fn event(&mut self, event: Event<'_>) {
        if let Some((_, alt)) = &mut self.image {
            match event {
                Event::Text(text) | Event::Code(text) => {
                    alt.push_str(&text);
                    return;
                }
                Event::End(TagEnd::Image) => {}
                _ => return,
            }
        }
        if let Some((_, code, _)) = &mut self.code {
            match event {
                Event::Text(text) => {
                    code.push_str(&text);
                    return;
                }
                Event::End(TagEnd::CodeBlock) => {}
                _ => return,
            }
        }
        if let Event::Text(text) = &event {
            self.text_seen(text);
            self.pending_text.push_str(text);
            return;
        }
        self.flush_text();
        match event {
            Event::Start(tag) => {
                self.start(tag);
                self.depth += 1;
            }
            Event::End(tag) => {
                self.depth = self.depth.saturating_sub(1);
                self.end(tag);
                if self.depth == 0 {
                    self.finish_block();
                }
            }
            Event::Code(code) => {
                self.text_seen(&code);
                self.out.push_str("<code class=\"");
                self.out.push_str(css::INLINE_CODE);
                self.out.push_str("\">");
                self.out.push_str(&escape_html(&code));
                self.out.push_str("</code>");
            }
            Event::Html(html) | Event::InlineHtml(html) => self.raw_html(&html),
            Event::SoftBreak => self.out.push('\n'),
            Event::HardBreak => self.out.push_str("<br>"),
            Event::Rule => {
                let attr = self.block_attr();
                self.out.push_str(&format!("<hr{attr}>"));
                if self.depth == 0 {
                    self.finish_block();
                }
            }
            Event::TaskListMarker(done) => {
                self.out.push_str(&format!(
                    "<span aria-hidden=\"true\" class=\"{}\"{}></span><span class=\"sr-only\">{}</span>",
                    css::TASK_BOX,
                    if done { " data-checked=\"\"" } else { "" },
                    escape_html(if done {
                        self.options.done_label
                    } else {
                        self.options.todo_label
                    })
                ));
            }
            Event::Text(_)
            | Event::InlineMath(_)
            | Event::DisplayMath(_)
            | Event::FootnoteReference(_) => {}
        }
    }

    /// Raw HTML is shown as text, or dropped; only a bare `<br>` in a table cell
    /// becomes a line break, since a cell cannot hold one otherwise.
    fn raw_html(&mut self, html: &str) {
        if self.options.skip_html {
            return;
        }
        let trimmed = html.trim().to_ascii_lowercase();
        let bare_break = trimmed.starts_with("<br")
            && trimmed.ends_with('>')
            && trimmed[3..trimmed.len() - 1]
                .trim_end_matches('/')
                .trim()
                .is_empty();
        if self.in_cell && bare_break {
            self.out.push_str("<br>");
            return;
        }
        self.text_seen(html);
        self.out.push_str(&escape_html(html));
    }

    fn start(&mut self, tag: Tag<'_>) {
        let attr = self.block_attr();
        match tag {
            Tag::Paragraph => self.out.push_str(&format!("<p{attr}>")),
            // An HTML block is shown as a paragraph of text.
            Tag::HtmlBlock => self.out.push_str(&format!("<p{attr}>")),
            Tag::Heading { level, .. } => {
                let (tag, data) = self.heading(level);
                self.out.push_str(&format!(
                    "<{tag}{attr} class=\"{}\" data-level=\"{data}\">",
                    css::HEADING
                ));
                self.headings.push(tag);
            }
            Tag::BlockQuote(_) => self.out.push_str(&format!("<blockquote{attr}>")),
            Tag::CodeBlock(kind) => {
                let language = match kind {
                    CodeBlockKind::Fenced(info) => info
                        .split(|c: char| c.is_whitespace() || c == ',' || c == '{')
                        .next()
                        .unwrap_or_default()
                        .to_owned(),
                    CodeBlockKind::Indented => String::new(),
                };
                let block = (self.depth == 0).then_some(self.blocks.len());
                self.code = Some((language, String::new(), block));
            }
            Tag::List(Some(start)) if start != 1 => {
                self.out.push_str(&format!("<ol{attr} start=\"{start}\">"));
            }
            Tag::List(Some(_)) => self.out.push_str(&format!("<ol{attr}>")),
            Tag::List(None) => self.out.push_str(&format!("<ul{attr}>")),
            Tag::Item => self.out.push_str("<li>"),
            Tag::Table(alignments) => {
                self.alignments = alignments;
                self.body_open = false;
                self.out.push_str(&format!(
                    "<div class=\"{}\" tabindex=\"0\"{attr}><table class=\"{}\">",
                    css::TABLE_WRAP,
                    css::TABLE
                ));
            }
            Tag::TableHead => {
                self.in_head = true;
                self.head_text = false;
                self.head_start = Some(self.out.len());
                self.cell = 0;
                self.out.push_str("<thead><tr>");
            }
            Tag::TableRow => {
                if !self.body_open {
                    self.out.push_str("<tbody>");
                    self.body_open = true;
                }
                self.cell = 0;
                self.out.push_str("<tr>");
            }
            Tag::TableCell => {
                self.in_cell = true;
                self.cell_text = false;
                self.cell_start = Some(self.out.len());
                let align = match self.alignments.get(self.cell) {
                    Some(Alignment::Left) => " data-align=\"left\"",
                    Some(Alignment::Center) => " data-align=\"center\"",
                    Some(Alignment::Right) => " data-align=\"right\"",
                    _ => "",
                };
                let tag = if self.in_head { "th" } else { "td" };
                self.out.push_str(&format!("<{tag}{align}>"));
            }
            Tag::Emphasis => self.out.push_str("<em>"),
            Tag::Strong => self.out.push_str("<strong>"),
            Tag::Strikethrough => self.out.push_str("<del>"),
            Tag::Superscript => self.out.push_str("<sup>"),
            Tag::Subscript => self.out.push_str("<sub>"),
            Tag::Link {
                link_type,
                dest_url,
                title,
                ..
            } => self.link(link_type, &dest_url, &title),
            Tag::Image { dest_url, .. } => {
                self.image = Some((dest_url.into_string(), String::new()));
            }
            Tag::FootnoteDefinition(_)
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition
            | Tag::MetadataBlock(_) => {}
        }
    }

    fn heading(&self, level: HeadingLevel) -> (String, usize) {
        let level = level as usize;
        let at = match self.options.heading_level {
            Some(fixed) => usize::from(fixed.clamp(1, 6)),
            None => (level + usize::from(self.options.heading_offset)).min(6),
        };
        (format!("h{at}"), level)
    }

    fn link(&mut self, link_type: LinkType, dest_url: &CowStr<'_>, title: &CowStr<'_>) {
        let url = if link_type == LinkType::Email {
            format!("mailto:{dest_url}")
        } else {
            dest_url.to_string()
        };
        let Some(href) = safe_url(&url, UrlUse::Link) else {
            self.links.push(LinkKind::Dropped);
            return;
        };
        let external = !href.to_ascii_lowercase().starts_with("mailto:");
        let title = if title.is_empty() {
            String::new()
        } else {
            format!(" title=\"{}\"", escape_html(title))
        };
        self.out.push_str(&format!(
            "<a href=\"{}\"{title} class=\"{}\" data-tone=\"default\"{}>",
            escape_html(href),
            bitop_ui::css::text_link::LINK,
            if external {
                " target=\"_blank\" rel=\"noreferrer noopener\" data-external=\"\""
            } else {
                ""
            }
        ));
        self.links.push(if external {
            LinkKind::External
        } else {
            LinkKind::Mail
        });
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.out.push_str("</p>"),
            TagEnd::HtmlBlock => self.out.push_str("</p>"),
            TagEnd::Heading(_) => {
                let tag = self.headings.pop().unwrap_or_else(|| "h3".to_owned());
                self.out.push_str(&format!("</{tag}>"));
            }
            TagEnd::BlockQuote(_) => self.out.push_str("</blockquote>"),
            TagEnd::CodeBlock => {
                if let Some((language, code, block)) = self.code.take() {
                    self.flush_html();
                    self.segments.push(Segment::Code {
                        language,
                        code,
                        block,
                    });
                }
            }
            TagEnd::List(true) => self.out.push_str("</ol>"),
            TagEnd::List(false) => self.out.push_str("</ul>"),
            TagEnd::Item => self.out.push_str("</li>"),
            TagEnd::Table => {
                if self.body_open {
                    self.out.push_str("</tbody>");
                }
                self.out.push_str("</table></div>");
            }
            TagEnd::TableHead => {
                self.out.push_str("</tr></thead>");
                self.in_head = false;
                // A header row with no text says nothing: leave it out.
                if !self.head_text
                    && let Some(start) = self.head_start
                {
                    self.out.truncate(start);
                }
                self.head_start = None;
            }
            TagEnd::TableRow => self.out.push_str("</tr>"),
            TagEnd::TableCell => {
                let header = self.in_head;
                let mut tag = if header { "th" } else { "td" };
                // A header cell with no text is a plain cell.
                if header
                    && !self.cell_text
                    && let Some(start) = self.cell_start
                {
                    self.out.replace_range(start..start + 3, "<td");
                    tag = "td";
                }
                self.out.push_str(&format!("</{tag}>"));
                self.in_cell = false;
                self.cell_start = None;
                self.cell += 1;
            }
            TagEnd::Emphasis => self.out.push_str("</em>"),
            TagEnd::Strong => self.out.push_str("</strong>"),
            TagEnd::Strikethrough => self.out.push_str("</del>"),
            TagEnd::Superscript => self.out.push_str("</sup>"),
            TagEnd::Subscript => self.out.push_str("</sub>"),
            TagEnd::Link => match self.links.pop() {
                Some(LinkKind::External) => {
                    self.out.push_str(&format!(
                        "<span data-icon=\"arrow-up-right\" aria-hidden=\"true\" class=\"{}\"></span><span class=\"sr-only\"> {}</span></a>",
                        bitop_ui::css::text_link::ICON,
                        escape_html(self.options.new_tab_label)
                    ));
                }
                Some(LinkKind::Mail) => self.out.push_str("</a>"),
                Some(LinkKind::Dropped) | None => {}
            },
            TagEnd::Image => {
                if let Some((url, alt)) = self.image.take() {
                    self.image_html(&url, &alt);
                }
            }
            TagEnd::FootnoteDefinition
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::MetadataBlock(_) => {}
        }
    }

    fn image_html(&mut self, url: &str, alt: &str) {
        let alt_text = escape_html(alt);
        let as_text = if alt.is_empty() {
            escape_html(self.options.image_label)
        } else {
            escape_html(&self.options.image_alt_label.replace("{alt}", alt))
        };
        let safe = safe_url(url, UrlUse::Image);
        let Some(src) = safe.filter(|_| self.options.images != ResponseImages::Alt) else {
            self.out.push_str(&format!(
                "<span class=\"{}\">{as_text}</span>",
                css::IMAGE_ALT
            ));
            return;
        };
        let remote = !src.to_ascii_lowercase().starts_with("data:");
        if self.options.images == ResponseImages::Click && remote {
            let label = match hostname(src) {
                Some(host) => self.options.load_image_label.replace("{host}", &host),
                None => self.options.load_image_unknown_label.to_owned(),
            };
            let alt_part = if alt.is_empty() {
                String::new()
            } else {
                format!(
                    "<span class=\"sr-only\">: </span><span class=\"{}\">{alt_text}</span>",
                    css::IMAGE_LOAD_ALT
                )
            };
            self.out.push_str(&format!(
                "<button type=\"button\" class=\"{}\" data-response-image=\"{}\" data-alt=\"{alt_text}\" data-frame-class=\"{}\" data-image-class=\"{}\"><span>{}</span>{alt_part}</button>",
                css::IMAGE_LOAD,
                escape_html(src),
                css::IMAGE_FRAME,
                css::IMAGE,
                escape_html(&label)
            ));
            return;
        }
        self.out.push_str(&format!(
            "<img src=\"{}\" alt=\"{alt_text}\" loading=\"lazy\" class=\"{}\">",
            escape_html(src),
            css::IMAGE
        ));
    }
}

/// Renders Markdown to blocks of sanitised HTML, code blocks and citations.
#[must_use]
pub fn render_blocks(source: &str, options: &MarkdownOptions<'_>) -> Vec<Block> {
    let mut writer = Writer::new(options);
    for event in Parser::new_ext(source, self::options()) {
        writer.event(event);
    }
    writer.finish_block();
    writer.blocks
}

/// The blocks that changed since the last update, and how many there are now. Blocks
/// at or past `len` must be removed; that happens when later text merges two blocks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamUpdate {
    /// The positions of the blocks to send again.
    pub changed: Vec<usize>,
    /// How many blocks there are.
    pub len: usize,
}

impl StreamUpdate {
    /// True when the receiver already shows exactly this state.
    #[must_use]
    pub fn is_empty(&self, previous_len: usize) -> bool {
        self.changed.is_empty() && self.len == previous_len
    }
}

/// Tracks what one reader has been sent while an answer streams, so that each update
/// carries only the blocks that changed: almost always just the last one. Render
/// those with [`response_block`](crate::components::response::response_block) and
/// apply them in the browser with `bitopUi.patchResponse`.
#[derive(Default)]
pub struct ResponseStream {
    sent: Vec<String>,
}

impl ResponseStream {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// How many blocks the reader holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sent.len()
    }

    /// Whether the reader holds no blocks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sent.is_empty()
    }

    /// Compares the blocks of the source so far with what was last sent. Earlier
    /// blocks are compared rather than assumed final, because later text can change
    /// them: a link reference definition, or a `---` under a paragraph.
    pub fn update(&mut self, blocks: &[Block]) -> StreamUpdate {
        let keys: Vec<String> = blocks.iter().map(Block::key).collect();
        let changed = keys
            .iter()
            .enumerate()
            .filter(|(index, key)| self.sent.get(*index) != Some(*key))
            .map(|(index, _)| index)
            .collect();
        let len = keys.len();
        self.sent = keys;
        StreamUpdate { changed, len }
    }
}

// ---------- closing a partial stream ----------

struct Opener {
    ch: char,
    len: usize,
    pos: usize,
}

struct Link {
    start: usize,
    image: bool,
    text_end: usize,
    in_url: bool,
}

struct Scan {
    code: Option<(usize, usize)>,
    emphasis: Vec<Opener>,
    link: Option<Link>,
}

fn is_ws(c: Option<char>) -> bool {
    c.is_none_or(char::is_whitespace)
}

fn is_punct(c: Option<char>) -> bool {
    c.is_some_and(|c| {
        c.is_ascii_punctuation() || (!c.is_ascii() && !c.is_alphanumeric() && !c.is_whitespace())
    })
}

fn run_of(text: &[char], at: usize, c: char) -> usize {
    text[at..].iter().take_while(|&&x| x == c).count()
}

fn scan(text: &[char]) -> Scan {
    let mut emphasis: Vec<Opener> = Vec::new();
    let mut brackets: Vec<(usize, bool)> = Vec::new();
    let mut code: Option<(usize, usize)> = None;
    // An open link destination: start, image, text end, parenthesis depth.
    let mut url: Option<(usize, bool, usize, usize)> = None;
    let at = |i: usize| text.get(i).copied();
    let mut i = 0;
    while i < text.len() {
        let c = text[i];
        if let Some((_, len)) = code {
            if c == '`' {
                let n = run_of(text, i, '`');
                if n == len {
                    code = None;
                }
                i += n;
            } else {
                i += 1;
            }
            continue;
        }
        if c == '\\' {
            i += 2;
            continue;
        }
        if let Some((_, _, _, depth)) = &mut url {
            if c == '(' {
                *depth += 1;
            } else if c == ')' {
                *depth -= 1;
                if *depth == 0 {
                    url = None;
                }
            } else if c == '\n' || c == ' ' {
                // A space or a line break in the destination: it was not a link after all,
                // unless a title follows.
                let rest = text[i..].iter().find(|c| !c.is_whitespace());
                if !matches!(rest, Some('"' | '\'' | '(')) {
                    url = None;
                }
            }
            i += 1;
            continue;
        }
        if c == '`' {
            let n = run_of(text, i, '`');
            code = Some((i, n));
            i += n;
            continue;
        }
        if c == '[' || (c == '!' && at(i + 1) == Some('[')) {
            let image = c == '!';
            brackets.push((i, image));
            i += if image { 2 } else { 1 };
            continue;
        }
        if c == ']' {
            if let Some((pos, image)) = brackets.pop()
                && at(i + 1) == Some('(')
            {
                url = Some((pos, image, i, 1));
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if c == '*' || c == '_' || c == '~' {
            let n = run_of(text, i, c);
            let prev = if i > 0 { at(i - 1) } else { None };
            let next = at(i + n);
            let left = !is_ws(next) && !(is_punct(next) && !is_ws(prev) && !is_punct(prev));
            let right = !is_ws(prev) && !(is_punct(prev) && !is_ws(next) && !is_punct(next));
            let (can_open, can_close) = if c == '_' {
                (
                    left && (!right || is_punct(prev)),
                    right && (!left || is_punct(next)),
                )
            } else {
                (left, right)
            };
            let mut remaining = if c == '~' { n.min(2) } else { n };
            if can_close && emphasis.last().is_some_and(|top| top.ch == c) {
                while remaining > 0 && emphasis.last().is_some_and(|top| top.ch == c) {
                    let top = emphasis.last_mut().expect("checked above");
                    let used = top.len.min(remaining);
                    top.len -= used;
                    remaining -= used;
                    if top.len == 0 {
                        emphasis.pop();
                    }
                }
            }
            if remaining > 0 && can_open {
                if c == '*' && remaining == 3 {
                    emphasis.push(Opener {
                        ch: c,
                        len: 2,
                        pos: i,
                    });
                    emphasis.push(Opener {
                        ch: c,
                        len: 1,
                        pos: i + 2,
                    });
                } else {
                    emphasis.push(Opener {
                        ch: c,
                        len: remaining,
                        pos: i + (n - remaining),
                    });
                }
            }
            i += n;
            continue;
        }
        i += 1;
    }
    let link = if let Some((start, image, text_end, _)) = url {
        Some(Link {
            start,
            image,
            text_end,
            in_url: true,
        })
    } else {
        brackets.last().map(|&(start, image)| Link {
            start,
            image,
            text_end: text.len(),
            in_url: false,
        })
    };
    Scan {
        code,
        emphasis,
        link,
    }
}

fn is_blank(text: &[char]) -> bool {
    text.iter().all(|c| c.is_whitespace())
}

/// Whether `text` is "[" followed only by digits, commas and whitespace: a citation
/// marker still being typed.
fn partial_marker(text: &[char]) -> bool {
    text.first() == Some(&'[')
        && text[1..]
            .iter()
            .all(|c| c.is_ascii_digit() || *c == ',' || c.is_whitespace())
}

/// Closes the inline syntax of the last paragraph.
fn close_inline(input: &str) -> String {
    let mut text: Vec<char> = input.chars().collect();
    // A delimiter run at the very end after whitespace is an opener still waiting for
    // its text.
    let run = text
        .iter()
        .rev()
        .take_while(|c| matches!(c, '*' | '_' | '~'))
        .count();
    if run > 0 {
        let before = text.len() - run;
        if before == 0 || text[before - 1].is_whitespace() {
            text.truncate(before);
        }
    }
    for _ in 0..4 {
        let Some(link) = scan(&text).link else {
            break;
        };
        let open = if link.image { 2 } else { 1 };
        if link.in_url {
            // "[label](https://partial" shows "label"; an image is hidden until complete.
            let label: Vec<char> = if link.image {
                Vec::new()
            } else {
                text[link.start + open..link.text_end].to_vec()
            };
            text.truncate(link.start);
            text.extend(label);
        } else if partial_marker(&text[link.start..]) {
            text.truncate(link.start);
        } else {
            text.drain(link.start..link.start + open);
        }
    }

    let Scan { code, emphasis, .. } = scan(&text);
    let mut closers: Vec<String> = Vec::new();
    if let Some((pos, len)) = code {
        if is_blank(&text[pos + len..]) {
            text.truncate(pos);
        } else {
            closers.push("`".repeat(len));
        }
    }
    for opener in emphasis.iter().rev() {
        if code.is_some_and(|(pos, _)| opener.pos > pos) {
            continue;
        }
        let after_start = (opener.pos + opener.len).min(text.len());
        if closers.is_empty() && is_blank(&text[after_start..]) {
            text.drain(opener.pos..after_start);
        } else {
            closers.push(opener.ch.to_string().repeat(opener.len));
        }
    }
    let text: String = text.into_iter().collect();
    if closers.is_empty() {
        return text;
    }
    format!("{}{}", text.trim_end(), closers.concat())
}

/// A fence line: up to three spaces, then three or more backticks or tildes. Returns
/// the fence character, its length and the rest of the line.
fn fence(line: &str) -> Option<(char, usize, &str)> {
    let indent = line.len() - line.trim_start_matches(' ').len();
    if indent > 3 {
        return None;
    }
    let rest = &line[indent..];
    let ch = rest.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let len = rest.chars().take_while(|c| *c == ch).count();
    (len >= 3).then(|| (ch, len, &rest[len..]))
}

/// Makes a partial, still streaming Markdown string render the way it will once
/// complete, instead of flickering through raw syntax:
///
/// - an unterminated fence (three or more backticks or tildes) is closed, so the rest
///   renders as code;
/// - an unterminated inline code span is closed;
/// - open `**strong**`, `*em*`, `_em_`, `__strong__` and `~~strike~~` are closed,
///   innermost first; an opener with nothing after it yet is dropped;
/// - an incomplete link `[text](https://exa` shows just `text`, never a link to a cut
///   URL; an incomplete image is hidden; a dangling `[` is dropped, and a half-typed
///   citation marker `[1` is hidden.
///
/// Only the last block, after the last blank line or closing fence, is touched:
/// inline syntax cannot span blocks. It is for display only; never store its output.
///
/// ```
/// use bitop_ui_ai::markdown::close_markdown;
///
/// assert_eq!(close_markdown("This is **bold"), "This is **bold**");
/// assert_eq!(close_markdown("```rust\nfn main() {"), "```rust\nfn main() {\n```");
/// assert_eq!(close_markdown("See [the docs](https://exa"), "See the docs");
/// ```
#[must_use]
pub fn close_markdown(markdown: &str) -> String {
    if markdown.is_empty() {
        return String::new();
    }
    let lines: Vec<&str> = markdown.split('\n').collect();
    let mut open: Option<(char, usize)> = None;
    let mut tail_start = 0;
    for (n, line) in lines.iter().enumerate() {
        let found = fence(line);
        if let Some((ch, len)) = open {
            if let Some((c, l, rest)) = found
                && c == ch
                && l >= len
                && rest.trim().is_empty()
            {
                open = None;
                tail_start = n + 1;
            }
            continue;
        }
        if let Some((c, l, rest)) = found
            && !(c == '`' && rest.contains('`'))
        {
            open = Some((c, l));
            continue;
        }
        if line.trim().is_empty() {
            tail_start = n + 1;
        }
    }
    if let Some((ch, len)) = open {
        let newline = if markdown.ends_with('\n') { "" } else { "\n" };
        return format!("{markdown}{newline}{}", ch.to_string().repeat(len));
    }
    let head = lines[..tail_start].join("\n");
    let tail = lines[tail_start..].join("\n");
    if tail.trim().is_empty() {
        return markdown.to_owned();
    }
    let closed = close_inline(&tail);
    if tail_start == 0 {
        closed
    } else {
        format!("{head}\n{closed}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closes_partial_markdown() {
        let cases = [
            // Complete input is unchanged.
            ("Plain text.", "Plain text."),
            (
                "**bold** and *em* and `code`",
                "**bold** and *em* and `code`",
            ),
            (
                "[link](https://example.com) done",
                "[link](https://example.com) done",
            ),
            ("", ""),
            // Fences.
            ("```ts\nconst a = 1;", "```ts\nconst a = 1;\n```"),
            ("```ts\nconst a = 1;\n", "```ts\nconst a = 1;\n```"),
            ("~~~\nx", "~~~\nx\n~~~"),
            ("````md\n```\ninner", "````md\n```\ninner\n````"),
            ("```\na\n```\n\nThen **bo", "```\na\n```\n\nThen **bo**"),
            (
                "```py\nx = '**not emphasis'",
                "```py\nx = '**not emphasis'\n```",
            ),
            // Inline code.
            ("Run `npm i", "Run `npm i`"),
            ("Run `", "Run "),
            ("Use ``a ` b", "Use ``a ` b``"),
            ("`**` stays", "`**` stays"),
            // Emphasis.
            ("This is **bold", "This is **bold**"),
            ("This is *em", "This is *em*"),
            ("This is _em", "This is _em_"),
            ("This is __strong", "This is __strong__"),
            ("This is ~~gone", "This is ~~gone~~"),
            ("***both", "***both***"),
            ("**bold *and em", "**bold *and em***"),
            ("**bold** then *em", "**bold** then *em*"),
            ("trailing space **bold ", "trailing space **bold**"),
            ("Opener only **", "Opener only "),
            ("**bold `code", "**bold `code`**"),
            // Not emphasis.
            ("snake_case_name", "snake_case_name"),
            ("2 * 3 = 6", "2 * 3 = 6"),
            ("* list item", "* list item"),
            ("- item\n- another *em", "- item\n- another *em*"),
            ("escaped \\*star", "escaped \\*star"),
            // Links, images, citations.
            ("See [the docs](https://exa", "See the docs"),
            ("See [the **docs**](https://exa", "See the **docs**"),
            ("See [the docs", "See the docs"),
            ("See [the docs]", "See [the docs]"),
            ("![chart](https://img", ""),
            ("Revenue grew [1", "Revenue grew "),
            ("Revenue grew [1, 2", "Revenue grew "),
            ("Revenue grew [1]", "Revenue grew [1]"),
            ("- [ ] task", "- [ ] task"),
            // Only the last block is touched.
            ("Para *one\n\nPara **two", "Para *one\n\nPara **two**"),
            // Not ASCII.
            ("Größe **fett", "Größe **fett**"),
        ];
        for (input, expected) in cases {
            assert_eq!(close_markdown(input), expected, "{input:?}");
        }
    }

    #[test]
    fn closing_is_idempotent() {
        for s in ["This is **bold", "```ts\nx", "See [a](https://b", "Run `x"] {
            let once = close_markdown(s);
            assert_eq!(close_markdown(&once), once, "{s:?}");
        }
    }

    #[test]
    fn every_prefix_of_an_answer_closes() {
        let answer = "## Summary\n\nThe **2024 report** shows *growth* of `12%` [1]. See [the table](https://example.com/t?a=1_b) below.\n\n| Year | Growth |\n| --- | --- |\n| 2024 | 12% |\n\n```ts\nconst x = `template ${1}`;\n```\n\n1. First\n2. ~~Second~~ Third [2, 3]\n";
        let chars: Vec<char> = answer.chars().collect();
        for n in 0..=chars.len() {
            let prefix: String = chars[..n].iter().collect();
            let _ = close_markdown(&prefix);
            let _ = render_blocks(&close_markdown(&prefix), &MarkdownOptions::default());
        }
        assert_eq!(close_markdown(answer), answer);
    }

    fn html(source: &str) -> String {
        render_blocks(source, &MarkdownOptions::default())
            .iter()
            .flat_map(|block| block.segments.clone())
            .map(|segment| match segment {
                Segment::Html(html) => html,
                Segment::Code { code, .. } => format!("[code]{}", escape_html(&code)),
                Segment::Citation { index, .. } => format!("[cite {index:?}]"),
            })
            .collect()
    }

    #[test]
    fn script_tags_and_raw_html_are_text() {
        let out =
            html("<script>alert(1)</script>\n\nHi <img src=x onerror=alert(1)> there <b>bold</b>");
        assert!(!out.contains("<script"), "{out}");
        assert!(!out.contains("<img"), "{out}");
        assert!(!out.contains("<b>"), "{out}");
        assert!(
            out.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
            "{out}"
        );
        assert!(out.contains("&lt;img src=x onerror=alert(1)&gt;"), "{out}");
        let skipped = render_blocks(
            "a <b>b</b> c",
            &MarkdownOptions {
                skip_html: true,
                ..MarkdownOptions::default()
            },
        );
        assert_eq!(skipped[0].html(), Some("<p data-block=\"0\">a b c</p>"));
    }

    #[test]
    fn unsafe_links_are_dropped() {
        for url in [
            "javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            " javascript:alert(1)",
            "java\tscript:alert(1)",
            "jav&#x09;ascript:alert(1)",
            "data:text/html,<script>alert(1)</script>",
            "vbscript:x",
            "/admin/delete",
            "file:///etc/passwd",
        ] {
            let out = html(&format!("[click](<{url}>)"));
            assert!(!out.contains("href"), "{url} gave {out}");
            assert!(out.contains("click"), "{url} gave {out}");
        }
        let out = html("[docs](https://example.com/a?b=1&c=2)");
        assert!(
            out.contains("href=\"https://example.com/a?b=1&amp;c=2\""),
            "{out}"
        );
        assert!(out.contains("target=\"_blank\""), "{out}");
        assert!(out.contains("(opens in a new tab)"), "{out}");
        let mail = html("<ada@example.com>");
        assert!(mail.contains("href=\"mailto:ada@example.com\""), "{mail}");
        assert!(!mail.contains("_blank"), "{mail}");
        // An autolink to a script is not a link either.
        assert!(!html("<javascript:alert(1)>").contains("href"));
    }

    #[test]
    fn images_are_restricted_and_not_loaded_from_other_hosts() {
        // data: URLs other than pictures are refused.
        let out = html("![x](data:text/html;base64,PHNjcmlwdD4=)");
        assert!(!out.contains("data:text"), "{out}");
        assert!(out.contains("[Image: x]"), "{out}");
        let svg = html("![x](data:image/svg+xml;base64,AAAA)");
        assert!(!svg.contains("<img"), "{svg}");
        // A picture in the page loads.
        let png = html("![dot](data:image/png;base64,AAAA)");
        assert!(
            png.contains("<img src=\"data:image/png;base64,AAAA\" alt=\"dot\""),
            "{png}"
        );
        // Another host waits behind a button.
        let remote = html("![A chart](https://images.example.com/c.png)");
        assert!(!remote.contains("<img"), "{remote}");
        assert!(
            remote.contains("Load image from images.example.com"),
            "{remote}"
        );
        assert!(remote.contains("data-response-image=\"https://images.example.com/c.png\""));
        // Unless images are shown, or never loaded.
        let shown = render_blocks(
            "![A chart](https://images.example.com/c.png)",
            &MarkdownOptions {
                images: ResponseImages::Show,
                ..MarkdownOptions::default()
            },
        );
        assert!(
            shown[0]
                .key()
                .contains("<img src=\"https://images.example.com/c.png\"")
        );
        let alt = render_blocks(
            "![A chart](https://images.example.com/c.png) ![](https://x.example.com/y.png)",
            &MarkdownOptions {
                images: ResponseImages::Alt,
                ..MarkdownOptions::default()
            },
        );
        assert!(alt[0].key().contains("[Image: A chart]"));
        assert!(alt[0].key().contains("[Image]"));
        // A javascript: image is text.
        assert!(!html("![x](javascript:alert(1))").contains("javascript"));
    }

    #[test]
    fn headings_tables_and_tasks() {
        let out = html("# Title\n\n#### Deep");
        assert!(out.contains("<h3 data-block=\"0\""), "{out}");
        assert!(out.contains("data-level=\"1\">Title</h3>"), "{out}");
        assert!(out.contains("<h6 data-block=\"1\""), "{out}");
        let fixed = render_blocks(
            "# A\n\n### B",
            &MarkdownOptions {
                heading_level: Some(4),
                ..MarkdownOptions::default()
            },
        );
        assert!(fixed[0].key().starts_with("<h4"));
        assert!(fixed[1].key().starts_with("<h4"));

        let table = html(
            "| | Details |\n| --- | ---: |\n| Duration | 16 weeks<br>paid |\n| Split | <b>no</b> |",
        );
        assert!(
            table.contains("<td></td><th data-align=\"right\">Details</th>"),
            "{table}"
        );
        assert!(table.contains("16 weeks<br>paid"), "{table}");
        assert!(table.contains("&lt;b&gt;no&lt;/b&gt;"), "{table}");
        assert!(table.contains("tabindex=\"0\""), "{table}");
        let headless = html("| | |\n| --- | --- |\n| a | b |");
        assert!(!headless.contains("<thead>"), "{headless}");
        assert!(headless.contains("<tbody><tr><td>a</td>"), "{headless}");

        let tasks = html("- [x] done\n- [ ] open");
        assert!(
            tasks.contains("data-checked=\"\"></span><span class=\"sr-only\">Done: </span>done"),
            "{tasks}"
        );
        assert!(
            tasks.contains("<span class=\"sr-only\">To do: </span>open"),
            "{tasks}"
        );
        assert!(!tasks.contains("<input"), "{tasks}");
    }

    #[test]
    fn code_blocks_and_citations_are_segments() {
        let blocks = render_blocks(
            "Intro [1].\n\n```rust title\nfn main() {}\n```\n\n- item a[3] and [2, 1]\n- `[1]` in code and [9]",
            &MarkdownOptions {
                citations: 2,
                id: "r",
                ..MarkdownOptions::default()
            },
        );
        assert_eq!(blocks.len(), 3);
        assert_eq!(
            blocks[0].segments,
            [
                Segment::Html("<p data-block=\"0\">Intro ".to_owned()),
                Segment::Citation {
                    id: "r-cite-1".to_owned(),
                    index: vec![1]
                },
                Segment::Html(".</p>".to_owned()),
            ]
        );
        assert_eq!(
            blocks[1].segments,
            [Segment::Code {
                language: "rust".to_owned(),
                code: "fn main() {}\n".to_owned(),
                block: Some(1)
            }]
        );
        let list = blocks[2].key();
        assert!(list.contains("a[3] and "), "{list}");
        assert!(list.contains("r-cite-2[2, 1]"), "{list}");
        assert!(list.contains("<code class=\""), "{list}");
        assert!(list.contains("[1]</code>"), "{list}");
        assert!(list.contains("and [9]"), "{list}");
        // A marker in a link's text stays text.
        assert!(
            !render_blocks(
                "[see [1]](https://example.com)",
                &MarkdownOptions {
                    citations: 1,
                    ..MarkdownOptions::default()
                }
            )[0]
            .key()
            .contains("cite")
        );
    }

    #[test]
    fn markers() {
        let found = |text: &str| -> Vec<Vec<usize>> {
            citation_markers(text).into_iter().map(|(_, n)| n).collect()
        };
        assert_eq!(
            found("a [1] b [2,3] c [4 , 5]"),
            [vec![1], vec![2, 3], vec![4, 5]]
        );
        assert!(found("a[3] m[i][2] [3]int [1](x) [1234]").is_empty());
        assert_eq!(found("[1][2]"), [vec![1], vec![2]]);
    }

    #[test]
    fn stream_sends_only_changed_blocks() {
        let options = MarkdownOptions::default();
        let mut stream = ResponseStream::new();
        let first = stream.update(&render_blocks("One\n\nTwo", &options));
        assert_eq!(first.changed, [0, 1]);
        let second = stream.update(&render_blocks("One\n\nTwo and more", &options));
        assert_eq!(
            second,
            StreamUpdate {
                changed: vec![1],
                len: 2
            }
        );
        assert!(
            stream
                .update(&render_blocks("One\n\nTwo and more", &options))
                .is_empty(2)
        );

        // A line of dashes turns the paragraph above it into a heading.
        let mut stream = ResponseStream::new();
        stream.update(&render_blocks("One\n\nTwo\n", &options));
        let update = stream.update(&render_blocks("One\n\nTwo\n---\n", &options));
        assert_eq!(
            update,
            StreamUpdate {
                changed: vec![1],
                len: 2
            }
        );

        // A later reference definition changes an earlier block.
        let mut stream = ResponseStream::new();
        stream.update(&render_blocks("See [the docs][d].\n\nMore text.", &options));
        let update = stream.update(&render_blocks(
            "See [the docs][d].\n\nMore text.\n\n[d]: https://example.com",
            &options,
        ));
        assert!(update.changed.contains(&0));
        assert_eq!(update.len, 2);
    }

    #[test]
    fn an_open_fence_renders_as_code_while_streaming() {
        let blocks = render_blocks(
            &close_markdown("Intro\n\n```rust\nfn main() {"),
            &MarkdownOptions::default(),
        );
        assert_eq!(blocks.len(), 2);
        assert!(
            matches!(&blocks[1].segments[0], Segment::Code { language, code, .. } if language == "rust" && code == "fn main() {\n")
        );
    }
}
