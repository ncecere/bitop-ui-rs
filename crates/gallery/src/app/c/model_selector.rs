use bitop_ui::components::badge::BadgeTone;
use bitop_ui_ai::components::model_selector::{ModelOption, ModelSelectorSize, model_selector};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header, query_string};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let query = query_string(cx);
    let models = [
        ModelOption::new("claude-sonnet", "Claude Sonnet", "Anthropic")
            .description("Balanced speed and quality")
            .capabilities(&["vision", "tools", "reasoning"])
            .context_window(200_000),
        ModelOption::new("claude-haiku", "Claude Haiku", "Anthropic")
            .description("Fast and light")
            .capabilities(&["tools"])
            .context_window(200_000),
        ModelOption::new("gpt-large", "GPT Large", "OpenAI")
            .capabilities(&["vision", "tools"])
            .context_window(128_000),
        ModelOption::new("gpt-mini", "GPT Mini", "OpenAI").context_window(32_768),
        ModelOption::new("local-llama", "Llama 8B", "Local")
            .description("Runs on this machine")
            .capabilities(&["slow"])
            .context_window(8_192),
    ];
    let labels = [
        ("vision", "Vision"),
        ("tools", "Tools"),
        ("reasoning", "Reasoning"),
    ];
    let tones = [("slow", BadgeTone::Warning)];
    Ok(view! {
        page_header(slug: "model-selector")
        example(
            title: "Grouped by provider, with badges",
            <form action="/c/model-selector" data-example="form">
                model_selector(
                    id: "model",
                    name: "model",
                    label: "Model",
                    models: &models,
                    value: "claude-sonnet",
                    capability_labels: &labels,
                    capability_tones: &tones
                )
                <output data-query="">(query)</output>
            </form>
        )
        example(
            title: "Small, with nothing chosen",
            <div data-example="small">
                model_selector(
                    id: "small-model",
                    name: "small",
                    label: "Model",
                    models: &models,
                    size: ModelSelectorSize::Sm
                )
            </div>
        )
        example(
            title: "Disabled",
            <div data-example="disabled">
                model_selector(
                    id: "disabled-model",
                    name: "locked",
                    label: "Model",
                    models: &models,
                    value: "gpt-mini",
                    disabled: true
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de">
                model_selector(
                    id: "german-model",
                    name: "modell",
                    label: "Modell",
                    models: &models,
                    placeholder: "Modell wählen",
                    search_placeholder: "Modelle suchen…",
                    search_label: "Modelle suchen",
                    empty: "Keine Modelle gefunden.",
                    context_label: "{tokens} Kontext",
                    capability_labels: &[("vision", "Bilder"), ("tools", "Werkzeuge")]
                )
            </div>
        )
    })
}
