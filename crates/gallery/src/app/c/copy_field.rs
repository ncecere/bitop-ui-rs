use bitop_ui::components::copy_field::copy_field;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let key_id = element_id(cx);
    let curl_id = element_id(cx);
    let name_id = element_id(cx);
    Ok(view! {
        page_header(slug: "copy-field")
        example(
            title: "A secret, with a description",
            <div style="inline-size: 100%; max-inline-size: 28rem">
                copy_field(
                    id: &key_id,
                    label: "API key",
                    value: "sk-live-4f9a8c2e71b04d6f",
                    description: "Shown once. Store it somewhere safe."
                )
            </div>
        )
        example(
            title: "Multiline",
            <div style="inline-size: 100%; max-inline-size: 28rem">
                copy_field(
                    id: &curl_id,
                    label: "Example request",
                    name: "request",
                    value: "curl https://api.example.com/v1/chat \\\n  -H \"Authorization: Bearer $KEY\" \\\n  -d '{\"message\": \"Hello\"}'",
                    multiline: true
                )
            </div>
        )
        example(
            title: "Not monospace",
            <div style="inline-size: 100%; max-inline-size: 28rem">
                copy_field(
                    id: &name_id,
                    label: "Workspace name",
                    value: "Northwind Research",
                    sans: true
                )
            </div>
        )
    })
}
