use bitop_ui::components::{
    button::{ButtonVariant, button},
    dialog::{DialogSize, alert_dialog, dialog, dialog_close, dialog_trigger},
    field::{field, field_label},
    input::{input, textarea},
    skeleton::skeleton_text,
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
    let new_id = element_id(cx);
    let name_id = element_id(cx);
    let about_id = element_id(cx);
    let small_id = element_id(cx);
    let large_id = element_id(cx);
    let confirm_id = element_id(cx);
    let footer_id = new_id.clone();

    Ok(view! {
        page_header(slug: "dialog")
        example(
            title: "Dialog",
            dialog_trigger(
                target: &new_id,
                variant: ButtonVariant::Primary,
                "New project"
            )
            dialog(
                id: &new_id,
                title: "New project",
                description: "Projects group deployments, domains and environment variables.",
                footer: view! {
                    dialog_close(target: &footer_id, "Cancel")
                    button(
                        attrs: attributes! {
                            commandfor=(&footer_id)
                            command="close"
                            data-gallery-toast="success"
                            data-gallery-title="Project created"
                        },
                        "Create"
                    )
                },
                field(
                    field_label(target: &name_id, "Name")
                    input(
                        attrs: attributes! { id=(&name_id) placeholder="marketing-site" }
                    )
                )
                field(
                    field_label(target: &about_id, "Description")
                    textarea(attrs: attributes! { id=(&about_id) })
                )
            )
        )
        example(
            title: "Sizes",
            dialog_trigger(target: &small_id, "Small")
            dialog(
                id: &small_id,
                size: DialogSize::Sm,
                title: "Small dialog",
                description: "For short confirmations or single inputs.",
                <p>"Sizes: sm, md, lg, xl."</p>
            )
            dialog_trigger(target: &large_id, "Large")
            dialog(
                id: &large_id,
                size: DialogSize::Lg,
                title: "Large dialog",
                description: "Room for tables and multi-column forms.",
                skeleton_text(lines: 5)
            )
        )
        example(
            title: "Alert dialog",
            dialog_trigger(
                target: &confirm_id,
                variant: ButtonVariant::Danger,
                attrs: attributes! { data-gallery-reset-dialog=(&confirm_id) },
                "Delete project…"
            )
            alert_dialog(
                id: &confirm_id,
                title: "Delete “marketing-site”?",
                description: "Its 42 deployments and 3 domains will be removed. This can't be undone.",
                confirm_label: "Delete project",
                confirm_attrs: attributes! {
                    data-gallery-fail="The project still has an active production domain. Remove it first."
                }
            )
        )
    })
}
