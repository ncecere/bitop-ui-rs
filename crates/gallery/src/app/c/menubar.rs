use bitop_ui::components::menubar::{
    MenuItemTone, menubar, menubar_checkbox_item, menubar_group, menubar_item, menubar_menu,
    menubar_radio_group, menubar_radio_item, menubar_separator, menubar_submenu,
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
    let recent_id = element_id(cx);
    let export_id = element_id(cx);
    let edit_id = element_id(cx);
    let view_id = element_id(cx);
    let help_id = element_id(cx);
    let draw_id = element_id(cx);
    let text_id = element_id(cx);
    let format_id = element_id(cx);
    let insert_id = element_id(cx);
    let table_id = element_id(cx);
    let window_id = element_id(cx);
    let tools_id = element_id(cx);

    Ok(view! {
        page_header(slug: "menubar")
        example(
            title: "Editor menubar",
            menubar(
                label: "Editor",
                menubar_menu(
                    id: &file_id,
                    label: "File",
                    menubar_item(shortcut: "⌘N", "New file")
                    menubar_item(shortcut: "⌘O", "Open…")
                    menubar_submenu(
                        id: &recent_id,
                        label: "Open recent",
                        menubar_item("deploy.config.ts")
                        menubar_item("README.md")
                        menubar_separator()
                        menubar_item("Clear recently opened")
                    )
                    menubar_separator()
                    menubar_submenu(
                        id: &export_id,
                        label: "Export as",
                        menubar_item("PDF")
                        menubar_item("HTML")
                        menubar_item("Markdown")
                    )
                    menubar_separator()
                    menubar_item(
                        tone: MenuItemTone::Danger,
                        shortcut: "⌘W",
                        "Close editor"
                    )
                )
                menubar_menu(
                    id: &edit_id,
                    label: "Edit",
                    menubar_item(shortcut: "⌘Z", "Undo")
                    menubar_item(shortcut: "⇧⌘Z", disabled: true, "Redo")
                    menubar_separator()
                    menubar_item(shortcut: "⌘X", "Cut")
                    menubar_item(shortcut: "⌘C", "Copy")
                    menubar_item(shortcut: "⌘V", "Paste")
                )
                menubar_menu(
                    id: &view_id,
                    label: "View",
                    menubar_group(
                        label: "Editor",
                        menubar_checkbox_item(
                            checked: true,
                            shortcut: "⌥Z",
                            "Word wrap"
                        )
                        menubar_checkbox_item("Minimap")
                    )
                    menubar_separator()
                    menubar_radio_group(
                        label: "Bottom panel",
                        menubar_radio_item(value: "terminal", checked: true, "Terminal")
                        menubar_radio_item(value: "problems", "Problems")
                        menubar_radio_item(value: "output", "Output")
                    )
                )
                menubar_menu(
                    id: &help_id,
                    label: "Help",
                    menubar_item(href: "#editor", "Documentation")
                    menubar_item(href: "#editor", "Release notes")
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
        example(
            title: "Settings, groups, submenus and a disabled menu",
            menubar(
                label: "Document",
                menubar_menu(
                    id: &format_id,
                    label: "Format",
                    menubar_checkbox_item(checked: true, "Wrap lines")
                    menubar_checkbox_item("Line numbers")
                    menubar_separator()
                    menubar_radio_group(
                        label: "Indent with",
                        menubar_radio_item(value: "spaces", checked: true, "Spaces")
                        menubar_radio_item(value: "tabs", "Tabs")
                    )
                )
                menubar_menu(
                    id: &insert_id,
                    label: "Insert",
                    menubar_item("Image")
                    menubar_submenu(
                        id: &table_id,
                        label: "Table",
                        menubar_item("Two columns")
                        menubar_item("Three columns")
                    )
                    menubar_group(label: "Text", menubar_item("Footnote"))
                )
                menubar_menu(
                    id: &window_id,
                    label: "Window",
                    disabled: true,
                    menubar_item("Minimize")
                )
                menubar_menu(id: &tools_id, label: "Review", menubar_item("Spelling"))
            )
        )
    })
}
