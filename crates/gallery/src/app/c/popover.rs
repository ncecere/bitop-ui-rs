use bitop_ui::components::{
    popover::{PopupSide, popover, popover_trigger},
    text_link::text_link,
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
    let explainer_id = element_id(cx);
    let hover_id = element_id(cx);

    Ok(view! {
        page_header(slug: "popover")
        example(
            title: "Explainer",
            popover_trigger(target: &explainer_id, "What is a preview deployment?")
            popover(
                id: &explainer_id,
                title: "Preview deployments",
                description: "Every pull request gets its own URL, so reviewers can try changes before they are merged.",
                text_link(href: "#explainer", "Read more")
            )
        )
        example(
            title: "Opens on hover",
            popover_trigger(target: &hover_id, "Build minutes")
            popover(
                id: &hover_id,
                title: "Build minutes",
                side: PopupSide::Right,
                open_on_hover: true,
                "1,240 of 6,000 used this month. Resets on the 1st."
            )
        )
    })
}
