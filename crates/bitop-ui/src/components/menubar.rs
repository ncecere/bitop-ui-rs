//! An application-style menubar: a row of menus, on the WAI-ARIA menubar pattern.
//!
//! Each top-level menu is a [`menu`](super::menu) popover opened by its button, so
//! the items, their keyboard pattern and their script are the menu's. The library
//! script adds the bar: it is one tab stop; ArrowLeft and ArrowRight move between the
//! top-level menus (mirrored in a right-to-left page), Home and End go to the first
//! and last; ArrowDown, Enter or Space opens a menu on its first item and ArrowUp on
//! its last. Once a menu is open, ArrowLeft and ArrowRight open its neighbours, and so
//! does moving the pointer over another top-level menu. Escape closes the menu and
//! returns focus to its button.
//!
//! The items are [`menu`](super::menu)'s, re-exported under `menubar_*` names: icons,
//! shortcut hints, the danger tone, links, checkbox and radio items, groups and
//! submenus.
//!
//! Differences from upstream: the bar is not modal, so the page behind an open menu can
//! still be pointed at; a click outside closes the menu. The gap between the bar and a popup is the CSS variable `--popover-gap`,
//! not a prop.
//!
//! ```ignore
//! let file = element_id(cx);
//! let view_menu = element_id(cx);
//! view! {
//!     menubar(label: "Editor",
//!         menubar_menu(id: &file, label: "File",
//!             menubar_item("New file")
//!             menubar_separator()
//!             menubar_item("Close")
//!         )
//!         menubar_menu(id: &view_menu, label: "View",
//!             menubar_item("Zoom in")
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

pub use super::menu::{
    MenuItemTone, menu_checkbox_item as menubar_checkbox_item, menu_group as menubar_group,
    menu_item as menubar_item, menu_radio_group as menubar_radio_group,
    menu_radio_item as menubar_radio_item, menu_separator as menubar_separator,
    menu_submenu as menubar_submenu,
};
use super::popover::PopupAlign;
use crate::css::{menu as menu_css, menubar as css, popover as popover_css};

/// The bar. `label` names it, such as "Editor". Pass [`menubar_menu`]s as children.
///
/// `vertical` stacks the menus; ArrowUp and ArrowDown then move between them and the
/// arrow key that points at the popup opens it.
#[component]
pub async fn menubar(
    cx: &Cx,
    label: &str,
    #[default] vertical: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="menubar"
            aria-label=(label)
            aria-orientation=(vertical.then_some("vertical"))
            data-menubar=""
            data-orientation=(if vertical { "vertical" } else { "horizontal" })
            class=(class!(css::MENUBAR, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One top-level menu of a [`menubar`]: a button with the text `label` and its popup.
/// Pass [`menubar_item`], [`menubar_checkbox_item`], [`menubar_radio_group`],
/// [`menubar_group`], [`menubar_submenu`] and [`menubar_separator`] as children.
///
/// `id` goes on the popup. `attrs` are forwarded to the button.
#[component]
pub async fn menubar_menu(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] disabled: bool,
    #[default] align: PopupAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // No tabindex here: the script sets it, to make the bar one tab stop.
    Ok(view! {
        <button
            type="button"
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded="false"
            popovertarget=(id)
            disabled=(disabled)
            data-menubar-trigger=""
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            (label)
        </button>
        <div
            id=(id)
            popover="auto"
            class=(class!(popover_css::SURFACE, css::POPUP))
            data-align=(align.as_str())
            data-menubar-popup=""
        >
            <div role="menu" tabindex="-1" aria-label=(label) class=(menu_css::LIST)>
                (child)
            </div>
        </div>
    })
}
