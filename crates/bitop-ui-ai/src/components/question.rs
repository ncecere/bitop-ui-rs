//! A question an agent asks, answered in the chat: one choice, several, or free text,
//! with an optional "Other" answer.
//!
//! [`question`] is a real `<form>` and is sent as one: the choices under `name`, the
//! "Other" or free text under `{name}-other` (free text alone under `name`), and Skip
//! as `skip_name=skip_value`. The library script adds what the form needs:
//!
//! - Submitting without an answer shows `required_label` on the group, marks it
//!   invalid and moves focus to its first control. Skip needs no answer.
//! - The "Other" text box shows while "Other" is chosen, and is sent only then.
//! - Before the form is sent, a cancelable `bitop:question-submit` event goes out from
//!   it with `detail.skipped`, `detail.selected` (the chosen values) and `detail.text`.
//!   A page that answers in script cancels it and sets `data-state` on the form.
//! - When `data-state` becomes `answered` or `skipped`, the controls lock, the actions
//!   go, the status is announced, and focus that was on a button that has gone moves
//!   to the form.
//!
//! [`question_fields`] is the controls alone, for a flow of several questions such as
//! [`questionnaire`](super::questionnaire::questionnaire).
//!
//! ```ignore
//! let options = [
//!     QuestionChoice::new("staging", "Staging"),
//!     QuestionChoice::new("prod", "Production").description("Needs approval"),
//! ];
//! view! {
//!     question(id: "deploy", name: "environment", options: &options,
//!         prompt: "Which environment should I deploy to?", allow_other: true,
//!         skippable: true)
//! }
//! ```
//!
//! Differences from upstream: `onSubmit` and `onSkip` are the form's submission and the
//! event above, and a controlled `value` is the initial `selected` and `text`. The
//! choices' labels are text. The form is named with `aria-label` rather than by its
//! prompt's ID, as Playwright's Firefox crashes on an element named by one inside it.
//! The groups are drawn with the base set's radio group and checkbox styles around
//! the question's own `<fieldset>`, which carries the question's state.

use bitop_ui::{
    components::{
        button::{ButtonSize, ButtonVariant, button},
        checkbox::checkbox,
        icon::{Icon, icon},
        input::{input, textarea},
        radio_group::radio,
    },
    css::{field as field_css, radio_group as radio_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::question as css;

/// What kind of answer a question takes. Rendered as `data-kind`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestionKind {
    /// One choice: radio buttons.
    #[default]
    Single,
    /// Any number of choices: checkboxes.
    Multiple,
    /// Text: a text box.
    Freeform,
}

impl QuestionKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
            Self::Freeform => "freeform",
        }
    }
}

/// One choice of a question.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QuestionChoice<'a> {
    /// What is sent.
    pub value: &'a str,
    /// What is shown.
    pub label: &'a str,
    /// Help text under the label.
    pub description: &'a str,
    /// Shown, but cannot be chosen.
    pub disabled: bool,
}

impl<'a> QuestionChoice<'a> {
    /// A choice with a value and a label.
    #[must_use]
    pub const fn new(value: &'a str, label: &'a str) -> Self {
        Self {
            value,
            label,
            description: "",
            disabled: false,
        }
    }

    /// Adds help text.
    #[must_use]
    pub const fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Makes it impossible to choose.
    #[must_use]
    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Where a [`question`] stands. Rendered as `data-state`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestionState {
    #[default]
    Pending,
    Answered,
    Skipped,
}

impl QuestionState {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Answered => "answered",
            Self::Skipped => "skipped",
        }
    }
}

/// The message shown when a question of `kind` is sent without an answer, unless one
/// is given.
#[must_use]
pub fn default_required_label(kind: QuestionKind) -> &'static str {
    if kind == QuestionKind::Freeform {
        "Type an answer."
    } else {
        "Choose an answer."
    }
}

/// The controls of one question, without a form or actions.
///
/// - `id` goes on the group (or the text box, as `{id}-text`), and is the base of the
///   IDs that connect the parts. `id_prefix` is put before it, for a list of
///   questions whose IDs are their own.
/// - `prompt` names the group or the text box; `level` (2 to 6) makes it a heading
///   that can take focus, with the ID `{id}-heading`.
/// - `kind` picks the controls; `options` are the choices; `allow_other` adds an
///   "Other" choice (`other_label`, sent as `other_value`) with a text box
///   (`other_input_label`).
/// - `name` is the form field; `selected` the values chosen at first; `text` the
///   first text of the "Other" or free text box; `placeholder` is that box's.
/// - `description` is help text; `error` a message shown at first; `required_label`
///   the one the script shows (by default from [`default_required_label`]).
/// - `disabled` disables every control.
#[component]
pub async fn question_fields(
    cx: &Cx,
    id: &str,
    #[default] id_prefix: &str,
    prompt: &str,
    name: &str,
    #[default] kind: QuestionKind,
    #[default] options: &[QuestionChoice<'_>],
    #[default] allow_other: bool,
    #[default] selected: &[&str],
    #[default] text: &str,
    #[default] placeholder: &str,
    #[default] description: &str,
    #[default] error: &str,
    #[default] required_label: &str,
    #[default]
    #[into]
    level: Option<u8>,
    #[default] disabled: bool,
    #[default("Other")] other_label: &str,
    #[default("Your answer")] other_input_label: &str,
    #[default("__other__")] other_value: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let required = if required_label.is_empty() {
        default_required_label(kind)
    } else {
        required_label
    };
    let id = format!("{id_prefix}{id}");
    let description_id = format!("{id}-description");
    let error_id = format!("{id}-error");
    let heading_id = format!("{id}-heading");
    let labelled_by = level.is_some().then(|| heading_id.clone());
    let text_id = format!("{id}-text");
    let other_id = format!("{id}-other");
    let described_by = [
        (!description.is_empty()).then_some(description_id.as_str()),
        (!error.is_empty()).then_some(error_id.as_str()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ");
    let described_by = (!described_by.is_empty()).then_some(described_by);
    let invalid = (!error.is_empty()).then_some("true");
    let heading_class = class!(css::HEADING);
    let other_checked = selected.contains(&other_value);
    let other_name = format!("{name}-other");
    let prompt_view = view! {
        match level {
            Some(2) => <h2 id=(&heading_id) tabindex="-1" class=(heading_class)>
                (prompt)
            </h2>,
            Some(3) => <h3 id=(&heading_id) tabindex="-1" class=(heading_class)>
                (prompt)
            </h3>,
            Some(4) => <h4 id=(&heading_id) tabindex="-1" class=(heading_class)>
                (prompt)
            </h4>,
            Some(5) => <h5 id=(&heading_id) tabindex="-1" class=(heading_class)>
                (prompt)
            </h5>,
            Some(6) => <h6 id=(&heading_id) tabindex="-1" class=(heading_class)>
                (prompt)
            </h6>,
            _ => <span class=(css::PROMPT)>(prompt)</span>,
        }
    };
    let error_view = view! {
        <p
            id=(&error_id)
            hidden=(error.is_empty())
            data-question-error=""
            class=(class!(radio_css::ERROR, css::ERROR))
        >
            icon(
                name: Icon::CircleAlert,
                attrs: attributes! { class=(radio_css::ERROR_ICON) }
            )
            <span>(if error.is_empty() { required } else { error })</span>
        </p>
    };
    Ok(Child::new(view! {
        <div
            data-question-fields=""
            data-kind=(kind.as_str())
            data-required=(required)
            data-name=(name)
            class=(class!(css::FIELDS, attrs.remove("class")))
            (attrs)
        >
            if kind == QuestionKind::Freeform {
                <div class=(field_css::ROOT)>
                    if level.is_some() {
                        (prompt_view)
                    } else {
                        <label for=(&text_id) class=(css::PROMPT)>(prompt)</label>
                    }
                    if !description.is_empty() {
                        <p id=(&description_id) class=(field_css::DESCRIPTION)>
                            (description)
                        </p>
                    }
                    textarea(
                        attrs: attributes! {
                            id=(&text_id)
                            name=(name)
                            placeholder=((!placeholder.is_empty()).then_some(placeholder))
                            disabled=(disabled)
                            aria-labelledby=(labelled_by)
                            aria-describedby=(described_by.clone())
                            aria-invalid=(invalid)
                            data-question-text=""
                        },
                        (text)
                    )
                    (error_view)
                </div>
            } else {
                <div class=(radio_css::GROUP)>
                    <fieldset
                        id=(&id)
                        role=((kind == QuestionKind::Single).then_some("radiogroup"))
                        aria-describedby=(described_by)
                        aria-invalid=(invalid)
                        disabled=(disabled)
                        data-question-group=""
                        class=(radio_css::FIELDSET)
                    >
                        <legend class=(radio_css::LEGEND)>(prompt_view)</legend>
                        if !description.is_empty() {
                            <p
                                id=(&description_id)
                                class=(radio_css::GROUP_DESCRIPTION)
                            >
                                (description)
                            </p>
                        }
                        <div
                            class=(radio_css::OPTIONS)
                            data-orientation="vertical"
                            data-variant="default"
                        >
                            for option in options {
                                if kind == QuestionKind::Single {
                                    radio(
                                        name: name,
                                        value: option.value,
                                        checked: selected.contains(&option.value),
                                        disabled: option.disabled,
                                        description: option.description,
                                        (option.label)
                                    )
                                } else {
                                    checkbox(
                                        name: name,
                                        value: option.value,
                                        checked: selected.contains(&option.value),
                                        disabled: option.disabled,
                                        description: option.description,
                                        (option.label)
                                    )
                                }
                            }
                            if allow_other {
                                if kind == QuestionKind::Single {
                                    radio(
                                        name: name,
                                        value: other_value,
                                        checked: other_checked,
                                        attrs: attributes! { data-question-other="" },
                                        (other_label)
                                    )
                                } else {
                                    checkbox(
                                        name: name,
                                        value: other_value,
                                        checked: other_checked,
                                        attrs: attributes! { data-question-other="" },
                                        (other_label)
                                    )
                                }
                            }
                        </div>
                    </fieldset>
                    if allow_other {
                        <div class=(class!(field_css::ROOT, css::OTHER))>
                            <label for=(&other_id) class=(field_css::LABEL)>
                                (other_input_label)
                            </label>
                            input(
                                attrs: attributes! {
                                    id=(&other_id)
                                    name=(&other_name)
                                    value=(text)
                                    placeholder=((!placeholder.is_empty()).then_some(
                                        placeholder,
                                    ))
                                    disabled=(disabled || !other_checked)
                                    data-question-other-text=""
                                }
                            )
                        </div>
                    }
                    (error_view)
                </div>
            }
        </div>
    }))
}

/// A question as a form, with Submit, an optional Skip, and a status once answered.
///
/// The fields are [`question_fields`]'s, the prompt names the form, and `child` is
/// extra content under the controls, such as why the agent asks.
///
/// - `state` is where it stands; `answered_label` and `skipped_label` are the status.
/// - `action` and `method` are the form's; `post` by default.
/// - `skippable` adds Skip, which sends `skip_name=skip_value`.
/// - `submit_label` and `skip_label` are the buttons.
/// - `attrs` are forwarded to the `<form>`.
#[component]
pub async fn question(
    cx: &Cx,
    id: &str,
    prompt: &str,
    name: &str,
    #[default] kind: QuestionKind,
    #[default] options: &[QuestionChoice<'_>],
    #[default] allow_other: bool,
    #[default] selected: &[&str],
    #[default] text: &str,
    #[default] placeholder: &str,
    #[default] description: &str,
    #[default] error: &str,
    #[default] state: QuestionState,
    #[default] disabled: bool,
    #[default]
    #[into]
    level: Option<u8>,
    #[default] action: &str,
    #[default("post")] method: &str,
    #[default] skippable: bool,
    #[default("action")] skip_name: &str,
    #[default("skip")] skip_value: &str,
    #[default("Submit")] submit_label: &str,
    #[default("Skip")] skip_label: &str,
    #[default] required_label: &str,
    #[default("Answered")] answered_label: &str,
    #[default("Skipped")] skipped_label: &str,
    #[default("Other")] other_label: &str,
    #[default("Your answer")] other_input_label: &str,
    #[default("__other__")] other_value: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let locked = state != QuestionState::Pending;
    let status = match state {
        QuestionState::Pending => "",
        QuestionState::Answered => answered_label,
        QuestionState::Skipped => skipped_label,
    };
    Ok(Child::new(view! {
        <form
            action=((!action.is_empty()).then_some(action))
            method=(method)
            novalidate=(true)
            tabindex="-1"
            aria-label=(prompt)
            data-question=""
            data-state=(state.as_str())
            data-answered-label=(answered_label)
            data-skipped-label=(skipped_label)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::ICON)>
                icon(
                    name: Icon::MessageCircleQuestion,
                    attrs: attributes! { data-when="pending" }
                )
                icon(
                    name: Icon::CircleCheck,
                    attrs: attributes! { data-when="answered" }
                )
                icon(
                    name: Icon::CircleSlash,
                    attrs: attributes! { data-when="skipped" }
                )
            </span>
            <div class=(css::BODY)>
                question_fields(
                    id: id,
                    prompt: prompt,
                    name: name,
                    kind: kind,
                    options: options,
                    allow_other: allow_other,
                    selected: selected,
                    text: text,
                    placeholder: placeholder,
                    description: description,
                    error: error,
                    required_label: required_label,
                    level: level,
                    disabled: disabled || locked,
                    other_label: other_label,
                    other_input_label: other_input_label,
                    other_value: other_value
                )
                (child)
                <p role="status" data-question-status="" class=(css::STATUS)>
                    (status)
                </p>
                <div class=(css::ACTIONS)>
                    if skippable {
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            attrs: attributes! {
                                type="submit"
                                name=(skip_name)
                                value=(skip_value)
                                formnovalidate=(true)
                                disabled=(disabled)
                                data-question-skip=""
                            },
                            (skip_label)
                        )
                    }
                    button(
                        size: ButtonSize::Sm,
                        attrs: attributes! {
                            type="submit"
                            disabled=(disabled)
                            data-question-submit=""
                        },
                        (submit_label)
                    )
                </div>
            </div>
        </form>
    }))
}

#[cfg(test)]
mod tests {
    use super::{QuestionKind, default_required_label};

    #[test]
    fn required_labels() {
        assert_eq!(
            default_required_label(QuestionKind::Single),
            "Choose an answer."
        );
        assert_eq!(
            default_required_label(QuestionKind::Freeform),
            "Type an answer."
        );
    }
}
