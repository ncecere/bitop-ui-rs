//! A short label for a status or a count.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::badge as css;

/// The colour of a [`badge`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

impl BadgeVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Accent => "accent",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

/// A badge. Colour alone does not carry meaning, so say the status in the text.
#[component]
pub async fn badge(
    cx: &Cx,
    #[default] variant: BadgeVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            class=(class!(css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            (attrs)
        >
            (child)
        </span>
    })
}
