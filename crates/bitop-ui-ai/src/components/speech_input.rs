//! A microphone toggle that turns speech into text.
//!
//! The button is a toggle (`aria-pressed`). The library script picks how to listen:
//!
//! - With the Web Speech API (`SpeechRecognition`), final phrases go into the text box
//!   whose ID is `target` (a [`prompt_input_textarea`], say), and the phrase being
//!   spoken is shown under the button.
//! - Otherwise, with `record`, it records with `MediaRecorder`. When recording stops,
//!   a `bitop:speech-audio` event goes out with `detail.audio` (a `Blob`); a page that
//!   transcribes it calls `detail.respondWith(promise)` with a promise of the text.
//!   Without that, the audio is posted to `transcribe_url`, whose response is the text.
//! - With neither, the button is disabled and `unsupported_text` says why; nothing
//!   throws.
//!
//! Before text goes into the target, a cancelable `bitop:speech-result` event goes out
//! with `detail.text`; `bitop:speech-interim`, `bitop:speech-listening` and
//! `bitop:speech-error` (with `detail.code`) say the rest. "Listening…",
//! "Transcribing…" and errors are announced in a polite status line; the phrase being
//! spoken is shown but not announced word by word.
//!
//! ```ignore
//! view! {
//!     prompt_input_textarea(attrs: attributes! { id="message" })
//!     speech_input(id: "dictate", target: "message", record: true,
//!         transcribe_url: "/transcribe")
//! }
//! ```
//!
//! Differences from upstream: the callbacks are the events above and the text box in
//! `target`; `onAudioRecorded` is the `bitop:speech-audio` event or
//! `transcribe_url`, and `record` turns the recorder on, as upstream does with the
//! presence of that callback.
//!
//! [`prompt_input_textarea`]: super::prompt_input::prompt_input_textarea

use bitop_ui::{
    components::{
        icon::{Icon, icon},
        spinner::{SpinnerSize, spinner},
    },
    css::button as button_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::css::speech_input as css;

/// The size of a [`speech_input`]'s button. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpeechInputSize {
    Sm,
    #[default]
    Md,
}

impl SpeechInputSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A speech input.
///
/// - `id` is the base of the parts' IDs.
/// - `target` is the ID of the text box the text goes into.
/// - `label` names the toggle; `lang` is the language listened for (by default the
///   page's, or `en-US`).
/// - `record` records where speech recognition is missing; `transcribe_url` is where
///   the audio is posted, unless the page answers `bitop:speech-audio`.
/// - `show_interim` shows the phrase being spoken; `show_status` shows the status
///   line, which is always announced and always shown for errors.
/// - `listening_text`, `processing_text`, `unsupported_text`, `blocked_text`,
///   `no_speech_text`, `not_found_text`, `network_text` and `error_text` are the
///   strings.
/// - `size` and `disabled` are the button's.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn speech_input(
    cx: &Cx,
    id: &str,
    #[default] target: &str,
    #[default("Voice input")] label: &str,
    #[default] lang: &str,
    #[default] record: bool,
    #[default] transcribe_url: &str,
    #[default(true)] show_interim: bool,
    #[default] show_status: bool,
    #[default("Listening…")] listening_text: &str,
    #[default("Transcribing…")] processing_text: &str,
    #[default("Voice input isn't supported in this browser.")] unsupported_text: &str,
    #[default("Microphone access is blocked.")] blocked_text: &str,
    #[default("No speech was detected.")] no_speech_text: &str,
    #[default("No microphone was found.")] not_found_text: &str,
    #[default("Speech recognition needs a network connection.")] network_text: &str,
    #[default("Voice input stopped because of an error.")] error_text: &str,
    #[default] size: SpeechInputSize,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let status_id = format!("{id}-status");
    Ok(view! {
        <div
            id=(id)
            data-speech-input=""
            data-status="idle"
            data-mode="none"
            data-target=((!target.is_empty()).then_some(target))
            data-lang=((!lang.is_empty()).then_some(lang))
            data-record=(record.then_some(""))
            data-transcribe-url=((!transcribe_url.is_empty()).then_some(transcribe_url))
            data-show-interim=(show_interim.then_some(""))
            data-show-status=(show_status.then_some(""))
            data-disabled=(disabled.then_some(""))
            data-size=(size.as_str())
            data-listening-text=(listening_text)
            data-processing-text=(processing_text)
            data-unsupported-text=(unsupported_text)
            data-blocked-text=(blocked_text)
            data-no-speech-text=(no_speech_text)
            data-not-found-text=(not_found_text)
            data-network-text=(network_text)
            data-error-text=(error_text)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <span class=(css::BUTTON)>
                <span aria-hidden="true" class=(css::PULSE)></span>
                <button
                    type="button"
                    aria-label=(label)
                    aria-pressed="false"
                    disabled=(true)
                    data-speech-toggle=""
                    data-variant="outline"
                    data-size="icon"
                    class=(class!(button_css::ROOT, css::TOGGLE))
                >
                    icon(name: Icon::Mic)
                    icon(name: Icon::Square)
                    spinner(
                        size: SpinnerSize::Sm,
                        attrs: attributes! { class=(css::SPINNER) }
                    )
                </button>
            </span>
            <span
                id=(&status_id)
                role="status"
                class=(class!(css::STATUS, "sr-only"))
                data-speech-status=""
            ></span>
            <span class=(css::INTERIM) data-speech-interim=""></span>
        </div>
    })
}
