use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button, icon_button},
    icon::{Icon, icon},
    tooltip::tooltip,
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
        page_header(slug: "button")
        example(
            title: "Variants",
            button("Create project")
            button(variant: ButtonVariant::Secondary, "Cancel")
            button(variant: ButtonVariant::Ghost, "Dismiss")
            button(variant: ButtonVariant::Danger, "Delete")
            button(variant: ButtonVariant::Link, "View details")
        )
        example(
            title: "Sizes and icons",
            button(
                size: ButtonSize::Sm,
                icon(name: Icon::Plus)
                " Small"
            )
            button(
                icon(name: Icon::Plus)
                " Medium"
            )
            button(
                variant: ButtonVariant::Secondary,
                size: ButtonSize::Sm,
                icon(name: Icon::Share2)
                " Share"
            )
            button(
                variant: ButtonVariant::Secondary,
                "Continue "
                icon(name: Icon::ArrowRight)
            )
        )
        example(
            title: "Loading and disabled",
            button(attrs: attributes! { data-gallery-loading="" }, "Save changes")
            button(variant: ButtonVariant::Secondary, loading: true, "Uploading")
            button(disabled: true, "Disabled")
        )
        example(
            title: "Icon buttons",
            icon_button(
                label: "Edit",
                attrs: attributes! { aria-describedby="tip-edit" data-tooltip="tip-edit" },
                icon(name: Icon::Pencil)
            )
            tooltip(id: "tip-edit", "Edit")
            icon_button(
                label: "Copy",
                variant: ButtonVariant::Secondary,
                attrs: attributes! { aria-describedby="tip-copy" data-tooltip="tip-copy" },
                icon(name: Icon::Copy)
            )
            tooltip(id: "tip-copy", "Copy")
            icon_button(
                label: "Delete",
                variant: ButtonVariant::Danger,
                attrs: attributes! { aria-describedby="tip-delete" data-tooltip="tip-delete" },
                icon(name: Icon::Trash2)
            )
            tooltip(id: "tip-delete", "Delete")
            icon_button(
                label: "Add",
                variant: ButtonVariant::Primary,
                size: ButtonSize::Sm,
                icon(name: Icon::Plus)
            )
        )
        example(
            title: "As a link",
            <a
                href="#as-a-link"
                class=(bitop_ui::components::button::BUTTON)
                data-variant="secondary"
                data-size="md"
            >
                "Button rendered as a link"
            </a>
        )
        example(
            title: "Full width",
            <div style="inline-size: 100%">button(block: true, "Continue")</div>
        )
    })
}
