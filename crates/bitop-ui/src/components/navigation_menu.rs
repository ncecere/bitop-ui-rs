//! Site navigation: a `<nav>` with top-level links, and buttons that reveal panels of
//! more links. It follows the WAI-ARIA disclosure navigation pattern.
//!
//! A panel is a popover anchored to its button, so the browser handles Escape, a
//! click outside and the top layer. The library script opens a panel when the pointer
//! rests on its button, and closes it shortly after the pointer leaves both (a click
//! on the button of a panel the pointer opened keeps it open instead); keeps
//! `aria-expanded` on the button in step; opens the panel on ArrowDown and moves into
//! it; moves between the top-level entries with ArrowLeft, ArrowRight, Home and End
//! (mirrored in a right-to-left page), and between a panel's links with ArrowDown and
//! ArrowUp; closes a panel when focus leaves it; and returns focus to the button on
//! Escape. Tab moves from an open panel's button into the panel, so render each
//! [`navigation_menu_content`] right after its [`navigation_menu_trigger`].
//!
//! Top-level links look like the buttons; links inside a panel are rows with an
//! optional description and icon.
//!
//! Differences from upstream: each panel is its own popup, anchored to its own button.
//! Upstream moves one shared popup between the buttons and animates its size, which
//! depends on Base UI measuring the panels.
//!
//! ```ignore
//! let products = element_id(cx);
//! view! {
//!     navigation_menu(label: "Main",
//!         navigation_menu_list(
//!             navigation_menu_item(
//!                 navigation_menu_trigger(target: &products, "Products")
//!                 navigation_menu_content(id: &products,
//!                     navigation_menu_link(href: "/deploy", description: "Ship from git.",
//!                         "Deployments")
//!                 )
//!             )
//!             navigation_menu_item(
//!                 navigation_menu_link(href: "/pricing", active: true, "Pricing")
//!             )
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    icon::{Icon, icon},
    popover::PopupAlign,
};
use crate::css::navigation_menu as css;

/// The root `<nav>`. `label` names the landmark, such as "Main".
///
/// `align` is how every panel lines up with its button.
#[component]
pub async fn navigation_menu(
    cx: &Cx,
    label: &str,
    #[default] align: PopupAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            aria-label=(label)
            data-navigation-menu=""
            data-align=(align.as_str())
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </nav>
    })
}

/// The list (`<ul>`) of top-level items. `vertical` stacks them.
#[component]
pub async fn navigation_menu_list(
    cx: &Cx,
    #[default] vertical: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            data-orientation=(if vertical { "vertical" } else { "horizontal" })
            class=(class!(css::LIST, attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One top-level entry (`<li>`): a trigger with its content, or a single link.
#[component]
pub async fn navigation_menu_item(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!(css::ITEM, attrs.remove("class"))) (attrs)>(child)</li>
    })
}

/// A button that reveals the [`navigation_menu_content`] whose ID is `target`.
#[component]
pub async fn navigation_menu_trigger(
    cx: &Cx,
    target: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(target)
            aria-expanded="false"
            aria-controls=(target)
            disabled=(disabled)
            data-navigation-trigger=""
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            (child)
            <span class=(css::ICON)>icon(name: Icon::ChevronDown)</span>
        </button>
    })
}

/// The panel shown while its item is open. Pass [`navigation_menu_link`]s as
/// children; `columns` (1 to 3) lays them out in a grid, which is one column on a
/// narrow screen.
#[component]
pub async fn navigation_menu_content(
    cx: &Cx,
    id: &str,
    #[default(1)] columns: u8,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div id=(id) popover="auto" class=(css::POPUP) data-navigation-content="">
            <div
                class=(class!(css::CONTENT, attrs.remove("class")))
                data-columns=(columns.clamp(1, 3).to_string())
                (attrs)
            >
                (child)
            </div>
        </div>
    })
}

/// A link. `active` marks the current page with `aria-current="page"`.
///
/// Directly inside a [`navigation_menu_item`] it looks like a trigger. Inside a
/// [`navigation_menu_content`] it is a row, which can have a `description` under its
/// text and a leading `icon`.
#[component]
pub async fn navigation_menu_link(
    cx: &Cx,
    href: &str,
    #[default] active: bool,
    #[default] description: &str,
    #[default] icon: Option<Icon>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let leading = icon;
    Ok(view! {
        <a
            href=(href)
            aria-current=(active.then_some("page"))
            data-navigation-link=""
            class=(class!(css::LINK, attrs.remove("class")))
            (attrs)
        >
            if description.is_empty() && leading.is_none() {
                (child)
            } else {
                if let Some(name) = leading {
                    <span class=(css::LINK_ICON)>icon(name: name)</span>
                }
                <span class=(css::LINK_TEXT)>
                    <span class=(css::LINK_TITLE)>(child)</span>
                    if !description.is_empty() {
                        <span class=(css::LINK_DESCRIPTION)>(description)</span>
                    }
                </span>
            }
        </a>
    })
}
