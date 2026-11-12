use bitop_ui::components::{
    icon::{Icon, icon},
    toggle::{ToggleSize, ToggleVariant, toggle},
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
        page_header(slug: "toggle")
        example(
            title: "Variants",
            toggle("Ghost")
            toggle(pressed: true, "Ghost, pressed")
            toggle(variant: ToggleVariant::Outline, "Outline")
            toggle(variant: ToggleVariant::Outline, pressed: true, "Outline, pressed")
        )
        example(
            title: "Sizes and icon only",
            toggle(size: ToggleSize::Sm, variant: ToggleVariant::Outline, "Small")
            toggle(
                icon_only: true,
                variant: ToggleVariant::Outline,
                attrs: attributes! { aria-label="Pin" },
                icon(name: Icon::Check)
            )
            toggle(
                icon_only: true,
                size: ToggleSize::Sm,
                attrs: attributes! { aria-label="Search in files" },
                icon(name: Icon::Search)
            )
        )
        example(
            title: "Disabled",
            toggle(disabled: true, "Disabled")
            toggle(
                disabled: true,
                pressed: true,
                variant: ToggleVariant::Outline,
                "Disabled, pressed"
            )
        )
        example(
            title: "In a form",
            <form id="toggle-form" style="display: flex; gap: var(--space-2)">
                toggle(variant: ToggleVariant::Outline, name: "preview", "Preview")
                toggle(
                    variant: ToggleVariant::Outline,
                    name: "wrap",
                    value: "yes",
                    pressed: true,
                    "Wrap lines"
                )
            </form>
        )
    })
}
