//! A line between sections, or between items in a row.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::separator as css;

/// The space around a [`separator`]. Rendered as `data-spacing`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorSpacing {
    None,
    Sm,
    #[default]
    Md,
    Lg,
}

impl SeparatorSpacing {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// A separator. Horizontal unless `vertical` is set.
///
/// With a `label`, such as "or", it is a horizontal line with the text in the middle.
#[component]
pub async fn separator(
    cx: &Cx,
    #[default] vertical: bool,
    #[default] spacing: SeparatorSpacing,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        if label.is_empty() {
            <div
                role="separator"
                aria-orientation=(if vertical { "vertical" } else { "horizontal" })
                data-orientation=(if vertical { "vertical" } else { "horizontal" })
                data-spacing=(spacing.as_str())
                class=(class!(css::SEPARATOR, attrs.remove("class")))
                (attrs)
            ></div>
        } else {
            <div
                data-spacing=(spacing.as_str())
                class=(class!(css::LABELLED, attrs.remove("class")))
                (attrs)
            >
                <div
                    role="presentation"
                    data-orientation="horizontal"
                    class=(css::LINE)
                ></div>
                <span class=(css::LABEL)>(label)</span>
                <div
                    role="presentation"
                    data-orientation="horizontal"
                    class=(css::LINE)
                ></div>
            </div>
        }
    })
}
