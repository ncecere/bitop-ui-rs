use bitop_ui::components::drop_zone::drop_zone;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let documents_id = element_id(cx);
    let avatar_id = element_id(cx);
    let busy_id = element_id(cx);
    let locked_id = element_id(cx);

    Ok(view! {
        page_header(slug: "drop-zone")
        example(
            title: "Documents: PDF or images, 1 MB each, two at most",
            <form
                id="upload-form"
                method="post"
                enctype="multipart/form-data"
                style="inline-size: 100%"
            >
                drop_zone(
                    id: &documents_id,
                    name: "documents",
                    label: "Drag and drop documents here, or",
                    accept: ".pdf,image/*",
                    max_size: 1024 * 1024,
                    max_files: 2,
                    description: "PDF or images, up to 1 MB each. Two files at most."
                )
            </form>
        )
        example(
            title: "One file",
            drop_zone(
                id: &avatar_id,
                name: "avatar",
                label: "Drag and drop a picture here, or",
                button_label: "Choose a picture",
                accept: "image/png,image/jpeg",
                single: true
            )
        )
        example(
            title: "Busy",
            drop_zone(
                id: &busy_id,
                name: "busy",
                button_label: "Uploading…",
                busy: true
            )
        )
        example(
            title: "Disabled",
            drop_zone(id: &locked_id, name: "locked", disabled: true)
        )
    })
}
