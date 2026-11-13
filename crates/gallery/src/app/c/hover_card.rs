use bitop_ui::components::{
    hover_card::{HoverCardSide, hover_card, hover_card_trigger},
    popover::PopupAlign,
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
    let ada_id = element_id(cx);
    let repo_id = element_id(cx);
    let side_id = element_id(cx);

    Ok(view! {
        page_header(slug: "hover-card")
        example(
            title: "A profile, with a link inside",
            <p>
                "Reviewed by "
                hover_card_trigger(
                    target: &ada_id,
                    href: "/c/hover-card?user=ada",
                    "@ada"
                )
                hover_card(
                    id: &ada_id,
                    label: "Ada Lovelace",
                    <strong>"Ada Lovelace"</strong>
                    <span>"Mathematician and writer. Joined in 1843."</span>
                    text_link(href: "/c/hover-card?profile=ada", "View profile")
                )
                " last week."
            </p>
        )
        example(
            title: "Above, aligned to the start",
            <p>
                "See "
                hover_card_trigger(
                    target: &repo_id,
                    href: "/c/hover-card?repo=engine",
                    "analytical-engine"
                )
                hover_card(
                    id: &repo_id,
                    label: "analytical-engine",
                    side: HoverCardSide::Top,
                    align: PopupAlign::Start,
                    <strong>"analytical-engine"</strong>
                    <span>"A general-purpose mechanical computer."</span>
                )
                " for the design."
            </p>
        )
        example(
            title: "To the side",
            <p>
                hover_card_trigger(
                    target: &side_id,
                    href: "/c/hover-card?doc=notes",
                    "Notes"
                )
                hover_card(
                    id: &side_id,
                    label: "Notes",
                    side: HoverCardSide::Right,
                    <span>"Notes on the translation of Menabrea's memoir."</span>
                )
            </p>
        )
    })
}
