use bitop_ui::components::{
    field::{field, field_description, field_label},
    number_input::number_input,
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
    let limit_id = element_id(cx);
    let limit_help_id = element_id(cx);
    let price_id = element_id(cx);
    let seats_id = element_id(cx);
    Ok(view! {
        page_header(slug: "number-input")
        example(
            title: "Grouping, steps and a unit",
            <form id="limit-form" style="inline-size: 100%; max-inline-size: 20rem">
                field(
                    field_label(target: &limit_id, "Monthly limit")
                    number_input(
                        name: "limit",
                        value: "50000",
                        min: "0",
                        max: "100000",
                        step: "1000",
                        fraction_digits: 0,
                        locale: "en-US",
                        unit: "tokens",
                        attrs: attributes! { id=(&limit_id) aria-describedby=(&limit_help_id) }
                    )
                    field_description(
                        id: &limit_help_id,
                        "In tokens. Leave empty for no limit."
                    )
                )
            </form>
        )
        example(
            title: "Decimals, in German",
            <form id="price-form" style="inline-size: 100%; max-inline-size: 20rem">
                field(
                    field_label(target: &price_id, "Price in euros")
                    number_input(
                        name: "price",
                        value: "1234.5",
                        step: "0.5",
                        locale: "de-DE",
                        unit: "EUR",
                        attrs: attributes! { id=(&price_id) }
                    )
                )
            </form>
        )
        example(
            title: "Without step buttons",
            <div style="inline-size: 100%; max-inline-size: 20rem">
                field(
                    field_label(target: &seats_id, "Seats")
                    number_input(
                        name: "seats",
                        value: "12",
                        min: "1",
                        fraction_digits: 0,
                        hide_steppers: true,
                        attrs: attributes! { id=(&seats_id) }
                    )
                )
            </div>
        )
        example(
            title: "Disabled",
            number_input(
                name: "quota",
                value: "2500",
                disabled: true,
                attrs: attributes! { aria-label="Quota, disabled" }
            )
        )
    })
}
