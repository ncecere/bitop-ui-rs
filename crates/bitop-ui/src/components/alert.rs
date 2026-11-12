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

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::alert as css;

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
