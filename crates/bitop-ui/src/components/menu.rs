//! A menu of actions, on the Popover API plus the library script.
//!
//! The browser handles opening, light dismiss, Escape and focus return. The script adds
//! the ARIA menu keyboard pattern: arrow keys, Home, End, typeahead, and closing on
//! selection.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     menu_trigger(target: &id, "Options")
//!     menu(id: &id,
//!         menu_item("Rename")
//!         menu_separator()
//!         menu_item(variant: MenuItemVariant::Danger, "Delete")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    popover::PopupAlign,
};
use crate::css::{button as button_css, menu as css, popover as popover_css};

/// A button that toggles the menu whose ID is `target`.
#[component]
pub async fn menu_trigger(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Outline)] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(target)
            aria-haspopup="menu"
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}

/// The menu popup. Pass [`menu_item`] and [`menu_separator`] as children.
#[component]
pub async fn menu(
    cx: &Cx,
    id: &str,
    #[default] align: PopupAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            popover="auto"
            class=(class!(popover_css::POPUP, attrs.remove("class")))
            data-align=(align.as_str())
            (attrs)
        >
            <div role="menu" class=(css::LIST)>(child)</div>
        </div>
    })
}

/// The visual style of a [`menu_item`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemVariant {
    #[default]
    Default,
    Danger,
}

impl MenuItemVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Danger => "danger",
        }
    }
}

/// One action in a [`menu`].
///
/// Renders a `<button>`, or an `<a>` when `href` is given. Choosing it closes the menu;
/// add `data-keep-open` in `attrs` to prevent that.
#[component]
pub async fn menu_item(
    cx: &Cx,
    #[default]
    #[into]
    href: Option<&str>,
    #[default] variant: MenuItemVariant,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::ITEM, attrs.remove("class"));
    // Items are focused by the script, not by Tab, so they leave the tab order.
    Ok(view! {
        if let Some(href) = href {
            <a
                href=(href)
                role="menuitem"
                tabindex="-1"
                class=(class)
                data-variant=(variant.as_str())
                if disabled {
                    aria-disabled="true"
                }
                (attrs)
            >
                (child)
            </a>
        } else {
            <button
                type="button"
                role="menuitem"
                tabindex="-1"
                class=(class)
                data-variant=(variant.as_str())
                if disabled {
                    aria-disabled="true"
                }
                (attrs)
            >
                (child)
            </button>
        }
    })
}

/// A line between groups of items.
#[component]
pub async fn menu_separator(cx: &Cx) -> Result<impl View> {
    Ok(view! { <hr role="separator" class=(css::SEPARATOR)> })
}
