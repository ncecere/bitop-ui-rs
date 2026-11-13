use bitop_ui::components::icon::{Icon, icon};
use bitop_ui_ai::components::queue::{
    queue, queue_content, queue_header, queue_item, queue_item_action,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let removed = query_param(cx, "remove");
    Ok(view! {
        page_header(slug: "queue")
        example(
            title: "Queued messages, with actions that submit",
            <form
                data-example="queued"
                method="get"
                action="/c/queue"
                style="inline-size: 100%"
            >
                queue(
                    open: true,
                    queue_header(title: "Queued", count: 2)
                    queue_content(
                        label: "Queued messages",
                        queue_item(
                            actions: view! {
                                queue_item_action(
                                    label: "Edit",
                                    tooltip: "queue-edit-1",
                                    icon(name: Icon::SquarePen)
                                )
                                queue_item_action(
                                    label: "Remove",
                                    tooltip: "queue-remove-1",
                                    name: "remove",
                                    value: "q-1",
                                    icon(name: Icon::X)
                                )
                            },
                            "Also compare with last year"
                        )
                        queue_item(
                            description: "Sent after the current answer",
                            actions: view! {
                                queue_item_action(
                                    label: "Remove",
                                    name: "remove",
                                    value: "q-2",
                                    icon(name: Icon::X)
                                )
                            },
                            "Make it shorter"
                        )
                    )
                )
                if let Some(id) = removed {
                    <p role="status" data-removed="">
                        "Removed "
                        (id)
                    </p>
                }
            </form>
        )
        example(
            title: "A to-do list, closed",
            <div data-example="todo" style="inline-size: 100%">
                queue(
                    attrs: attributes! { id="todo-queue" },
                    queue_header(title: "To do", count: 1)
                    queue_content(
                        queue_item(completed: true, "Read the policy")
                        queue_item("Summarise the changes")
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                queue(
                    open: true,
                    queue_header(
                        title: "Warteschlange",
                        count: 1,
                        item_one: "Eintrag",
                        item_other: "Einträge"
                    )
                    queue_content(
                        queue_item(
                            completed: true,
                            done_label: "(erledigt)",
                            "Richtlinie lesen"
                        )
                    )
                )
            </div>
        )
    })
}
