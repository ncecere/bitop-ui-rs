use bitop_ui::components::icon::Icon;
use bitop_ui_ai::components::task::{task, task_content, task_item, task_item_file, task_trigger};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "task")
        example(
            title: "Open, with files",
            <div data-example="open" style="inline-size: 100%">
                task(
                    open: true,
                    task_trigger(title: "Searched 3 folders")
                    task_content(
                        task_item(
                            "Read "
                            task_item_file("refunds.md")
                        )
                        task_item(
                            "Read "
                            task_item_file("returns-2025.pdf")
                            " and "
                            task_item_file("faq.md")
                        )
                        task_item("Found 2 matching sections")
                    )
                )
            </div>
        )
        example(
            title: "Running",
            <div data-example="running" style="inline-size: 100%">
                task(
                    task_trigger(title: "Reading the policy library", running: true)
                    task_content(
                        task_item(
                            "Opened "
                            task_item_file("handbook.pdf")
                        )
                    )
                )
            </div>
        )
        example(
            title: "Closed, with its own icon and a long title",
            <div
                data-example="closed"
                style="inline-size: 100%; max-inline-size: 20rem"
            >
                task(
                    attrs: attributes! { id="closed-task" },
                    task_trigger(
                        title: "Compared the refund rules of every regional office with the head office",
                        icon_name: Icon::BookOpen
                    )
                    task_content(
                        task_item(task_item_file(icon_name: Icon::File, "offices.csv"))
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                task(
                    task_trigger(
                        title: "Durchsucht die Richtlinien",
                        running: true,
                        in_progress_label: "(läuft)"
                    )
                    task_content(
                        task_item(
                            "Gelesen: "
                            task_item_file("richtlinie.md")
                        )
                    )
                )
            </div>
        )
    })
}
