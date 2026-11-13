use bitop_ui_ai::components::mic_selector::{MicSelectorSize, mic_selector};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "mic-selector")
        example(
            title: "In a form",
            <form
                data-example="form"
                style="inline-size: 100%; max-inline-size: 22rem"
                method="get"
                action="/c/mic-selector"
            >
                mic_selector(id: "mic", name: "microphone", label: "Microphone")
            </form>
        )
        example(
            title: "Small, label hidden, and disabled",
            <div
                data-example="variants"
                style="display: grid; gap: var(--space-4); inline-size: 100%; max-inline-size: 22rem"
            >
                mic_selector(
                    id: "mic-small",
                    label: "Input",
                    hide_label: true,
                    size: MicSelectorSize::Sm
                )
                mic_selector(
                    id: "mic-disabled",
                    label: "Recording input",
                    disabled: true
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 22rem"
            >
                mic_selector(
                    id: "mic-german",
                    label: "Mikrofon",
                    placeholder: "Mikrofon wählen",
                    unnamed_label: "Mikrofon {n}",
                    permission_label: "Mikrofonzugriff erlauben",
                    unsupported_text: "Die Mikrofonauswahl gibt es in diesem Browser nicht.",
                    blocked_text: "Der Mikrofonzugriff ist gesperrt."
                )
            </div>
        )
    })
}
