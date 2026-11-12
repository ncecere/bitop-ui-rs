use bitop_ui::components::{
    checkbox::{checkbox, switch},
    field::field_choice,
};
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
        page_header(slug: "checkbox")
        example(
            title: "Checkbox",
            field_choice(
                checkbox(attrs: attributes! { name="terms" })
                "Accept the policy"
            )
            field_choice(
                checkbox(attrs: attributes! { name="updates" checked=(true) })
                "Checked"
            )
            field_choice(
                checkbox(attrs: attributes! { disabled=(true) })
                "Disabled"
            )
        )
        example(
            title: "Switch",
            field_choice(
                switch(attrs: attributes! { name="memory" })
                "Use memory"
            )
            field_choice(
                switch(attrs: attributes! { name="search" checked=(true) })
                "Web search"
            )
            field_choice(
                switch(attrs: attributes! { disabled=(true) })
                "Disabled"
            )
        )
    })
}
