use bitop_ui::components::{
    field::{field, field_label},
    select::select,
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
    let select_id = element_id(cx);

    Ok(view! {
        page_header(slug: "select")
        example(
            title: "With a label",
            field(
                field_label(target: &select_id, "Retention")
                select(
                    attrs: attributes! { id=(&select_id) name="retention" },
                    <option value="30">"30 days"</option>
                    <option value="90" selected=(true)>"90 days"</option>
                    <option value="365">"1 year"</option>
                )
            )
        )
        example(
            title: "Disabled",
            select(
                attrs: attributes! { aria-label="Disabled select" disabled=(true) },
                <option>"Unavailable"</option>
            )
        )
    })
}
