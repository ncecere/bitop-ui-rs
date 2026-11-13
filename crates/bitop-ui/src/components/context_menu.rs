//! A menu that opens at the pointer on right-click or long-press.
//!
//! The popup is a [`menu`](super::menu): the same items, the same keyboard pattern
//! (arrow keys, Home, End, typeahead, Escape) and the same script. This module adds the
//! area that opens it. From the keyboard, Shift+F10 or the context-menu key opens it
//! at the focused element, when that element is inside the area or is the area itself.
//!
//! A context menu is an enhancement: always offer the same actions through a visible
//! control too, because touch, keyboard and screen-reader users may never open it.
//!
//! Differences from upstream: the items are [`menu`](super::menu)'s, re-exported
//! under `context_menu_*` names, checkbox items, radio items, groups and submenus
//! included.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     context_menu_trigger(target: &id, <p>"Right-click here"</p>)
//!     context_menu(id: &id,
//!         context_menu_item("Copy")
//!         context_menu_separator()
//!         context_menu_item(variant: MenuItemVariant::Danger, "Delete")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

pub use super::menu::{
    MenuItemVariant, menu_checkbox_item as context_menu_checkbox_item,
    menu_group as context_menu_group, menu_item as context_menu_item,
    menu_radio_group as context_menu_radio_group, menu_radio_item as context_menu_radio_item,
    menu_separator as context_menu_separator, menu_submenu as context_menu_submenu,
};
use crate::css::{context_menu as css, menu as menu_css, popover as popover_css};

/// The area that opens the context menu whose ID is `target`.
///
/// With `disabled` the browser's own menu shows instead. To let the keyboard open the
/// menu on the area itself, make it focusable and name it in `attrs`, for example
/// `tabindex="0" role="group" aria-label="…"`.
#[component]
pub async fn context_menu_trigger(
    cx: &Cx,
    target: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-context-menu=((!disabled).then_some(target))
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The menu popup. Pass [`context_menu_item`], [`context_menu_checkbox_item`],
/// [`context_menu_radio_group`], [`context_menu_group`], [`context_menu_submenu`] and
/// [`context_menu_separator`] as children. `label` names the menu for assistive
/// technology.
#[component]
pub async fn context_menu(
    cx: &Cx,
    id: &str,
    #[default("Context menu")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            popover="auto"
            class=(class!(popover_css::POPUP, css::POPUP, attrs.remove("class")))
            data-context-menu-popup=""
            (attrs)
        >
            <div role="menu" aria-label=(label) class=(menu_css::LIST)>(child)</div>
        </div>
    })
}
