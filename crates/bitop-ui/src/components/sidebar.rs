//! The application shell, ported from bitop-ui's `AppShell`: a sidebar beside a column
//! with a top bar and the main content, and a skip link to the main content.
//!
//! - [`sidebar_layout`] is the shell (`AppShell`). It takes the [`sidebar`] and the
//!   [`top_bar`] as slots and the page as its content, usually a [`shell_main`].
//! - [`sidebar`] is an `<aside>` landmark holding a [`sidebar_header`] (with a
//!   [`brand`], a [`sidebar_mode_switch`] and a [`workspace_switcher`]), a
//!   [`sidebar_content`] that scrolls on its own (a [`sidebar_nav`] of
//!   [`sidebar_section`]s of [`sidebar_item`]s), and a [`sidebar_footer`] (a
//!   [`sidebar_user`]).
//! - [`top_bar`] is the `<header>` landmark, with the [`sidebar_toggle`] at its start.
//!
//! On a wide window the toggle collapses the sidebar to an icon rail: labels are
//! visually hidden but still announced, and show as tooltips on hover and keyboard
//! focus; trailing counts go, "new" dots move onto the icon. The library script stores
//! the choice in the `bitop_sidebar` cookie (`collapsed` or `expanded`), so that the
//! server can render the same state next time through [`sidebar_layout`]'s
//! `collapsed`.
//!
//! Below 600px ([`SIDEBAR_DRAWER_QUERY`]) there is no rail: the sidebar is a modal
//! drawer on the Popover API, opened by the same toggle, with its full labels. The
//! script makes it a dialog, moves focus into it, keeps Tab inside it and makes the
//! rest of the page inert; Escape, a click outside, its close button or following a
//! link close it, and focus returns to the toggle, or, after a link to this page, goes
//! to the main content.
//!
//! ```ignore
//! let id = element_id(cx);
//! let menu = element_id(cx);
//! view! {
//!     sidebar_layout(
//!         id: &id,
//!         collapsed: collapsed_from_cookie,
//!         sidebar: view! {
//!             sidebar(
//!                 sidebar_header(brand(name: "Acme Cloud"))
//!                 sidebar_content(
//!                     sidebar_nav(label: "Workspace",
//!                         sidebar_section(
//!                             sidebar_item(href: "/", icon: Icon::House, current: true,
//!                                 label: "Overview")
//!                             sidebar_item(href: "/deploys", icon: Icon::Rocket,
//!                                 label: "Deployments", trailing: view! { "12" })
//!                         )
//!                     )
//!                 )
//!                 sidebar_footer(
//!                     sidebar_user(id: &menu, name: "Ada Lovelace", email: "ada@example.com",
//!                         menu_item("Sign out"))
//!                 )
//!             )
//!         },
//!         topbar: view! { top_bar(sidebar: &id, start: view! { "Overview" }) },
//!         shell_main(<h1>"Overview"</h1>)
//!     )
//! }
//! ```
//!
//! Differences from upstream: the collapsed state is server-rendered from a cookie and
//! changed by the script, not React state, so there is no `onCollapsedChange` (the
//! cookie is the record). A collapsible section's open state is the server's initial
//! state; the script toggles it. Parts are links (`href`), not router `render` props.
//! `SidebarModeSwitch`'s items are [`sidebar_mode_item`] parts. The drawer breakpoint
//! is fixed at 600px, not a `drawerQuery`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    avatar::{AvatarShape, AvatarSize, avatar},
    button::{ButtonSize, icon_button},
    icon::{Icon, icon},
    layout::container,
    menu::{MenuWidth, menu},
    popover::{PopupAlign, PopupSide},
};
use crate::css::{sidebar as css, tooltip as tooltip_css};

/// The cookie in which the script stores the collapsed state.
pub const SIDEBAR_COOKIE: &str = "bitop_sidebar";

/// Below this width the sidebar is a drawer, as bitop-ui's `NARROW_QUERY`.
pub const SIDEBAR_DRAWER_QUERY: &str = "(max-width: 37.5rem)";

/// The shell: a skip link, then the `sidebar` slot beside a column with the `topbar`
/// slot over `child`.
///
/// - `id` names the drawer that holds the sidebar; pass the same ID to [`top_bar`] or
///   [`sidebar_toggle`].
/// - `collapsed` renders the sidebar as an icon rail, as read from [`SIDEBAR_COOKIE`].
/// - `cookie` is where the script stores the state; `None` keeps it for the page only,
///   as for an example inside a page.
/// - `skip_to` is the ID of the main content, the skip link's target (`"main"`, as
///   [`shell_main`]'s default); `None` leaves the skip link out.
/// - `contained` sizes the shell for a box inside a page rather than the window: the
///   sidebar and the top bar are not sticky.
#[component]
pub async fn sidebar_layout(
    cx: &Cx,
    id: &str,
    #[default] collapsed: bool,
    #[default(Some(SIDEBAR_COOKIE))] cookie: Option<&str>,
    #[default(Some("main"))] skip_to: Option<&str>,
    #[default("Skip to content")] skip_label: &str,
    #[default("Navigation")] drawer_label: &str,
    #[default("Close navigation")] close_label: &str,
    #[default] contained: bool,
    #[default]
    #[into]
    sidebar: Child<'_>,
    #[default]
    #[into]
    topbar: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let skip_href = skip_to.map(|target| format!("#{target}"));
    Ok(view! {
        if let Some(href) = &skip_href {
            skip_link(href: href, (skip_label))
        }
        <div
            class=(class!(css::SHELL, attrs.remove("class")))
            data-sidebar-layout=""
            data-collapsed=(collapsed.then_some(""))
            data-contained=(contained.then_some(""))
            data-sidebar-cookie=(cookie)
            data-sidebar-content=(skip_to)
            (attrs)
        >
            <div
                id=(id)
                popover="auto"
                class=(css::DRAWER)
                data-sidebar-drawer=""
                data-label=(drawer_label)
            >
                (sidebar)
                icon_button(
                    label: close_label,
                    attrs: attributes! {
                        popovertarget=(id)
                        popovertargetaction="hide"
                        class=(css::DRAWER_CLOSE)
                    },
                    icon(name: Icon::X)
                )
            </div>
            <div class=(css::COLUMN)>
                (topbar)
                (child)
            </div>
        </div>
    })
}

/// A link to the main content, hidden until it is focused. [`sidebar_layout`] renders
/// one first; use this part for a page without the shell.
#[component]
pub async fn skip_link(
    cx: &Cx,
    #[default("#main")] href: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a href=(href) class=(class!(css::SKIP_LINK, attrs.remove("class"))) (attrs)>
            (child)
        </a>
    })
}

/// The sidebar: an `<aside>` landmark named by `label`. Give each sidebar on a page its
/// own name.
#[component]
pub async fn sidebar(
    cx: &Cx,
    #[default("Sidebar")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <aside
            aria-label=(label)
            class=(class!(css::SIDEBAR, attrs.remove("class")))
            data-sidebar=""
            (attrs)
        >
            (child)
            // The rail's tooltip, which the script fills with the label of the item it
            // is over. The label is announced already, so the tooltip is hidden from
            // assistive technology.
            <div
                popover="manual"
                aria-hidden="true"
                class=(class!(tooltip_css::POPUP, css::RAIL_TIP))
                data-side="right"
                data-align="center"
                data-sidebar-tip=""
            ></div>
        </aside>
    })
}

/// The top of the sidebar, for the [`brand`], a [`sidebar_mode_switch`] and a
/// [`workspace_switcher`]. It does not scroll.
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

/// The middle of the sidebar: it fills the sidebar and scrolls on its own, so that the
/// header and the footer stay in place. When its content overflows, the script makes
/// it a focusable region named `scroll_label`. It keeps the current page's item in
/// view.
#[component]
pub async fn sidebar_content(
    cx: &Cx,
    #[default("Sidebar navigation")] scroll_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(css::SCROLL) data-sidebar-scroll="" data-label=(scroll_label)>
            <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>
                (child)
            </div>
        </div>
    })
}

/// The bottom of the sidebar, for a [`sidebar_user`]. It does not scroll.
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

/// The navigation landmark inside the sidebar, named by `label`.
#[component]
pub async fn sidebar_nav(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav aria-label=(label) class=(class!(css::NAV, attrs.remove("class"))) (attrs)>
            (child)
        </nav>
    })
}

/// A list of [`sidebar_item`]s under a small uppercase `label`.
///
/// With `collapsible` (and a label), the label is a button that shows or hides the
/// items, and `open` is whether they show at first. On the icon rail every item shows.
/// Keep the section of the current page open. `id` is the list's ID, from which the
/// label's is made; pass one when sections are rendered in a loop, as `element_id`
/// gives the same ID at each turn.
#[component]
pub async fn sidebar_section(
    cx: &Cx,
    #[default] label: &str,
    #[default] collapsible: bool,
    #[default(true)] open: bool,
    #[default] id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_label = !label.is_empty();
    let toggles = collapsible && has_label;
    let open = !toggles || open;
    let list_id = if id.is_empty() {
        topcoat::view::element_id(cx)
    } else {
        id.to_owned()
    };
    // The list is named with aria-label, not by its label's ID: in Playwright's
    // Firefox a role query by name crashed the page with aria-labelledby here.
    let toggle_id = format!("{list_id}-toggle");
    Ok(view! {
        <div
            class=(class!(css::SECTION, attrs.remove("class")))
            data-collapsible=(toggles.then_some(""))
            (attrs)
        >
            if toggles {
                <button
                    type="button"
                    id=(&toggle_id)
                    class=(css::SECTION_TOGGLE)
                    aria-expanded=(if open { "true" } else { "false" })
                    aria-controls=(&list_id)
                    data-sidebar-section=""
                >
                    <span class=(css::SECTION_TOGGLE_TEXT)>(label)</span>
                    icon(
                        name: Icon::ChevronRight,
                        attrs: attributes! { class=(css::SECTION_CHEVRON) }
                    )
                </button>
            } else if has_label {
                <p class=(css::SECTION_LABEL)>(label)</p>
            }
            <ul
                id=(&list_id)
                aria-label=(has_label.then_some(label))
                class=(css::ITEMS)
                hidden=((!open).then_some(""))
            >
                (child)
            </ul>
        </div>
    })
}

/// A link in a [`sidebar_section`]. Set `current` on the page being shown.
///
/// - `icon` is decorative; on the icon rail it is all that shows.
/// - `trailing` is a count or a badge after the label, left out on the rail.
/// - `dot` adds a small "new" dot, which is decorative: say "new" in the label if it
///   matters.
/// - `description` is a muted second line, to tell items with the same label apart.
///   The two lines are read as one text ("Report, Finance").
#[component]
pub async fn sidebar_item(
    cx: &Cx,
    href: &str,
    label: &str,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default] current: bool,
    #[default] dot: bool,
    #[default] description: &str,
    #[default]
    #[into]
    trailing: Child<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let spoken = format!("{label}, {description}");
    Ok(view! {
        <li class=(css::ITEM_ROW)>
            <a
                href=(href)
                aria-current=(current.then_some("page"))
                class=(class!(css::ITEM, attrs.remove("class")))
                data-description=((!description.is_empty()).then_some(""))
                data-sidebar-item=""
                (attrs)
            >
                if let Some(name) = icon {
                    <span class=(css::ITEM_ICON)>icon(name: name)</span>
                }
                if description.is_empty() {
                    <span class=(css::ITEM_LABEL) data-sidebar-label="">(label)</span>
                } else {
                    <span class=(css::ITEM_TEXT)>
                        <span class="sr-only">(spoken)</span>
                        <span
                            class=(css::ITEM_LABEL)
                            aria-hidden="true"
                            data-sidebar-label=""
                        >
                            (label)
                        </span>
                        <span class=(css::ITEM_DESCRIPTION) aria-hidden="true">
                            (description)
                        </span>
                    </span>
                }
                if dot {
                    <span aria-hidden="true" class=(css::NEW_DOT)></span>
                }
                <span class=(css::ITEM_TRAILING)>(trailing)</span>
            </a>
        </li>
    })
}

/// The icon button that collapses or expands the sidebar on a wide window, and opens
/// or closes the drawer on a narrow one. `target` is the [`sidebar_layout`]'s `id`.
/// [`top_bar`] renders one; place this part yourself for a toggle elsewhere, such as in
/// the [`sidebar_footer`]. The script keeps its label and `aria-expanded` in step.
#[component]
pub async fn sidebar_toggle(
    cx: &Cx,
    target: &str,
    #[default("Collapse sidebar")] collapse_label: &str,
    #[default("Expand sidebar")] expand_label: &str,
    #[default("Open navigation")] open_label: &str,
    #[default("Close navigation")] close_label: &str,
    #[default] collapsed: bool,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    let label = if collapsed {
        expand_label
    } else {
        collapse_label
    };
    Ok(view! {
        icon_button(
            label: label,
            size: ButtonSize::Sm,
            attrs: attributes! {
                popovertarget=(target)
                aria-controls=(target)
                aria-expanded=(if collapsed { "false" } else { "true" })
                data-sidebar-toggle=""
                data-label-collapse=(collapse_label)
                data-label-expand=(expand_label)
                data-label-open=(open_label)
                data-label-close=(close_label)
                (attrs)
            },
            icon(name: Icon::PanelLeft)
        )
    })
}

/// A segmented switch between the top-level modes of an app, such as Workspace and
/// Admin: a small navigation landmark named by `label`, holding
/// [`sidebar_mode_item`]s. On the icon rail the options stack, as icons with tooltips.
#[component]
pub async fn sidebar_mode_switch(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            aria-label=(label)
            class=(class!(css::MODE_SWITCH, attrs.remove("class")))
            (attrs)
        >
            (child)
        </nav>
    })
}

/// One mode in a [`sidebar_mode_switch`]: a link, marked `current` for the active mode.
#[component]
pub async fn sidebar_mode_item(
    cx: &Cx,
    href: &str,
    label: &str,
    icon: Icon,
    #[default] current: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            aria-current=(current.then_some("page"))
            class=(class!(css::MODE_OPTION, attrs.remove("class")))
            data-sidebar-item=""
            (attrs)
        >
            <span class=(css::MODE_ICON)>icon(name: icon)</span>
            <span class=(css::MODE_LABEL) data-sidebar-label="">(label)</span>
        </a>
    })
}

/// The product's mark and name, linking home. The mark is a tile in `--color-primary`
/// with a `--color-highlight` accent, unless `logo` gives your own (decorative: the
/// name is the link's text).
#[component]
pub async fn brand(
    cx: &Cx,
    name: &str,
    #[default("/")] href: &str,
    #[default]
    #[into]
    logo: Child<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <a href=(href) class=(class!(css::BRAND, attrs.remove("class"))) (attrs)>
            <span aria-hidden="true" class=(css::BRAND_LOGO)>(logo)</span>
            <span aria-hidden="true" class=(css::BRAND_MARK)>
                <span class=(css::BRAND_ACCENT)></span>
            </span>
            <span class=(css::BRAND_NAME)>(name)</span>
        </a>
    })
}

/// The button at the top of the sidebar that opens a menu for switching workspace or
/// team. `id` is the menu's; `child` is its content (`menu_item`, `menu_group`,
/// `menu_separator`). `description` is a second line, such as the person's role.
#[component]
pub async fn workspace_switcher(
    cx: &Cx,
    id: &str,
    name: &str,
    #[default] description: &str,
    #[default("Current workspace: ")] prefix: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(id)
            aria-haspopup="menu"
            aria-expanded="false"
            class=(class!(css::SWITCHER, attrs.remove("class")))
            (attrs)
        >
            avatar(
                name: name,
                shape: AvatarShape::Square,
                size: AvatarSize::Sm,
                decorative: true
            )
            <span class=(css::SWITCHER_TEXT)>
                <span class="sr-only">(prefix)</span>
                <span class=(css::SWITCHER_NAME)>(name)</span>
                if !description.is_empty() {
                    <span class=(css::SWITCHER_DESCRIPTION)>(description)</span>
                }
            </span>
            icon(
                name: Icon::ChevronsUpDown,
                attrs: attributes! { class=(css::SWITCHER_CHEVRON) }
            )
        </button>
        menu(id: id, align: PopupAlign::Start, width: MenuWidth::Trigger, (child))
    })
}

/// The account button at the bottom of the sidebar, which opens a menu above it. `id`
/// is the menu's; `child` is its content, such as a "Sign out" `menu_item`.
#[component]
pub async fn sidebar_user(
    cx: &Cx,
    id: &str,
    name: &str,
    #[default] email: &str,
    #[default]
    #[into]
    avatar_src: Option<&str>,
    #[default("Account: ")] prefix: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(id)
            aria-haspopup="menu"
            aria-expanded="false"
            class=(class!(css::SWITCHER, attrs.remove("class")))
            (attrs)
        >
            avatar(name: name, src: avatar_src, size: AvatarSize::Sm, decorative: true)
            <span class=(css::SWITCHER_TEXT)>
                <span class="sr-only">(prefix)</span>
                <span class=(css::SWITCHER_NAME)>(name)</span>
                if !email.is_empty() {
                    <span class=(css::SWITCHER_DESCRIPTION)>(email)</span>
                }
            </span>
            icon(
                name: Icon::ChevronsUpDown,
                attrs: attributes! { class=(css::SWITCHER_CHEVRON) }
            )
        </button>
        menu(
            id: id,
            side: PopupSide::Top,
            align: PopupAlign::Start,
            width: MenuWidth::Trigger,
            (child)
        )
    })
}

/// The top bar, a `<header>` landmark. `sidebar` is the [`sidebar_layout`]'s `id`, for
/// the [`sidebar_toggle`] at its start; `None` leaves the toggle out. `start` holds
/// breadcrumbs or a search; `end` holds actions. `collapsed` gives the toggle its label
/// for a collapsed sidebar before the script runs.
#[component]
pub async fn top_bar(
    cx: &Cx,
    #[default]
    #[into]
    sidebar: Option<&str>,
    #[default] collapsed: bool,
    #[default]
    #[into]
    start: Child<'_>,
    #[default]
    #[into]
    end: Child<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <header class=(class!(css::TOPBAR, attrs.remove("class"))) (attrs)>
            <div class=(css::TOPBAR_START)>
                if let Some(target) = sidebar {
                    sidebar_toggle(target: target, collapsed: collapsed)
                }
                (start)
            </div>
            <div class=(css::TOPBAR_END)>(end)</div>
        </header>
    })
}

/// The main content: the `<main>` landmark and the skip link's target, focusable from
/// script so that the skip link lands there. `contained` puts the content in the
/// 1200px [`container`]. With `landmark: false` it is a `<div>`, for a shell shown
/// inside a page that has its own `<main>`.
#[component]
pub async fn shell_main(
    cx: &Cx,
    #[default("main")] id: &str,
    #[default(true)] contained: bool,
    #[default(true)] landmark: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::MAIN, attrs.remove("class"));
    Ok(view! {
        if landmark {
            <main id=(id) tabindex="-1" class=(class) (attrs)>
                if contained {
                    container((child))
                } else {
                    (child)
                }
            </main>
        } else {
            <div id=(id) tabindex="-1" class=(class) (attrs)>
                if contained {
                    container((child))
                } else {
                    (child)
                }
            </div>
        }
    })
}
