//! Picks a text-to-speech voice in a dialog, with a search box and a list of voices,
//! each with its details and an optional preview button.
//!
//! The trigger opens the base set's dialog. The voices are native radio buttons in a
//! group named by `label`: the arrow keys move between them and choose, Tab reaches the
//! preview buttons, and the chosen voice is sent with the form under `name`. The dialog
//! stays open so that voices can be compared; Done and Escape close it. The library
//! script filters the list as the search box changes (every word must match) and says
//! how many voices match, keeps the trigger's text on the chosen voice, and empties the
//! search when the dialog closes.
//!
//! A voice's preview button sends a cancelable `bitop:voice-preview` event from the
//! selector with `detail.id` and `detail.playing`. A voice with a `preview_url` is
//! played by the script itself, in an `HTMLAudioElement`, unless the event is
//! canceled; a page that plays previews itself sets `data-previewing` (and
//! `data-preview-loading`) on the selector to the voice's ID, and the buttons follow.
//!
//! ```ignore
//! let voices = [
//!     Voice::new("ava", "Ava").gender("Female").accent("British")
//!         .preview_url("/voices/ava.mp3"),
//!     Voice::new("leo", "Leo").description("Calm, for long reads"),
//! ];
//! view! {
//!     voice_selector(id: "voice", name: "voice", label: "Voice", voices: &voices,
//!         value: "ava", previews: true)
//! }
//! ```
//!
//! Differences from upstream: `onValueChange` is the radios' `change` event and the
//! form, `onPreview` the event above (or `preview_url`), and `previewingId` and
//! `previewLoadingId` are the attributes above or the first state. A voice's icon and
//! a custom trigger are not ported.

use bitop_ui::{
    components::{
        button::{ButtonSize, ButtonVariant},
        dialog::{dialog, dialog_actions, dialog_close, dialog_description, dialog_title},
        icon::{Icon, icon},
        input::input,
        spinner::{SpinnerSize, spinner},
    },
    css::button as button_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::voice_selector as css;

/// One voice of a [`voice_selector`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Voice<'a> {
    /// What is sent.
    pub id: &'a str,
    /// What is shown, and what the trigger says once it is chosen.
    pub name: &'a str,
    /// A line under the details.
    pub description: &'a str,
    /// Free text, such as "Female" or "Neutral".
    pub gender: &'a str,
    /// Free text, such as "British".
    pub accent: &'a str,
    /// A language name or tag, such as "English (UK)".
    pub language: &'a str,
    /// Such as "Young adult".
    pub age: &'a str,
    /// A sample the script plays for the preview button.
    pub preview_url: &'a str,
}

impl<'a> Voice<'a> {
    /// A voice with an id and a name.
    #[must_use]
    pub const fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            description: "",
            gender: "",
            accent: "",
            language: "",
            age: "",
            preview_url: "",
        }
    }

    /// Adds a description.
    #[must_use]
    pub const fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Adds a gender.
    #[must_use]
    pub const fn gender(mut self, gender: &'a str) -> Self {
        self.gender = gender;
        self
    }

    /// Adds an accent.
    #[must_use]
    pub const fn accent(mut self, accent: &'a str) -> Self {
        self.accent = accent;
        self
    }

    /// Adds a language.
    #[must_use]
    pub const fn language(mut self, language: &'a str) -> Self {
        self.language = language;
        self
    }

    /// Adds an age.
    #[must_use]
    pub const fn age(mut self, age: &'a str) -> Self {
        self.age = age;
        self
    }

    /// Adds a sample to play.
    #[must_use]
    pub const fn preview_url(mut self, url: &'a str) -> Self {
        self.preview_url = url;
        self
    }

    /// The details on one line: "Female · British · English (UK)".
    ///
    /// ```
    /// use bitop_ui_ai::components::voice_selector::Voice;
    ///
    /// let ava = Voice::new("ava", "Ava").gender("Female").language("English (UK)");
    /// assert_eq!(ava.meta(), "Female · English (UK)");
    /// ```
    #[must_use]
    pub fn meta(&self) -> String {
        [self.gender, self.accent, self.language, self.age]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// What the search looks in, in lower case.
    fn search_text(&self) -> String {
        [self.name, self.description, &self.meta(), self.id]
            .join(" ")
            .to_lowercase()
    }
}

/// The size of a [`voice_selector`]'s trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VoiceSelectorSize {
    Sm,
    #[default]
    Md,
}

struct Row<'a> {
    voice: Voice<'a>,
    meta: String,
    search: String,
    preview_label: String,
    checked: bool,
}

/// A voice selector: a trigger and its dialog.
///
/// - `id` goes on the dialog and is the base of the parts' IDs.
/// - `label` names the trigger ("Voice: Ava") and the list; `name` is the form field;
///   `value` the chosen voice's id.
/// - `previews` adds a preview button to each voice; `previewing` and
///   `preview_loading` are the voices whose sample is playing or loading.
/// - `title`, `description`, `placeholder`, `search_placeholder`, `search_label`,
///   `empty_text`, `count_one` and `count_other` (with `{n}`), `preview_label` (with
///   `{name}`) and `done_label` are the strings.
/// - `size` and `disabled` are the trigger's.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn voice_selector(
    cx: &Cx,
    id: &str,
    label: &str,
    voices: &[Voice<'_>],
    #[default] name: &str,
    #[default] value: &str,
    #[default] previews: bool,
    #[default] previewing: &str,
    #[default] preview_loading: &str,
    #[default("Choose a voice")] title: &str,
    #[default] description: &str,
    #[default("Select a voice")] placeholder: &str,
    #[default("Search voices…")] search_placeholder: &str,
    #[default("Search voices")] search_label: &str,
    #[default("No voices match your search.")] empty_text: &str,
    #[default("{n} voice")] count_one: &str,
    #[default("{n} voices")] count_other: &str,
    #[default("Preview {name}")] preview_label: &str,
    #[default("Done")] done_label: &str,
    #[default] size: VoiceSelectorSize,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let selected = voices.iter().find(|voice| voice.id == value);
    let shown = selected.map_or(placeholder, |voice| voice.name);
    let trigger_label = format!("{label}: {shown}");
    let title_id = format!("{id}-title");
    let list_id = format!("{id}-list");
    let field = if name.is_empty() {
        format!("{id}-voice")
    } else {
        name.to_owned()
    };
    let rows: Vec<Row<'_>> = voices
        .iter()
        .map(|voice| Row {
            voice: *voice,
            meta: voice.meta(),
            search: voice.search_text(),
            preview_label: preview_label.replace("{name}", voice.name),
            checked: voice.id == value,
        })
        .collect();
    let preview_class = format!("{} {}", button_css::ROOT, css::PREVIEW);
    let close_id = id.to_owned();
    Ok(Child::new(view! {
        <div
            data-voice-selector=""
            data-label=(label)
            data-count-one=(count_one)
            data-count-other=(count_other)
            data-previewing=((!previewing.is_empty()).then_some(previewing))
            data-preview-loading=((!preview_loading.is_empty()).then_some(
                preview_loading,
            ))
            class=(attrs.remove("class"))
            (attrs)
        >
            <button
                type="button"
                commandfor=(id)
                command="show-modal"
                aria-haspopup="dialog"
                aria-label=(&trigger_label)
                disabled=(disabled)
                data-voice-trigger=""
                data-variant="secondary"
                data-size=(if size == VoiceSelectorSize::Sm { "sm" } else { "md" })
                class=(class!(button_css::ROOT, css::TRIGGER))
            >
                <span aria-hidden="true" class=(css::TRIGGER_ICON)>
                    icon(name: Icon::AudioLines)
                </span>
                <span
                    class=(css::TRIGGER_VALUE)
                    data-voice-value=""
                    data-placeholder=(selected.is_none().then_some(""))
                >
                    (shown)
                </span>
                icon(
                    name: Icon::ChevronsUpDown,
                    attrs: attributes! { class=(css::CHEVRON) }
                )
            </button>
            dialog(
                id: id,
                labelled_by: &title_id,
                attrs: attributes! { class=(css::DIALOG) },
                dialog_title(id: &title_id, (title))
                if !description.is_empty() {
                    dialog_description((description))
                }
                <div class=(css::BODY)>
                    input(
                        attrs: attributes! {
                            type="search"
                            aria-label=(search_label)
                            aria-controls=(&list_id)
                            placeholder=(search_placeholder)
                            autocomplete="off"
                            data-voice-search=""
                            class=(css::SEARCH)
                        }
                    )
                    <div
                        id=(&list_id)
                        role="radiogroup"
                        aria-label=(label)
                        class=(css::LIST)
                    >
                        for row in rows {
                            <div
                                data-voice-row=(row.voice.id)
                                data-name=(row.voice.name)
                                data-search=(row.search)
                                class=(css::ROW)
                            >
                                <label class=(css::OPTION)>
                                    <input
                                        type="radio"
                                        name=(field.as_str())
                                        value=(row.voice.id)
                                        checked=(row.checked)
                                        class=(css::RADIO)
                                    >
                                    <span class=(css::TEXT)>
                                        <span class=(css::NAME)>(row.voice.name)</span>
                                        if !row.meta.is_empty() {
                                            <span class=(css::META)>(row.meta)</span>
                                        }
                                        if !row.voice.description.is_empty() {
                                            <span class=(css::DESCRIPTION)>
                                                (row.voice.description)
                                            </span>
                                        }
                                    </span>
                                </label>
                                if previews {
                                    <button
                                        type="button"
                                        aria-label=(row.preview_label)
                                        aria-pressed=(if previewing == row.voice.id {
                                            "true"
                                        } else {
                                            "false"
                                        })
                                        aria-busy=((preview_loading == row.voice.id).then_some(
                                            "true",
                                        ))
                                        data-voice-preview=(row.voice.id)
                                        data-preview-url=((!row.voice.preview_url.is_empty()).then_some(
                                            row.voice.preview_url,
                                        ))
                                        data-variant="ghost"
                                        data-size="icon"
                                        class=(preview_class.as_str())
                                    >
                                        icon(name: Icon::Play)
                                        icon(name: Icon::Pause)
                                        spinner(
                                            size: SpinnerSize::Sm,
                                            attrs: attributes! { class=(css::SPINNER) }
                                        )
                                    </button>
                                }
                            </div>
                        }
                    </div>
                    <div role="status" data-voice-status=""></div>
                    <template data-voice-empty="">
                        <p class=(css::EMPTY)>(empty_text)</p>
                    </template>
                </div>
                dialog_actions(
                    dialog_close(
                        target: &close_id,
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Md,
                        (done_label)
                    )
                )
            )
        </div>
    }))
}

#[cfg(test)]
mod tests {
    use super::Voice;

    #[test]
    fn meta_and_search() {
        let leo = Voice::new("leo", "Leo")
            .accent("American")
            .age("Middle-aged")
            .description("Calm");
        assert_eq!(leo.meta(), "American · Middle-aged");
        assert_eq!(leo.search_text(), "leo calm american · middle-aged leo");
        assert_eq!(Voice::new("x", "X").meta(), "");
    }
}
