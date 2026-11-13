use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    icon::{Icon, icon},
};
use bitop_ui_ai::{
    ChatStatus,
    components::prompt_input::{
        PromptCount, prompt_input, prompt_input_attach_button, prompt_input_attachments,
        prompt_input_button, prompt_input_submit, prompt_input_textarea, prompt_input_toolbar,
        prompt_input_tools,
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_string};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let query = query_string(cx);
    Ok(view! {
        page_header(slug: "prompt-input")
        example(
            title: "With attachments and tools",
            <div
                data-example="full"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                prompt_input(
                    action: "/c/prompt-input",
                    method: "get",
                    attachments: true,
                    accept: "image/*,.pdf,.md,.txt",
                    max_files: 3,
                    max_file_size: 1024 * 1024,
                    attrs: attributes! { id="composer" },
                    prompt_input_attachments()
                    prompt_input_textarea(placeholder: "Ask about a policy…")
                    prompt_input_toolbar(
                        prompt_input_tools(
                            prompt_input_attach_button(tooltip: "attach-tip")
                            prompt_input_button(
                                label: "Search the web",
                                show_label: true,
                                pressed: false,
                                name: "web",
                                icon(name: Icon::Globe)
                            )
                        )
                        prompt_input_submit()
                    )
                )
                <output data-query="">(query)</output>
            </div>
        )
        example(
            title: "Following the chat's status",
            <div
                data-example="status"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                prompt_input(
                    action: "/c/prompt-input",
                    method: "get",
                    status: ChatStatus::Streaming,
                    attrs: attributes! { id="streaming-composer" },
                    prompt_input_textarea(value: "Draft a reply")
                    prompt_input_toolbar(
                        prompt_input_submit(status: ChatStatus::Streaming)
                    )
                )
                <div style="display: flex; flex-wrap: wrap; gap: var(--space-2)">
                    for status in ["ready", "submitted", "streaming", "error"] {
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Outline,
                            attrs: attributes! {
                                data-gallery-attr="data-status"
                                data-gallery-target="streaming-composer"
                                data-gallery-value=(status)
                            },
                            (status)
                        )
                    }
                </div>
            </div>
        )
        example(
            title: "Waiting, with and without Stop",
            <div
                data-example="waiting"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                prompt_input(
                    status: ChatStatus::Submitted,
                    attrs: attributes! { id="sending-composer" },
                    prompt_input_textarea()
                    prompt_input_toolbar(
                        prompt_input_submit(status: ChatStatus::Submitted)
                    )
                )
                prompt_input(
                    status: ChatStatus::Submitted,
                    attrs: attributes! { id="early-stop-composer" },
                    prompt_input_textarea()
                    prompt_input_toolbar(
                        prompt_input_submit(
                            status: ChatStatus::Submitted,
                            stop_while_submitted: true,
                            show_label: true
                        )
                    )
                )
            </div>
        )
        example(
            title: "A character limit",
            <div
                data-example="limit"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                prompt_input(
                    attrs: attributes! { id="limit-composer" },
                    prompt_input_textarea(max_length: 40, label: "Short message")
                    prompt_input_toolbar(prompt_input_submit(show_label: true))
                )
                prompt_input(
                    prompt_input_textarea(
                        max_length: 2000,
                        show_count: PromptCount::Always,
                        label: "Long message",
                        submit_on_enter: false
                    )
                    prompt_input_toolbar(prompt_input_submit())
                )
            </div>
        )
        example(
            title: "Disabled",
            <div data-example="disabled" style="inline-size: 100%">
                prompt_input(
                    disabled: true,
                    attachments: true,
                    attrs: attributes! { id="disabled-composer" },
                    prompt_input_attachments(disabled: true)
                    prompt_input_textarea(
                        disabled: true,
                        placeholder: "Sign in to chat"
                    )
                    prompt_input_toolbar(
                        prompt_input_tools(
                            prompt_input_attach_button(disabled: true)
                            prompt_input_button(
                                label: "Search the web",
                                pressed: false,
                                disabled: true,
                                icon(name: Icon::Globe)
                            )
                        )
                        prompt_input_submit(disabled: true)
                    )
                )
            </div>
        )
        example(
            title: "One image at a time",
            <div data-example="single" style="inline-size: 100%">
                prompt_input(
                    attachments: true,
                    multiple: false,
                    accept: "image/png",
                    attrs: attributes! { id="single-composer" },
                    prompt_input_attachments()
                    prompt_input_textarea()
                    prompt_input_toolbar(
                        prompt_input_tools(prompt_input_attach_button())
                        prompt_input_submit()
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                prompt_input(
                    attachments: true,
                    accept: ".pdf",
                    type_error: "{name} wird nicht unterstützt.",
                    attrs: attributes! { id="german-composer" },
                    prompt_input_attachments(
                        label: "Anhänge",
                        remove_label: "{name} entfernen"
                    )
                    prompt_input_textarea(
                        placeholder: "Frag etwas…",
                        label: "Nachricht",
                        max_length: 10,
                        count_label: "Zeichen",
                        limit_label: "Höchstens {max} Zeichen"
                    )
                    prompt_input_toolbar(
                        prompt_input_tools(
                            prompt_input_attach_button(label: "Dateien anhängen")
                        )
                        prompt_input_submit(
                            send_label: "Nachricht senden",
                            stop_label: "Antwort stoppen",
                            sending_label: "Wird gesendet",
                            show_label: true,
                            send_text: "Senden",
                            stop_text: "Stopp",
                            sending_text: "Senden…"
                        )
                    )
                )
            </div>
        )
    })
}
