use bitop_ui::components::menu::{
    MenuItemVariant, menu, menu_checkbox_item, menu_group, menu_item, menu_radio_group,
    menu_radio_item, menu_separator, menu_submenu, menu_trigger,
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
    let menu_id = element_id(cx);
    let view_id = element_id(cx);
    let export_id = element_id(cx);
    let image_id = element_id(cx);

    Ok(view! {
        page_header(slug: "menu")
        example(
            title: "Actions and a link",
            menu_trigger(target: &menu_id, "Options")
            menu(
                id: &menu_id,
                menu_item("Rename")
                menu_item("Duplicate")
                menu_item(disabled: true, "Archive")
                menu_item(href: "/", "Home")
                menu_separator()
                menu_item(variant: MenuItemVariant::Danger, "Delete")
            )
        )
        example(
            title: "Settings, groups and submenus, in a form",
            <form action="/c/menu" data-menu-form="">
                menu_trigger(target: &view_id, "View")
                menu(
                    id: &view_id,
                    menu_checkbox_item(name: "sidebar", checked: true, "Show sidebar")
                    menu_checkbox_item(name: "minimap", "Show minimap")
                    menu_checkbox_item(disabled: true, "Show rulers")
                    menu_separator()
                    menu_radio_group(
                        label: "Density",
                        menu_radio_item(name: "density", value: "compact", "Compact")
                        menu_radio_item(
                            name: "density",
                            value: "comfortable",
                            checked: true,
                            "Comfortable"
                        )
                        menu_radio_item(name: "density", value: "spacious", "Spacious")
                    )
                    menu_separator()
                    menu_group(
                        label: "Share",
                        menu_item("Copy link")
                        menu_submenu(
                            id: &export_id,
                            label: "Export as",
                            menu_item("PDF")
                            menu_item("Markdown")
                            menu_submenu(
                                id: &image_id,
                                label: "Image",
                                menu_item("PNG")
                                menu_item("SVG")
                            )
                        )
                        menu_submenu(
                            id: "menu-unused",
                            label: "Publish to",
                            disabled: true,
                            menu_item("Web")
                        )
                    )
                )
            </form>
        )
    })
}
