use bitop_ui_ai::components::question::{QuestionChoice, QuestionKind, QuestionState, question};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let environments = [
        QuestionChoice::new("staging", "Staging"),
        QuestionChoice::new("production", "Production").description("Needs a second approval"),
        QuestionChoice::new("legacy", "Legacy").disabled(),
    ];
    let checks = [
        QuestionChoice::new("tests", "Run the tests"),
        QuestionChoice::new("lint", "Lint the code"),
        QuestionChoice::new("docs", "Build the docs").description("Takes about five minutes"),
    ];
    let languages = [
        QuestionChoice::new("en", "Englisch"),
        QuestionChoice::new("de", "Deutsch"),
    ];
    // The form example's answer comes back in the query, as a server would record it.
    let state = if query_param(cx, "action").as_deref() == Some("skip") {
        QuestionState::Skipped
    } else if query_param(cx, "environment").is_some() {
        QuestionState::Answered
    } else {
        QuestionState::Pending
    };
    // Owned by the view: a choice from the list, as `'static` text.
    let selected = [match query_param(cx, "environment").as_deref() {
        Some("staging") => "staging",
        Some("production") => "production",
        _ => "",
    }];
    Ok(view! {
        page_header(slug: "question")
        example(
            title: "One choice, with Other, sent with the form",
            <div data-example="single" style="inline-size: 100%">
                question(
                    id: "environment",
                    name: "environment",
                    prompt: "Which environment should I deploy to?",
                    description: "The release is tagged v2.4.0.",
                    options: &environments,
                    allow_other: true,
                    skippable: true,
                    state: state,
                    selected: &selected,
                    action: "/c/question",
                    method: "get",
                    attrs: attributes! { id="single-question" }
                )
            </div>
        )
        example(
            title: "Several choices, answered in script",
            <div data-example="multiple" style="inline-size: 100%">
                question(
                    id: "checks",
                    name: "checks",
                    kind: QuestionKind::Multiple,
                    prompt: "What should I run before the release?",
                    options: &checks,
                    allow_other: true,
                    selected: &["tests"],
                    attrs: attributes! { id="multiple-question" data-gallery-answer="" },
                    <p
                        style="color: var(--color-text-muted); font-size: var(--font-size-sm)"
                    >
                        "I'll wait for your answer before going on."
                    </p>
                )
            </div>
        )
        example(
            title: "Free text, with the prompt as a heading",
            <div data-example="freeform" style="inline-size: 100%">
                question(
                    id: "notes",
                    name: "notes",
                    kind: QuestionKind::Freeform,
                    prompt: "Anything I should mention in the release notes?",
                    placeholder: "For example: the new export button",
                    level: 3,
                    skippable: true,
                    attrs: attributes! { id="freeform-question" data-gallery-answer="" }
                )
            </div>
        )
        example(
            title: "Answered and skipped",
            <div
                data-example="done"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                question(
                    id: "answered",
                    name: "answered",
                    prompt: "Where should the logs go?",
                    options: &environments,
                    selected: &["staging"],
                    state: QuestionState::Answered
                )
                question(
                    id: "skipped",
                    name: "skipped",
                    kind: QuestionKind::Freeform,
                    prompt: "Anything else?",
                    state: QuestionState::Skipped
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                question(
                    id: "sprache",
                    name: "sprache",
                    prompt: "In welcher Sprache soll ich antworten?",
                    options: &languages,
                    allow_other: true,
                    skippable: true,
                    submit_label: "Senden",
                    skip_label: "Überspringen",
                    required_label: "Wähle eine Antwort.",
                    other_label: "Andere",
                    other_input_label: "Deine Antwort",
                    attrs: attributes! { id="german-question" data-gallery-answer="" }
                )
            </div>
        )
    })
}
