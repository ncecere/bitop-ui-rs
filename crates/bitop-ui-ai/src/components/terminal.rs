//! Command output: a header with a title, a running status, copy and clear, and the
//! output with ANSI colours, which follows new output to the bottom.
//!
//! [`parse_ansi`] is a small SGR parser: bold, dim, italic, underline, inverse, the 16
//! standard and bright colours (`38;5;n` for `n` under 16) and resets. Colours map to
//! the theme's text tokens (`--color-danger-text` for red), not literal colours, so the
//! output keeps its contrast in light and dark themes. Truecolour (`38;2`) and other
//! escape sequences (cursor moves, window titles) are dropped. A bare `\r` rewrites the
//! line, as progress bars expect.
//!
//! The output is a focusable region named by the title, as it scrolls, and stays left
//! to right in a right-to-left page. It is not a live region, which streamed output
//! would flood; the status says "Running" and `aria-busy` is set while the root has
//! `data-streaming`.
//!
//! The library script keeps the view at the newest output while the reader is at the
//! bottom (within 24 pixels), as an app streams spans into the output element (render
//! them with [`terminal_chunk`]) or replaces it. The copy button copies the output's
//! text as it is then, without the escape codes. The clear button sends
//! `bitop:terminal-clear` from the root and, unless that is cancelled, empties the
//! output.
//!
//! ```ignore
//! view! {
//!     terminal(id: "build", output: &log, title: "cargo build", streaming: running,
//!         clearable: true)
//! }
//! ```
//!
//! Differences from upstream: the output is parsed on the server; streamed output is
//! HTML the app appends ([`terminal_chunk`]), whose styles start fresh at each chunk
//! and whose `\r` rewrites only lines within it. `onClear` is the clear event and
//! `clearable` shows the button. Parts of a custom composition take the root's `id`.

use bitop_ui::components::{
    copy_button::copy_button,
    icon::{Icon, icon},
    loader::{LoaderSize, loader},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::terminal as css;

/// One of the eight ANSI colours, or gray (bright black). Bright colours share their
/// base colour's token. Rendered as `data-fg` and `data-bg`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
}

impl AnsiColor {
    const BASE: [Self; 8] = [
        Self::Black,
        Self::Red,
        Self::Green,
        Self::Yellow,
        Self::Blue,
        Self::Magenta,
        Self::Cyan,
        Self::White,
    ];

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::Red => "red",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Magenta => "magenta",
            Self::Cyan => "cyan",
            Self::White => "white",
            Self::Gray => "gray",
        }
    }

    fn base(index: u32) -> Option<Self> {
        Self::BASE.get(usize::try_from(index).ok()?).copied()
    }

    /// Bright black is the usual gray; the other bright colours are their base colour.
    fn bright(index: u32) -> Option<Self> {
        if index == 0 {
            Some(Self::Gray)
        } else {
            Self::base(index)
        }
    }
}

/// The style of a run of text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnsiStyle {
    pub fg: Option<AnsiColor>,
    pub bg: Option<AnsiColor>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
}

/// A run of text in one style.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnsiSpan {
    pub style: AnsiStyle,
    pub text: String,
}

/// Applies `\r` overwrites and turns `\r\n` into `\n`.
fn collapse_carriage_returns(text: &str) -> String {
    text.replace("\r\n", "\n")
        .split('\n')
        .map(|line| line.rsplit_once('\r').map_or(line, |(_, last)| last))
        .collect::<Vec<_>>()
        .join("\n")
}

/// What an escape sequence at the start of a text is.
enum Escape<'a> {
    /// `ESC [ params final`, with its parameters and final byte.
    Csi(&'a str, char),
    /// Another sequence, which is dropped.
    Other,
}

/// The escape sequence at the start of `text`, which starts with ESC, and its length;
/// `None` when the ESC starts no sequence and stays as text.
fn escape_at(text: &str) -> Option<(Escape<'_>, usize)> {
    let rest = &text[1..];
    // CSI: ESC [ then digits, ";" and "?", then a final byte from @ to ~.
    if let Some(body) = rest.strip_prefix('[') {
        let params = body.len()
            - body
                .trim_start_matches(|c: char| c.is_ascii_digit() || c == ';' || c == '?')
                .len();
        if let Some(last) = body[params..].chars().next()
            && ('@'..='~').contains(&last)
        {
            return Some((Escape::Csi(&body[..params], last), 2 + params + 1));
        }
    }
    // OSC: ESC ] then anything but BEL and ESC, ended by BEL or ESC \.
    if let Some(body) = rest.strip_prefix(']') {
        let end = body.find(['\u{7}', '\u{1b}']);
        if let Some(end) = end {
            if body[end..].starts_with('\u{7}') {
                return Some((Escape::Other, 2 + end + 1));
            }
            if body[end..].starts_with("\u{1b}\\") {
                return Some((Escape::Other, 2 + end + 2));
            }
        }
    }
    // Two bytes: ESC and one of @ to Z, \ to _.
    match rest.chars().next() {
        Some(c) if ('@'..='Z').contains(&c) || ('\\'..='_').contains(&c) => {
            Some((Escape::Other, 1 + c.len_utf8()))
        }
        _ => None,
    }
}

fn apply_sgr(style: AnsiStyle, params: &str) -> AnsiStyle {
    let codes: Vec<u32> = if params.is_empty() {
        vec![0]
    } else {
        params.split(';').map(|p| p.parse().unwrap_or(0)).collect()
    };
    let mut next = style;
    let mut i = 0;
    while i < codes.len() {
        let code = codes[i];
        match code {
            0 => next = AnsiStyle::default(),
            1 => next.bold = true,
            2 => next.dim = true,
            3 => next.italic = true,
            4 => next.underline = true,
            7 => next.inverse = true,
            22 => {
                next.bold = false;
                next.dim = false;
            }
            23 => next.italic = false,
            24 => next.underline = false,
            27 => next.inverse = false,
            30..=37 => next.fg = AnsiColor::base(code - 30),
            39 => next.fg = None,
            40..=47 => next.bg = AnsiColor::base(code - 40),
            49 => next.bg = None,
            90..=97 => next.fg = AnsiColor::bright(code - 90),
            100..=107 => next.bg = AnsiColor::bright(code - 100),
            38 | 48 => match codes.get(i + 1) {
                Some(5) => {
                    let color = codes.get(i + 2).and_then(|&n| match n {
                        0..=7 => AnsiColor::base(n),
                        8..=15 => AnsiColor::bright(n - 8),
                        _ => None,
                    });
                    if let Some(color) = color {
                        if code == 38 {
                            next.fg = Some(color);
                        } else {
                            next.bg = Some(color);
                        }
                    }
                    i += 2;
                }
                // Truecolour: no token to map it to.
                Some(2) => i += 4,
                _ => {}
            },
            _ => {}
        }
        i += 1;
    }
    next
}

/// Splits text with ANSI escape codes into styled runs. Runs of the same style are
/// joined.
///
/// ```
/// use bitop_ui_ai::components::terminal::{AnsiColor, parse_ansi};
///
/// let spans = parse_ansi("\u{1b}[1;31merror\u{1b}[0m: failed");
/// assert_eq!(spans[0].text, "error");
/// assert_eq!(spans[0].style.fg, Some(AnsiColor::Red));
/// assert!(spans[0].style.bold);
/// assert_eq!(spans[1].text, ": failed");
/// ```
#[must_use]
pub fn parse_ansi(input: &str) -> Vec<AnsiSpan> {
    let text = collapse_carriage_returns(input);
    let mut spans: Vec<AnsiSpan> = Vec::new();
    let mut style = AnsiStyle::default();
    let mut push = |chunk: &str, style: AnsiStyle| {
        if chunk.is_empty() {
            return;
        }
        match spans.last_mut() {
            Some(last) if last.style == style => last.text.push_str(chunk),
            _ => spans.push(AnsiSpan {
                style,
                text: chunk.to_owned(),
            }),
        }
    };
    let mut rest = text.as_str();
    let mut plain = String::new();
    while let Some(at) = rest.find('\u{1b}') {
        plain.push_str(&rest[..at]);
        match escape_at(&rest[at..]) {
            Some((escape, length)) => {
                push(&plain, style);
                plain.clear();
                // Private-mode sequences such as ESC[?25l (hide the cursor) are not SGR.
                if let Escape::Csi(params, 'm') = escape
                    && !params.starts_with('?')
                {
                    style = apply_sgr(style, params);
                }
                rest = &rest[at + length..];
            }
            None => {
                plain.push('\u{1b}');
                rest = &rest[at + 1..];
            }
        }
    }
    plain.push_str(rest);
    push(&plain, style);
    spans
}

/// The text without ANSI escape codes, with `\r` overwrites applied.
///
/// ```
/// use bitop_ui_ai::components::terminal::strip_ansi;
///
/// assert_eq!(strip_ansi("\u{1b}[32mok\u{1b}[0m\r\n"), "ok\n");
/// ```
#[must_use]
pub fn strip_ansi(input: &str) -> String {
    parse_ansi(input)
        .into_iter()
        .map(|span| span.text)
        .collect()
}

/// The strings of a terminal.
#[derive(Clone, Copy, Debug)]
pub struct TerminalLabels<'a> {
    /// The status while streaming.
    pub running: &'a str,
    /// Shown when there is no output.
    pub placeholder: &'a str,
    /// The clear button's name.
    pub clear: &'a str,
    /// What the copy button copies, as "Copy {label}".
    pub copy: &'a str,
    pub copy_label: &'a str,
    pub copied_label: &'a str,
}

impl TerminalLabels<'_> {
    /// The English strings.
    pub const ENGLISH: TerminalLabels<'static> = TerminalLabels {
        running: "Running",
        placeholder: "No output",
        clear: "Clear output",
        copy: "output",
        copy_label: "Copy {label}",
        copied_label: "Copied {label}",
    };
}

impl Default for TerminalLabels<'_> {
    fn default() -> Self {
        TerminalLabels::ENGLISH
    }
}

/// A terminal, with the default composition: a header with the title, the status, the
/// copy button and, with `clearable`, the clear button; then the output.
///
/// - `id` names the parts: the title is `{id}-title`, the output `{id}-output`.
/// - `output` is the raw output, escape codes and all.
/// - `streaming` says output is still arriving; an app that streams sets and removes
///   `data-streaming` on the root itself.
/// - `auto_scroll` keeps the view at the newest output while the reader is at the
///   bottom.
/// - `title` names the terminal; `max_height` caps the output (`24rem` by default).
/// - `custom` leaves the default composition out; `child` is then the content.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn terminal(
    cx: &Cx,
    id: &str,
    #[default] output: &str,
    #[default] streaming: bool,
    #[default(true)] auto_scroll: bool,
    #[default] clearable: bool,
    #[default("Terminal")] title: &str,
    #[default] max_height: &str,
    #[default] labels: TerminalLabels<'_>,
    #[default] custom: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(Child::new(view! {
        <div
            id=(id)
            data-terminal=""
            data-streaming=(streaming.then_some(""))
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            if !custom {
                terminal_header(
                    terminal_title(id: id, title: title)
                    terminal_status(label: labels.running)
                    terminal_actions(
                        terminal_copy_button(id: id, labels: labels)
                        if clearable {
                            terminal_clear_button(label: labels.clear)
                        }
                    )
                )
                terminal_content(
                    id: id,
                    output: output,
                    streaming: streaming,
                    auto_scroll: auto_scroll,
                    max_height: max_height,
                    placeholder: labels.placeholder
                )
            }
            (child)
        </div>
    }))
}

/// The header row.
#[component]
pub async fn terminal_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The title, which names the output: `{id}-title`. `icon_name` replaces the terminal
/// icon.
#[component]
pub async fn terminal_title(
    cx: &Cx,
    id: &str,
    #[default("Terminal")] title: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <p
            id=(format!("{id}-title"))
            class=(class!(css::TITLE, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::TITLE_ICON)>
                icon(name: icon_name.unwrap_or(Icon::SquareTerminal))
            </span>
            (title)
        </p>
    })
}

/// "Running" (`label`) with a loader, shown while the root has `data-streaming`.
#[component]
pub async fn terminal_status(
    cx: &Cx,
    #[default("Running")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-terminal-status=""
            class=(class!(css::STATUS, attrs.remove("class")))
            (attrs)
        >
            loader(size: LoaderSize::Sm)
            (label)
        </span>
    })
}

/// The row of buttons at the end of the header.
#[component]
pub async fn terminal_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// Copies the output's text when pressed, named "Copy output". `id` is the root's.
#[component]
pub async fn terminal_copy_button(
    cx: &Cx,
    id: &str,
    #[default] labels: TerminalLabels<'_>,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    let output = format!("{id}-output");
    Ok(view! {
        copy_button(
            target: &output,
            label: labels.copy,
            copy_label: labels.copy_label,
            copied_label: labels.copied_label,
            attrs: attrs
        )
    })
}

/// The clear button, named `label`, with an eraser.
#[component]
pub async fn terminal_clear_button(
    cx: &Cx,
    #[default("Clear output")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            data-terminal-clear=""
            data-variant="ghost"
            data-size="icon"
            class=(class!(
                bitop_ui::css::button::ROOT,
                css::ACTION,
                attrs.remove("class"),
            ))
            (attrs)
        >
            icon(name: Icon::Eraser)
        </button>
    })
}

/// The output: a focusable region named by the title (or by `label`), as it scrolls
/// past `max_height`.
///
/// - `id` is the root's: the output element is `{id}-output`.
/// - `streaming` adds `aria-busy` and the cursor; the script follows the root's
///   `data-streaming` after that.
/// - `placeholder` is shown while there is no output and nothing is streaming.
#[component]
pub async fn terminal_content(
    cx: &Cx,
    id: &str,
    #[default] output: &str,
    #[default] streaming: bool,
    #[default(true)] auto_scroll: bool,
    #[default] max_height: &str,
    #[default] label: &str,
    #[default("No output")] placeholder: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let style = bitop_ui::merge_style(
        &mut attrs,
        &if max_height.is_empty() {
            String::new()
        } else {
            format!("--terminal-max-height: {max_height}")
        },
    );
    let named = !label.is_empty();
    Ok(view! {
        <div
            role="region"
            tabindex="0"
            aria-label=(named.then_some(label))
            aria-labelledby=((!named).then(|| format!("{id}-title")))
            aria-busy=(streaming.then_some("true"))
            data-terminal-content=""
            data-auto-scroll=(auto_scroll.then_some(""))
            style=(style)
            class=(class!(css::CONTENT, attrs.remove("class")))
            (attrs)
        >
            <pre dir="ltr" class=(css::PRE)>
                <code
                    id=(format!("{id}-output"))
                    data-terminal-output=""
                    class=(css::OUTPUT)
                >
                    terminal_chunk(output: output)
                </code>
                <span class=(css::PLACEHOLDER)>(placeholder)</span>
                <span aria-hidden="true" class=(css::CURSOR)></span>
            </pre>
        </div>
    })
}

/// Output as styled spans, for the output element: the whole output, or a chunk that
/// an app appends while it streams.
#[component]
pub async fn terminal_chunk(cx: &Cx, output: &str) -> Result<impl View> {
    let spans = parse_ansi(output);
    Ok(view! {
        for span in spans {
            <span
                data-fg=(span.style.fg.map(AnsiColor::as_str))
                data-bg=(span.style.bg.map(AnsiColor::as_str))
                data-bold=(span.style.bold.then_some(""))
                data-dim=(span.style.dim.then_some(""))
                data-italic=(span.style.italic.then_some(""))
                data-underline=(span.style.underline.then_some(""))
                data-inverse=(span.style.inverse.then_some(""))
                class=(css::SPAN)
            >
                (span.text)
            </span>
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn styled(input: &str) -> Vec<(String, AnsiStyle)> {
        parse_ansi(input)
            .into_iter()
            .map(|s| (s.text, s.style))
            .collect()
    }

    #[test]
    fn plain() {
        assert_eq!(
            styled("hello\nworld"),
            vec![("hello\nworld".into(), AnsiStyle::default())]
        );
        assert!(parse_ansi("").is_empty());
    }

    #[test]
    fn colours_and_resets() {
        let red = AnsiStyle {
            fg: Some(AnsiColor::Red),
            ..AnsiStyle::default()
        };
        assert_eq!(
            styled("\u{1b}[31mred\u{1b}[39m plain"),
            vec![("red".into(), red), (" plain".into(), AnsiStyle::default())]
        );
        // Bright black is gray; other bright colours are their base; backgrounds too.
        let spans = parse_ansi("\u{1b}[90ma\u{1b}[94mb\u{1b}[42mc\u{1b}[103md");
        assert_eq!(spans[0].style.fg, Some(AnsiColor::Gray));
        assert_eq!(spans[1].style.fg, Some(AnsiColor::Blue));
        assert_eq!(spans[2].style.bg, Some(AnsiColor::Green));
        assert_eq!(spans[3].style.bg, Some(AnsiColor::Yellow));
        // ESC[m and ESC[0m reset everything.
        let spans = parse_ansi("\u{1b}[1;4;7mx\u{1b}[my");
        assert!(spans[0].style.bold && spans[0].style.underline && spans[0].style.inverse);
        assert_eq!(spans[1].style, AnsiStyle::default());
    }

    #[test]
    fn attributes() {
        let spans = parse_ansi("\u{1b}[1;2;3ma\u{1b}[22mb\u{1b}[23;24;27mc");
        assert!(spans[0].style.bold && spans[0].style.dim && spans[0].style.italic);
        assert!(!spans[1].style.bold && !spans[1].style.dim && spans[1].style.italic);
        assert_eq!(spans[2].style, AnsiStyle::default());
    }

    #[test]
    fn extended_colours() {
        let spans = parse_ansi("\u{1b}[38;5;1ma\u{1b}[38;5;8mb\u{1b}[48;5;12mc\u{1b}[38;5;200md");
        assert_eq!(spans[0].style.fg, Some(AnsiColor::Red));
        assert_eq!(spans[1].style.fg, Some(AnsiColor::Gray));
        assert_eq!(spans[2].style.bg, Some(AnsiColor::Blue));
        // 256 colours beyond 16 keep the colour before; truecolour is skipped whole.
        assert_eq!(spans[2].style.fg, Some(AnsiColor::Gray));
        let spans = parse_ansi("\u{1b}[38;2;255;0;0;1mx");
        assert_eq!(spans[0].style.fg, None);
        assert!(spans[0].style.bold);
    }

    #[test]
    fn other_sequences() {
        // Cursor moves, private modes and window titles are dropped.
        assert_eq!(strip_ansi("a\u{1b}[2Kb\u{1b}[?25lc"), "abc");
        assert_eq!(strip_ansi("\u{1b}]0;title\u{7}x"), "x");
        assert_eq!(
            strip_ansi("\u{1b}]8;;http://x\u{1b}\\link\u{1b}]8;;\u{1b}\\"),
            "link"
        );
        assert_eq!(strip_ansi("\u{1b}Mup"), "up");
        // A lone ESC that starts no sequence stays.
        assert_eq!(strip_ansi("a\u{1b}[\u{1}b"), "a\u{1b}[\u{1}b");
        // A private mode is not SGR.
        assert_eq!(parse_ansi("\u{1b}[?1mx")[0].style, AnsiStyle::default());
    }

    #[test]
    fn carriage_returns() {
        assert_eq!(strip_ansi("10%\r50%\r100%\ndone"), "100%\ndone");
        assert_eq!(strip_ansi("a\r\nb\r\n"), "a\nb\n");
        assert_eq!(strip_ansi("line\r"), "");
    }

    #[test]
    fn joins_runs() {
        let spans = parse_ansi("\u{1b}[31ma\u{1b}[31mb");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].text, "ab");
    }
}
