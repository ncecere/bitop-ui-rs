use bitop_ui::components::{
    button::{ButtonVariant, button},
    button_group::{button_group, button_group_separator, button_group_text},
    icon::{Icon, icon},
    input::input,
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
        page_header(slug: "button-group")
        example(
            title: "Joined",
            button_group(
                label: "Pagination",
                button(variant: ButtonVariant::Secondary, "Previous")
                button_group_text("Page 2 of 9")
                button(variant: ButtonVariant::Secondary, "Next")
            )
        )
        example(
            title: "With a separator",
            button_group(
                label: "Save options",
                button("Save")
                button_group_separator()
                button(
                    icon_only: true,
                    attrs: attributes! { aria-label="More save options" },
                    icon(name: Icon::ChevronDown)
                )
            )
        )
        example(
            title: "With an input",
            button_group(
                label: "Invite by email",
                attrs: attributes! { style="inline-size: 100%; max-inline-size: 24rem" },
                button_group_text("To")
                input(
                    attrs: attributes! {
                        type="email"
                        name="invite"
                        aria-label="Email address"
                        placeholder="ada@example.com"
                    }
                )
                button(variant: ButtonVariant::Secondary, "Invite")
            )
        )
        example(
            title: "Vertical",
            button_group(
                label: "Zoom",
                vertical: true,
                button(
                    variant: ButtonVariant::Secondary,
                    icon_only: true,
                    attrs: attributes! { aria-label="Zoom in" },
                    icon(name: Icon::Plus)
                )
                button(
                    variant: ButtonVariant::Secondary,
                    icon_only: true,
                    attrs: attributes! { aria-label="Zoom out" },
                    icon(name: Icon::Minus)
                )
            )
        )
        example(
            title: "A spaced group of groups",
            button_group(
                label: "Editor toolbar",
                spaced: true,
                button_group(
                    label: "History",
                    button(variant: ButtonVariant::Secondary, "Undo")
                    button(variant: ButtonVariant::Secondary, "Redo")
                )
                button_group(
                    label: "Clipboard",
                    button(variant: ButtonVariant::Secondary, "Cut")
                    button(variant: ButtonVariant::Secondary, "Copy")
                    button(
                        variant: ButtonVariant::Secondary,
                        attrs: attributes! { disabled=(true) },
                        "Paste"
                    )
                )
            )
        )
    })
}
