use bitop_ui::components::{
    button::button,
    field::{
        FormActionsAlign, field, field_description, field_error, field_label, form, form_actions,
    },
    input_otp::{InputOtpSize, OtpNormalize, OtpValidation, input_otp},
    layout::stack,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    // The form sends the code back to this page, which checks it.
    let code = query_param(cx, "code");
    let verified = code.as_deref() == Some("424242");
    let wrong = code.is_some() && !verified;
    let described_by = if wrong {
        "code-description code-error"
    } else {
        "code-description"
    };
    Ok(view! {
        page_header(slug: "input-otp")
        example(
            title: "Verification code in a form",
            form(
                attrs: attributes! { id="code-form" action="/c/input-otp" },
                field(
                    field_label(target: "code", "Verification code")
                    input_otp(
                        id: "code",
                        name: "code",
                        length: 6,
                        groups: &[3, 3],
                        invalid: wrong,
                        attrs: attributes! { aria-describedby=(described_by) }
                    )
                    field_description(
                        id: "code-description",
                        if verified {
                            "Verified. You can close this window."
                        } else {
                            "Enter the 6-digit code we sent to •••• 4821. Try 424242."
                        }
                    )
                    if wrong {
                        field_error(
                            id: "code-error",
                            "That code is incorrect or has expired."
                        )
                    }
                )
                form_actions(
                    align: FormActionsAlign::Start,
                    button(attrs: attributes! { type="submit" }, "Verify")
                )
            )
        )
        example(
            title: "Alphanumeric, masked, small and disabled",
            stack(
                gap: 5,
                field(
                    field_label(target: "recovery", "Recovery code")
                    input_otp(
                        id: "recovery",
                        name: "recovery",
                        length: 8,
                        groups: &[4, 4],
                        validation: OtpValidation::Alphanumeric,
                        normalize: OtpNormalize::Upper,
                        attrs: attributes! { aria-describedby="recovery-description" }
                    )
                    field_description(
                        id: "recovery-description",
                        "Letters and numbers, e.g. A7C9-XZ42."
                    )
                )
                field(
                    field_label(target: "pin", "PIN")
                    input_otp(
                        id: "pin",
                        name: "pin",
                        length: 4,
                        mask: true,
                        size: InputOtpSize::Sm,
                        attrs: attributes! { aria-describedby="pin-description" }
                    )
                    field_description(id: "pin-description", "Hidden as you type.")
                )
                field(
                    field_label(target: "locked", "Disabled")
                    input_otp(
                        id: "locked",
                        name: "locked",
                        length: 6,
                        value: "123",
                        disabled: true
                    )
                )
            )
        )
        example(
            title: "No visible label",
            input_otp(
                id: "invite",
                name: "invite",
                length: 5,
                label: "Invite code",
                value: "12"
            )
        )
    })
}
