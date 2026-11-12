use bitop_ui::components::{
    button::button,
    context_menu::{
        MenuItemVariant, context_menu, context_menu_item, context_menu_separator,
        context_menu_trigger,
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

const AREA: &str = "display: grid; place-items: center; inline-size: 20rem; max-inline-size: 100%; block-size: 9rem; border: 1.5px dashed var(--color-border-strong); border-radius: var(--radius-card); color: var(--color-text-muted)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let canvas_id = element_id(cx);
    let row_id = element_id(cx);
    let off_id = element_id(cx);

    Ok(view! {
        page_header(slug: "context-menu")
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
                context_menu_item("Copy")
                context_menu_item("Paste")
                context_menu_item(disabled: true, "Paste in place")
                context_menu_item("Duplicate")
                context_menu_separator()
                context_menu_item(variant: MenuItemVariant::Danger, "Delete")
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
