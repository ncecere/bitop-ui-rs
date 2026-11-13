use bitop_ui::components::{
    button::{ButtonVariant, button},
    empty_state::empty_state,
    icon::{Icon, icon},
};
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
                    title: "No projects yet",
                    description: "Import a Git repository or start from a template to create your first project.",
                    icon: Some(Icon::Inbox),
                    button(
                        icon(name: Icon::Plus)
                        "New project"
                    )
                    button(variant: ButtonVariant::Ghost, "Read the guide")
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
