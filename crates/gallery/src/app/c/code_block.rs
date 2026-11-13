use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    icon::{Icon, icon},
    input::textarea,
};
use bitop_ui_ai::components::code_block::code_block;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const RUST: &str = "use bitop_ui_ai::components::code_block::code_block;\n\nfn main() {\n    let greeting = \"Hello, world\";\n    println!(\"{greeting}\");\n}\n";

const LONG: &str = "curl -X POST https://api.example.com/v1/chat/completions -H \"Authorization: Bearer $TOKEN\" -H \"Content-Type: application/json\" -d '{\"model\": \"atlas-large\", \"messages\": [{\"role\": \"user\", \"content\": \"Hello\"}]}'";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let tall: String = (1..=40)
        .map(|n| format!("line {n}: the block scrolls past its maximum height\n"))
        .collect();
    Ok(view! {
        page_header(slug: "code-block")
        example(
            title: "With a language",
            <div data-example="language" style="inline-size: 100%">
                code_block(code: RUST, language: "rs")
            </div>
        )
        example(
            title: "File name and line numbers",
            <div data-example="numbers" style="inline-size: 100%">
                code_block(
                    code: RUST,
                    language: "rust",
                    filename: "src/main.rs",
                    show_line_numbers: true
                )
            </div>
        )
        example(
            title: "A long line scrolls, or wraps",
            <div
                data-example="scroll"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                code_block(code: LONG, language: "bash")
                <div data-example="wrap">
                    code_block(
                        code: LONG,
                        language: "bash",
                        wrap: true,
                        show_line_numbers: true
                    )
                </div>
            </div>
        )
        example(
            title: "Maximum height",
            <div data-example="tall" style="inline-size: 100%">
                code_block(code: &tall, language: "text", max_height: "12rem")
            </div>
        )
        example(
            title: "No header, and no language",
            <div
                data-example="bare"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                code_block(code: "npm install bitop-ui", hide_header: true)
                <div data-example="plain">
                    code_block(code: "plain text\nwithout a language")
                </div>
            </div>
        )
        example(
            title: "Header actions, and not copyable",
            <div data-example="actions" style="inline-size: 100%">
                code_block(
                    code: "SELECT id, title FROM notes WHERE archived = false;",
                    language: "sql",
                    copyable: false,
                    actions: view! {
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            attrs: attributes! { data-run="" },
                            icon(name: Icon::ArrowRight)
                            "Run"
                        )
                    }
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                code_block(
                    code: "echo hallo",
                    fallback_label: "Quelltext",
                    copy_label_plain: "Quelltext",
                    copy_button_label: "{label} kopieren",
                    copied_button_label: "{label} kopiert",
                    copied_status: "{label} kopiert",
                    attrs: attributes! { data-strings="" }
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
