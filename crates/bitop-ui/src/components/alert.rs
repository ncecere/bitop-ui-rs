//! An inline message about the page or a section of it.
//!
//! A danger alert has `role="alert"` and is announced at once; the other tones have
//! `role="status"` and are announced politely. Render the alert when the message
//! appears, so that it is announced.
//!
//! ```ignore
//! view! {
//!     alert(tone: AlertTone::Warning, title: "Quota almost used",
//!         "You have used 90% of this month's tokens."
//!     )
//! }
//! ```
//!
//! [`error_alert`] is an alert for a failed request. [`describe_error`] maps an HTTP
//! status to a plain title, a message and a tone, as upstream's `describeError` does:
//!
//! ```ignore
//! view! { error_alert(status: 404, retry_href: "/projects") }
//! ```
//!
//! Differences from upstream: upstream's `ErrorAlert` reads the status and the
//! message from any thrown value and retries through a click handler. Here the caller
//! passes the status and the message, and retry is a link that requests the page
//! again.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::{alert as css, button as button_css};

/// The meaning of an [`alert`]. Rendered as `data-tone`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertTone {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertTone {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    fn icon(self) -> Icon {
        match self {
            Self::Info => Icon::Info,
            Self::Success => Icon::CircleCheck,
            Self::Warning => Icon::TriangleAlert,
            Self::Danger => Icon::CircleAlert,
        }
    }
}

/// An alert. `child` is the message under the optional `title`.
///
/// `dismissible` adds a button that removes the alert from the page; the library script
/// handles it. `actions` is a slot for buttons or links beside the message:
/// `alert(actions: view! { button("Retry") }, "Could not save.")`.
#[component]
pub async fn alert(
    cx: &Cx,
    #[default] tone: AlertTone,
    #[default] title: &str,
    #[default] hide_icon: bool,
    #[default] dismissible: bool,
    #[default("Dismiss")] dismiss_label: &str,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role=(if tone == AlertTone::Danger { "alert" } else { "status" })
            data-tone=(tone.as_str())
            data-alert=""
            class=(class!(css::ALERT, attrs.remove("class")))
            (attrs)
        >
            if !hide_icon {
                icon(name: tone.icon(), attrs: attributes! { class=(css::ICON) })
            }
            <div class=(css::BODY)>
                if !title.is_empty() {
                    <p class=(css::TITLE)>(title)</p>
                }
                <div class=(css::CONTENT)>(child)</div>
            </div>
            <div class=(css::ACTIONS)>(actions)</div>
            if dismissible {
                <button
                    type="button"
                    class=(css::DISMISS)
                    aria-label=(dismiss_label)
                    data-alert-dismiss=""
                >
                    icon(name: Icon::X)
                </button>
            }
        </div>
    })
}

/// How an error is presented: what [`error_alert`] shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorDescription {
    /// A title in plain language, for a status that has one.
    pub title: Option<&'static str>,
    /// The error's own message, or a default when it has none.
    pub message: String,
    /// Danger, except for 429, which is a warning.
    pub tone: AlertTone,
    /// The HTTP status the description was made from.
    pub status: Option<u16>,
}

/// Maps an error to a title, a message and a tone.
///
/// An error with an HTTP status gets a title in plain language: 401, 403, 404, 408
/// and 504, 429 (as a warning) and the other 5xx. The error's own `message` is kept;
/// when it is empty a default for the status is used. Everything else is a danger
/// alert with the message only, or "Something went wrong.".
///
/// ```
/// use bitop_ui::components::alert::{AlertTone, describe_error};
///
/// let error = describe_error(Some(429), "");
/// assert_eq!(error.title, Some("Too many requests"));
/// assert_eq!(error.message, "Wait a moment and try again.");
/// assert_eq!(error.tone, AlertTone::Warning);
/// ```
#[must_use]
pub fn describe_error(status: Option<u16>, message: &str) -> ErrorDescription {
    let known: Option<(&'static str, &'static str, AlertTone)> = match status {
        Some(401) => Some((
            "You're signed out",
            "Sign in again to continue.",
            AlertTone::Danger,
        )),
        Some(403) => Some((
            "You don't have access",
            "Ask an administrator for access if you need it.",
            AlertTone::Danger,
        )),
        Some(404) => Some((
            "Not found",
            "It may have been moved or deleted.",
            AlertTone::Danger,
        )),
        Some(408) => Some((
            "The request timed out",
            "Check your connection and try again.",
            AlertTone::Danger,
        )),
        Some(429) => Some((
            "Too many requests",
            "Wait a moment and try again.",
            AlertTone::Warning,
        )),
        Some(504) => Some((
            "The request timed out",
            "The server took too long to respond. Try again.",
            AlertTone::Danger,
        )),
        Some(500..=599) => Some((
            "Something went wrong on our side",
            "Try again in a moment.",
            AlertTone::Danger,
        )),
        _ => None,
    };
    let message = message.trim();
    match known {
        Some((title, default, tone)) => ErrorDescription {
            title: Some(title),
            message: if message.is_empty() { default } else { message }.to_owned(),
            tone,
            status,
        },
        None => ErrorDescription {
            title: None,
            message: if message.is_empty() {
                "Something went wrong."
            } else {
                message
            }
            .to_owned(),
            tone: AlertTone::Danger,
            status,
        },
    }
}

/// An [`alert`] for a failed request, described by [`describe_error`].
///
/// - `status` is the HTTP status, if there is one, and `message` the error's own
///   message.
/// - `title` and `tone` replace what the status gives.
/// - `retry_href` adds a link, styled as a button, that requests that URL; usually
///   the page's own. `retry_label` is its text.
/// - `actions` is a slot for more buttons or links, after the retry link.
///
/// The default titles and messages are English. For another language, pass `title`
/// and `message` for the status yourself.
#[component]
pub async fn error_alert(
    cx: &Cx,
    #[default]
    #[into]
    status: Option<u16>,
    #[default] message: &str,
    #[default] title: &str,
    #[default]
    #[into]
    tone: Option<AlertTone>,
    #[default] retry_href: &str,
    #[default("Try again")] retry_label: &str,
    #[default] hide_icon: bool,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let described = describe_error(status, message);
    let tone = tone.unwrap_or(described.tone);
    let title = if title.is_empty() {
        described.title.unwrap_or("")
    } else {
        title
    };
    let status = described.status.map(|status| status.to_string());
    Ok(view! {
        <div
            role=(if tone == AlertTone::Danger { "alert" } else { "status" })
            data-tone=(tone.as_str())
            data-alert=""
            data-error-status=(status)
            class=(class!(css::ALERT, attrs.remove("class")))
            (attrs)
        >
            if !hide_icon {
                icon(name: tone.icon(), attrs: attributes! { class=(css::ICON) })
            }
            <div class=(css::BODY)>
                if !title.is_empty() {
                    <p class=(css::TITLE)>(title)</p>
                }
                <div class=(css::CONTENT)>(described.message)</div>
            </div>
            <div class=(css::ACTIONS)>
                if !retry_href.is_empty() {
                    <a
                        href=(retry_href)
                        class=(button_css::ROOT)
                        data-variant="secondary"
                        data-size="sm"
                        data-error-retry=""
                    >
                        (retry_label)
                    </a>
                }
                (actions)
            </div>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::{AlertTone, describe_error};

    #[test]
    fn statuses_with_a_title() {
        let cases = [
            (401, "You're signed out", "Sign in again to continue."),
            (
                403,
                "You don't have access",
                "Ask an administrator for access if you need it.",
            ),
            (404, "Not found", "It may have been moved or deleted."),
            (
                408,
                "The request timed out",
                "Check your connection and try again.",
            ),
            (
                504,
                "The request timed out",
                "The server took too long to respond. Try again.",
            ),
            (
                500,
                "Something went wrong on our side",
                "Try again in a moment.",
            ),
            (
                503,
                "Something went wrong on our side",
                "Try again in a moment.",
            ),
            (
                599,
                "Something went wrong on our side",
                "Try again in a moment.",
            ),
        ];
        for (status, title, message) in cases {
            let error = describe_error(Some(status), "");
            assert_eq!(error.title, Some(title), "{status}");
            assert_eq!(error.message, message, "{status}");
            assert_eq!(error.tone, AlertTone::Danger, "{status}");
            assert_eq!(error.status, Some(status));
        }
    }

    #[test]
    fn too_many_requests_is_a_warning() {
        let error = describe_error(Some(429), "");
        assert_eq!(error.title, Some("Too many requests"));
        assert_eq!(error.message, "Wait a moment and try again.");
        assert_eq!(error.tone, AlertTone::Warning);
    }

    #[test]
    fn the_errors_own_message_is_kept() {
        let error = describe_error(Some(404), "No project called apollo.");
        assert_eq!(error.title, Some("Not found"));
        assert_eq!(error.message, "No project called apollo.");
        let error = describe_error(None, "  The disk is full.  ");
        assert_eq!(error.title, None);
        assert_eq!(error.message, "The disk is full.");
    }

    #[test]
    fn other_statuses_and_no_status() {
        for status in [None, Some(400), Some(418), Some(600), Some(200)] {
            let error = describe_error(status, "");
            assert_eq!(error.title, None, "{status:?}");
            assert_eq!(error.message, "Something went wrong.");
            assert_eq!(error.tone, AlertTone::Danger);
            assert_eq!(error.status, status);
        }
    }
}
