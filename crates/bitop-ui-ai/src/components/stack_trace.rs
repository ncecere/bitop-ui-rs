//! A JavaScript error's stack, parsed: the error's type and message in a header that
//! opens to list the frames, with a copy button for the raw text.
//!
//! [`parse_stack_trace`] reads V8's format (Chrome, Node, Edge: `    at fn (file:1:2)`)
//! and Firefox's and Safari's (`fn@file:1:2`). Lines before the first frame are the
//! message, so a message of several lines survives. Each frame shows its function and
//! `file:line:column`; runs of internal frames (`node_modules`, `node:` built-ins,
//! native code) are dimmed and fold into their own "3 internal frames" disclosure.
//!
//! The header's trigger is a button with `aria-expanded` that shows and hides the
//! frames (the WAI-ARIA disclosure pattern), beside the copy button; the library script
//! does the showing and hiding. The frames are a focusable region, as they scroll, and
//! stay left to right in a right-to-left page.
//!
//! ```ignore
//! view! {
//!     stack_trace(id: "crash", trace: &error_stack)
//!     stack_trace(id: "crash", trace: &error_stack, open: true, frame_events: true,
//!         internal_frames: InternalFrames::Hide)
//! }
//! ```
//!
//! Differences from upstream: the trace is parsed on the server. `onFrameClick` is
//! `frame_events`, which makes the locations buttons that send `bitop:stack-frame`
//! with the frame in `detail`; `isInternalFrame` is a function pointer; there is no
//! `onOpenChange`, but the trigger sends `bitop:collapsible-toggle`. The parts of a
//! custom composition take the parsed trace and the IDs themselves.

use bitop_ui::components::{
    copy_button::copy_button,
    icon::{Icon, icon},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::stack_trace as css;

/// A test for internal frames, given a frame's path; see [`is_internal_path`].
pub type IsInternal = fn(&str) -> bool;

/// One frame of a stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackFrame {
    /// The line as written, trimmed.
    pub raw: String,
    pub function_name: Option<String>,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    /// In `node_modules`, a `node:` built-in, native code or the like.
    pub is_internal: bool,
}

impl StackFrame {
    /// `file.ts:12:5`: the path, line and column that are known.
    #[must_use]
    pub fn location(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(path) = &self.file_path {
            parts.push(path.clone());
        }
        if let Some(line) = self.line_number {
            parts.push(line.to_string());
        }
        if let Some(column) = self.column_number {
            parts.push(column.to_string());
        }
        parts.join(":")
    }
}

/// A parsed stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedStackTrace {
    /// `TypeError`, `RangeError [ERR_OUT_OF_RANGE]`; `None` when the first lines are not
    /// "Type: message".
    pub error_type: Option<String>,
    /// The message, or the whole header when it has no type.
    pub error_message: String,
    pub frames: Vec<StackFrame>,
    /// The text that was parsed.
    pub raw: String,
}

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// The default test for internal frames, given a frame's path: `node_modules`, `node:`
/// built-ins, `internal/` modules, `<anonymous>`, native code and webpack's bootstrap.
///
/// ```
/// use bitop_ui_ai::components::stack_trace::is_internal_path;
///
/// assert!(is_internal_path("node:internal/timers"));
/// assert!(is_internal_path("/app/node_modules/react/index.js"));
/// assert!(!is_internal_path("/app/src/main.ts"));
/// ```
#[must_use]
pub fn is_internal_path(location: &str) -> bool {
    let internal_module = location.match_indices("internal/").any(|(at, _)| {
        location[..at]
            .chars()
            .next_back()
            .is_none_or(|before| !is_word(before))
    });
    location.contains("node_modules")
        || location.starts_with("node:")
        || internal_module
        || location == "<anonymous>"
        || location == "native"
        || location.contains("[native code]")
        || location.starts_with("webpack/bootstrap")
}

/// `text` without a final `:digits`, and the number, if it ends so.
fn number_at_end(text: &str) -> Option<(&str, u32)> {
    let digits = text.len() - text.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 {
        return None;
    }
    let (rest, number) = text.split_at(text.len() - digits);
    Some((rest.strip_suffix(':')?, number.parse().ok()?))
}

/// `path`, `path:line` or `path:line:column`, split.
fn split_location(location: &str) -> (Option<String>, Option<u32>, Option<u32>) {
    let (path, line, column) = match number_at_end(location) {
        Some((rest, last)) => match number_at_end(rest) {
            Some((path, line)) => (path, Some(line), Some(last)),
            None => (rest, Some(last), None),
        },
        None => (location, None, None),
    };
    ((!path.is_empty()).then(|| path.to_owned()), line, column)
}

/// `fn (location)`: the function, and what is between the first parenthesis that
/// follows a space and the final one.
fn split_v8_body(body: &str) -> Option<(&str, &str)> {
    let inner = body.strip_suffix(')')?;
    body.char_indices()
        .filter(|(_, c)| c.is_whitespace())
        .find_map(|(at, _)| {
            let after = body[at..].trim_start();
            let open = body.len() - after.len();
            after
                .starts_with('(')
                .then(|| inner.get(open + 1..))
                .flatten()
                .map(|location| (&body[..at], location))
        })
}

/// In `eval at fn (file:1:2), <anonymous>:3:4`, the outer `file:1:2`: the first text
/// in parentheses that ends with a line and a column.
fn eval_origin(location: &str) -> Option<&str> {
    location.match_indices('(').find_map(|(open, _)| {
        let after = &location[open + 1..];
        after.match_indices(')').find_map(|(close, _)| {
            let inside = &after[..close];
            number_at_end(inside)
                .and_then(|(rest, _)| number_at_end(rest))
                .map(|_| inside)
        })
    })
}

/// Parses one line of a stack, or `None` when it is not a frame. `is_internal` is
/// given the frame's path (or its location, without one).
///
/// ```
/// use bitop_ui_ai::components::stack_trace::{is_internal_path, parse_stack_frame};
///
/// let frame = parse_stack_frame("    at render (src/app.tsx:12:5)", is_internal_path).unwrap();
/// assert_eq!(frame.function_name.as_deref(), Some("render"));
/// assert_eq!(frame.location(), "src/app.tsx:12:5");
/// ```
#[must_use]
pub fn parse_stack_frame(line: &str, is_internal: IsInternal) -> Option<StackFrame> {
    let raw = line.trim();
    let mut function_name = None;
    let mut location;
    let v8 = raw
        .strip_prefix("at")
        .filter(|rest| rest.starts_with(char::is_whitespace))
        .map(str::trim_start);
    if let Some(body) = v8 {
        if let Some((name, inner)) = split_v8_body(body) {
            function_name = Some(name.to_owned());
            location = inner;
            if location.starts_with("eval at")
                && let Some(origin) = eval_origin(location)
            {
                location = origin;
            }
        } else {
            location = body;
        }
    } else {
        let (name, rest) = raw.split_once('@')?;
        if name.contains(char::is_whitespace) || rest.is_empty() {
            return None;
        }
        let has_number = rest
            .match_indices(':')
            .any(|(at, _)| rest[at + 1..].starts_with(|c: char| c.is_ascii_digit()));
        if !has_number {
            return None;
        }
        function_name = (!name.is_empty()).then(|| name.to_owned());
        location = rest;
    }
    let (file_path, line_number, column_number) = split_location(location);
    let is_internal = is_internal(file_path.as_deref().unwrap_or(location));
    Some(StackFrame {
        raw: raw.to_owned(),
        function_name,
        file_path,
        line_number,
        column_number,
        is_internal,
    })
}

/// `TypeError: message` split into the type and the message.
fn parse_error_header(header: &str) -> Option<(String, String)> {
    fn from(text: &str) -> Option<(String, String)> {
        let run = text.len()
            - text
                .trim_start_matches(|c: char| is_word(c) || c == '$' || c == '.')
                .len();
        let word = &text[..run];
        let named = word.ends_with("Error") || word.ends_with("Exception");
        let bare = word == "Error" || word == "Exception";
        if !named || !(bare || word.starts_with(|c: char| c.is_ascii_uppercase())) {
            return None;
        }
        let mut end = run;
        // An optional code in brackets: "RangeError [ERR_OUT_OF_RANGE]".
        let spaced = text[run..].trim_start();
        if let Some(code) = spaced.strip_prefix('[')
            && let Some(close) = code.find(']')
            && close > 0
            && code[..close].chars().all(|c| is_word(c) || c == '-')
        {
            end = text.len() - code.len() + close + 1;
        }
        let rest = &text[end..];
        if rest.is_empty() {
            return Some((text[..end].to_owned(), String::new()));
        }
        let message = rest.strip_prefix(':')?.trim_start();
        Some((text[..end].to_owned(), message.to_owned()))
    }
    let uncaught = header
        .strip_prefix("Uncaught")
        .filter(|rest| rest.starts_with(char::is_whitespace))
        .map(str::trim_start);
    uncaught.and_then(from).or_else(|| from(header))
}

/// Parses a stack (`error.stack`) into the error's header and its frames.
///
/// ```
/// use bitop_ui_ai::components::stack_trace::{is_internal_path, parse_stack_trace};
///
/// let trace = parse_stack_trace("TypeError: x is undefined\n    at f (a.js:1:2)", is_internal_path);
/// assert_eq!(trace.error_type.as_deref(), Some("TypeError"));
/// assert_eq!(trace.error_message, "x is undefined");
/// assert_eq!(trace.frames.len(), 1);
/// ```
#[must_use]
pub fn parse_stack_trace(trace: &str, is_internal: IsInternal) -> ParsedStackTrace {
    let mut header: Vec<&str> = Vec::new();
    let mut frames = Vec::new();
    for line in trace.split('\n') {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.trim().is_empty() {
            continue;
        }
        if let Some(frame) = parse_stack_frame(line, is_internal) {
            frames.push(frame);
        } else if frames.is_empty() {
            header.push(line.trim());
        }
    }
    let first = header.join("\n");
    let (error_type, error_message) = match parse_error_header(&first) {
        Some((kind, message)) => (Some(kind), message),
        None => (None, first),
    };
    ParsedStackTrace {
        error_type,
        error_message,
        frames,
        raw: trace.to_owned(),
    }
}

/// What the frame list does with internal frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InternalFrames {
    /// Folds each run of them into a disclosure.
    #[default]
    Collapse,
    /// Lists them, dimmed.
    Show,
    /// Leaves them out.
    Hide,
}

/// The strings of a stack trace.
#[derive(Clone, Copy, Debug)]
pub struct StackTraceLabels<'a> {
    /// The trigger's text when the trace has neither a type nor a message.
    pub fallback: &'a str,
    /// Before each frame.
    pub at: &'a str,
    /// Said when there are no frames.
    pub empty: &'a str,
    /// A run of one internal frame, and of more (`{n}`).
    pub internal_one: &'a str,
    pub internal_other: &'a str,
    /// What the copy button copies, as "Copy {label}".
    pub copy: &'a str,
    pub copy_label: &'a str,
    pub copied_label: &'a str,
}

impl StackTraceLabels<'_> {
    /// The English strings.
    pub const ENGLISH: StackTraceLabels<'static> = StackTraceLabels {
        fallback: "Error",
        at: "at ",
        empty: "No stack frames",
        internal_one: "{n} internal frame",
        internal_other: "{n} internal frames",
        copy: "stack trace",
        copy_label: "Copy {label}",
        copied_label: "Copied {label}",
    };
}

impl Default for StackTraceLabels<'_> {
    fn default() -> Self {
        StackTraceLabels::ENGLISH
    }
}

/// A stack trace, with the default composition: a header with the trigger and the copy
/// button, and the frames.
///
/// - `id` names the parts: the frames are `{id}-frames`, the error text `{id}-error`.
/// - `trace` is the stack text, usually `error.stack` (or `"{name}: {message}\n{stack}"`
///   from Firefox).
/// - `open` shows the frames at first.
/// - `frame_events` makes the locations buttons that send `bitop:stack-frame`.
/// - `is_internal` decides which frames are internal, given the path.
/// - `internal_frames`, `max_height` and `labels` are those of [`stack_trace_frames`].
/// - `custom` leaves the default composition out; `child` is then the content, built
///   from the parts with a [`ParsedStackTrace`].
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn stack_trace(
    cx: &Cx,
    id: &str,
    trace: &str,
    #[default] open: bool,
    #[default] frame_events: bool,
    #[default(is_internal_path)] is_internal: IsInternal,
    #[default] internal_frames: InternalFrames,
    #[default] max_height: &str,
    #[default] labels: StackTraceLabels<'_>,
    #[default] custom: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let parsed = parse_stack_trace(trace, is_internal);
    Ok(Child::new(view! {
        <div
            id=(id)
            data-stack-trace=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            if !custom {
                stack_trace_header(
                    stack_trace_trigger(
                        id: id,
                        trace: &parsed,
                        open: open,
                        labels: labels
                    )
                    stack_trace_actions(
                        stack_trace_copy_button(trace: trace, labels: labels)
                    )
                )
                stack_trace_content(
                    id: id,
                    open: open,
                    stack_trace_frames(
                        id: id,
                        trace: &parsed,
                        frame_events: frame_events,
                        internal_frames: internal_frames,
                        max_height: max_height,
                        labels: labels
                    )
                )
            }
            (child)
        </div>
    }))
}

/// The header row: the trigger and the actions.
#[component]
pub async fn stack_trace_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The button that shows and hides the frames: an alert icon, the error's type and
/// message, and a chevron. `id` is the root's; `open` the panel's first state.
#[component]
pub async fn stack_trace_trigger(
    cx: &Cx,
    id: &str,
    trace: &ParsedStackTrace,
    #[default] open: bool,
    #[default] labels: StackTraceLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let message = if trace.error_message.is_empty() && trace.error_type.is_none() {
        labels.fallback.to_owned()
    } else {
        trace.error_message.clone()
    };
    let kind = trace.error_type.as_ref().map(|kind| {
        if trace.error_message.is_empty() {
            kind.clone()
        } else {
            format!("{kind}:")
        }
    });
    Ok(view! {
        <button
            type="button"
            aria-expanded=(if open { "true" } else { "false" })
            aria-controls=(format!("{id}-frames"))
            data-collapsible=""
            data-open=(open.then_some(""))
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            icon(
                name: Icon::TriangleAlert,
                attrs: attributes! { class=(css::ALERT_ICON) }
            )
            <span id=(format!("{id}-error")) dir="ltr" class=(css::ERROR)>
                if let Some(kind) = kind {
                    <span class=(css::ERROR_TYPE)>(kind)</span>
                    " "
                }
                <span class=(css::ERROR_MESSAGE)>(message)</span>
            </span>
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
        </button>
    })
}

/// The row of buttons beside the trigger.
#[component]
pub async fn stack_trace_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// Copies `trace`, the raw text, named "Copy stack trace".
#[component]
pub async fn stack_trace_copy_button(
    cx: &Cx,
    trace: &str,
    #[default] labels: StackTraceLabels<'_>,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        copy_button(
            value: trace,
            label: labels.copy,
            copy_label: labels.copy_label,
            copied_label: labels.copied_label,
            attrs: attrs
        )
    })
}

/// The panel the trigger shows and hides. `id` is the root's; `open` its first state.
#[component]
pub async fn stack_trace_content(
    cx: &Cx,
    id: &str,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(format!("{id}-frames"))
            hidden=(!open)
            class=(class!(css::PANEL, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A run of frames: internal ones folded together, or the others.
struct FrameRun<'a> {
    internal: bool,
    frames: Vec<&'a StackFrame>,
}

/// One frame's parts as text.
struct FrameText {
    internal: bool,
    function: Option<String>,
    location: Option<String>,
    rest: Option<String>,
    file: String,
    line: String,
    column: String,
}

fn frame_text(frame: &StackFrame) -> FrameText {
    let rest = (frame.file_path.is_none() && frame.function_name.is_none()).then(|| {
        frame
            .raw
            .strip_prefix("at")
            .filter(|rest| rest.starts_with(char::is_whitespace))
            .map_or(frame.raw.as_str(), str::trim_start)
            .to_owned()
    });
    FrameText {
        internal: frame.is_internal,
        function: frame.function_name.clone(),
        location: frame.file_path.is_some().then(|| frame.location()),
        rest,
        file: frame.file_path.clone().unwrap_or_default(),
        line: frame.line_number.map(|n| n.to_string()).unwrap_or_default(),
        column: frame
            .column_number
            .map(|n| n.to_string())
            .unwrap_or_default(),
    }
}

/// The frames: a focusable region named by the error text (`{id}-error`), as it
/// scrolls past `max_height` (a CSS length, `24rem` by default).
///
/// - `internal_frames` folds, shows or hides internal frames.
/// - `frame_events` makes each location a button that sends `bitop:stack-frame`, with
///   `file`, `line`, `column` and `raw` in `detail`, from the root.
#[component]
pub async fn stack_trace_frames(
    cx: &Cx,
    id: &str,
    trace: &ParsedStackTrace,
    #[default] frame_events: bool,
    #[default] internal_frames: InternalFrames,
    #[default] max_height: &str,
    #[default] labels: StackTraceLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let mut runs: Vec<FrameRun<'_>> = Vec::new();
    for frame in &trace.frames {
        if internal_frames == InternalFrames::Hide && frame.is_internal {
            continue;
        }
        let internal = internal_frames == InternalFrames::Collapse && frame.is_internal;
        match runs.last_mut() {
            Some(run) if run.internal == internal => run.frames.push(frame),
            _ => runs.push(FrameRun {
                internal,
                frames: vec![frame],
            }),
        }
    }
    let empty = runs.is_empty();
    let groups: Vec<(bool, String, Vec<FrameText>)> = runs
        .into_iter()
        .map(|run| {
            let n = run.frames.len();
            let summary = if n == 1 {
                labels.internal_one
            } else {
                labels.internal_other
            }
            .replace("{n}", &n.to_string());
            (
                run.internal,
                summary,
                run.frames.into_iter().map(frame_text).collect(),
            )
        })
        .collect();
    let style = bitop_ui::merge_style(
        &mut attrs,
        &if max_height.is_empty() {
            String::new()
        } else {
            format!("--stack-max-height: {max_height}")
        },
    );
    let at = labels.at;
    let empty_text = labels.empty;
    Ok(Child::new(view! {
        <div
            role="region"
            tabindex="0"
            dir="ltr"
            aria-labelledby=(format!("{id}-error"))
            data-frame-events=(frame_events.then_some(""))
            style=(style)
            class=(class!(css::FRAMES, attrs.remove("class")))
            (attrs)
        >
            if empty {
                <p class=(css::EMPTY)>(empty_text)</p>
            } else {
                <ol class=(css::LIST)>
                    for (internal, summary, frames) in groups {
                        if internal {
                            <li class=(css::INTERNAL_GROUP)>
                                <details class=(css::INTERNAL_DETAILS)>
                                    <summary class=(css::INTERNAL_TRIGGER)>
                                        icon(
                                            name: Icon::ChevronRight,
                                            attrs: attributes! { class=(css::INTERNAL_CHEVRON) }
                                        )
                                        (summary)
                                    </summary>
                                    <ol class=(css::LIST)>
                                        for frame in frames {
                                            frame_item(frame: frame, at: at, events: frame_events)
                                        }
                                    </ol>
                                </details>
                            </li>
                        } else {
                            for frame in frames {
                                frame_item(frame: frame, at: at, events: frame_events)
                            }
                        }
                    }
                </ol>
            }
        </div>
    }))
}

/// One frame: "at fn (file:1:2)".
#[component]
async fn frame_item(cx: &Cx, frame: FrameText, at: &str, events: bool) -> Result<impl View> {
    Ok(view! {
        <li class=(css::FRAME) data-internal=(frame.internal.then_some(""))>
            <span class=(css::AT)>(at)</span>
            if let Some(function) = &frame.function {
                <span class=(css::FN)>(function)</span>
                " "
            }
            if let Some(location) = &frame.location {
                <span class=(css::PAREN)>"("</span>
                if events {
                    <button
                        type="button"
                        data-stack-frame=""
                        data-file=(&frame.file)
                        data-line=(&frame.line)
                        data-column=(&frame.column)
                        class=(css::LOCATION)
                    >
                        (location)
                    </button>
                } else {
                    <span class=(css::LOCATION_TEXT)>(location)</span>
                }
                <span class=(css::PAREN)>")"</span>
            } else if let Some(rest) = &frame.rest {
                <span>(rest)</span>
            }
        </li>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(line: &str) -> Option<StackFrame> {
        parse_stack_frame(line, is_internal_path)
    }

    #[test]
    fn v8_frames() {
        let f = frame("    at Object.render (/app/src/view.tsx:12:5)").unwrap();
        assert_eq!(f.raw, "at Object.render (/app/src/view.tsx:12:5)");
        assert_eq!(f.function_name.as_deref(), Some("Object.render"));
        assert_eq!(f.file_path.as_deref(), Some("/app/src/view.tsx"));
        assert_eq!((f.line_number, f.column_number), (Some(12), Some(5)));
        assert!(!f.is_internal);

        // No function: the whole body is the location.
        let f = frame("at /app/index.js:3:1").unwrap();
        assert_eq!(f.function_name, None);
        assert_eq!(f.location(), "/app/index.js:3:1");

        // Built-ins and dependencies are internal.
        let f =
            frame("at processTicksAndRejections (node:internal/process/task_queues:95:5)").unwrap();
        assert!(f.is_internal);
        assert_eq!(
            f.file_path.as_deref(),
            Some("node:internal/process/task_queues")
        );
        assert!(
            frame("at h (/app/node_modules/x/y.js:1:1)")
                .unwrap()
                .is_internal
        );
        assert!(frame("at Array.map (<anonymous>)").unwrap().is_internal);
        assert!(frame("at new Promise (native)").unwrap().is_internal);

        // A URL with a port keeps its port in the path.
        let f = frame("at load (http://localhost:3000/app.js:10:20)").unwrap();
        assert_eq!(f.file_path.as_deref(), Some("http://localhost:3000/app.js"));
        assert_eq!(f.line_number, Some(10));

        // async frames and names with spaces.
        let f = frame("at async Promise.all (index 0)").unwrap();
        assert_eq!(f.function_name.as_deref(), Some("async Promise.all"));
        assert_eq!(f.file_path.as_deref(), Some("index 0"));

        // eval frames point at the outer file.
        let f = frame("at eval (eval at compile (/app/a.js:5:9), <anonymous>:1:1)").unwrap();
        assert_eq!(f.function_name.as_deref(), Some("eval"));
        assert_eq!(f.location(), "/app/a.js:5:9");
    }

    #[test]
    fn gecko_frames() {
        let f = frame("render@http://localhost/app.js:12:5").unwrap();
        assert_eq!(f.function_name.as_deref(), Some("render"));
        assert_eq!(f.file_path.as_deref(), Some("http://localhost/app.js"));
        assert_eq!((f.line_number, f.column_number), (Some(12), Some(5)));
        let f = frame("@debugger eval code:1:1").unwrap();
        assert_eq!(f.function_name, None);
        assert_eq!(f.file_path.as_deref(), Some("debugger eval code"));
        // Not frames: no line number, a space before the @, or nothing after it.
        assert_eq!(frame("user@example.com"), None);
        assert_eq!(frame("see this@file.js:1"), None);
        assert_eq!(frame("fn@"), None);
        assert_eq!(frame("Error: boom"), None);
        assert_eq!(frame("attention: not a frame"), None);
    }

    #[test]
    fn locations() {
        assert_eq!(
            split_location("a.js:1:2"),
            (Some("a.js".into()), Some(1), Some(2))
        );
        assert_eq!(
            split_location("a.js:7"),
            (Some("a.js".into()), Some(7), None)
        );
        assert_eq!(split_location("a.js"), (Some("a.js".into()), None, None));
        assert_eq!(
            split_location("a:1:2:3"),
            (Some("a:1".into()), Some(2), Some(3))
        );
        assert_eq!(split_location(":5"), (None, Some(5), None));
        assert_eq!(split_location("a.js:"), (Some("a.js:".into()), None, None));
    }

    #[test]
    fn internal_paths() {
        assert!(is_internal_path("internal/modules/cjs/loader"));
        assert!(is_internal_path("/x/internal/y"));
        assert!(!is_internal_path("/x/myinternal/y"));
        assert!(is_internal_path("[native code]"));
        assert!(is_internal_path("webpack/bootstrap 1a2b"));
        assert!(!is_internal_path("src/native.js"));
    }

    #[test]
    fn headers() {
        let parse = |text| parse_stack_trace(text, is_internal_path);
        let t = parse(
            "TypeError: Cannot read properties of undefined (reading 'id')\n    at a (x.js:1:1)",
        );
        assert_eq!(t.error_type.as_deref(), Some("TypeError"));
        assert_eq!(
            t.error_message,
            "Cannot read properties of undefined (reading 'id')"
        );

        let t = parse("Uncaught RangeError [ERR_OUT_OF_RANGE]: bad offset\nat a (x.js:1:1)");
        assert_eq!(
            t.error_type.as_deref(),
            Some("RangeError [ERR_OUT_OF_RANGE]")
        );
        assert_eq!(t.error_message, "bad offset");

        // A message of several lines is kept, and lines after the frames are dropped.
        let t =
            parse("Error: first\nsecond line\r\n  at a (x.js:1:1)\nnot a frame\n  at b (y.js:2:2)");
        assert_eq!(t.error_type.as_deref(), Some("Error"));
        assert_eq!(t.error_message, "first\nsecond line");
        assert_eq!(t.frames.len(), 2);

        let t = parse("Error\n  at a (x.js:1:1)");
        assert_eq!(
            (t.error_type.as_deref(), t.error_message.as_str()),
            (Some("Error"), "")
        );
        let t = parse("NullPointerException: x");
        assert_eq!(t.error_type.as_deref(), Some("NullPointerException"));
        let t = parse("my.pkg.CustomError: y");
        assert_eq!(t.error_type, None);
        assert_eq!(t.error_message, "my.pkg.CustomError: y");
        let t = parse("Something went wrong\n  at a (x.js:1:1)");
        assert_eq!(t.error_type, None);
        assert_eq!(t.error_message, "Something went wrong");
        let t = parse("ErrorBoundary caught it");
        assert_eq!(t.error_type, None);
        let t = parse("");
        assert_eq!(
            (t.error_type, t.error_message.as_str(), t.frames.len()),
            (None, "", 0)
        );
    }
}
