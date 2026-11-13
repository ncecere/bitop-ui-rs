use bitop_ui::components::{
    icon::Icon,
    menu::{
        MenuItemTone, menu, menu_checkbox_item, menu_group, menu_header, menu_item,
        menu_radio_group, menu_radio_item, menu_separator, menu_submenu, menu_trigger,
    },
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
    let actions_id = element_id(cx);
    let view_id = element_id(cx);
    let more_id = element_id(cx);
    let account_id = element_id(cx);
    let export_id = element_id(cx);
    let image_id = element_id(cx);

    Ok(view! {
        page_header(slug: "menu")
        example(
            title: "Actions menu",
            menu_trigger(target: &actions_id, "Actions")
            menu(
                id: &actions_id,
                menu_group(
                    label: "Project",
                    menu_item(icon: Icon::Pencil, shortcut: "R", "Rename")
                    menu_item(icon: Icon::Share2, "Share")
                    menu_item(href: "#actions", icon: Icon::FileClock, "View audit log")
                )
                menu_separator()
                menu_item(disabled: true, icon: Icon::Settings, "Settings (no access)")
                menu_item(
                    tone: MenuItemTone::Danger,
                    icon: Icon::Trash2,
                    shortcut: "⌫",
                    "Delete"
                )
            )
        )
        example(
            title: "Checkbox, radio and submenu items",
            <form action="/c/menu" data-menu-form="">
                menu_trigger(target: &view_id, "View")
                menu(
                    id: &view_id,
                    menu_group(
                        label: "Columns",
                        menu_checkbox_item(name: "status", checked: true, "Status")
                        menu_checkbox_item(name: "branch", checked: true, "Branch")
                        menu_checkbox_item(name: "author", "Author")
                    )
                    menu_separator()
                    menu_radio_group(
                        label: "Density",
                        menu_radio_item(
                            name: "density",
                            value: "comfortable",
                            checked: true,
                            "Comfortable"
                        )
                        menu_radio_item(name: "density", value: "compact", "Compact")
                    )
                    menu_separator()
                    menu_submenu(
                        id: &more_id,
                        label: "More options",
                        icon: Icon::SlidersHorizontal,
                        menu_item("Reset columns")
                        menu_item("Export as CSV")
                    )
                )
            </form>
        )
        example(
            title: "A header, nested submenus and disabled items",
            menu_trigger(target: &account_id, "Account")
            menu(
                id: &account_id,
                menu_header(
                    <strong>"Ada Lovelace"</strong>
                    "ada@example.com"
                )
                menu_separator()
                menu_checkbox_item(disabled: true, "Show rulers")
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
                menu_item(href: "/", "Home")
            )
        )
    })
}
