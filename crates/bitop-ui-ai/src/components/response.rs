//! An assistant's Markdown answer, rendered on the server and safe to render again
//! with every streamed token.
//!
//! - Tables, task lists, strikethrough and autolinks (GitHub's Markdown) are
//!   supported. Code blocks are [`code_block`]s; `[n]` markers become
//!   [`inline_citation`]s for the `citations` given.
//! - With `streaming`, [`close_markdown`] closes what is half written (an open code
//!   fence, `**bold`, `[link](http…`), so the answer renders as it will once complete.
//! - Model output is untrusted. Raw HTML is shown as text (or dropped with
//!   `skip_html`); a bare `<br>` in a table cell is the one exception. Links and images
//!   keep only safe URLs ([`safe_url`]), and by default an image from another host is a
//!   "Load image from example.com" button that the library script swaps for the image
//!   when pressed: an answer steered by a prompt-injected document must not make the
//!   browser fetch any URL it likes.
//! - Task-list items say "Done:" or "To do:" instead of an unlabelled checkbox; links
//!   to other sites open in a new tab and say so; wide tables scroll and can be
//!   focused; headings are demoted (`#` is an `<h3>`) so that an answer never competes
//!   with the page's own headings.
//!
//! While an answer streams, an app can send only the blocks that changed:
//! [`ResponseStream`] compares the blocks of each update with what was sent,
//! [`response_block`] renders one, and `bitopUi.patchResponse(root, patches, len)` in
//! the browser puts them in place, where `patches` is a list of `{ index, html }`.
//!
//! ```ignore
//! let sources = [CitationSource::new("Leave policy").href("https://hr.example.com")];
//! view! {
//!     response(id: "answer-7", source: &text, streaming: true, citations: &sources)
//! }
//! ```
//!
//! Differences from upstream: the Markdown is rendered on the server with
//! `pulldown-cmark`, so there are no `components` or `remarkPlugins`. Relative and
//! same-site URLs are refused, not allowed: the stricter rule of the code this is
//! adapted from. There is no `highlight` function, as [`code_block`] has none, and no
//! `LazyResponse`, as nothing is sent to the browser to render Markdown.
//!
//! [`code_block`]: super::code_block::code_block
//! [`inline_citation`]: super::inline_citation::inline_citation
//! [`close_markdown`]: crate::markdown::close_markdown
//! [`safe_url`]: crate::markdown::safe_url
//! [`ResponseStream`]: crate::markdown::ResponseStream

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, Unescaped, View, attributes, class, component, element_id, view},
};

use super::inline_citation::inline_citation;
use super::{code_block::code_block, inline_citation::CitationSource};
use crate::css::response as css;
use crate::markdown::{
    Block, MarkdownOptions, ResponseImages, Segment, close_markdown, render_blocks,
};

/// Every string a [`response`] shows or reads out, with English defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResponseStrings<'a> {
    /// Read after a link that opens in a new tab.
    pub new_tab_label: &'a str,
    /// An image's load button, with `{host}`.
    pub load_image_label: &'a str,
    /// The same, when the host cannot be read.
    pub load_image_unknown_label: &'a str,
    /// An image that is not loaded, with `{alt}`.
    pub image_alt_label: &'a str,
    /// An image without alt text that is not loaded.
    pub image_label: &'a str,
    /// Read before a done task.
    pub done_label: &'a str,
    /// Read before an open task.
    pub todo_label: &'a str,
    /// A code block's header without a language.
    pub code_label: &'a str,
    /// What a code block's copy button copies, with `{language}`; and without one.
    pub copy_code_label: &'a str,
    pub copy_code_label_plain: &'a str,
    /// The copy button's name, with `{label}`, and once copied.
    pub copy_label: &'a str,
    pub copied_label: &'a str,
    /// Announced once copied, with `{label}`.
    pub copied_status: &'a str,
    /// A citation's name: one source, several, and how many more.
    pub source_label: &'a str,
    pub sources_label: &'a str,
    pub more_label: &'a str,
    /// A citation card's pager.
    pub previous_source_label: &'a str,
    pub next_source_label: &'a str,
    pub page_label: &'a str,
}

impl Default for ResponseStrings<'_> {
    fn default() -> Self {
        Self {
            new_tab_label: "(opens in a new tab)",
            load_image_label: "Load image from {host}",
            load_image_unknown_label: "Load image from another site",
            image_alt_label: "[Image: {alt}]",
            image_label: "[Image]",
            done_label: "Done: ",
            todo_label: "To do: ",
            code_label: "Code",
            copy_code_label: "{language} code",
            copy_code_label_plain: "code",
            copy_label: "Copy {label}",
            copied_label: "Copied {label}",
            copied_status: "Copied {label} to clipboard",
            source_label: "Source",
            sources_label: "Sources",
            more_label: " and {count} more",
            previous_source_label: "Previous source",
            next_source_label: "Next source",
            page_label: "{page} of {count}",
        }
    }
}

/// The blocks of a response, as [`response`] renders them: for an app that streams,
/// to compare with [`ResponseStream`](crate::markdown::ResponseStream) and render
/// with [`response_block`]. The arguments are [`response`]'s.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn response_blocks(
    source: &str,
    id: &str,
    streaming: bool,
    citations: usize,
    heading_offset: u8,
    heading_level: Option<u8>,
    skip_html: bool,
    images: ResponseImages,
    strings: &ResponseStrings<'_>,
) -> Vec<Block> {
    let closed;
    let source = if streaming {
        closed = close_markdown(source);
        closed.as_str()
    } else {
        source
    };
    render_blocks(
        source,
        &MarkdownOptions {
            heading_offset,
            heading_level,
            skip_html,
            images,
            citations,
            id,
            new_tab_label: strings.new_tab_label,
            load_image_label: strings.load_image_label,
            load_image_unknown_label: strings.load_image_unknown_label,
            image_alt_label: strings.image_alt_label,
            image_label: strings.image_label,
            done_label: strings.done_label,
            todo_label: strings.todo_label,
        },
    )
}

/// A response.
///
/// - `source` is the Markdown, partial while `streaming`.
/// - `id` goes on the root, and citation cards' IDs are made from it; each response on
///   a page needs its own when it has citations. Without it one is made.
/// - `citations` are the sources of `[n]` markers: `[1]` is the first. A marker with a
///   number that has no source stays text.
/// - `heading_offset` is added to Markdown heading levels; `heading_level` renders
///   every heading at one level instead, for quoted text whose outline is not the
///   page's.
/// - `skip_html` drops raw HTML instead of showing it as text.
/// - `images` says how images load.
/// - `strings` are the words it shows and reads out.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn response(
    cx: &Cx,
    source: &str,
    #[default] id: &str,
    #[default] streaming: bool,
    #[default] citations: &[CitationSource<'_>],
    #[default(2)] heading_offset: u8,
    #[default]
    #[into]
    heading_level: Option<u8>,
    #[default] skip_html: bool,
    #[default] images: ResponseImages,
    #[default] strings: ResponseStrings<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let id = if id.is_empty() {
        element_id(cx)
    } else {
        id.to_owned()
    };
    let blocks = response_blocks(
        source,
        &id,
        streaming,
        citations.len(),
        heading_offset,
        heading_level,
        skip_html,
        images,
        &strings,
    );
    Ok(view! {
        <div
            id=(&id)
            data-response=""
            data-streaming=(streaming.then_some(""))
            class=(class!(css::RESPONSE, attrs.remove("class")))
            (attrs)
        >
            for block in &blocks {
                response_block(block: block, citations: citations, strings: strings)
            }
        </div>
    })
}

/// One block of a response, for an app that sends only the blocks that changed.
/// `citations` and `strings` must be the response's.
#[component]
pub async fn response_block(
    cx: &Cx,
    block: &Block,
    #[default] citations: &[CitationSource<'_>],
    #[default] strings: ResponseStrings<'_>,
) -> Result<impl View> {
    let parts: Vec<Part<'_>> = block
        .segments
        .iter()
        .map(|segment| match segment {
            Segment::Html(html) => Part::Html(html.clone()),
            Segment::Code {
                language,
                code,
                block,
            } => Part::Code {
                language: language.clone(),
                code: code.clone(),
                block: *block,
            },
            Segment::Citation { id, index } => Part::Citation {
                id: id.clone(),
                index: index.clone(),
                sources: index
                    .iter()
                    .filter_map(|n| n.checked_sub(1).and_then(|n| citations.get(n)).copied())
                    .collect(),
            },
        })
        .collect();
    // Boxed: the nested type of this view is deep enough that a page holding several
    // overflows the compiler's recursion limit when it checks that the page is `Send`.
    Ok(Child::new(view! {
        for part in &parts {
            match part {
                Part::Html(html) => {
                    (Unescaped::new_unchecked(html.clone()))
                }
                Part::Code { language, code, block } => {
                    code_block(
                        code: code,
                        language: language,
                        fallback_label: strings.code_label,
                        copy_label: strings.copy_code_label,
                        copy_label_plain: strings.copy_code_label_plain,
                        copy_button_label: strings.copy_label,
                        copied_button_label: strings.copied_label,
                        copied_status: strings.copied_status,
                        attrs: attributes! { class=(css::CODE_BLOCK) data-block=(*block) }
                    )
                }
                Part::Citation { id, index, sources } => {
                    inline_citation(
                        id: id,
                        index: index,
                        sources: sources,
                        source_label: strings.source_label,
                        sources_label: strings.sources_label,
                        more_label: strings.more_label,
                        previous_label: strings.previous_source_label,
                        next_label: strings.next_source_label,
                        page_label: strings.page_label,
                        new_tab_label: strings.new_tab_label
                    )
                }
            }
        }
    }))
}

enum Part<'a> {
    Html(String),
    Code {
        language: String,
        code: String,
        block: Option<usize>,
    },
    Citation {
        id: String,
        index: Vec<usize>,
        sources: Vec<CitationSource<'a>>,
    },
}
