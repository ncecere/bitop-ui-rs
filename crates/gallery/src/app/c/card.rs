use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    card::{card, card_body, card_footer, card_header},
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
    let title_id = element_id(cx);

    Ok(view! {
        page_header(slug: "card")
        example(
            title: "Header, body and footer",
            card(
                labelled_by: &title_id,
                attrs: attributes! { style="inline-size: 100%" },
                card_header(
                    title: "Members",
                    title_id: &title_id,
                    description: "People with access to this workspace.",
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        "Invite"
                    )
                )
                card_body("Ada Lovelace, Grace Hopper and Alan Turing.")
                card_footer("3 members")
            )
        )
        example(
            title: "Body only",
            card(
                attrs: attributes! { style="inline-size: 100%" },
                card_body("A plain card.")
            )
        )
    })
}
