use bitop_ui::components::{
    button::ButtonVariant,
    field::{field, field_error, field_label},
    icon::{Icon, icon},
    input_group::{
        InputGroupAlign, InputGroupSize, input_group, input_group_addon, input_group_button,
        input_group_input, input_group_text, input_group_textarea,
    },
    kbd::kbd,
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
    let site_id = element_id(cx);
    let amount_id = element_id(cx);
    let amount_error_id = element_id(cx);
    let message_id = element_id(cx);
    Ok(view! {
        page_header(slug: "input-group")
        example(
            title: "Text and a button",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                field(
                    field_label(target: &site_id, "Website")
                    input_group(
                        input_group_addon(input_group_text("https://"))
                        input_group_input(
                            attrs: attributes! { id=(&site_id) name="site" placeholder="example.com" }
                        )
                        input_group_addon(
                            align: InputGroupAlign::End,
                            input_group_button(
                                icon_only: true,
                                attrs: attributes! { aria-label="Copy URL" },
                                icon(name: Icon::Copy)
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Icon and keyboard hint, small",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                input_group(
                    size: InputGroupSize::Sm,
                    input_group_addon(icon(name: Icon::Search))
                    input_group_input(
                        attrs: attributes! {
                            type="search"
                            name="q"
                            aria-label="Search"
                            placeholder="Search"
                        }
                    )
                    input_group_addon(align: InputGroupAlign::End, kbd("/"))
                )
            </div>
        )
        example(
            title: "Invalid",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                field(
                    field_label(target: &amount_id, "Amount")
                    input_group(
                        input_group_input(
                            attrs: attributes! {
                                id=(&amount_id)
                                name="amount"
                                value="abc"
                                aria-invalid="true"
                                aria-describedby=(&amount_error_id)
                            }
                        )
                        input_group_addon(
                            align: InputGroupAlign::End,
                            input_group_text("USD")
                        )
                    )
                    field_error(id: &amount_error_id, "Enter a number.")
                )
            </div>
        )
        example(
            title: "Textarea with a toolbar below",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                field(
                    field_label(target: &message_id, "Message")
                    input_group(
                        input_group_textarea(
                            attrs: attributes! {
                                id=(&message_id)
                                name="message"
                                placeholder="Ask anything"
                            }
                        )
                        input_group_addon(
                            align: InputGroupAlign::BlockEnd,
                            input_group_text("0 / 280")
                            <span style="flex: 1"></span>
                            input_group_button(
                                variant: ButtonVariant::Primary,
                                icon(name: Icon::SendHorizontal)
                                "Send"
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Disabled and read-only",
            <div
                style="display: grid; gap: var(--space-3); inline-size: 100%; max-inline-size: 24rem"
            >
                input_group(
                    input_group_addon(input_group_text("@"))
                    input_group_input(
                        attrs: attributes! {
                            aria-label="Handle, disabled"
                            value="ada"
                            disabled=(true)
                        }
                    )
                )
                input_group(
                    input_group_addon(input_group_text("@"))
                    input_group_input(
                        attrs: attributes! {
                            aria-label="Handle, read-only"
                            value="ada"
                            readonly=(true)
                        }
                    )
                )
            </div>
        )
    })
}
