use bitop_ui_ai::components::checkpoint::checkpoint;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let restored = query_param(cx, "checkpoint");
    Ok(view! {
        page_header(slug: "checkpoint")
        example(
            title: "A marker only",
            <div data-example="plain" style="inline-size: 100%">
                checkpoint(time: "10:42", datetime: "2026-11-12T10:42")
            </div>
        )
        example(
            title: "Restore submits a form",
            <form
                data-example="form"
                method="get"
                action="/c/checkpoint"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                checkpoint(
                    time: "10:42",
                    restore: true,
                    restore_name: "checkpoint",
                    restore_value: "c-41"
                )
                checkpoint(
                    time: "11:05",
                    restore: true,
                    restore_name: "checkpoint",
                    restore_value: "c-42"
                )
                if let Some(id) = restored {
                    <p role="status" data-restored="">
                        "Restored "
                        (id)
                    </p>
                }
            </form>
        )
        example(
            title: "Restore handled in script, and disabled",
            <div
                data-example="script"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                checkpoint(
                    label: "Saved state",
                    name: "Before the refactor",
                    restore: true,
                    id: "before-refactor",
                    attrs: attributes! { id="script-checkpoint" }
                )
                checkpoint(time: "09:15", restore: true, disabled: true)
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                checkpoint(
                    label: "Prüfpunkt",
                    time: "10:42",
                    restore: true,
                    restore_label: "Wiederherstellen",
                    restore_aria_label: "{name} {restore}"
                )
            </div>
        )
    })
}
