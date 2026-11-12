use bitop_ui::components::{
    combobox::{combobox, combobox_option},
    field::{field, field_label},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let combobox_id = element_id(cx);

    Ok(view! {
        page_header(slug: "combobox")
        example(
            title: "Filter and choose",
            <div style="inline-size: 100%; max-inline-size: 18rem">
                field(
                    field_label(target: &combobox_id, "Model")
                    combobox(
                        id: &combobox_id,
                        name: "model",
                        label: "Models",
                        placeholder: "Choose a model",
                        combobox_option(value: "atlas-large", "Atlas Large")
                        combobox_option(value: "atlas-small", "Atlas Small")
                        combobox_option(value: "borealis", "Borealis")
                        combobox_option(value: "cirrus", disabled: true, "Cirrus")
                        combobox_option(value: "meridian", "Meridian")
                    )
                )
            </div>
        )
    })
}
