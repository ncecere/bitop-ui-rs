use bitop_ui::components::popover::{popover, popover_trigger};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let popover_id = element_id(cx);

    Ok(view! {
        page_header(slug: "popover")
        example(
            title: "Anchored to its trigger",
            popover_trigger(target: &popover_id, "Filters")
            popover(
                id: &popover_id,
                <p>"Anchored by the browser, with no positioning script."</p>
            )
        )
    })
}
