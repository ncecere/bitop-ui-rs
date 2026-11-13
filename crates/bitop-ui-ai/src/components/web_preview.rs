//! A preview of a generated page or app in a sandboxed `<iframe>`, under a bar like a
//! browser's (back, forward, reload, an address field and open in a new tab), with a
//! console panel for its logs.
//!
//! The frame is sandboxed by default ([`DEFAULT_SANDBOX`], `allow-scripts
//! allow-forms`: no same-origin access, no navigation of the page, no popups); widen
//! `sandbox` only for content you trust. It may show a URL or `srcdoc`.
//!
//! The library script runs the bar. Enter in the address field goes to the address,
//! with `https://` added to a bare host (`http://` for `localhost`) and `javascript:`
//! refused, as [`normalize_url`] does on the server. Back and forward walk the
//! addresses visited through the bar (a cross-origin frame's own history cannot be
//! read), and reload loads the address again. While the frame loads, a spinner covers
//! it and `aria-busy` is set. Each move sends `bitop:web-preview-navigate` from the root
//! with the address in `detail.url`.
//!
//! ```ignore
//! view! {
//!     web_preview(url: "/preview",
//!         web_preview_navigation(
//!             web_preview_back() web_preview_forward() web_preview_reload()
//!             web_preview_url(url: "/preview")
//!             web_preview_open(url: "/preview")
//!         )
//!         web_preview_body(title: "Preview of the landing page", url: "/preview")
//!         web_preview_console(logs: &logs)
//!     )
//! }
//! ```
//!
//! Differences from upstream: the parts take the address themselves, as there is no
//! context, and there is no `useWebPreview`, `onUrlChange` (the event above) or
//! controlled `url`. A load that starts before the script runs shows no spinner. The
//! console's logs are rendered on the server; a timestamp is the base set's
//! [`time`](bitop_ui::components::time::time), which the script writes in the
//! visitor's time zone.

use bitop_ui::{
    components::{
        icon::{Icon, icon},
        input_group::{InputGroupSize, input_group, input_group_addon, input_group_input},
        spinner::{SpinnerSize, spinner},
        time::{TimeFormat, time},
    },
    css::{button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::web_preview as css;

/// The frame's default sandbox: scripts and forms, nothing else.
pub const DEFAULT_SANDBOX: &str = "allow-scripts allow-forms";

fn starts_with_ignore_case(text: &str, prefix: &str) -> bool {
    text.get(..prefix.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
}

/// An address as the bar goes to it: `https://` added to a bare host (`http://` for
/// `localhost`, `127.0.0.1` and `[::1]`), absolute and relative URLs kept, and
/// `javascript:` URLs refused (an empty string).
///
/// ```
/// use bitop_ui_ai::components::web_preview::normalize_url;
///
/// assert_eq!(normalize_url("example.com"), "https://example.com");
/// assert_eq!(normalize_url("localhost:3000/app"), "http://localhost:3000/app");
/// assert_eq!(normalize_url("/preview?page=2"), "/preview?page=2");
/// assert_eq!(normalize_url("javascript:alert(1)"), "");
/// ```
#[must_use]
pub fn normalize_url(input: &str) -> String {
    let value = input.trim();
    if value.is_empty() || starts_with_ignore_case(value, "javascript:") {
        return String::new();
    }
    // A scheme and "//": letters, then letters, digits, "+", "." or "-".
    let has_scheme = value.find("://").is_some_and(|at| {
        let scheme = &value[..at];
        scheme.starts_with(|c: char| c.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
    });
    if has_scheme
        || ["about:", "data:", "blob:"]
            .iter()
            .any(|scheme| starts_with_ignore_case(value, scheme))
        || value.starts_with(['/', '.', '?', '#'])
    {
        return value.to_owned();
    }
    let local = ["localhost", "127.0.0.1", "[::1]"].iter().any(|host| {
        starts_with_ignore_case(value, host) && {
            let rest = &value[host.len()..];
            let rest = rest.strip_prefix(':').map_or(Some(rest), |port| {
                let digits =
                    port.len() - port.trim_start_matches(|c: char| c.is_ascii_digit()).len();
                (digits > 0).then(|| &port[digits..])
            });
            rest.is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
        }
    });
    if local {
        format!("http://{value}")
    } else {
        format!("https://{value}")
    }
}

/// The root. `url` is the first address, for the script's history; give the same to
/// the parts that show it.
#[component]
pub async fn web_preview(
    cx: &Cx,
    #[default] url: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let url = normalize_url(url);
    Ok(view! {
        <div
            data-web-preview=""
            data-url=(url)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The bar: a group of controls named `label`.
#[component]
pub async fn web_preview_navigation(
    cx: &Cx,
    #[default("Preview navigation")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            class=(class!(css::NAVIGATION, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// An icon button for the bar, named `label`, with `icon_name`.
///
/// - `action` is what the script does when it is pressed: `back`, `forward` or
///   `reload`; empty for a button of the app's own.
/// - `disabled` disables it.
/// - `tooltip` is an ID for a tooltip that says `label` (or `tooltip_text`).
#[component]
pub async fn web_preview_navigation_button(
    cx: &Cx,
    label: &str,
    icon_name: Icon,
    #[default] action: &str,
    #[default] disabled: bool,
    #[default] tooltip: &str,
    #[default] tooltip_text: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
    let tip = if tooltip_text.is_empty() {
        label
    } else {
        tooltip_text
    };
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            data-web-preview-action=((!action.is_empty()).then_some(action))
            disabled=(disabled)
            data-variant="ghost"
            data-size="icon"
            class=(class!(button_css::ROOT, css::BUTTON, attrs.remove("class")))
            (attrs)
        >
            icon(name: icon_name)
        </button>
        if has_tooltip {
            <span
                id=(tooltip)
                role="tooltip"
                popover="manual"
                class=(tooltip_css::POPUP)
            >
                (tip)
            </span>
        }
    })
}

/// Back: goes to the address before, through the bar. Disabled until there is one.
#[component]
pub async fn web_preview_back(
    cx: &Cx,
    #[default("Back")] label: &str,
    #[default] tooltip: &str,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        web_preview_navigation_button(
            label: label,
            icon_name: Icon::ArrowLeft,
            action: "back",
            disabled: true,
            tooltip: tooltip,
            attrs: attrs
        )
    })
}

/// Forward: goes to the address after, once the reader has gone back.
#[component]
pub async fn web_preview_forward(
    cx: &Cx,
    #[default("Forward")] label: &str,
    #[default] tooltip: &str,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        web_preview_navigation_button(
            label: label,
            icon_name: Icon::ArrowRight,
            action: "forward",
            disabled: true,
            tooltip: tooltip,
            attrs: attrs
        )
    })
}

/// Reload: loads the address again. `url` is the first address; without one it is
/// disabled until there is.
#[component]
pub async fn web_preview_reload(
    cx: &Cx,
    #[default] url: &str,
    #[default("Reload")] label: &str,
    #[default] tooltip: &str,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        web_preview_navigation_button(
            label: label,
            icon_name: Icon::RotateCw,
            action: "reload",
            disabled: url.trim().is_empty(),
            tooltip: tooltip,
            attrs: attrs
        )
    })
}

/// The address field, named `label`, in a form: Enter goes to what it holds. `url` is
/// the first address. The address stays left to right.
#[component]
pub async fn web_preview_url(
    cx: &Cx,
    #[default] url: &str,
    #[default("Address")] label: &str,
    #[default("Enter a URL")] placeholder: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let url = normalize_url(url);
    Ok(view! {
        <form
            data-web-preview-form=""
            class=(class!(css::URL_FORM, attrs.remove("class")))
            (attrs)
        >
            input_group(
                size: InputGroupSize::Sm,
                attrs: attributes! { class=(css::URL) },
                input_group_addon(icon(name: Icon::Globe))
                input_group_input(
                    attrs: attributes! {
                        type="text"
                        inputmode="url"
                        autocomplete="off"
                        spellcheck="false"
                        dir="ltr"
                        aria-label=(label)
                        placeholder=(placeholder)
                        value=(url)
                        data-web-preview-address=""
                    }
                )
            )
        </form>
    })
}

/// Opens the address in a new tab: a link that looks like the bar's buttons, hidden
/// while there is no address. `url` is the first address.
#[component]
pub async fn web_preview_open(
    cx: &Cx,
    #[default] url: &str,
    #[default("Open in new tab")] label: &str,
    #[default] tooltip: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let url = normalize_url(url);
    let has_tooltip = !tooltip.is_empty();
    Ok(view! {
        <a
            href=((!url.is_empty()).then_some(&url))
            hidden=(url.is_empty())
            target="_blank"
            rel="noopener noreferrer"
            aria-label=(label)
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            data-web-preview-open=""
            data-variant="ghost"
            data-size="icon"
            class=(class!(button_css::ROOT, css::BUTTON, attrs.remove("class")))
            (attrs)
        >
            icon(name: Icon::ExternalLink)
        </a>
        if has_tooltip {
            <span
                id=(tooltip)
                role="tooltip"
                popover="manual"
                class=(tooltip_css::POPUP)
            >
                (label)
            </span>
        }
    })
}

/// The frame.
///
/// - `title` says what the frame shows, for assistive technology.
/// - `url` is the first address; or `srcdoc` is HTML to show.
/// - `sandbox` is the frame's sandbox, [`DEFAULT_SANDBOX`] by default.
/// - `empty_text` is shown while there is neither; `loading_label` names the spinner.
/// - `frame_attrs` are forwarded to the `<iframe>`, `attrs` to the wrapper.
#[component]
pub async fn web_preview_body(
    cx: &Cx,
    title: &str,
    #[default] url: &str,
    #[default] srcdoc: &str,
    #[default(DEFAULT_SANDBOX)] sandbox: &str,
    #[default("Enter an address to load the preview.")] empty_text: &str,
    #[default("Loading preview")] loading_label: &str,
    #[default] frame_attrs: Attributes,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let url = normalize_url(url);
    let has_content = !url.is_empty() || !srcdoc.is_empty();
    Ok(view! {
        <div
            data-web-preview-body=""
            class=(class!(css::BODY, attrs.remove("class")))
            (attrs)
        >
            <iframe
                title=(title)
                src=((!url.is_empty()).then_some(&url))
                srcdoc=((!srcdoc.is_empty()).then_some(srcdoc))
                sandbox=(sandbox)
                referrerpolicy="no-referrer"
                hidden=(!has_content)
                class=(css::FRAME)
                (frame_attrs)
            ></iframe>
            <div data-web-preview-loading="" hidden="" class=(css::LOADING)>
                spinner(size: SpinnerSize::Lg, label: loading_label)
            </div>
            <p data-web-preview-empty="" hidden=(has_content) class=(css::EMPTY)>
                (empty_text)
            </p>
        </div>
    })
}

/// How serious a console message is. Rendered as `data-level`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogLevel {
    #[default]
    Log,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

/// One console message.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebPreviewLog {
    pub level: LogLevel,
    pub message: String,
    /// When it was logged: a Unix time in seconds.
    pub timestamp: Option<i64>,
}

impl WebPreviewLog {
    /// A message at `level`.
    #[must_use]
    pub fn new(level: LogLevel, message: &str) -> Self {
        Self {
            level,
            message: message.to_owned(),
            timestamp: None,
        }
    }

    /// Sets when it was logged.
    #[must_use]
    pub fn at(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
}

/// The strings of the console.
#[derive(Clone, Copy, Debug)]
pub struct ConsoleLabels<'a> {
    pub log: &'a str,
    pub info: &'a str,
    pub warn: &'a str,
    pub error: &'a str,
    /// The counts after the trigger's label, with `{n}`.
    pub errors_one: &'a str,
    pub errors_other: &'a str,
    pub warnings_one: &'a str,
    pub warnings_other: &'a str,
    /// Between the counts.
    pub separator: &'a str,
}

impl ConsoleLabels<'_> {
    /// The English strings.
    pub const ENGLISH: ConsoleLabels<'static> = ConsoleLabels {
        log: "Log",
        info: "Info",
        warn: "Warning",
        error: "Error",
        errors_one: "{n} error",
        errors_other: "{n} errors",
        warnings_one: "{n} warning",
        warnings_other: "{n} warnings",
        separator: ", ",
    };

    fn level(&self, level: LogLevel) -> &str {
        match level {
            LogLevel::Log => self.log,
            LogLevel::Info => self.info,
            LogLevel::Warn => self.warn,
            LogLevel::Error => self.error,
        }
    }

    /// "1 error, 2 warnings", leaving out what there is none of.
    #[must_use]
    pub fn counts(&self, logs: &[WebPreviewLog]) -> String {
        let count = |level| logs.iter().filter(|log| log.level == level).count();
        let say = |n: usize, one: &str, other: &str| {
            (n > 0).then(|| (if n == 1 { one } else { other }).replace("{n}", &n.to_string()))
        };
        [
            say(count(LogLevel::Error), self.errors_one, self.errors_other),
            say(
                count(LogLevel::Warn),
                self.warnings_one,
                self.warnings_other,
            ),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(self.separator)
    }
}

impl Default for ConsoleLabels<'_> {
    fn default() -> Self {
        ConsoleLabels::ENGLISH
    }
}

/// The console: a `<details>` whose trigger says `label` and how many errors and
/// warnings there are, over a focusable region (`region_label`) of the logs. Each
/// message's level is spelled out, never shown by colour alone.
///
/// - `open` opens it; `empty_text` is shown without logs.
/// - `child` follows the logs.
#[component]
pub async fn web_preview_console(
    cx: &Cx,
    #[default] logs: &[WebPreviewLog],
    #[default] open: bool,
    #[default("Console")] label: &str,
    #[default("Console output")] region_label: &str,
    #[default("No console output.")] empty_text: &str,
    #[default] labels: ConsoleLabels<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let counts = labels.counts(logs);
    let rows: Vec<(LogLevel, String, String, Option<i64>)> = logs
        .iter()
        .map(|log| {
            (
                log.level,
                labels.level(log.level).to_owned(),
                log.message.clone(),
                log.timestamp,
            )
        })
        .collect();
    let empty = rows.is_empty();
    Ok(Child::new(view! {
        <details
            open=(open)
            data-web-preview-console=""
            class=(class!(css::CONSOLE, attrs.remove("class")))
            (attrs)
        >
            <summary class=(css::CONSOLE_TRIGGER)>
                icon(
                    name: Icon::ChevronDown,
                    attrs: attributes! { class=(css::CHEVRON) }
                )
                <span>(label)</span>
                if !counts.is_empty() {
                    " "
                    <span class=(css::COUNTS)>(counts)</span>
                }
            </summary>
            <div
                role="region"
                tabindex="0"
                aria-label=(region_label)
                class=(css::CONSOLE_BODY)
            >
                if empty {
                    <p class=(css::EMPTY)>(empty_text)</p>
                } else {
                    <ol dir="ltr" class=(css::LOGS)>
                        for (level, word, message, timestamp) in rows {
                            <li class=(css::LOG) data-level=(level.as_str())>
                                <span class=(css::LEVEL)>(word)</span>
                                if let Some(at) = timestamp {
                                    time(
                                        timestamp: at,
                                        format: TimeFormat::Time,
                                        attrs: attributes! { class=(css::TIMESTAMP) }
                                    )
                                }
                                <span class=(css::MESSAGE)>(message)</span>
                            </li>
                        }
                    </ol>
                }
                (child)
            </div>
        </details>
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls() {
        assert_eq!(normalize_url("  example.com/a "), "https://example.com/a");
        assert_eq!(normalize_url("http://example.com"), "http://example.com");
        assert_eq!(normalize_url("HTTPS://Example.com"), "HTTPS://Example.com");
        assert_eq!(normalize_url("git+ssh://host/repo"), "git+ssh://host/repo");
        assert_eq!(normalize_url("about:blank"), "about:blank");
        assert_eq!(normalize_url("data:text/html,hi"), "data:text/html,hi");
        assert_eq!(normalize_url("./page"), "./page");
        assert_eq!(normalize_url("#top"), "#top");
        assert_eq!(normalize_url("?q=1"), "?q=1");
        assert_eq!(normalize_url("localhost"), "http://localhost");
        assert_eq!(
            normalize_url("127.0.0.1:3918/preview"),
            "http://127.0.0.1:3918/preview"
        );
        assert_eq!(normalize_url("[::1]:8080"), "http://[::1]:8080");
        assert_eq!(
            normalize_url("localhost.example.com"),
            "https://localhost.example.com"
        );
        assert_eq!(normalize_url("localhost:abc"), "https://localhost:abc");
        assert_eq!(normalize_url("JavaScript:void(0)"), "");
        assert_eq!(normalize_url("   "), "");
        assert_eq!(normalize_url("1http://x"), "https://1http://x");
    }

    #[test]
    fn counts() {
        let logs = vec![
            WebPreviewLog::new(LogLevel::Error, "a"),
            WebPreviewLog::new(LogLevel::Warn, "b"),
            WebPreviewLog::new(LogLevel::Warn, "c"),
            WebPreviewLog::new(LogLevel::Log, "d"),
        ];
        assert_eq!(ConsoleLabels::ENGLISH.counts(&logs), "1 error, 2 warnings");
        assert_eq!(ConsoleLabels::ENGLISH.counts(&logs[3..]), "");
    }
}
