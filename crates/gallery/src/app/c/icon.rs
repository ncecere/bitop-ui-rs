use bitop_ui::components::icon::{Icon, icon};
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
        page_header(slug: "icon")
        example(
            title: "The set",
            <div
                style="display: flex; flex-wrap: wrap; gap: var(--space-4); font-size: 1.25rem"
            >
                icon(name: Icon::Info)
                icon(name: Icon::CircleCheck)
                icon(name: Icon::TriangleAlert)
                icon(name: Icon::CircleAlert)
                icon(name: Icon::ArrowUpRight)
                icon(name: Icon::X)
                icon(name: Icon::Check)
                icon(name: Icon::ChevronDown)
                icon(name: Icon::ChevronRight)
                icon(name: Icon::Search)
                icon(name: Icon::Inbox)
                icon(name: Icon::Minus)
                icon(name: Icon::Plus)
                icon(name: Icon::Copy)
            </div>
        )
        example(
            title: "Takes the text colour",
            <span style="color: var(--color-danger-text); font-size: 1.5rem">
                icon(name: Icon::CircleAlert)
            </span>
        )
    })
}
