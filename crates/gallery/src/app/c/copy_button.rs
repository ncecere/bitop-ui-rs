use bitop_ui::components::{button::ButtonVariant, copy_button::copy_button, input::textarea};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let tooltip_id = element_id(cx);
    let code_id = element_id(cx);
    Ok(view! {
        page_header(slug: "copy-button")
        example(
            title: "Icon only",
            <code>"npm install bitop-ui"</code>
            copy_button(value: "npm install bitop-ui", label: "command")
        )
        example(
            title: "With a tooltip",
            copy_button(value: "sk-test-1234", label: "key", tooltip: &tooltip_id)
        )
        example(
            title: "With text, copying another element",
            <code id=(&code_id)>"cargo add bitop-ui"</code>
            copy_button(
                target: &code_id,
                label: "code",
                show_text: true,
                variant: ButtonVariant::Secondary,
                timeout_ms: 600
            )
        )
        example(
            title: "Paste here to try it",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                textarea(attrs: attributes! { aria-label="Paste target" })
            </div>
        )
    })
}
