//! A menu of actions, on the Popover API plus the library script.
//!
//! The browser handles opening, light dismiss, Escape and focus return. The script adds
//! the ARIA menu keyboard pattern: arrow keys, Home, End, typeahead, and closing on
//! selection.
//!
//! Besides actions and links, a menu can hold settings: [`menu_checkbox_item`] turns one
//! option on or off, and [`menu_radio_item`]s in a [`menu_radio_group`] choose one of a
//! few. Choosing either keeps the menu open, sends a `change` event from the item, and,
//! with a `name`, submits `name=value` with the form around the menu. [`menu_group`]
//! puts a label over related items. [`menu_submenu`] nests a menu: its item opens it on
//! hover, Enter, Space or the arrow key that points at it (ArrowRight; ArrowLeft in a
//! right-to-left page), and the opposite arrow key or Escape closes it again.
//!
//! Differences from upstream: a checkbox or radio item is not controlled from outside;
//! the server renders `checked` and the script changes it. A radio item takes the form
//! `name` itself, as a toggle group's items do. Items have no `shortcut` hint and there
//! is no `MenuHeader`.
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
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
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

/// The menu popup. Pass [`menu_item`], [`menu_checkbox_item`], [`menu_radio_group`],
/// [`menu_group`], [`menu_submenu`] and [`menu_separator`] as children.
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

/// Related items under a visible `label`, which also names the group.
#[component]
pub async fn menu_group(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            <div aria-hidden="true" class=(css::GROUP_LABEL)>(label)</div>
            (child)
        </div>
    })
}

/// An item that turns a setting on or off (`role="menuitemcheckbox"`).
///
/// Choosing it keeps the menu open, so that several settings can be changed, and sends a
/// `change` event from the item. With a `name`, a checked item submits `name=value`
/// with its form; `value` defaults to `on`, as a checkbox's does.
#[component]
pub async fn menu_checkbox_item(
    cx: &Cx,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] name: &str,
    #[default("on")] value: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            role="menuitemcheckbox"
            aria-checked=(if checked { "true" } else { "false" })
            tabindex="-1"
            class=(class!(css::ITEM, attrs.remove("class")))
            if disabled {
                aria-disabled="true"
            }
            (attrs)
        >
            <span class=(css::INDICATOR)>icon(name: Icon::Check)</span>
            <span class=(css::LABEL)>(child)</span>
            if !name.is_empty() {
                <input type="hidden" name=(name) value=(value) disabled=(!checked)>
            }
        </button>
    })
}

/// A set of [`menu_radio_item`]s, of which one is checked. `label` names the group and
/// shows above it unless `hide_label` is set.
#[component]
pub async fn menu_radio_group(
    cx: &Cx,
    label: &str,
    #[default] hide_label: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-menu-radio-group=""
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            if !hide_label {
                <div aria-hidden="true" class=(css::GROUP_LABEL)>(label)</div>
            }
            (child)
        </div>
    })
}

/// One option of a [`menu_radio_group`] (`role="menuitemradio"`). `value` identifies it.
///
/// Choosing it unchecks the others, keeps the menu open and sends a `change` event from
/// the item. With a `name`, which every item of the group shares, the checked item
/// submits `name=value` with its form.
#[component]
pub async fn menu_radio_item(
    cx: &Cx,
    value: &str,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            role="menuitemradio"
            aria-checked=(if checked { "true" } else { "false" })
            tabindex="-1"
            data-value=(value)
            class=(class!(css::ITEM, attrs.remove("class")))
            if disabled {
                aria-disabled="true"
            }
            (attrs)
        >
            <span class=(css::INDICATOR)><span class=(css::RADIO_DOT)></span></span>
            <span class=(css::LABEL)>(child)</span>
            if !name.is_empty() {
                <input type="hidden" name=(name) value=(value) disabled=(!checked)>
            }
        </button>
    })
}

/// A nested menu: an item with the text `label`, and the popup it opens beside itself.
/// Pass the submenu's items as children; a submenu can hold another.
///
/// `id` goes on the popup. `attrs` are forwarded to the item.
#[component]
pub async fn menu_submenu(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded="false"
            popovertarget=(id)
            tabindex="-1"
            data-submenu-trigger=""
            class=(class!(css::ITEM, attrs.remove("class")))
            if disabled {
                aria-disabled="true"
            }
            (attrs)
        >
            <span class=(css::LABEL)>(label)</span>
            icon(name: Icon::ChevronRight, attrs: attributes! { class=(css::CHEVRON) })
        </button>
        <div
            id=(id)
            popover="auto"
            class=(class!(popover_css::POPUP, css::SUBMENU))
            data-submenu=""
        >
            <div role="menu" aria-label=(label) class=(css::LIST)>(child)</div>
        </div>
    })
}
