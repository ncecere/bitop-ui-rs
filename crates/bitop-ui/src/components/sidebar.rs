//! The application shell: a navigation sidebar beside the main content.
//!
//! On a wide screen the sidebar sits in the layout and [`sidebar_toggle`] collapses it.
//! The library script stores that choice in the `bitop_sidebar` cookie (`collapsed` or
//! `expanded`), so the server can render the same state next time through
//! [`sidebar_layout`]'s `collapsed`.
//!
//! On a narrow screen the sidebar is a drawer on the Popover API: the toggle opens it
//! over the page, and the browser closes it on Escape or a click outside.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     sidebar_layout(
//!         sidebar(id: &id, label: "Chats",
//!             sidebar_header("bitop")
//!             sidebar_content(
//!                 sidebar_group(label: "Today",
//!                     sidebar_item(href: "/c/1", current: true, "Quarterly plan")
//!                 )
//!             )
//!             sidebar_footer("Ada Lovelace")
//!         )
//!         sidebar_main(
//!             sidebar_toggle(target: &id)
//!             <main>"..."</main>
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant};
use crate::css::{button as button_css, sidebar as css};

/// The cookie in which the script stores the collapsed state.
pub const SIDEBAR_COOKIE: &str = "bitop_sidebar";

/// The shell. `child` is a [`sidebar`] followed by a [`sidebar_main`].
#[component]
pub async fn sidebar_layout(
    cx: &Cx,
    #[default] collapsed: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::LAYOUT, attrs.remove("class")))
            data-sidebar-layout=""
            if collapsed {
                data-collapsed=""
            }
            (attrs)
        >
            (child)
        </div>
    })
}

/// The sidebar, a navigation landmark named by `label`.
#[component]
pub async fn sidebar(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            id=(id)
            popover="auto"
            aria-label=(label)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </nav>
    })
}

/// The top of the sidebar, which does not scroll.
#[component]
pub async fn sidebar_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The scrolling middle of the sidebar.
#[component]
pub async fn sidebar_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The bottom of the sidebar, which does not scroll.
#[component]
pub async fn sidebar_footer(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::FOOTER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A labelled list of [`sidebar_item`]s.
#[component]
pub async fn sidebar_group(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section
            aria-label=(label)
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            <div class=(css::GROUP_LABEL) aria-hidden="true">(label)</div>
            (child)
        </section>
    })
}

/// A link in the sidebar. Set `current` on the page being shown.
#[component]
pub async fn sidebar_item(
    cx: &Cx,
    href: &str,
    #[default] current: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            if current {
                aria-current="page"
            }
            class=(class!(css::ITEM, attrs.remove("class")))
            (attrs)
        >
            (child)
        </a>
    })
}

/// The column beside the sidebar. Put the page's `<main>` inside it.
#[component]
pub async fn sidebar_main(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::MAIN, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// An icon button that collapses or opens the sidebar whose ID is `target`.
#[component]
pub async fn sidebar_toggle(
    cx: &Cx,
    target: &str,
    #[default("Toggle sidebar")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(target)
            aria-label=(label)
            data-sidebar-toggle=""
            class=(class!(button_css::ROOT, css::TOGGLE, attrs.remove("class")))
            data-variant=(ButtonVariant::Ghost.as_str())
            data-size=(ButtonSize::Md.as_str())
            data-icon-only=""
            (attrs)
        ></button>
    })
}
