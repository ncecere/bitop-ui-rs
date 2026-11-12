//! Three pulsing dots for "waiting for the first token" and other pending states in
//! chat interfaces. Use [`spinner`](super::spinner::spinner) for buttons and for a
//! page that is loading.
//!
//! Decorative unless given a `label`, in which case it is a polite status. Under
//! reduced motion the dots stop pulsing and stay visible.
//!
//! ```ignore
//! view! { loader(label: "Thinking", show_label: true) }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::loader as css;

/// The size of a [`loader`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoaderSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl LoaderSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// A loader.
///
/// `label` announces it as a status; without one it is hidden from assistive
/// technology. `show_label` also shows the label beside the dots.
#[component]
pub async fn loader(
    cx: &Cx,
    #[default] size: LoaderSize,
    #[default] label: &str,
    #[default] show_label: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let labelled = !label.is_empty();
    Ok(view! {
        <span
            role=(labelled.then_some("status"))
            aria-hidden=((!labelled).then_some("true"))
            data-size=(size.as_str())
            data-loader=""
            class=(class!(css::LOADER, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::DOTS)>
                <span class=(css::DOT)></span>
                <span class=(css::DOT)></span>
                <span class=(css::DOT)></span>
            </span>
            if labelled {
                <span class=(if show_label { css::LABEL } else { "sr-only" })>
                    (label)
                </span>
            }
        </span>
    })
}
