//! Several questions, one step at a time, then a review of the answers before they are
//! sent.
//!
//! The server renders every step in one `<form>`, and the library script shows one at
//! a time: Next checks the current question (unless it is optional) and goes on, Back
//! goes back, Skip empties an optional question and goes on, and each answer's Edit
//! button on the review step returns to its question. Whenever the step changes, focus
//! moves to the new step's heading, and the progress bar says "Question 2 of 3".
//!
//! On the last step Submit sends the form, with every question's answer under its
//! `id` (and "Other" text under `{id}-other`), as [`question_fields`] sends them.
//! Before it goes, a cancelable `bitop:questionnaire-submit` event goes out from the
//! form with `detail.answers`: for each question's `id`, `{ selected, text }`, or
//! `null` when it was skipped. A page that sends the answers itself cancels it.
//!
//! ```ignore
//! let regions = [QuestionChoice::new("eu", "Europe"), QuestionChoice::new("us", "US")];
//! let questions = [
//!     QuestionnaireQuestion::new("region", "Where should it run?").options(&regions),
//!     QuestionnaireQuestion::new("notes", "Anything else?")
//!         .kind(QuestionKind::Freeform).optional(),
//! ];
//! view! {
//!     questionnaire(id: "setup", label: "Project setup", questions: &questions,
//!         action: "/setup")
//! }
//! ```
//!
//! Differences from upstream: the steps change in the browser among steps the server
//! rendered, so without the script only the first step shows. `onSubmit` is the form's
//! submission and the event above; controlled answers and step are the initial
//! `selected`, `text` and `step`. Labels are strings; `progress` has `{current}` and
//! `{total}` rather than being a function. Edit is a ghost button, as the base set has
//! no link variant.
//!
//! [`question_fields`]: super::question::question_fields

use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    progress::progress,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::question::{QuestionChoice, QuestionKind, question_fields};
use crate::css::questionnaire as css;

/// One question of a [`questionnaire`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QuestionnaireQuestion<'a> {
    /// The form field, and the key of its answer.
    pub id: &'a str,
    /// The question, as the step's heading.
    pub prompt: &'a str,
    /// Help text under it.
    pub description: &'a str,
    /// What kind of answer it takes.
    pub kind: QuestionKind,
    /// The choices.
    pub options: &'a [QuestionChoice<'a>],
    /// Adds an "Other" choice with a text box.
    pub allow_other: bool,
    /// The text box's placeholder.
    pub placeholder: &'a str,
    /// May be skipped: Skip shows, and Next goes on without an answer.
    pub optional: bool,
    /// The values chosen at first.
    pub selected: &'a [&'a str],
    /// The first text of the "Other" or free text box.
    pub text: &'a str,
}

impl<'a> QuestionnaireQuestion<'a> {
    /// A single-choice question with an id and a prompt.
    #[must_use]
    pub const fn new(id: &'a str, prompt: &'a str) -> Self {
        Self {
            id,
            prompt,
            description: "",
            kind: QuestionKind::Single,
            options: &[],
            allow_other: false,
            placeholder: "",
            optional: false,
            selected: &[],
            text: "",
        }
    }

    /// Sets the kind of answer.
    #[must_use]
    pub const fn kind(mut self, kind: QuestionKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the choices.
    #[must_use]
    pub const fn options(mut self, options: &'a [QuestionChoice<'a>]) -> Self {
        self.options = options;
        self
    }

    /// Adds help text.
    #[must_use]
    pub const fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Adds an "Other" choice with a text box.
    #[must_use]
    pub const fn allow_other(mut self) -> Self {
        self.allow_other = true;
        self
    }

    /// Sets the text box's placeholder.
    #[must_use]
    pub const fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Lets the question be skipped.
    #[must_use]
    pub const fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Sets the first answer.
    #[must_use]
    pub const fn answer(mut self, selected: &'a [&'a str], text: &'a str) -> Self {
        self.selected = selected;
        self.text = text;
        self
    }

    /// The answer as text for the review: the chosen labels and the text, joined by
    /// ", ", or `None` when there is none.
    #[must_use]
    pub fn answer_text(&self, other_value: &str) -> Option<String> {
        let text = self.text.trim();
        let mut parts: Vec<&str> = Vec::new();
        if self.kind == QuestionKind::Freeform {
            parts.extend((!text.is_empty()).then_some(text));
        } else {
            parts.extend(
                self.selected
                    .iter()
                    .filter(|value| **value != other_value)
                    .map(|value| {
                        self.options
                            .iter()
                            .find(|option| option.value == *value)
                            .map_or(*value, |option| option.label)
                    }),
            );
            if self.selected.contains(&other_value) && !text.is_empty() {
                parts.push(text);
            }
        }
        (!parts.is_empty()).then(|| parts.join(", "))
    }
}

/// The questionnaire: a `<form>` named by `label`.
///
/// - `id` goes on the form and is the base of the questions' IDs.
/// - `questions` are the steps; `step` is the one shown first, counted from 0 (the
///   number of questions is the review).
/// - `review` adds the review step.
/// - `level` is the level of the steps' headings, 2 by default.
/// - `action` and `method` are the form's; `post` by default.
/// - `back_label`, `next_label`, `skip_label`, `submit_label`, `progress_label` (with
///   `{current}` and `{total}`), `review_label`, `review_description`, `edit_label`,
///   `skipped_label`, `required_label`, `other_label` and `other_input_label` are the
///   strings.
/// - `attrs` are forwarded to the `<form>`.
#[component]
pub async fn questionnaire(
    cx: &Cx,
    id: &str,
    label: &str,
    questions: &[QuestionnaireQuestion<'_>],
    #[default] step: usize,
    #[default(true)] review: bool,
    #[default(2)] level: u8,
    #[default] action: &str,
    #[default("post")] method: &str,
    #[default("Back")] back_label: &str,
    #[default("Next")] next_label: &str,
    #[default("Skip")] skip_label: &str,
    #[default("Submit")] submit_label: &str,
    #[default("Question {current} of {total}")] progress_label: &str,
    #[default("Review your answers")] review_label: &str,
    #[default("Check your answers before you submit.")] review_description: &str,
    #[default("Edit")] edit_label: &str,
    #[default("Skipped")] skipped_label: &str,
    #[default("Answer this question to continue.")] required_label: &str,
    #[default("Other")] other_label: &str,
    #[default("Your answer")] other_input_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let total = questions.len();
    let last = if review {
        total
    } else {
        total.saturating_sub(1)
    };
    let step = step.min(last);
    let on_review = review && step == total;
    #[allow(clippy::cast_precision_loss)]
    let (value, max) = (
        if on_review { total } else { step + 1 } as f64,
        total.max(1) as f64,
    );
    let shown_progress = if on_review {
        review_label.to_owned()
    } else {
        progress_label
            .replace("{current}", &(step + 1).to_string())
            .replace("{total}", &total.to_string())
    };
    let optional_now = questions.get(step).is_some_and(|q| q.optional) && !on_review;
    let prefix = format!("{id}-");
    let answers: Vec<(Option<String>, &str)> = questions
        .iter()
        .map(|q| (q.answer_text("__other__"), q.prompt))
        .collect();
    let heading_class = class!(css::HEADING);
    let review_heading = view! {
        match level {
            3 => <h3 tabindex="-1" class=(heading_class)>(review_label)</h3>,
            4 => <h4 tabindex="-1" class=(heading_class)>(review_label)</h4>,
            5 => <h5 tabindex="-1" class=(heading_class)>(review_label)</h5>,
            6 => <h6 tabindex="-1" class=(heading_class)>(review_label)</h6>,
            _ => <h2 tabindex="-1" class=(heading_class)>(review_label)</h2>,
        }
    };
    Ok(Child::new(view! {
        <form
            id=(id)
            action=((!action.is_empty()).then_some(action))
            method=(method)
            novalidate=(true)
            aria-label=(label)
            data-questionnaire=""
            data-step=(step)
            data-total=(total)
            data-progress-label=(progress_label)
            data-review-label=(review_label)
            data-next-label=(next_label)
            data-submit-label=(submit_label)
            data-skipped-label=(skipped_label)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            progress(
                label: &shown_progress,
                value: Some(value),
                max: max,
                show_value: false,
                small: true,
                attrs: attributes! { class=(css::PROGRESS) data-questionnaire-progress="" }
            )
            for (index, question) in questions.iter().enumerate() {
                <div
                    data-questionnaire-step=(index)
                    data-optional=(question.optional.then_some(""))
                    hidden=(index != step)
                    class=(css::STEP)
                >
                    question_fields(
                        id: question.id,
                        id_prefix: &prefix,
                        prompt: question.prompt,
                        name: question.id,
                        kind: question.kind,
                        options: question.options,
                        allow_other: question.allow_other,
                        selected: question.selected,
                        text: question.text,
                        placeholder: question.placeholder,
                        description: question.description,
                        required_label: required_label,
                        level: level,
                        other_label: other_label,
                        other_input_label: other_input_label
                    )
                </div>
            }
            if review {
                <div
                    data-questionnaire-step=(total)
                    data-review=""
                    hidden=(!on_review)
                    class=(css::STEP)
                >
                    <div class=(css::REVIEW)>
                        (review_heading)
                        <p class=(css::DESCRIPTION)>(review_description)</p>
                        <dl class=(css::SUMMARY)>
                            for (index, (answer, prompt)) in answers
                                .into_iter()
                                .enumerate() {
                                <div class=(css::SUMMARY_ROW)>
                                    <dt class=(css::SUMMARY_PROMPT)>(prompt)</dt>
                                    <dd
                                        data-questionnaire-answer=(index)
                                        data-skipped=(answer.is_none().then_some(""))
                                        class=(css::SUMMARY_ANSWER)
                                    >
                                        (answer.unwrap_or_else(|| skipped_label.to_owned()))
                                    </dd>
                                    <dd class=(css::SUMMARY_EDIT)>
                                        button(
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Sm,
                                            attrs: attributes! { data-questionnaire-edit=(index) },
                                            (edit_label)
                                            " "
                                            <span class="sr-only">(prompt)</span>
                                        )
                                    </dd>
                                </div>
                            }
                        </dl>
                    </div>
                </div>
            }
            <div class=(css::ACTIONS)>
                button(
                    variant: ButtonVariant::Secondary,
                    attrs: attributes! {
                        hidden=(step == 0)
                        data-questionnaire-back=""
                        class=(css::BACK)
                    },
                    (back_label)
                )
                button(
                    variant: ButtonVariant::Ghost,
                    attrs: attributes! { hidden=(!optional_now) data-questionnaire-skip="" },
                    (skip_label)
                )
                button(
                    attrs: attributes! { type="submit" data-questionnaire-next="" },
                    (if step == last { submit_label } else { next_label })
                )
            </div>
        </form>
    }))
}

#[cfg(test)]
mod tests {
    use super::{QuestionChoice, QuestionKind, QuestionnaireQuestion};

    #[test]
    fn answer_text() {
        let options = [
            QuestionChoice::new("eu", "Europe"),
            QuestionChoice::new("us", "US"),
        ];
        let q = QuestionnaireQuestion::new("region", "Where?").options(&options);
        assert_eq!(q.answer_text("__other__"), None);
        let both = q.answer(&["eu", "__other__"], " Asia ");
        assert_eq!(
            both.answer_text("__other__").as_deref(),
            Some("Europe, Asia")
        );
        let other_empty = q.answer(&["__other__"], " ");
        assert_eq!(other_empty.answer_text("__other__"), None);
        let free = QuestionnaireQuestion::new("notes", "Notes?")
            .kind(QuestionKind::Freeform)
            .answer(&[], "Hi");
        assert_eq!(free.answer_text("__other__").as_deref(), Some("Hi"));
    }
}
