use bitop_ui::components::{
    button::{ButtonVariant, button},
    icon::Icon,
};
use bitop_ui_ai::components::attachments::{AttachmentStatus, attachment, attachments};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{PICTURE, example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let removed = query_param(cx, "remove").unwrap_or_default();
    Ok(view! {
        page_header(slug: "attachments")
        example(
            title: "Files in a sent message",
            <div data-example="sent">
                attachments(
                    attachment(
                        name: "quarterly-report.pdf",
                        size: 482_133,
                        mime_type: "application/pdf"
                    )
                    attachment(
                        name: "dashboard.png",
                        size: 96_000,
                        mime_type: "image/png",
                        url: PICTURE
                    )
                    attachment(name: "notes.md", size: 812, mime_type: "text/markdown")
                    attachment(
                        name: "archive.zip",
                        size: 12_582_912,
                        mime_type: "application/zip"
                    )
                    attachment(
                        name: "a very long file name that does not fit in its chip.xlsx",
                        size: 20_480
                    )
                    attachment(name: "logo", icon_name: Icon::Globe)
                )
            </div>
        )
        example(
            title: "Uploading and failed",
            <div data-example="states">
                attachments(
                    attachment(
                        name: "recording.mp3",
                        mime_type: "audio/mpeg",
                        status: AttachmentStatus::Uploading
                    )
                    attachment(
                        name: "scan.pdf",
                        mime_type: "application/pdf",
                        status: AttachmentStatus::Error,
                        removable: true
                    )
                )
            </div>
        )
        example(
            title: "Removable, in the browser",
            <div data-example="removable" style="display: grid; gap: var(--space-3)">
                attachments(
                    label: "Attached files",
                    attrs: attributes! { data-attachments-focus="after-removing" },
                    attachment(
                        name: "brief.docx",
                        size: 30_720,
                        id: "f1",
                        removable: true
                    )
                    attachment(
                        name: "photo.png",
                        size: 204_800,
                        mime_type: "image/png",
                        url: PICTURE,
                        id: "f2",
                        removable: true
                    )
                    attachment(name: "data.csv", size: 4096, id: "f3", removable: true)
                )
                button(
                    variant: ButtonVariant::Secondary,
                    attrs: attributes! { id="after-removing" style="justify-self: start" },
                    "Focus goes here when the list is empty"
                )
            </div>
        )
        example(
            title: "Removed by the server, through a form",
            <form action="/c/attachments" data-example="form">
                attachments(
                    if removed != "budget" {
                        attachment(
                            name: "budget.xlsx",
                            size: 18_432,
                            removable: true,
                            remove_name: "remove",
                            remove_value: "budget"
                        )
                    }
                    attachment(
                        name: "plan.pdf",
                        mime_type: "application/pdf",
                        size: 220_000,
                        removable: true,
                        remove_name: "remove",
                        remove_value: "plan",
                        disabled: true
                    )
                )
            </form>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de">
                attachments(
                    label: "Anhänge",
                    attachment(
                        name: "bericht.pdf",
                        status: AttachmentStatus::Uploading,
                        uploading_label: "Wird hochgeladen…"
                    )
                    attachment(
                        name: "bild.png",
                        status: AttachmentStatus::Error,
                        error_label: "Hochladen fehlgeschlagen",
                        removable: true,
                        remove_label: "{name} entfernen",
                        remove_tooltip: "Entfernen"
                    )
                )
            </div>
        )
    })
}
