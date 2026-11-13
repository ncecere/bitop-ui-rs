use bitop_ui::components::{
    command::{command_group, command_item, command_palette, command_palette_trigger},
    icon::Icon,
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
            title: "Palette: press Ctrl+K (⌘K) on this page too",
            command_palette_trigger(target: &palette_id, label: "Search or jump to…")
            command_palette(
                id: &palette_id,
                shortcut: "k",
                command_group(
                    label: "Jump to",
                    command_item(
                        href: "/",
                        icon: Icon::ArrowRight,
                        hint: "Page",
                        "Projects"
                    )
                    command_item(
                        href: "/c/button",
                        icon: Icon::ArrowRight,
                        hint: "Page",
                        keywords: "invoices",
                        "Billing"
                    )
                )
                command_group(
                    label: "Actions",
                    command_item(
                        icon: Icon::Bell,
                        attrs: attributes! {
                            data-gallery-toast="success"
                            data-gallery-title="Hello from the palette"
                        },
                        "Show a toast"
                    )
                    command_item(
                        icon: Icon::Moon,
                        shortcut: &["shift", "D"],
                        attrs: attributes! {
                            data-gallery-toast="info"
                            data-gallery-title="Use the header toggle"
                        },
                        "Toggle dark mode"
                    )
                )
                command_group(
                    label: "Settings",
                    command_item(keywords: "people, team", "Add member")
                    command_item(keywords: "rules", "Retention policy")
                    command_item(keywords: "budget, spend", "Usage")
                    command_item("Legal holds")
                )
            )
        )
    })
}
