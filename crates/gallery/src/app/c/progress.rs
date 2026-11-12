use bitop_ui::components::progress::{ProgressTone, progress};
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
        page_header(slug: "progress")
        example(
            title: "Known and unknown amounts",
            <div style="display: grid; gap: var(--space-5); inline-size: 100%">
                progress(label: "Uploading", value: Some(40.0))
                progress(label: "Indexing", value: None)
                progress(
                    label: "Done",
                    value: Some(7.0),
                    max: 7.0,
                    tone: ProgressTone::Success
                )
                progress(
                    label: "Storage",
                    value: Some(96.0),
                    tone: ProgressTone::Danger,
                    small: true,
                    hide_label: true
                )
            </div>
        )
    })
}
