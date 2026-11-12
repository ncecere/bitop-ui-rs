use bitop_ui::components::{
    field::{field, field_description, field_label},
    input_otp::{InputOtpSize, input_otp},
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
    let code_id = element_id(cx);
    let code_help_id = element_id(cx);
    let pin_id = element_id(cx);
    let invite_id = element_id(cx);
    let wrong_id = element_id(cx);
    let locked_id = element_id(cx);
    Ok(view! {
        page_header(slug: "input-otp")
        example(
            title: "Six digits in two groups",
            <form id="code-form">
                field(
                    field_label(target: &code_id, "Verification code")
                    input_otp(
                        id: &code_id,
                        name: "code",
                        length: 6,
                        groups: &[3, 3],
                        attrs: attributes! { aria-describedby=(&code_help_id) }
                    )
                    field_description(
                        id: &code_help_id,
                        "We sent it to the number ending in 4821."
                    )
                )
            </form>
        )
        example(
            title: "Small, with a value and no visible label",
            input_otp(
                id: &pin_id,
                name: "pin",
                length: 4,
                value: "12",
                label: "PIN",
                size: InputOtpSize::Sm
            )
        )
        example(
            title: "Letters and digits",
            input_otp(
                id: &invite_id,
                name: "invite",
                length: 5,
                label: "Invite code",
                alphanumeric: true
            )
        )
        example(
            title: "Invalid, and disabled",
            input_otp(
                id: &wrong_id,
                name: "wrong",
                length: 4,
                value: "9999",
                label: "Wrong code",
                invalid: true
            )
            input_otp(
                id: &locked_id,
                name: "locked",
                length: 4,
                value: "1234",
                label: "Disabled code",
                disabled: true
            )
        )
    })
}
