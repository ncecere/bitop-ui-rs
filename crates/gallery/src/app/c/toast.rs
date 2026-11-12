use bitop_ui::components::button::{ButtonVariant, button};
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
        page_header(slug: "toast")
        example(
            title: "Raised by a script",
            button(
                variant: ButtonVariant::Outline,
                attrs: attributes! { data-gallery-toast="neutral" },
                "Show toast"
            )
            button(
                variant: ButtonVariant::Outline,
                attrs: attributes! { data-gallery-toast="success" },
                "Show success"
            )
            button(
                variant: ButtonVariant::Outline,
                attrs: attributes! { data-gallery-toast="danger" },
                "Show error"
            )
        )
    })
}
