use std::sync::LazyLock;

use bitop_ui_ai::components::voice_selector::{Voice, VoiceSelectorSize, voice_selector};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, silent_wav_data_url};

const VOICES: &[Voice<'static>] = &[
    Voice::new("ava", "Ava")
        .gender("Female")
        .accent("British")
        .language("English (UK)"),
    Voice::new("leo", "Leo")
        .gender("Male")
        .accent("American")
        .age("Middle-aged")
        .description("Calm, for long reads"),
    Voice::new("sam", "Sam")
        .gender("Neutral")
        .language("English (US)"),
    Voice::new("mia", "Mia")
        .accent("Australian")
        .description("Bright and quick"),
];

/// Two seconds of silence, made once: the voices borrow it for the whole run.
static SAMPLE: LazyLock<String> = LazyLock::new(|| silent_wav_data_url(2));

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let sampled = [
        Voice::new("ava", "Ava")
            .gender("Female")
            .preview_url(&SAMPLE),
        Voice::new("leo", "Leo").gender("Male").preview_url(&SAMPLE),
    ];
    Ok(view! {
        page_header(slug: "voice-selector")
        example(
            title: "In a form, with previews played by the page",
            <form data-example="form" method="get" action="/c/voice-selector">
                voice_selector(
                    id: "voice",
                    name: "voice",
                    label: "Voice",
                    voices: VOICES,
                    value: "leo",
                    previews: true,
                    description: "Every voice reads the same sentence.",
                    attrs: attributes! { data-gallery-voice="" }
                )
            </form>
        )
        example(
            title: "Samples played by the script, nothing chosen, small",
            <div data-example="samples">
                voice_selector(
                    id: "sampled",
                    label: "Narrator",
                    voices: &sampled,
                    previews: true,
                    size: VoiceSelectorSize::Sm
                )
            </div>
        )
        example(
            title: "Disabled, and no voices",
            <div data-example="other" style="display: flex; gap: var(--space-3)">
                voice_selector(
                    id: "locked",
                    label: "Locked voice",
                    voices: VOICES,
                    value: "ava",
                    disabled: true
                )
                voice_selector(id: "none", label: "Backup voice", voices: &[])
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de">
                voice_selector(
                    id: "stimme",
                    label: "Stimme",
                    voices: VOICES,
                    title: "Stimme wählen",
                    placeholder: "Stimme wählen",
                    search_placeholder: "Stimmen suchen…",
                    search_label: "Stimmen suchen",
                    empty_text: "Keine Stimme passt.",
                    count_one: "{n} Stimme",
                    count_other: "{n} Stimmen",
                    done_label: "Fertig"
                )
            </div>
        )
    })
}
