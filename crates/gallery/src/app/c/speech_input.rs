use bitop_ui::{components::input::textarea, css::field as field_css};
use bitop_ui_ai::components::speech_input::{SpeechInputSize, speech_input};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "speech-input")
        example(
            title: "Dictation into a text box, with the status shown",
            <div
                data-example="dictation"
                style="display: grid; gap: var(--space-2); inline-size: 100%"
            >
                <label for="notes" class=(field_css::LABEL)>"Notes"</label>
                textarea(attrs: attributes! { id="notes" name="notes" rows="2" })
                speech_input(id: "dictate", target: "notes", show_status: true)
            </div>
        )
        example(
            title: "Recording where speech recognition is missing",
            <div
                data-example="record"
                style="display: grid; gap: var(--space-2); inline-size: 100%"
            >
                <label for="message" class=(field_css::LABEL)>"Message"</label>
                textarea(attrs: attributes! { id="message" name="message" rows="2" })
                speech_input(
                    id: "record",
                    target: "message",
                    record: true,
                    transcribe_url: "/c/speech-input/transcribe",
                    label: "Dictate a message"
                )
            </div>
        )
        example(
            title: "Small, and disabled",
            <div data-example="variants" style="display: flex; gap: var(--space-3)">
                speech_input(
                    id: "small",
                    size: SpeechInputSize::Sm,
                    label: "Search by voice"
                )
                speech_input(id: "off", disabled: true, label: "Voice input off")
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de">
                speech_input(
                    id: "german",
                    lang: "de-DE",
                    label: "Spracheingabe",
                    show_status: true,
                    listening_text: "Hört zu…",
                    unsupported_text: "Spracheingabe gibt es in diesem Browser nicht.",
                    blocked_text: "Der Mikrofonzugriff ist gesperrt."
                )
            </div>
        )
    })
}
