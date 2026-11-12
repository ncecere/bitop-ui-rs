use bitop_ui::components::badge::{BadgeVariant, badge};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "badge")
        example(
            title: "Variants",
            badge("Draft")
            badge(variant: BadgeVariant::Accent, "New")
            badge(variant: BadgeVariant::Success, "Active")
            badge(variant: BadgeVariant::Warning, "Over quota")
            badge(variant: BadgeVariant::Danger, "Suspended")
        )
    })
}
