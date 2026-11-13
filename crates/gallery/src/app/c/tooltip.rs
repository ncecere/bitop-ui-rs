use bitop_ui::components::{
    button::{ButtonVariant, icon_button},
    icon::{Icon, icon},
    popover::PopupSide,
    tooltip::{tooltip, tooltip_text},
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
    let bell_id = element_id(cx);
    let copy_id = element_id(cx);
    let score_id = element_id(cx);

    Ok(view! {
        page_header(slug: "tooltip")
        example(
            title: "On icon buttons",
            icon_button(
                label: "Notifications",
                variant: ButtonVariant::Secondary,
                attrs: attributes! { aria-describedby=(&bell_id) data-tooltip=(&bell_id) },
                icon(name: Icon::Bell)
            )
            tooltip(id: &bell_id, side: PopupSide::Bottom, "Notifications")
            icon_button(
                label: "Copy link",
                variant: ButtonVariant::Secondary,
                attrs: attributes! { aria-describedby=(&copy_id) data-tooltip=(&copy_id) },
                icon(name: Icon::Copy)
            )
            tooltip(id: &copy_id, shortcut: "⌘C", "Copy link")
        )
        example(
            title: "On plain text",
            <p>
                "Latest score: "
                tooltip_text(target: &score_id, "82%")
            </p>
            // A <div>, so outside the paragraph.
            tooltip(
                id: &score_id,
                "Recall@5: the share of questions whose expected document was in the top 5 results."
            )
        )
    })
}
