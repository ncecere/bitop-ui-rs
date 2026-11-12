use bitop_ui::components::tooltip::{tooltip, tooltip_trigger};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let tooltip_id = element_id(cx);

    Ok(view! {
        page_header(slug: "tooltip")
        example(
            title: "On hover and focus",
            tooltip_trigger(target: &tooltip_id, "Share")
            tooltip(id: &tooltip_id, "Anyone with the link can read this chat")
        )
    })
}
