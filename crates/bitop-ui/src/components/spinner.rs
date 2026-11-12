//! A rotating ring for work in progress, and a labelled loading block.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::spinner as css;

/// The size of a [`spinner`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SpinnerSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// A spinner.
///
/// With a `label` it is announced as a status. Without one it is decorative, and the
/// caller must say what is happening some other way.
#[component]
pub async fn spinner(
    cx: &Cx,
    #[default] size: SpinnerSize,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            if label.is_empty() {
                aria-hidden="true"
            } else {
                role="status"
            }
            data-size=(size.as_str())
            class=(class!(css::SPINNER, attrs.remove("class")))
            (attrs)
        >
            <svg viewBox="0 0 24 24" fill="none" class=(css::SVG)>
                <circle
                    cx="12"
                    cy="12"
                    r="9"
                    stroke="currentColor"
                    stroke-width="2.5"
                    class=(css::TRACK)
                ></circle>
                <path
                    d="M21 12a9 9 0 0 0-9-9"
                    stroke="currentColor"
                    stroke-width="2.5"
                    stroke-linecap="round"
                ></path>
            </svg>
            if !label.is_empty() {
                <span class="sr-only">(label)</span>
            }
        </span>
    })
}

/// A spinner beside visible text, announced politely.
///
/// `block` centres it in a padded area, for a page or section that is loading.
#[component]
pub async fn loading(
    cx: &Cx,
    #[default("Loading…")] label: &str,
    #[default(true)] block: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="status"
            if block {
                data-block=""
            }
            class=(class!(css::LOADING, attrs.remove("class")))
            (attrs)
        >
            spinner(size: SpinnerSize::Sm)
            <span>(label)</span>
        </div>
    })
}
