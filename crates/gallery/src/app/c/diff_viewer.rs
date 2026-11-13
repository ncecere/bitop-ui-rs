use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    diff_viewer::{DiffFormat, DiffMode, DiffViewerLabels, diff_viewer},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const PROMPT_BEFORE: &str = "You are a support agent.\nAnswer in one paragraph.\nBe polite.\nCite the handbook.\nNever share internal links.\nAsk before closing a ticket.\nEscalate billing questions.\nUse the customer's name.\nKeep answers short.\nOffer a follow-up.\nSign off with your name.\nLog the conversation.\nTag the ticket.\nClose the ticket.";
const PROMPT_AFTER: &str = "You are a support agent for Acme.\nAnswer in one paragraph.\nBe polite.\nCite the handbook.\nNever share internal links.\nAsk before closing a ticket.\nEscalate billing questions.\nUse the customer's name.\nKeep answers short.\nOffer a follow-up.\nSign off with your name.\nLog the conversation.\nTag the ticket with a topic.\nSet the priority.\nClose the ticket.";

const SETTINGS_BEFORE: &str = r#"{"name":"Support agent","model":"small","limits":{"maxPages":10,"depth":2},"tags":["support","beta"],"owner":"ada"}"#;
const SETTINGS_AFTER: &str = r#"{"name":"Support agent","model":"large","limits":{"maxPages":25,"depth":2},"tags":["support"],"notify":{"email":true}}"#;

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "diff-viewer")
        example(
            title: "Text, unified, with folded unchanged lines",
            diff_viewer(
                label: "System prompt",
                before: PROMPT_BEFORE,
                after: PROMPT_AFTER,
                attrs: attributes! { id="prompt" style="inline-size: 100%" },
                actions: view! {
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Ghost,
                        "Copy new"
                    )
                }
            )
        )
        example(
            title: "JSON, split",
            diff_viewer(
                label: "Changes to the agent",
                format: DiffFormat::Json,
                before: SETTINGS_BEFORE,
                after: SETTINGS_AFTER,
                mode: DiffMode::Split,
                attrs: attributes! { id="settings" style="inline-size: 100%" }
            )
        )
        example(
            title: "Long lines that scroll, in a short area, without the toggle",
            diff_viewer(
                label: "Configuration",
                before: "url = https://example.com/a/very/long/path/that/does/not/wrap/because/wrapping/is/turned/off/for/this/viewer/and/goes/on\nretries = 3\ntimeout = 10\nregion = eu\ncache = on\nlog = info",
                after: "url = https://example.com/a/very/long/path/that/does/not/wrap/because/wrapping/is/turned/off/for/this/viewer/and/goes/on/further\nretries = 5\ntimeout = 10\nregion = eu\ncache = off\nlog = debug",
                wrap: false,
                context_lines: 0,
                max_height: "8rem",
                show_mode_toggle: false,
                attrs: attributes! { id="config" style="inline-size: 100%" }
            )
        )
        example(
            title: "No changes, and a value that was added",
            <div style="display: grid; gap: var(--space-4); inline-size: 100%">
                diff_viewer(
                    label: "Unchanged prompt",
                    before: "Same text.",
                    after: "Same text.",
                    attrs: attributes! { id="same" }
                )
                diff_viewer(
                    label: "New settings",
                    format: DiffFormat::Json,
                    before: "",
                    after: r#"{"enabled":true}"#,
                    attrs: attributes! { id="added" }
                )
            </div>
        )
        example(
            title: "Every string is an argument",
            diff_viewer(
                label: "Änderungen",
                before: "eins\nzwei",
                after: "eins\ndrei",
                labels: DiffViewerLabels {
                    before: "Vorher",
                    after: "Nachher",
                    added: "Hinzugefügt",
                    removed: "Entfernt",
                    unified: "Vereint",
                    split: "Geteilt",
                    mode: "Ansicht",
                    content: "Inhalt",
                    line: "Zeile {side}",
                    text_summary_one: "{added} Zeile hinzugefügt, {removed} entfernt",
                    ..DiffViewerLabels::default()
                },
                attrs: attributes! { id="german" style="inline-size: 100%" }
            )
        )
    })
}
