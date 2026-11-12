use bitop_ui::components::{
    field::{field, field_description, field_error, field_label},
    input::{input, textarea},
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
    let name_id = element_id(cx);
    let name_help_id = element_id(cx);
    let email_id = element_id(cx);
    let email_error_id = element_id(cx);
    let notes_id = element_id(cx);

    Ok(view! {
        page_header(slug: "input")
        example(
            title: "With a label and help text",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                field(
                    field_label(target: &name_id, "Display name")
                    input(
                        attrs: attributes! {
                            id=(&name_id)
                            name="name"
                            placeholder="Ada Lovelace"
                            aria-describedby=(&name_help_id)
                        }
                    )
                    field_description(id: &name_help_id, "Shown next to your messages.")
                )
            </div>
        )
        example(
            title: "Invalid",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                field(
                    field_label(target: &email_id, "Email")
                    input(
                        attrs: attributes! {
                            id=(&email_id)
                            type="email"
                            name="email"
                            value="ada@"
                            aria-invalid="true"
                            aria-describedby=(&email_error_id)
                        }
                    )
                    field_error(id: &email_error_id, "Enter a full email address.")
                )
            </div>
        )
        example(
            title: "Textarea",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                field(
                    field_label(target: &notes_id, "Custom instructions")
                    textarea(
                        attrs: attributes! {
                            id=(&notes_id)
                            name="notes"
                            placeholder="How should the assistant respond?"
                        }
                    )
                )
            </div>
        )
        example(
            title: "Disabled",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                input(
                    attrs: attributes! {
                        aria-label="Disabled input"
                        value="Disabled"
                        disabled=(true)
                    }
                )
            </div>
        )
    })
}
