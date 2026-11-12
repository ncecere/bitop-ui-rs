use bitop_ui::components::checklist::{ChecklistLevel, ChecklistStep, checklist};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let steps = [
        ChecklistStep::new("Add a data source").done(true),
        ChecklistStep::new("Create a knowledge base")
            .description("Group sources so that an assistant can search them.")
            .action("Create", "#create"),
        ChecklistStep::new("Invite your team").action("Invite", "#invite"),
        ChecklistStep::new("Read the guide"),
    ];
    let finished = [
        ChecklistStep::new("Verify your email")
            .done(true)
            .action("Verify", "#verify"),
        ChecklistStep::new("Choose a plan").done(true),
    ];
    let schritte = [
        ChecklistStep::new("Konto anlegen").done(true),
        ChecklistStep::new("Profil ausfüllen"),
    ];
    Ok(view! {
        page_header(slug: "checklist")
        example(
            title: "Steps, with a dismiss button",
            checklist(
                title: "Get started",
                description: "Four steps to your first answer.",
                steps: &steps,
                level: ChecklistLevel::H3,
                dismissible: true,
                attrs: attributes! { style="inline-size: 100%" }
            )
        )
        example(
            title: "Every step done",
            checklist(
                title: "Account setup",
                steps: &finished,
                level: ChecklistLevel::H3,
                complete: "All set. You can close this list.",
                attrs: attributes! { style="inline-size: 100%" }
            )
        )
        example(
            title: "Without the progress bar",
            checklist(
                title: "Before you deploy",
                steps: &steps[..2],
                level: ChecklistLevel::H3,
                hide_progress: true,
                attrs: attributes! { style="inline-size: 100%" }
            )
        )
        example(
            title: "Every string is an argument",
            checklist(
                title: "Erste Schritte",
                steps: &schritte,
                level: ChecklistLevel::H3,
                dismissible: true,
                summary_label: "{done} von {total} erledigt",
                done_label: "Erledigt",
                todo_label: "Offen",
                dismiss_label: "Ausblenden:",
                attrs: attributes! { style="inline-size: 100%" }
            )
        )
    })
}
