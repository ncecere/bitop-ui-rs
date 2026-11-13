//! A menu of actions, ported from bitop-ui's `Menu`, on the Popover API plus the
//! library script.
//!
//! The browser handles opening, light dismiss, Escape and focus return. The script adds
//! the ARIA menu keyboard pattern: arrow keys (which wrap), Home, End, typeahead, and
//! closing on selection. ArrowDown on the trigger opens the menu on its first item and
//! ArrowUp on its last; Enter and Space open it on the first. Opened by the pointer,
//! focus goes to the menu itself, and the pointer moves the highlight.
//!
//! Items have a leading `icon`, a `shortcut` hint (shown only; wire the shortcut
//! itself separately) and a danger `tone`; with an `href` an item is a link.
//! Besides actions and links, a menu can hold settings: [`menu_checkbox_item`] turns one
//! option on or off, and [`menu_radio_item`]s in a [`menu_radio_group`] choose one of a
//! few. Choosing either keeps the menu open, sends a `change` event from the item, and,
//! with a `name`, submits `name=value` with the form around the menu. [`menu_group`]
//! puts a label over related items, and [`menu_header`] shows content that is not an
//! item, such as the signed-in person. [`menu_submenu`] nests a menu: its item opens it
//! on hover, Enter, Space or the arrow key that points at it (ArrowRight; ArrowLeft in a
//! right-to-left page), and the opposite arrow key or Escape closes it again.
//!
//! Differences from upstream: the trigger is a part ([`menu_trigger`]), or any button
//! with `popovertarget` and `aria-haspopup="menu"`, connected by the menu's ID. A
//! checkbox or radio item is not controlled from outside; the server renders `checked`
//! and the script changes it. A radio item takes the form `name` itself, as a toggle
//! group's items do. The gap to the trigger is the CSS variable `--popover-gap`, not a
//! `sideOffset` prop, and the popup is in the top layer rather than in a landmark.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     menu_trigger(target: &id, "Actions")
//!     menu(id: &id,
//!         menu_item(icon: Icon::Pencil, shortcut: "R", "Rename")
//!         menu_item(href: "/audit", icon: Icon::FileClock, "View audit log")
//!         menu_separator()
//!         menu_item(tone: MenuItemTone::Danger, icon: Icon::Trash2, "Delete")
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
    popover::{PopupAlign, PopupSide},
};
use crate::css::{button as button_css, menu as css, popover as popover_css};

/// A button that toggles the menu whose ID is `target`.
#[component]
pub async fn menu_trigger(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Secondary)] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(target)
            aria-haspopup="menu"
            aria-expanded="false"
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}

/// How wide a [`menu`] is. Rendered as `data-width`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuWidth {
    /// As wide as its items.
    #[default]
    Auto,
    /// At least as wide as its trigger.
    Trigger,
}

impl MenuWidth {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Trigger => "trigger",
        }
    }
}

/// The menu popup. Pass [`menu_item`], [`menu_checkbox_item`], [`menu_radio_group`],
/// [`menu_group`], [`menu_header`], [`menu_submenu`] and [`menu_separator`] as
/// children.
///
/// `side` and `align` place it against the trigger; it flips when there is no room.
/// `width` makes it at least as wide as the trigger.
#[component]
pub async fn menu(
    cx: &Cx,
    id: &str,
    #[default] side: PopupSide,
    #[default] align: PopupAlign,
    #[default] width: MenuWidth,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            popover="auto"
            class=(class!(popover_css::SURFACE, attrs.remove("class")))
            data-side=(side.as_str())
            data-align=(align.as_str())
            data-width=(width.as_str())
            (attrs)
        >
            <div role="menu" tabindex="-1" class=(css::LIST)>(child)</div>
        </div>
    })
}

/// The tone of a [`menu_item`]. Rendered as `data-tone`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemTone {
    #[default]
    Default,
    /// For destructive actions.
    Danger,
}

impl MenuItemTone {
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
/// Renders a `<button>`, or an `<a>` when `href` is given. `icon` is a decorative
/// leading icon, and `shortcut` a keyboard shortcut shown at the end, such as "⌘D"; it
/// is hidden from assistive technology. Choosing the item closes the menu; add
/// `data-keep-open` in `attrs` to prevent that.
#[component]
pub async fn menu_item(
    cx: &Cx,
    #[default]
    #[into]
    href: Option<&str>,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default] shortcut: &str,
    #[default] tone: MenuItemTone,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(popover_css::ITEM, attrs.remove("class"));
    let leading = icon;
    let content = view! {
        if let Some(name) = leading {
            icon(name: name)
        }
        <span class=(css::LABEL)>(child)</span>
        if !shortcut.is_empty() {
            <span aria-hidden="true" class=(popover_css::ITEM_SHORTCUT)>
                (shortcut)
            </span>
        }
    };
    // Items are focused by the script, not by Tab, so they leave the tab order.
    Ok(view! {
        if let Some(href) = href {
            <a
                href=(href)
                role="menuitem"
                tabindex="-1"
                class=(class)
                data-tone=(tone.as_str())
                if disabled {
                    aria-disabled="true"
                }
                (attrs)
            >
                (content)
            </a>
        } else {
            <button
                type="button"
                role="menuitem"
                tabindex="-1"
                class=(class)
                data-tone=(tone.as_str())
                if disabled {
                    aria-disabled="true"
                }
                (attrs)
            >
                (content)
            </button>
        }
    })
}

/// A line between groups of items.
#[component]
pub async fn menu_separator(cx: &Cx) -> Result<impl View> {
    Ok(view! { <hr role="separator" class=(popover_css::SEPARATOR)> })
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
            <div aria-hidden="true" class=(popover_css::GROUP_LABEL)>(label)</div>
            (child)
        </div>
    })
}

/// Content at the top of a menu that is not an item, such as the signed-in person's
/// name (in a `<strong>`) and email.
#[component]
pub async fn menu_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div role="none" class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// An item that turns a setting on or off (`role="menuitemcheckbox"`).
///
/// Choosing it keeps the menu open, so that several settings can be changed, and sends a
/// `change` event from the item. With a `name`, a checked item submits `name=value`
/// with its form; `value` defaults to `on`, as a checkbox's does. `shortcut` is shown
/// at the end.
#[component]
pub async fn menu_checkbox_item(
    cx: &Cx,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] name: &str,
    #[default("on")] value: &str,
    #[default] shortcut: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            role="menuitemcheckbox"
            aria-checked=(if checked { "true" } else { "false" })
            tabindex="-1"
            class=(class!(popover_css::ITEM, attrs.remove("class")))
            if disabled {
                aria-disabled="true"
            }
            (attrs)
        >
            <span class=(css::INDICATOR)>icon(name: Icon::Check)</span>
            <span class=(css::LABEL)>(child)</span>
            if !shortcut.is_empty() {
                <span aria-hidden="true" class=(popover_css::ITEM_SHORTCUT)>
                    (shortcut)
                </span>
            }
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
                <div aria-hidden="true" class=(popover_css::GROUP_LABEL)>(label)</div>
            }
            (child)
        </div>
    })
}

/// One option of a [`menu_radio_group`] (`role="menuitemradio"`). `value` identifies it.
///
/// Choosing it unchecks the others, keeps the menu open and sends a `change` event from
/// the item. With a `name`, which every item of the group shares, the checked item
/// submits `name=value` with its form. `shortcut` is shown at the end.
#[component]
pub async fn menu_radio_item(
    cx: &Cx,
    value: &str,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] name: &str,
    #[default] shortcut: &str,
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
            class=(class!(popover_css::ITEM, attrs.remove("class")))
            if disabled {
                aria-disabled="true"
            }
            (attrs)
        >
            <span class=(css::INDICATOR)><span class=(css::RADIO_DOT)></span></span>
            <span class=(css::LABEL)>(child)</span>
            if !shortcut.is_empty() {
                <span aria-hidden="true" class=(popover_css::ITEM_SHORTCUT)>
                    (shortcut)
                </span>
            }
            if !name.is_empty() {
                <input type="hidden" name=(name) value=(value) disabled=(!checked)>
            }
        </button>
    })
}

/// A nested menu: an item with the text `label` and an optional leading `icon`, and the
/// popup it opens beside itself. Pass the submenu's items as children; a submenu can
/// hold another.
///
/// `id` goes on the popup. `attrs` are forwarded to the item.
#[component]
pub async fn menu_submenu(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let leading = icon;
    Ok(view! {
        <button
            type="button"
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded="false"
            popovertarget=(id)
            tabindex="-1"
            data-submenu-trigger=""
            class=(class!(
                popover_css::ITEM,
                css::SUBMENU_TRIGGER,
                attrs.remove("class"),
            ))
            if disabled {
                aria-disabled="true"
            }
            (attrs)
        >
            if let Some(name) = leading {
                icon(name: name)
            }
            <span class=(css::LABEL)>(label)</span>
            icon(name: Icon::ChevronRight, attrs: attributes! { class=(css::CHEVRON) })
        </button>
        <div
            id=(id)
            popover="auto"
            class=(class!(popover_css::SURFACE, css::SUBMENU))
            data-submenu=""
        >
            <div role="menu" tabindex="-1" aria-label=(label) class=(css::LIST)>
                (child)
            </div>
        </div>
    })
}
