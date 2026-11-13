use bitop_ui::components::{
    checkbox::{SwitchLabelPosition, checkbox, checkbox_group, switch},
    layout::stack,
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
        page_header(slug: "checkbox")
        example(
            title: "Checkbox group",
            <form id="scopes-form">
                checkbox_group(
                    id: "scopes",
                    legend: "Scopes",
                    description: "What this token can do.",
                    checkbox(
                        name: "scope",
                        value: "read",
                        checked: true,
                        description: "List projects and deployments.",
                        "Read"
                    )
                    checkbox(
                        name: "scope",
                        value: "write",
                        description: "Create and update deployments.",
                        "Write"
                    )
                    checkbox(
                        name: "scope",
                        value: "admin",
                        disabled: true,
                        description: "Manage members and billing.",
                        "Admin"
                    )
                )
            </form>
        )
        example(
            title: "Single checkboxes",
            stack(
                gap: 4,
                checkbox(name: "notify", checked: true, "Email me when a build fails")
                checkbox(indeterminate: true, "Partially selected")
                checkbox(disabled: true, "Disabled")
            )
        )
        example(
            title: "Switches",
            stack(
                gap: 4,
                switch(
                    name: "previews",
                    checked: true,
                    description: "Build every pull request.",
                    "Preview deployments"
                )
                switch(
                    label_position: SwitchLabelPosition::Start,
                    "Settings-row layout"
                )
                switch(disabled: true, "Disabled")
            )
        )
        example(
            title: "A group in a row, with an error",
            checkbox_group(
                id: "channels",
                legend: "Notify me by",
                error: "Choose at least one channel.",
                horizontal: true,
                checkbox(name: "channel", value: "email", "Email")
                checkbox(name: "channel", value: "sms", "Text message")
                checkbox(name: "channel", value: "push", "Push")
            )
        )
    })
}
