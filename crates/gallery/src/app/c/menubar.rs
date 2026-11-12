use bitop_ui::components::menubar::{
    MenuItemVariant, menubar, menubar_item, menubar_menu, menubar_separator,
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
    let file_id = element_id(cx);
    let edit_id = element_id(cx);
    let view_id = element_id(cx);
    let window_id = element_id(cx);
    let help_id = element_id(cx);
    let draw_id = element_id(cx);
    let text_id = element_id(cx);

    Ok(view! {
        page_header(slug: "menubar")
        example(
            title: "An editor's menus",
            menubar(
                label: "Editor",
                menubar_menu(
                    id: &file_id,
                    label: "File",
                    menubar_item("New file")
                    menubar_item("Open")
                    menubar_item(disabled: true, "Save")
                    menubar_separator()
                    menubar_item("Close")
                )
                menubar_menu(
                    id: &edit_id,
                    label: "Edit",
                    menubar_item("Undo")
                    menubar_item("Redo")
                    menubar_separator()
                    menubar_item("Cut")
                    menubar_item("Copy")
                    menubar_item("Paste")
                )
                menubar_menu(
                    id: &view_id,
                    label: "View",
                    menubar_item("Zoom in")
                    menubar_item("Zoom out")
                    menubar_item(href: "/c/menubar?full=1", "Full screen")
                )
                menubar_menu(
                    id: &window_id,
                    label: "Window",
                    disabled: true,
                    menubar_item("Minimize")
                )
                menubar_menu(
                    id: &help_id,
                    label: "Help",
                    menubar_item("Documentation")
                    menubar_item(variant: MenuItemVariant::Danger, "Report a problem")
                )
            )
        )
        example(
            title: "Vertical",
            menubar(
                label: "Tools",
                vertical: true,
                menubar_menu(
                    id: &draw_id,
                    label: "Draw",
                    menubar_item("Pencil")
                    menubar_item("Brush")
                )
                menubar_menu(
                    id: &text_id,
                    label: "Text",
                    menubar_item("Heading")
                    menubar_item("Paragraph")
                )
            )
        )
    })
}
