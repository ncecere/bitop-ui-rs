use bitop_ui::components::button::{ButtonSize, ButtonVariant, button};
use bitop_ui_ai::components::reasoning::{reasoning, reasoning_content, reasoning_trigger};
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
        page_header(slug: "reasoning")
        example(
            title: "Streaming: opens while thinking, closes after",
            <div
                data-example="streaming"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                reasoning(
                    streaming: true,
                    attrs: attributes! { id="live-reasoning" },
                    reasoning_trigger(streaming: true)
                    reasoning_content(
                        <p>
                            "The question is about parental leave. I'll search the handbook, then check the 2025 policy for duration and pay."
                        </p>
                        <p><a href="#policy">"The 2025 policy"</a></p>
                    )
                )
                <div style="display: flex; gap: var(--space-2)">
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        attrs: attributes! {
                            data-gallery-attr="data-streaming"
                            data-gallery-target="live-reasoning"
                        },
                        "Finish thinking"
                    )
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        attrs: attributes! {
                            data-gallery-attr="data-streaming"
                            data-gallery-target="live-reasoning"
                            data-gallery-value=""
                        },
                        "Think again"
                    )
                </div>
            </div>
        )
        example(
            title: "Finished, with a duration",
            <div data-example="done" style="inline-size: 100%">
                reasoning(
                    duration: 12,
                    reasoning_trigger(duration: 12)
                    reasoning_content(
                        <p>
                            "Checked three policies and the FAQ, then compared the dates."
                        </p>
                    )
                )
            </div>
        )
        example(
            title: "Finished, open, duration not known",
            <div data-example="open" style="inline-size: 100%">
                reasoning(
                    open: true,
                    reasoning_trigger()
                    reasoning_content(<p>"A short thought."</p>)
                )
            </div>
        )
        example(
            title: "One second, and its own label",
            <div
                data-example="labels"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                reasoning(
                    duration: 1,
                    reasoning_trigger(duration: 1)
                    reasoning_content(<p>"Quick."</p>)
                )
                reasoning(
                    reasoning_trigger(label: "Show the plan")
                    reasoning_content(<p>"Step one, then step two."</p>)
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                reasoning(
                    streaming: true,
                    auto_close: false,
                    attrs: attributes! { id="german-reasoning" },
                    reasoning_trigger(
                        streaming: true,
                        thinking_label: "Denkt nach…",
                        done_one: "{n} Sekunde nachgedacht",
                        done_other: "{n} Sekunden nachgedacht",
                        done_unknown: "Einige Sekunden nachgedacht"
                    )
                    reasoning_content(<p>"Zuerst die Richtlinie prüfen."</p>)
                )
            </div>
        )
    })
}
