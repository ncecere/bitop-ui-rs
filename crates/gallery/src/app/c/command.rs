use bitop_ui::components::{
    command::{command_group, command_item, command_palette},
    dialog::dialog_trigger,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let palette_id = element_id(cx);

    Ok(view! {
        page_header(slug: "command")
        example(
            title: "Open with the button or Ctrl+K",
            dialog_trigger(target: &palette_id, "Open palette")
            command_palette(
                id: &palette_id,
                label: "Command palette",
                shortcut: "k",
                command_group(
                    label: "Go to",
                    command_item(href: "/", "Home")
                    command_item(href: "/c/button", keywords: "components", "Button")
                )
                command_group(
                    label: "Actions",
                    command_item(
                        attrs: attributes! { data-gallery-toast="success" },
                        "Save changes"
                    )
                    command_item(
                        attrs: attributes! { data-gallery-toast="neutral" },
                        "Copy link"
                    )
                )
            )
        )
    })
}
