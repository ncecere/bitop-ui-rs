use bitop_ui::components::{
    breadcrumbs::{breadcrumb, breadcrumbs},
    button::icon_button,
    command::{command_group, command_item, command_palette, command_palette_trigger},
    icon::{Icon, icon},
    menu::{menu_group, menu_header, menu_item, menu_separator},
    page_header::{PageHeaderLevel, page_header as header},
    sidebar::{
        brand, shell_main, sidebar, sidebar_content, sidebar_footer, sidebar_header, sidebar_item,
        sidebar_layout, sidebar_mode_item, sidebar_mode_switch, sidebar_nav, sidebar_section,
        sidebar_user, top_bar, workspace_switcher,
    },
    stat_card::{StatDelta, StatTrend, stat_card},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

/// The frame around the example shell, as upstream's docs draw it.
const FRAME: &str = "inline-size: 100%; overflow: hidden; border-radius: var(--radius-card); box-shadow: var(--shadow-3)";
/// The frame around the short sidebar.
const FRAME_SHORT: &str = "display: flex; inline-size: 16rem; block-size: 22rem; margin-inline: auto; overflow: hidden; border-radius: var(--radius-card); box-shadow: var(--shadow-3)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let shell = element_id(cx);
    let toggle_target = shell.clone();
    let workspaces = element_id(cx);
    let account = element_id(cx);
    let palette = element_id(cx);
    let palette_target = palette.clone();

    Ok(view! {
        page_header(slug: "sidebar")
        example(
            title: "Dashboard",
            <p
                style="inline-size: 100%; margin: 0; color: var(--color-text-muted); font-size: var(--font-size-sm)"
            >
                "Use the toggle at the top left to collapse the sidebar to an icon rail; "
                "collapsed labels become tooltips. Narrow the window to turn it into a drawer."
            </p>
            <div style=(FRAME) data-example="dashboard">
                sidebar_layout(
                    id: &shell,
                    cookie: None,
                    skip_to: None,
                    contained: true,
                    sidebar: view! {
                        sidebar(
                            label: "App sidebar (example)",
                            sidebar_header(
                                brand(name: "Acme Cloud", href: "#dashboard")
                                sidebar_mode_switch(
                                    label: "Portal",
                                    sidebar_mode_item(
                                        href: "#dashboard",
                                        icon: Icon::LayoutDashboard,
                                        current: true,
                                        label: "Workspace"
                                    )
                                    sidebar_mode_item(
                                        href: "#dashboard",
                                        icon: Icon::Shield,
                                        label: "Admin"
                                    )
                                )
                                workspace_switcher(
                                    id: &workspaces,
                                    name: "Acme Inc",
                                    description: "Owner",
                                    menu_group(
                                        label: "Workspaces",
                                        menu_item(icon: Icon::Boxes, "Acme Inc")
                                        menu_item(icon: Icon::Boxes, "Side project")
                                    )
                                    menu_separator()
                                    menu_item(icon: Icon::Plus, "New workspace")
                                )
                            )
                            sidebar_content(
                                sidebar_nav(
                                    label: "Workspace (example)",
                                    sidebar_section(
                                        sidebar_item(
                                            href: "#dashboard",
                                            icon: Icon::House,
                                            current: true,
                                            label: "Overview"
                                        )
                                        sidebar_item(
                                            href: "#dashboard",
                                            icon: Icon::Rocket,
                                            label: "Deployments",
                                            trailing: view! { "12" }
                                        )
                                        sidebar_item(
                                            href: "#dashboard",
                                            icon: Icon::Layers,
                                            label: "Projects"
                                        )
                                        sidebar_item(
                                            href: "#dashboard",
                                            icon: Icon::KeyRound,
                                            dot: true,
                                            label: "API tokens"
                                        )
                                    )
                                    sidebar_section(
                                        label: "Settings",
                                        sidebar_item(
                                            href: "#dashboard",
                                            icon: Icon::Users,
                                            label: "Members"
                                        )
                                        sidebar_item(
                                            href: "#dashboard",
                                            icon: Icon::Settings,
                                            label: "General"
                                        )
                                    )
                                )
                            )
                            sidebar_footer(
                                sidebar_user(
                                    id: &account,
                                    name: "Ada Lovelace",
                                    email: "ada@example.com",
                                    menu_header(
                                        <strong>"Ada Lovelace"</strong>
                                        "ada@example.com"
                                    )
                                    menu_separator()
                                    menu_item(icon: Icon::Settings, "Preferences")
                                    menu_item(icon: Icon::LogOut, "Sign out")
                                )
                            )
                        )
                    },
                    topbar: view! {
                        top_bar(
                            sidebar: toggle_target.as_str(),
                            start: view! {
                                breadcrumbs(
                                    label: "Breadcrumb (shell example)",
                                    breadcrumb(label: "Acme Inc", href: "#dashboard")
                                    breadcrumb(label: "Overview")
                                )
                            },
                            end: view! {
                                command_palette_trigger(
                                    target: &palette_target,
                                    label: "Search…"
                                )
                                icon_button(label: "Notifications", icon(name: Icon::Bell))
                            }
                        )
                    },
                    shell_main(
                        id: "shell-example-main",
                        landmark: false,
                        header(
                            title: "Overview",
                            level: PageHeaderLevel::H3,
                            description: "Your workspace at a glance."
                        )
                        <div
                            style="display: grid; grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr)); gap: var(--space-4)"
                        >
                            stat_card(label: "Projects", value: "8", icon: Icon::Layers)
                            stat_card(
                                label: "Deployments",
                                value: "1,284",
                                icon: Icon::Rocket,
                                delta: StatDelta::new("12%", StatTrend::Up)
                            )
                            stat_card(
                                label: "Visitors today",
                                value: "12,902",
                                icon: Icon::Globe
                            )
                            stat_card(
                                label: "Open incidents",
                                value: "0",
                                icon: Icon::FileText,
                                hint: "Last 30 days"
                            )
                        </div>
                    )
                )
            </div>
            command_palette(
                id: &palette,
                command_group(
                    label: "Jump to",
                    command_item(
                        href: "#dashboard",
                        icon: Icon::House,
                        hint: "Page",
                        "Overview"
                    )
                    command_item(
                        href: "#dashboard",
                        icon: Icon::Rocket,
                        hint: "Page",
                        "Deployments"
                    )
                )
            )
        )
        example(
            title: "Collapsible sections and two-line items",
            <p
                style="inline-size: 100%; margin: 0; color: var(--color-text-muted); font-size: var(--font-size-sm)"
            >
                "A long sidebar keeps the current page's section and the top section open and "
                "the others collapsed to their headers. Items with the same label get a description."
            </p>
            <div style=(FRAME_SHORT) data-example="sections">
                sidebar(
                    label: "Sidebar with collapsible sections (example)",
                    attrs: topcoat::view::attributes! { style="position: relative; block-size: auto" },
                    sidebar_content(
                        sidebar_nav(
                            label: "Admin (example)",
                            sidebar_section(
                                sidebar_item(
                                    href: "#sections",
                                    icon: Icon::House,
                                    label: "Overview"
                                )
                            )
                            sidebar_section(
                                label: "People",
                                collapsible: true,
                                open: false,
                                sidebar_item(
                                    href: "#sections",
                                    icon: Icon::Users,
                                    label: "Users"
                                )
                            )
                            sidebar_section(
                                label: "Settings",
                                collapsible: true,
                                sidebar_item(
                                    href: "#sections",
                                    icon: Icon::Settings,
                                    current: true,
                                    label: "General"
                                )
                                sidebar_item(
                                    href: "#sections",
                                    icon: Icon::KeyRound,
                                    label: "API tokens"
                                )
                            )
                            sidebar_section(
                                label: "Recent",
                                sidebar_item(
                                    href: "#sections",
                                    icon: Icon::FileText,
                                    label: "Quarterly report",
                                    description: "Finance · yesterday"
                                )
                                sidebar_item(
                                    href: "#sections",
                                    icon: Icon::FileText,
                                    label: "Quarterly report",
                                    description: "Sales · last week"
                                )
                            )
                        )
                    )
                )
            </div>
        )
    })
}
