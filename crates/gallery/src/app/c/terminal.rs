use bitop_ui::components::{button::button, icon::Icon, input::textarea};
use bitop_ui_ai::components::terminal::{
    TerminalLabels, terminal, terminal_actions, terminal_chunk, terminal_content,
    terminal_copy_button, terminal_header, terminal_status, terminal_title,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const BUILD: &str = "\u{1b}[1m\u{1b}[32m   Compiling\u{1b}[0m bitop-ui v0.1.0\n\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[1m: unused variable: `x`\u{1b}[0m\n \u{1b}[34m-->\u{1b}[0m src/lib.rs:12:9\n\u{1b}[1m\u{1b}[31merror[E0308]\u{1b}[0m\u{1b}[1m: mismatched types\u{1b}[0m\n\u{1b}[2mnote: expected `u32`, found `&str`\u{1b}[0m\n\u{1b}[3mitalic\u{1b}[23m \u{1b}[4munderlined\u{1b}[24m \u{1b}[7minverse\u{1b}[27m \u{1b}[35mmagenta\u{1b}[0m \u{1b}[36mcyan\u{1b}[0m \u{1b}[90mgray\u{1b}[0m \u{1b}[41m red background \u{1b}[0m\nDownloading 10%\rDownloading 55%\rDownloading 100%\n\u{1b}[?25l\u{1b}]0;window title\u{7}\u{1b}[38;2;255;0;0mtruecolour is plain\u{1b}[0m";

/// The streamed run, a line at a time.
const STREAM: &[&str] = &[
    "\u{1b}[2m$ cargo test\u{1b}[0m\n",
    "running 24 tests\n",
    "test parses_v8 ... \u{1b}[32mok\u{1b}[0m\n",
    "test parses_gecko ... \u{1b}[32mok\u{1b}[0m\n",
    "test folds_internal ... \u{1b}[32mok\u{1b}[0m\n",
    "test strips_codes ... \u{1b}[32mok\u{1b}[0m\n",
    "test joins_runs ... \u{1b}[32mok\u{1b}[0m\n",
    "test carriage_returns ... \u{1b}[32mok\u{1b}[0m\n",
    "test extended_colours ... \u{1b}[32mok\u{1b}[0m\n",
    "test private_modes ... \u{1b}[33mignored\u{1b}[0m\n",
    "test windows_paths ... \u{1b}[31mFAILED\u{1b}[0m\n",
    "test unicode_names ... \u{1b}[32mok\u{1b}[0m\n",
    "\ntest result: \u{1b}[31mFAILED\u{1b}[0m. 10 passed; 1 failed; 1 ignored\n",
];

const GERMAN: TerminalLabels<'static> = TerminalLabels {
    running: "Läuft",
    placeholder: "Keine Ausgabe",
    clear: "Ausgabe leeren",
    copy: "Ausgabe",
    copy_label: "{label} kopieren",
    copied_label: "{label} kopiert",
};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "terminal")
        example(
            title: "Build output, with colours",
            <div data-example="build" style="inline-size: 100%; max-inline-size: 44rem">
                terminal(
                    id: "build",
                    output: BUILD,
                    title: "cargo build",
                    clearable: true
                )
            </div>
        )
        example(
            title: "Streaming",
            <div
                data-example="stream"
                style="display: grid; gap: var(--space-2); inline-size: 100%; max-inline-size: 44rem"
            >
                terminal(
                    id: "stream",
                    title: "cargo test",
                    max_height: "8rem",
                    clearable: true
                )
                <div data-terminal-chunks="stream" hidden="">
                    for chunk in STREAM {
                        <template>terminal_chunk(output: chunk)</template>
                    }
                </div>
                <div>
                    button(
                        attrs: attributes! { data-gallery-terminal="stream" },
                        "Run the tests"
                    )
                </div>
            </div>
        )
        example(
            title: "Running, before any output",
            <div
                data-example="waiting"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                terminal(id: "waiting", title: "npm install", streaming: true)
            </div>
        )
        example(
            title: "No output",
            <div data-example="empty" style="inline-size: 100%; max-inline-size: 44rem">
                terminal(id: "empty", title: "make lint")
            </div>
        )
        example(
            title: "Composed from the parts",
            <div
                data-example="custom"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                terminal(
                    id: "deploy",
                    custom: true,
                    terminal_header(
                        terminal_title(
                            id: "deploy",
                            title: "Deploy log",
                            icon_name: Icon::Globe
                        )
                        terminal_status()
                        terminal_actions(terminal_copy_button(id: "deploy"))
                    )
                    terminal_content(
                        id: "deploy",
                        output: "\u{1b}[32m✔\u{1b}[0m Uploaded 14 files\n\u{1b}[32m✔\u{1b}[0m Live at https://example.com",
                        auto_scroll: false
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                terminal(
                    id: "german",
                    title: "Konsole",
                    clearable: true,
                    labels: GERMAN
                )
            </div>
        )
        example(
            title: "Paste here to try copying",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                textarea(attrs: attributes! { aria-label="Paste target" })
            </div>
        )
    })
}
