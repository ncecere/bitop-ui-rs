use bitop_ui::components::{
    button::ButtonVariant,
    dialog::{
        alert_dialog, dialog, dialog_actions, dialog_close, dialog_description, dialog_title,
        dialog_trigger,
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let dialog_id = element_id(cx);
    let dialog_title_id = element_id(cx);
    let alert_id = element_id(cx);
    let alert_title_id = element_id(cx);
    let alert_description_id = element_id(cx);

    Ok(view! {
        page_header(slug: "dialog")
        example(
            title: "Dialog",
            dialog_trigger(target: &dialog_id, "Rename chat")
            dialog(
                id: &dialog_id,
                labelled_by: &dialog_title_id,
                dialog_title(id: &dialog_title_id, "Rename chat?")
                dialog_description("Clicking outside this dialog closes it.")
                dialog_actions(
                    dialog_close(target: &dialog_id, "Cancel")
                    dialog_close(
                        target: &dialog_id,
                        variant: ButtonVariant::Primary,
                        "Rename"
                    )
                )
            )
        )
        example(
            title: "Alert dialog",
            dialog_trigger(target: &alert_id, "Delete workspace")
            alert_dialog(
                id: &alert_id,
                labelled_by: &alert_title_id,
                described_by: &alert_description_id,
                dialog_title(id: &alert_title_id, "Delete workspace?")
                dialog_description(
                    attrs: attributes! { id=(&alert_description_id) },
                    "This permanently removes the workspace and all of its data."
                )
                dialog_actions(
                    dialog_close(target: &alert_id, "Cancel")
                    dialog_close(
                        target: &alert_id,
                        variant: ButtonVariant::Danger,
                        "Delete"
                    )
                )
            )
        )
    })
}
