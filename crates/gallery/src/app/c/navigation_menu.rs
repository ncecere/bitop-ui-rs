use bitop_ui::components::{
    icon::Icon,
    navigation_menu::{
        navigation_menu, navigation_menu_content, navigation_menu_item, navigation_menu_link,
        navigation_menu_list, navigation_menu_trigger,
    },
    popover::PopupAlign,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let products_id = element_id(cx);
    let resources_id = element_id(cx);
    let account_id = element_id(cx);

    Ok(view! {
        page_header(slug: "navigation-menu")
        example(
            title: "Panels and links",
            navigation_menu(
                label: "Main",
                navigation_menu_list(
                    navigation_menu_item(
                        navigation_menu_trigger(target: &products_id, "Products")
                        navigation_menu_content(
                            id: &products_id,
                            columns: 2,
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=deployments",
                                description: "Ship from git, with previews.",
                                icon: Some(Icon::ArrowUpRight),
                                "Deployments"
                            )
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=storage",
                                description: "Objects and files at the edge.",
                                icon: Some(Icon::Inbox),
                                "Storage"
                            )
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=search",
                                description: "Full-text search over your data.",
                                icon: Some(Icon::Search),
                                "Search"
                            )
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=status",
                                description: "Uptime and incidents.",
                                icon: Some(Icon::CircleCheck),
                                "Status"
                            )
                        )
                    )
                    navigation_menu_item(
                        navigation_menu_trigger(target: &resources_id, "Resources")
                        navigation_menu_content(
                            id: &resources_id,
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=guides",
                                "Guides"
                            )
                            navigation_menu_link(
                                href: "/c/navigation-menu",
                                active: true,
                                description: "The page you are on.",
                                "Components"
                            )
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=changes",
                                "Changelog"
                            )
                        )
                    )
                    navigation_menu_item(
                        navigation_menu_link(
                            href: "/c/navigation-menu",
                            active: true,
                            "Gallery"
                        )
                    )
                    navigation_menu_item(
                        navigation_menu_link(
                            href: "/c/navigation-menu?p=pricing",
                            "Pricing"
                        )
                    )
                )
            )
        )
        example(
            title: "Panels aligned to the end",
            <div style="display: flex; justify-content: flex-end; inline-size: 100%">
                navigation_menu(
                    label: "Account",
                    align: PopupAlign::End,
                    navigation_menu_list(
                        navigation_menu_item(
                            navigation_menu_link(
                                href: "/c/navigation-menu?p=help",
                                "Help"
                            )
                        )
                        navigation_menu_item(
                            navigation_menu_trigger(target: &account_id, "Account")
                            navigation_menu_content(
                                id: &account_id,
                                navigation_menu_link(
                                    href: "/c/navigation-menu?p=profile",
                                    "Profile"
                                )
                                navigation_menu_link(
                                    href: "/c/navigation-menu?p=billing",
                                    "Billing"
                                )
                            )
                        )
                    )
                )
            </div>
        )
    })
}
