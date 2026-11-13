use bitop_ui_ai::components::web_preview::{
    ConsoleLabels, LogLevel, WebPreviewLog, web_preview, web_preview_back, web_preview_body,
    web_preview_console, web_preview_forward, web_preview_navigation, web_preview_open,
    web_preview_reload, web_preview_url,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const HOME: &str = "/preview?page=home";

const SRCDOC: &str = "<!DOCTYPE html><html lang=\"en\"><head><title>Pricing card</title><style>body{margin:0;font:16px/1.5 system-ui,sans-serif;color:#1f2937;background:#fff}main{padding:16px 20px}</style></head><body><main aria-label=\"Pricing card\"><h1>Pro</h1><p>$12 a month, billed yearly.</p></main></body></html>";

const GERMAN: ConsoleLabels<'static> = ConsoleLabels {
    log: "Log",
    info: "Info",
    warn: "Warnung",
    error: "Fehler",
    errors_one: "{n} Fehler",
    errors_other: "{n} Fehler",
    warnings_one: "{n} Warnung",
    warnings_other: "{n} Warnungen",
    separator: ", ",
};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let logs = vec![
        WebPreviewLog::new(LogLevel::Log, "App mounted").at(1_789_000_000),
        WebPreviewLog::new(LogLevel::Info, "Fetched 12 products").at(1_789_000_001),
        WebPreviewLog::new(LogLevel::Warn, "Image without alt text: hero.png").at(1_789_000_002),
        WebPreviewLog::new(LogLevel::Warn, "Slow layout: 120 ms"),
        WebPreviewLog::new(
            LogLevel::Error,
            "TypeError: Cannot read properties of undefined (reading 'price')",
        )
        .at(1_789_000_003),
    ];
    let german_logs = vec![WebPreviewLog::new(LogLevel::Warn, "Veraltete API")];
    Ok(view! {
        page_header(slug: "web-preview")
        example(
            title: "A local page, with the bar and the console",
            <div data-example="full" style="inline-size: 100%; block-size: 30rem">
                web_preview(
                    url: HOME,
                    attrs: attributes! { id="full-preview" data-gallery-preview="" },
                    web_preview_navigation(
                        web_preview_back(tooltip: "full-back-tip")
                        web_preview_forward(tooltip: "full-forward-tip")
                        web_preview_reload(url: HOME, tooltip: "full-reload-tip")
                        web_preview_url(url: HOME)
                        web_preview_open(url: HOME, tooltip: "full-open-tip")
                    )
                    web_preview_body(title: "Preview of the generated site", url: HOME)
                    web_preview_console(logs: &logs)
                )
                <p data-preview-event="" style="margin-block-start: var(--space-2)"></p>
            </div>
        )
        example(
            title: "HTML in the frame",
            <div data-example="srcdoc" style="inline-size: 100%; block-size: 18rem">
                web_preview(
                    attrs: attributes! { id="srcdoc-preview" },
                    web_preview_body(
                        title: "Preview of the pricing card",
                        srcdoc: SRCDOC
                    )
                    web_preview_console(open: true)
                )
            </div>
        )
        example(
            title: "No address yet",
            <div data-example="empty" style="inline-size: 100%; block-size: 20rem">
                web_preview(
                    attrs: attributes! { id="empty-preview" },
                    web_preview_navigation(
                        label: "Empty preview navigation",
                        web_preview_back()
                        web_preview_forward()
                        web_preview_reload()
                        web_preview_url()
                        web_preview_open()
                    )
                    web_preview_body(title: "Preview of an address you enter")
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; block-size: 16rem"
            >
                web_preview(
                    attrs: attributes! { id="german-preview" },
                    web_preview_navigation(
                        label: "Vorschau-Navigation",
                        web_preview_back(label: "Zurück")
                        web_preview_forward(label: "Vorwärts")
                        web_preview_reload(label: "Neu laden")
                        web_preview_url(
                            label: "Adresse",
                            placeholder: "Adresse eingeben"
                        )
                    )
                    web_preview_body(
                        title: "Vorschau",
                        empty_text: "Geben Sie eine Adresse ein, um die Vorschau zu laden."
                    )
                    web_preview_console(
                        logs: &german_logs,
                        label: "Konsole",
                        region_label: "Konsolenausgabe",
                        labels: GERMAN
                    )
                )
            </div>
        )
    })
}
