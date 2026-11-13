//! Related buttons joined into one unit with shared borders.
//!
//! The group has `role="group"` and must be named with `label`, so that assistive
//! technology announces what the buttons belong to. Keyboard behaviour stays that of
//! the buttons themselves: Tab moves between them. For arrow-key navigation use a
//! [`toggle_group`](super::toggle_group::toggle_group).
//!
//! Groups can nest: set `spaced` on the outer one to keep the inner groups apart.
//!
//! ```ignore
//! view! {
//!     button_group(label: "Pagination",
//!         button(variant: ButtonVariant::Secondary, "Previous")
//!         button_group_text("Page 2 of 9")
//!         button(variant: ButtonVariant::Secondary, "Next")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::{button_group as css, separator as separator_css};

/// A group of joined buttons, inputs or menu triggers. `label` names the group.
///
/// `vertical` stacks the children. `spaced` keeps them apart instead of joining them,
/// for a group of groups.
#[component]
pub async fn button_group(
    cx: &Cx,
    label: &str,
    #[default] vertical: bool,
    #[default] spaced: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-orientation=(if vertical { "vertical" } else { "horizontal" })
            data-spaced=(spaced)
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A segment that is not interactive, such as "Page 2 of 9" or a unit beside an input.
#[component]
pub async fn button_group_text(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::TEXT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A line between two joined buttons. Vertical, for a horizontal group, unless
/// `horizontal` is set.
#[component]
pub async fn button_group_separator(
    cx: &Cx,
    #[default] horizontal: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let orientation = if horizontal { "horizontal" } else { "vertical" };
    Ok(view! {
        <div
            role="separator"
            aria-orientation=(orientation)
            data-orientation=(orientation)
            data-spacing="none"
            class=(class!(
                separator_css::SEPARATOR,
                css::SEPARATOR,
                attrs.remove("class"),
            ))
            (attrs)
        ></div>
    })
}
