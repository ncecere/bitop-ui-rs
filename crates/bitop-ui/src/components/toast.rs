//! Toasts, ported from bitop-ui's `Toaster` and `toast`: brief messages that stack in
//! a corner and dismiss themselves.
//!
//! Render one [`toaster`] per page, at the end of `<body>`. A toast gets there in one
//! of two ways:
//!
//! - The server renders a [`toast`] inside the toaster, for example after a redirect.
//! - A script calls the library's `bitopUi.toast`, as upstream's `toast`:
//!
//! ```js
//! bitopUi.toast.success("Deployment ready", "marketing-site is live.");
//! bitopUi.toast.info("Build queued");
//! bitopUi.toast.warning("Slow build", "Took 4m, 3× the usual time.");
//! bitopUi.toast.error("Deployment failed", "Check the build logs.");
//! const id = bitopUi.toast.add({
//!   title: "Token revoked",
//!   description: "",           // optional
//!   tone: "neutral",           // "info", "success", "warning" or "danger"
//!   timeout: 5000,             // 0 keeps it open
//!   action: { label: "Undo", onClick: () => bitopUi.toast.success("Token restored") },
//! });
//! bitopUi.toast.close(id);     // or close() for all
//! bitopUi.toast.promise(save(), {
//!   loading: "Saving…",
//!   success: (result) => ({ title: "Saved", description: result.name }),
//!   error: "Could not save",
//! });
//! ```
//!
//! `bitopUi.toast(options)` is `bitopUi.toast.add(options)`. Each returns the toast's
//! ID; `add` with the ID of an open toast updates it.
//!
//! The toasts collapse into a stack with the newest in front, and spread out while the
//! pointer is over them or the keyboard is in them. Timers pause meanwhile, and while
//! the window is in the background. The toaster is a region named "Notifications":
//! F6 moves focus to it, Tab moves through the toasts and their buttons, Escape closes
//! the focused toast, and focus then returns where it was. Each toast is a status
//! message; a danger toast is an alert, announced at once.
//!
//! Differences from upstream: toasts cannot be swiped away. A toast is not named by
//! its title through `aria-labelledby` (a crash in Playwright's Firefox, see the
//! contributing guide); its text is its content. A rejected `promise` shows a danger
//! toast, where upstream shows one of an unknown type without an icon. Toasts sit
//! above the page but below modal dialogs, which are in the top layer.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::toast as css;

/// The meaning of a [`toast`], which sets its icon. Rendered as `data-tone`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastTone {
    /// No icon.
    #[default]
    Neutral,
    Info,
    Success,
    Warning,
    /// Announced at once, as an alert.
    Danger,
}

impl ToastTone {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    fn icon(self) -> Option<Icon> {
        match self {
            Self::Neutral => None,
            Self::Info => Some(Icon::Info),
            Self::Success => Some(Icon::CircleCheck),
            Self::Warning => Some(Icon::TriangleAlert),
            Self::Danger => Some(Icon::CircleAlert),
        }
    }
}

/// Where a [`toaster`] shows its toasts. Rendered as `data-position`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPosition {
    #[default]
    BottomRight,
    /// Clear of actions kept at the bottom right, such as a sticky save bar.
    BottomCenter,
    /// Over a sidebar's footer, as wide as the sidebar on screens wide enough for one,
    /// so that toasts cover navigation, not page content.
    BottomLeft,
}

impl ToastPosition {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BottomRight => "bottom-right",
            Self::BottomCenter => "bottom-center",
            Self::BottomLeft => "bottom-left",
        }
    }
}

/// The region toasts appear in. Render one per page. `child` is any [`toast`]s to
/// show when the page loads, the newest first.
///
/// - `limit` is how many toasts are shown at once; older ones are hidden until they
///   time out.
/// - `position` is the corner they appear in.
/// - `label` names the region; `dismiss_label` names each toast's × button.
#[component]
pub async fn toaster(
    cx: &Cx,
    #[default(3)] limit: u32,
    #[default] position: ToastPosition,
    #[default("Notifications")] label: &str,
    #[default("Dismiss notification")] dismiss_label: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="region"
            aria-label=(label)
            aria-live="polite"
            aria-atomic="false"
            aria-relevant="additions text"
            tabindex="-1"
            class=(css::VIEWPORT)
            data-toast-region=""
            data-position=(position.as_str())
            data-limit=(limit.to_string())
        >
            (child)
            // The script clones this to build a toast, so it never needs class names.
            // It keeps the icon of the toast's tone and removes the others.
            <template>
                <div
                    class=(css::TOAST)
                    role="status"
                    aria-atomic="true"
                    tabindex="0"
                    data-toast=""
                >
                    <div class=(css::CONTENT) data-toast-content="">
                        for tone in [
                            ToastTone::Info,
                            ToastTone::Success,
                            ToastTone::Warning,
                            ToastTone::Danger,
                        ] {
                            icon(
                                name: tone.icon().unwrap_or(Icon::Info),
                                attrs: attributes! { class=(css::ICON) data-toast-icon=(tone.as_str()) }
                            )
                        }
                        <div class=(css::TEXT)>
                            <div class=(css::TITLE) data-toast-title=""></div>
                            <div class=(css::DESCRIPTION) data-toast-description=""></div>
                        </div>
                        <button
                            type="button"
                            class=(css::ACTION)
                            data-toast-action=""
                            hidden=(true)
                        ></button>
                        <button
                            type="button"
                            class=(css::CLOSE)
                            aria-label=(dismiss_label)
                            data-toast-close=""
                        >
                            icon(name: Icon::X)
                        </button>
                    </div>
                </div>
            </template>
        </div>
    })
}

/// One toast, rendered by the server inside a [`toaster`].
///
/// - `timeout` is how long it stays, in milliseconds; zero keeps it until dismissed.
/// - `action_label` adds an action button, such as Undo; `action_attrs` are forwarded
///   to it, such as `type="submit"` and a `form`.
#[component]
pub async fn toast(
    cx: &Cx,
    title: &str,
    #[default] description: &str,
    #[default] tone: ToastTone,
    #[default(5000)] timeout: u32,
    #[default] action_label: &str,
    #[default] action_attrs: Attributes,
    #[default("Dismiss notification")] dismiss_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let typed = action_attrs.contains_key("type");
    Ok(view! {
        <div
            class=(class!(css::TOAST, attrs.remove("class")))
            role=(if tone == ToastTone::Danger { "alert" } else { "status" })
            aria-atomic="true"
            tabindex="0"
            data-toast=""
            data-tone=(tone.as_str())
            data-timeout=(timeout.to_string())
            (attrs)
        >
            <div class=(css::CONTENT) data-toast-content="">
                if let Some(name) = tone.icon() {
                    icon(
                        name: name,
                        attrs: attributes! { class=(css::ICON) data-toast-icon=(tone.as_str()) }
                    )
                }
                <div class=(css::TEXT)>
                    <div class=(css::TITLE) data-toast-title="">(title)</div>
                    <div class=(css::DESCRIPTION) data-toast-description="">
                        (description)
                    </div>
                </div>
                // Always rendered, so that a script can give the toast an action.
                <button
                    if !typed {
                        type="button"
                    }
                    class=(css::ACTION)
                    data-toast-action=""
                    hidden=(action_label.is_empty())
                    (action_attrs)
                >
                    (action_label)
                </button>
                <button
                    type="button"
                    class=(css::CLOSE)
                    aria-label=(dismiss_label)
                    data-toast-close=""
                >
                    icon(name: Icon::X)
                </button>
            </div>
        </div>
    })
}
