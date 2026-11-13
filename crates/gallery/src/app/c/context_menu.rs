use bitop_ui::components::{
    button::button,
    context_menu::{
        MenuItemTone, context_menu, context_menu_checkbox_item, context_menu_group,
        context_menu_item, context_menu_radio_group, context_menu_radio_item,
        context_menu_separator, context_menu_submenu, context_menu_trigger,
    },
    icon::{Icon, icon},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

const FILE_ROW: &str = "display: flex; align-items: center; gap: var(--space-3); inline-size: min(100%, 24rem); padding: var(--space-3) var(--space-4); border-radius: var(--radius-control); background-color: var(--color-surface); box-shadow: var(--shadow-1); font-size: var(--font-size-md)";
const OPTIONS_AREA: &str = "display: grid; place-items: center; inline-size: min(100%, 22rem); block-size: 9rem; border: 1px dashed var(--color-border-strong); border-radius: var(--radius-card); background-color: var(--color-surface); color: var(--color-text-muted); font-size: var(--font-size-sm); text-align: center";

const AREA: &str = "display: grid; place-items: center; inline-size: 20rem; max-inline-size: 100%; block-size: 9rem; border: 1.5px dashed var(--color-border-strong); border-radius: var(--radius-card); color: var(--color-text-muted)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let file_id = element_id(cx);
    let share_id = element_id(cx);
    let options_id = element_id(cx);
    let canvas_id = element_id(cx);
    let row_id = element_id(cx);
    let off_id = element_id(cx);
    let layer_id = element_id(cx);
    let arrange_id = element_id(cx);

    Ok(view! {
        page_header(slug: "context-menu")
        example(
            title: "File actions",
            context_menu_trigger(
                target: &file_id,
                attrs: attributes! { style=(FILE_ROW) },
                <span
                    style="display: inline-flex; color: var(--color-text-muted); font-size: 1rem"
                >
                    icon(name: Icon::FileText)
                </span>
                "quarterly-report.pdf"
                <span
                    style="margin-inline-start: auto; color: var(--color-text-muted); font-size: var(--font-size-sm)"
                >
                    "2.4 MB"
                </span>
            )
            context_menu(
                id: &file_id,
                label: "File actions",
                context_menu_item(icon: Icon::Pencil, shortcut: "F2", "Rename")
                context_menu_item(icon: Icon::Copy, shortcut: "⌘D", "Duplicate")
                context_menu_item(
                    href: "#file-actions",
                    icon: Icon::Download,
                    "Download"
                )
                context_menu_submenu(
                    id: &share_id,
                    label: "Share",
                    icon: Icon::Share2,
                    context_menu_item("Copy link")
                    context_menu_item("Email…")
                    context_menu_item("Slack…")
                )
                context_menu_separator()
                context_menu_item(
                    tone: MenuItemTone::Danger,
                    icon: Icon::Trash2,
                    shortcut: "⌫",
                    "Move to trash"
                )
            )
        )
        example(
            title: "Checkbox and radio items",
            context_menu_trigger(
                target: &options_id,
                attrs: attributes! { style=(OPTIONS_AREA) },
                "Right-click (or long-press) in this area"
            )
            context_menu(
                id: &options_id,
                label: "View options",
                context_menu_group(
                    label: "View",
                    context_menu_checkbox_item(shortcut: "⌘⇧.", "Show hidden files")
                    context_menu_checkbox_item(checked: true, "Show file extensions")
                )
                context_menu_separator()
                context_menu_radio_group(
                    label: "Sort by",
                    context_menu_radio_item(value: "name", checked: true, "Name")
                    context_menu_radio_item(value: "modified", "Date modified")
                    context_menu_radio_item(value: "size", "Size")
                )
                context_menu_separator()
                context_menu_item(disabled: true, "Paste (clipboard is empty)")
            )
        )
        example(
            title: "An area that can be focused",
            context_menu_trigger(
                target: &canvas_id,
                attrs: attributes! {
                    tabindex="0"
                    role="group"
                    aria-label="Canvas"
                    style=(AREA)
                },
                "Right-click here, or press Shift+F10"
            )
            context_menu(
                id: &canvas_id,
                label: "Canvas actions",
                context_menu_item(icon: Icon::Copy, shortcut: "⌘C", "Copy")
                context_menu_item("Paste")
                context_menu_item(disabled: true, "Paste in place")
                context_menu_item("Duplicate")
                context_menu_separator()
                context_menu_item(
                    tone: MenuItemTone::Danger,
                    icon: Icon::Trash2,
                    "Delete"
                )
            )
        )
        example(
            title: "Around a control",
            context_menu_trigger(
                target: &row_id,
                attrs: attributes! {
                    style="padding: var(--space-4); border: 1px solid var(--color-border); border-radius: var(--radius-card)"
                },
                button("Quarterly report")
            )
            context_menu(
                id: &row_id,
                label: "Report actions",
                context_menu_item("Open")
                context_menu_item("Rename")
                context_menu_item(href: "/c/context-menu?download=1", "Download")
            )
        )
        example(
            title: "Settings, a group and a submenu",
            context_menu_trigger(
                target: &layer_id,
                attrs: attributes! {
                    tabindex="0"
                    role="group"
                    aria-label="Layer"
                    style=(AREA)
                },
                "Right-click this layer"
            )
            context_menu(
                id: &layer_id,
                label: "Layer actions",
                context_menu_checkbox_item(checked: true, "Visible")
                context_menu_checkbox_item("Locked")
                context_menu_separator()
                context_menu_radio_group(
                    label: "Blend mode",
                    context_menu_radio_item(value: "normal", checked: true, "Normal")
                    context_menu_radio_item(value: "multiply", "Multiply")
                )
                context_menu_separator()
                context_menu_group(
                    label: "Order",
                    context_menu_submenu(
                        id: &arrange_id,
                        label: "Arrange",
                        context_menu_item("Bring to front")
                        context_menu_item("Send to back")
                    )
                )
            )
        )
        example(
            title: "Disabled: the browser's own menu shows",
            context_menu_trigger(
                target: &off_id,
                disabled: true,
                attrs: attributes! { style=(AREA) },
                "Nothing of ours opens here"
            )
            context_menu(id: &off_id, context_menu_item("Never shown"))
        )
    })
}
