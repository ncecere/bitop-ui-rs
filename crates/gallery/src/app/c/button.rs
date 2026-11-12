use bitop_ui::components::button::{ButtonSize, ButtonVariant, button};
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
            button("Solid")
            button(variant: ButtonVariant::Outline, "Outline")
            button(variant: ButtonVariant::Ghost, "Ghost")
            button(variant: ButtonVariant::Danger, "Danger")
        )
        example(
            title: "Sizes",
            button(size: ButtonSize::Sm, "Small")
            button("Medium")
            button(size: ButtonSize::Lg, "Large")
        )
        example(
            title: "Disabled",
            button(attrs: attributes! { disabled=(true) }, "Disabled")
        )
    })
}
