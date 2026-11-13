//! A breadcrumb trail: the pages above the current one, then the current page.
//!
//! The trail is a navigation landmark holding an ordered list. A [`breadcrumb`] with
//! an `href` is a link; the one without is the current page (`aria-current="page"`)
//! and comes last. A long trail can collapse its middle into one "…" item,
//! [`breadcrumb_collapsed`], that opens a [`menu`](super::menu) of the hidden crumbs.
//!
//! The trail stays on one line. While it fits, every crumb shows in full; when it does
//! not (a long record name, a phone), the current page is cut first, down to a
//! readable floor, then the crumbs before it down to theirs, with an ellipsis. A
//! crumb's full text shows on hover (`title`). `wrap` lets the trail wrap onto more
//! lines instead. The library script measures the crumbs for the floors; without it a
//! crumb may be cut shorter.
//!
//! Differences from upstream: upstream takes the trail as one `items` array; here
//! each crumb is a part. A crumb's label is text, with an optional icon from the set.
//! The separators point the reading direction in a right-to-left page.
//!
//! ```ignore
//! let hidden = element_id(cx);
//! view! {
//!     breadcrumbs(
//!         breadcrumb(href: "/", label: "Acme")
//!         breadcrumb_collapsed(id: &hidden, label: "Show 2 hidden levels",
//!             breadcrumb_collapsed_item(href: "/p", "Projects")
//!             breadcrumb_collapsed_item(href: "/p/web", "Web")
//!         )
//!         breadcrumb(label: "v2.1")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
pub use super::menu::menu_item as breadcrumb_collapsed_item;
use crate::css::{breadcrumbs as css, menu as menu_css, popover as popover_css};

/// The trail. `label` names the landmark. Pass [`breadcrumb`]s and
/// [`breadcrumb_collapsed`] as children.
///
/// `wrap` lets a long trail wrap onto more lines, instead of cutting crumbs with an
/// ellipsis.
#[component]
pub async fn breadcrumbs(
    cx: &Cx,
    #[default("Breadcrumb")] label: &str,
    #[default] wrap: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            aria-label=(label)
            class=(class!(css::NAV, attrs.remove("class")))
            data-breadcrumbs=""
            if wrap {
                data-wrap=""
            }
            (attrs)
        >
            <ol class=(css::LIST)>(child)</ol>
        </nav>
    })
}

/// One crumb. With an `href` it is a link to a page above; without, it is the current
/// page and the last crumb of the trail.
///
/// `label` is its text and its tooltip, for when it is cut. `attrs` are forwarded to
/// the link, or to the current page's `<span>`.
#[component]
pub async fn breadcrumb(
    cx: &Cx,
    label: &str,
    #[default]
    #[into]
    href: Option<&str>,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <li class=(css::ITEM)>
            if let Some(href) = href {
                <a
                    href=(href)
                    title=(label)
                    class=(class!(css::LINK, attrs.remove("class")))
                    (attrs)
                >
                    if let Some(name) = icon_name {
                        icon(name: name)
                    }
                    (label)
                </a>
                icon(
                    name: Icon::ChevronRight,
                    attrs: attributes! { class=(css::SEPARATOR) }
                )
            } else {
                <span
                    aria-current="page"
                    title=(label)
                    class=(class!(css::CURRENT, attrs.remove("class")))
                    (attrs)
                >
                    if let Some(name) = icon_name {
                        icon(name: name)
                    }
                    (label)
                </span>
            }
        </li>
    })
}

/// Crumbs hidden behind one "…" button, which opens a menu of them. Pass
/// [`breadcrumb_collapsed_item`]s with an `href` as children, in the trail's order.
///
/// - `id` goes on the menu's popup.
/// - `label` is the button's accessible name, such as "Show 3 hidden levels".
/// - `title` is its tooltip, such as the hidden crumbs as a path.
/// - `attrs` are forwarded to the button.
#[component]
pub async fn breadcrumb_collapsed(
    cx: &Cx,
    id: &str,
    #[default("Show hidden levels")] label: &str,
    #[default] title: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li class=(css::ITEM)>
            <button
                type="button"
                popovertarget=(id)
                aria-haspopup="menu"
                title=((!title.is_empty()).then_some(title))
                class=(class!(css::MORE, attrs.remove("class")))
                (attrs)
            >
                <span aria-hidden="true">"…"</span>
                <span class="sr-only">(label)</span>
            </button>
            <div id=(id) popover="auto" class=(class!(popover_css::POPUP, css::MENU))>
                <div role="menu" aria-label=(label) class=(menu_css::LIST)>(child)</div>
            </div>
            icon(
                name: Icon::ChevronRight,
                attrs: attributes! { class=(css::SEPARATOR) }
            )
        </li>
    })
}
