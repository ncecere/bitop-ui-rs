//! Brief messages that appear in a corner and dismiss themselves.
//!
//! Render one [`toast_region`] per page, at the end of `<body>`. A toast gets there in
//! one of two ways:
//!
//! - The server renders a [`toast`] inside the region, for example after a redirect.
//! - A script calls `bitopUi.toast({ title, description, variant, duration })`.
//!
//! The library script removes a toast after its duration, pauses the timer while the
//! pointer or focus is on the region, and handles the dismiss button. Toasts are
//! announced politely; danger toasts are announced at once.
//!
//! Toasts sit above the page but below modal dialogs, so do not raise one while a
//! dialog is open.

use topcoat::{
    Result,
    context::Cx,
    view::{Child, View, component, view},
};

use crate::css::toast as css;

/// The colour of a [`toast`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Neutral,
    Success,
    Danger,
}

impl ToastVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Success => "success",
            Self::Danger => "danger",
        }
    }
}

/// Where toasts appear. `child` is any [`toast`]s to show when the page loads.
#[component]
pub async fn toast_region(
    cx: &Cx,
    #[default("Notifications")] label: &str,
    #[default("Dismiss")] dismiss_label: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div role="region" aria-label=(label) class=(css::REGION) data-toast-region="">
            (child)
            // The script clones this to build a toast, so it never needs class names.
            <template>
                <div class=(css::ROOT) role="status" data-toast="">
                    <div class=(css::CONTENT)>
                        <div class=(css::TITLE) data-toast-title=""></div>
                        <div class=(css::DESCRIPTION) data-toast-description=""></div>
                    </div>
                    <button
                        type="button"
                        class=(css::CLOSE)
                        aria-label=(dismiss_label)
                        data-toast-close=""
                    ></button>
                </div>
            </template>
        </div>
    })
}

/// One toast, rendered by the server inside a [`toast_region`].
///
/// `duration_ms` is how long it stays; zero keeps it until dismissed.
#[component]
pub async fn toast(
    cx: &Cx,
    title: &str,
    #[default]
    #[into]
    description: Option<&str>,
    #[default] variant: ToastVariant,
    #[default(5000)] duration_ms: u32,
    #[default("Dismiss")] dismiss_label: &str,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(css::ROOT)
            role=(if variant == ToastVariant::Danger { "alert" } else { "status" })
            data-toast=""
            data-variant=(variant.as_str())
            data-duration=(duration_ms.to_string())
        >
            <div class=(css::CONTENT)>
                <div class=(css::TITLE)>(title)</div>
                if let Some(description) = description {
                    <div class=(css::DESCRIPTION)>(description)</div>
                }
            </div>
            <button
                type="button"
                class=(css::CLOSE)
                aria-label=(dismiss_label)
                data-toast-close=""
            ></button>
        </div>
    })
}
