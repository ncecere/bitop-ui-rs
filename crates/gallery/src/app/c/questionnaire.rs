use bitop_ui_ai::components::{
    question::{QuestionChoice, QuestionKind},
    questionnaire::{QuestionnaireQuestion, questionnaire},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_string};

const REGIONS: &[QuestionChoice<'static>] = &[
    QuestionChoice::new("eu", "Europe"),
    QuestionChoice::new("us", "United States"),
    QuestionChoice::new("ap", "Asia Pacific").description("Tokyo and Singapore"),
];
const FEATURES: &[QuestionChoice<'static>] = &[
    QuestionChoice::new("auth", "Sign-in"),
    QuestionChoice::new("billing", "Billing"),
    QuestionChoice::new("search", "Search"),
];
const QUESTIONS: &[QuestionnaireQuestion<'static>] = &[
    QuestionnaireQuestion::new("region", "Where should it run?").options(REGIONS),
    QuestionnaireQuestion::new("features", "What do you need?")
        .kind(QuestionKind::Multiple)
        .options(FEATURES)
        .allow_other(),
    QuestionnaireQuestion::new("notes", "Anything else?")
        .kind(QuestionKind::Freeform)
        .placeholder("Optional")
        .optional(),
];
const ANSWERED: &[QuestionnaireQuestion<'static>] = &[
    QuestionnaireQuestion::new("size", "How big is the team?")
        .options(&[
            QuestionChoice::new("small", "Up to 10"),
            QuestionChoice::new("large", "More than 10"),
        ])
        .answer(&["small"], ""),
    QuestionnaireQuestion::new("deadline", "When is the deadline?")
        .kind(QuestionKind::Freeform)
        .optional(),
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let sent = query_string(cx);
    Ok(view! {
        page_header(slug: "questionnaire")
        example(
            title: "Three questions and a review",
            <div
                data-example="setup"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                questionnaire(
                    id: "setup",
                    label: "Project setup",
                    questions: QUESTIONS,
                    level: 3,
                    action: "/c/questionnaire",
                    method: "get"
                )
                if !sent.is_empty() {
                    <p role="status" data-sent="">(sent)</p>
                }
            </div>
        )
        example(
            title: "Starting at the review, without one, and other strings",
            <div
                data-example="variants"
                style="display: grid; gap: var(--space-4); inline-size: 100%"
            >
                questionnaire(
                    id: "team",
                    label: "Team",
                    questions: ANSWERED,
                    step: 2,
                    level: 3,
                    attrs: attributes! { data-gallery-questionnaire="" }
                )
                questionnaire(
                    id: "short",
                    label: "Kurz",
                    questions: ANSWERED,
                    review: false,
                    level: 3,
                    back_label: "Zurück",
                    next_label: "Weiter",
                    skip_label: "Überspringen",
                    submit_label: "Absenden",
                    progress_label: "Frage {current} von {total}",
                    required_label: "Beantworte diese Frage.",
                    attrs: attributes! { lang="de" data-gallery-questionnaire="" }
                )
            </div>
        )
    })
}
