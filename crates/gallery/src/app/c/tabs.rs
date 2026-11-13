use bitop_ui::components::{
    icon::Icon,
    tabs::{TabsOverflow, TabsVariant, nav_tab, nav_tabs, tab, tab_list, tab_panel, tabs},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "tabs")
        example(
            title: "Tabs",
            tabs(
                attrs: attributes! { style="inline-size: 100%" },
                tab_list(
                    label: "Project views",
                    tab(
                        panel: "tab-deployments",
                        selected: true,
                        count: 42,
                        "Deployments"
                    )
                    tab(panel: "tab-domains", "Domains")
                    tab(panel: "tab-settings", "Settings")
                )
                tab_panel(
                    id: "tab-deployments",
                    selected: true,
                    <p>"Deployments panel."</p>
                )
                tab_panel(id: "tab-domains", <p>"Domains panel."</p>)
                tab_panel(id: "tab-settings", <p>"Settings panel."</p>)
            )
        )
        example(
            title: "Pills",
            tabs(
                tab_list(
                    label: "Range",
                    variant: TabsVariant::Pills,
                    tab(panel: "range-day", selected: true, "24h")
                    tab(panel: "range-week", "7d")
                    tab(panel: "range-month", "30d")
                )
                tab_panel(id: "range-day", selected: true, <p>"Last 24 hours."</p>)
                tab_panel(id: "range-week", <p>"Last 7 days."</p>)
                tab_panel(id: "range-month", <p>"Last 30 days."</p>)
            )
        )
        example(
            title: "NavTabs (route tabs)",
            nav_tabs(
                label: "Project sections (example)",
                attrs: attributes! { style="inline-size: 100%" },
                nav_tab(href: "/c/tabs", current: true, "Overview")
                nav_tab(href: "/c/tabs?section=domains", count: 3, "Domains")
                nav_tab(href: "/c/tabs?section=analytics", "Analytics")
                nav_tab(href: "/c/tabs?section=settings", "Settings")
            )
        )
        example(
            title: "Icons, a disabled tab, and scrolling when they do not fit",
            <div style="inline-size: 20rem; max-inline-size: 100%">
                tabs(
                    tab_list(
                        label: "Workspace",
                        overflow: TabsOverflow::Scroll,
                        tab(
                            panel: "ws-files",
                            selected: true,
                            icon: Icon::Folder,
                            "Files"
                        )
                        tab(
                            panel: "ws-activity",
                            icon: Icon::Bell,
                            count: 1250,
                            "Activity"
                        )
                        tab(panel: "ws-billing", disabled: true, "Billing")
                        tab(panel: "ws-settings", icon: Icon::Settings, "Settings")
                    )
                    tab_panel(id: "ws-files", selected: true, <p>"Files panel."</p>)
                    tab_panel(id: "ws-activity", <p>"Activity panel."</p>)
                    tab_panel(id: "ws-billing", <p>"Billing panel."</p>)
                    tab_panel(id: "ws-settings", <p>"Settings panel."</p>)
                )
            </div>
        )
    })
}
