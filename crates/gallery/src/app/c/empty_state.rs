use bitop_ui::components::{button::button, empty_state::empty_state, icon::Icon};
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
        page_header(slug: "empty-state")
        example(
            title: "With an icon and an action",
            <div style="inline-size: 100%">
                empty_state(
                    title: "No chats yet",
                    description: "Start one and it will show up here.",
                    icon: Some(Icon::Inbox),
                    button("New chat")
                )
            </div>
        )
        example(
            title: "Compact",
            <div style="inline-size: 100%">
                empty_state(
                    title: "No results",
                    description: "Try a different search.",
                    compact: true
                )
            </div>
        )
    })
}
