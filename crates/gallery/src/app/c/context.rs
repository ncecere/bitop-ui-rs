use bitop_ui_ai::components::context::{ContextCost, ContextUsage, context};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let usage = ContextUsage {
        input: Some(30_000),
        output: Some(9_000),
        reasoning: Some(2_000),
        cached: Some(1_000),
    };
    let cost = ContextCost {
        input: Some(0.09),
        output: Some(0.135),
        total: Some(0.225),
        ..ContextCost::default()
    };
    Ok(view! {
        page_header(slug: "context")
        example(
            title: "Levels: normal, high, critical",
            <div data-example="levels" style="display: flex; gap: var(--space-3)">
                context(id: "context-low", used_tokens: 12_000, max_tokens: 200_000)
                context(id: "context-high", used_tokens: 160_000, max_tokens: 200_000)
                context(
                    id: "context-critical",
                    used_tokens: 1_900_000,
                    max_tokens: 2_000_000
                )
            </div>
        )
        example(
            title: "With a breakdown and the cost",
            <div data-example="full">
                context(
                    id: "context-full",
                    used_tokens: 42_000,
                    max_tokens: 200_000,
                    usage: usage,
                    cost: cost,
                    model_name: "Claude Sonnet"
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de">
                context(
                    id: "context-german",
                    used_tokens: 50_000,
                    max_tokens: 100_000,
                    cost: ContextCost { total: Some(1.5), ..ContextCost::default() },
                    label: "Kontextfenster",
                    trigger_label: "{label}: {percent} % belegt, {used} von {max} Tokens",
                    total_label: "Gesamtkosten",
                    money_format: "{amount} €"
                )
            </div>
        )
    })
}
