//! A whole chat screen built from the AI parts, as upstream's "Chat example" page. It
//! needs no server API: the reply is canned, rendered here in templates, and
//! `client/chat-example.js` plays it into the page as a server streaming HTML would.

use bitop_ui::components::{
    button::BUTTON,
    icon::{Icon, icon},
};
use bitop_ui_ai::{
    ChatStatus,
    components::{
        conversation::{
            EmptyTitle, conversation, conversation_announcer, conversation_content,
            conversation_empty_state, conversation_scroll_button,
        },
        inline_citation::CitationSource,
        message::{
            MessageRole, message, message_action, message_actions, message_content,
            message_copy_action,
        },
        model_selector::{ModelOption, ModelSelectorSize, model_selector},
        prompt_input::{
            prompt_input, prompt_input_attach_button, prompt_input_attachments,
            prompt_input_button, prompt_input_submit, prompt_input_textarea, prompt_input_toolbar,
            prompt_input_tools,
        },
        reasoning::{reasoning, reasoning_content, reasoning_trigger},
        response::{ResponseStrings, response, response_block, response_blocks},
        sources::{source, sources, sources_content, sources_trigger},
        suggestion::{suggestion, suggestions},
    },
    markdown::{Block, ResponseImages, ResponseStream},
};
use topcoat::{
    Result,
    asset::asset,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::page_header;

const REASONING: &str = "The question is about parental leave. I'll check the 2025 policy for duration and pay, and the FAQ for whether it can be split. Cite each claim.";

const ANSWER: &str = "Under the **2025 parental leave policy**, eligible employees get **16 weeks of fully paid leave** [1]. It applies to birth, adoption and foster parents, and can be taken any time within 12 months of the child's arrival [1].

| | Details |
| --- | --- |
| Duration | 16 weeks, fully paid |
| Window | Within 12 months |
| Split | Up to 3 blocks of 2+ weeks [2] |

To take it, submit a request in the HR portal at least **8 weeks** before you start [3]:

```text
HR portal → Time off → New request → Parental leave
```

Want me to draft the request?";

/// One step of the canned reply: the blocks that changed, and how many there are.
struct Stage {
    changed: Vec<(usize, Block)>,
    len: usize,
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let models = [
        ModelOption::new("claude-sonnet", "Claude Sonnet", "Anthropic")
            .capabilities(&["vision", "tools"])
            .context_window(200_000),
        ModelOption::new("claude-haiku", "Claude Haiku", "Anthropic").context_window(200_000),
        ModelOption::new("gpt-large", "GPT Large", "OpenAI").context_window(128_000),
    ];
    let citations = [
        CitationSource::new("Parental leave policy 2025")
            .href("https://hr.example.com/policies/parental-leave")
            .description("Sixteen weeks of fully paid leave for every new parent.")
            .target("reply-source-1"),
        CitationSource::new("Leave FAQ")
            .site_name("HR help centre")
            .description("Leave can be split into up to three blocks.")
            .target("reply-source-2"),
        CitationSource::new("Requesting parental leave.pdf")
            .description("Steps and deadlines for a request.")
            .target("reply-source-3"),
    ];
    let strings = ResponseStrings::default();
    let mut stream = ResponseStream::new();
    let mut stages = Vec::new();
    let chars: Vec<usize> = ANSWER.char_indices().map(|(at, _)| at).collect();
    let mut cut = 0;
    while cut < chars.len() {
        cut = (cut + 18).min(chars.len());
        let end = chars.get(cut).copied().unwrap_or(ANSWER.len());
        let done = cut == chars.len();
        let blocks = response_blocks(
            &ANSWER[..end],
            "reply",
            !done,
            citations.len(),
            2,
            None,
            false,
            ResponseImages::Click,
            &strings,
        );
        let update = stream.update(&blocks);
        stages.push(Stage {
            changed: update
                .changed
                .iter()
                .map(|&index| (index, blocks[index].clone()))
                .collect(),
            len: update.len,
        });
    }
    Ok(view! {
        page_header(slug: "chat-example")
        <script type="module" src=(asset!("../../client/chat-example.js"))></script>
        <div
            data-chat-example=""
            style="display: grid; grid-template-rows: auto minmax(0, 1fr) auto; inline-size: 100%; block-size: 40rem; overflow: hidden; border-radius: var(--radius-5); background-color: var(--color-bg); box-shadow: var(--shadow-2)"
        >
            <header
                style="display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); block-size: 3.25rem; padding: 0 var(--space-3); border-block-end: 1px solid var(--color-border); background-color: var(--color-surface)"
            >
                model_selector(
                    id: "chat-model",
                    name: "model",
                    label: "Model",
                    models: &models,
                    value: "claude-sonnet",
                    size: ModelSelectorSize::Sm
                )
                <a
                    href="/c/chat-example"
                    aria-label="New chat"
                    class=(BUTTON)
                    data-variant="ghost"
                    data-size="icon"
                >
                    icon(name: Icon::SquarePen)
                </a>
            </header>
            <div style="display: flex; min-block-size: 0">
                conversation(
                    label: "Chat with the handbook assistant",
                    stick_to_bottom: false,
                    viewport_attrs: attributes! { id="chat-log" },
                    conversation_empty_state(
                        title: "Ask the handbook",
                        description: "Answers come from your company's policies, with sources you can check.",
                        icon_name: Icon::BookOpenText,
                        title_as: EmptyTitle::H2,
                        attrs: attributes! { id="chat-empty" },
                        suggestions(
                            suggestion(
                                suggestion: "What is our parental leave policy?",
                                target: "chat-message",
                                send: true
                            )
                            suggestion(
                                suggestion: "How do I request parental leave?",
                                target: "chat-message",
                                send: true
                            )
                            suggestion(
                                suggestion: "Can I split my leave into blocks?",
                                target: "chat-message",
                                send: true
                            )
                        )
                    )
                    conversation_content(
                        attrs: attributes! { id="chat-turns" hidden="" }
                    )
                    conversation_scroll_button()
                )
            </div>
            <div
                style="display: flex; flex-direction: column; gap: var(--space-2); inline-size: 100%; max-inline-size: 48rem; margin-inline: auto; padding: 0 var(--space-4) var(--space-3)"
            >
                prompt_input(
                    attachments: true,
                    accept: "image/*,.pdf,.md,.txt,.docx",
                    attrs: attributes! { id="chat-composer" },
                    prompt_input_attachments()
                    prompt_input_textarea(
                        placeholder: "Ask about a policy…",
                        max_length: 4000,
                        attrs: attributes! { id="chat-message" }
                    )
                    prompt_input_toolbar(
                        prompt_input_tools(
                            prompt_input_attach_button()
                            prompt_input_button(
                                label: "Search the web",
                                show_label: true,
                                pressed: false,
                                name: "web",
                                icon(name: Icon::Globe)
                            )
                        )
                        prompt_input_submit(stop_while_submitted: true)
                    )
                )
                <p
                    style="color: var(--color-text-muted); font-size: var(--font-size-xs); text-align: center"
                >
                    "Simulated responses: nothing leaves your browser."
                </p>
            </div>
            conversation_announcer(
                status: ChatStatus::Ready,
                attrs: attributes! { id="chat-announcer" }
            )
        </div>

        // What the script copies into the log: a user's turn, and the canned reply.
        <template data-chat-user="">
            message(
                from: MessageRole::User,
                <div
                    data-user-files=""
                    style="display: flex; flex-wrap: wrap; justify-content: flex-end; gap: var(--space-1)"
                ></div>
                message_content(<span data-user-text=""></span>)
            )
        </template>
        <template data-chat-file="">
            <span
                style="display: inline-flex; align-items: center; gap: var(--space-1); block-size: 1.75rem; padding: 0 var(--space-2); border-radius: var(--radius-2); background-color: var(--color-surface); box-shadow: var(--shadow-ring); color: var(--color-text-muted); font-size: var(--font-size-xs)"
            >
                icon(name: Icon::FileText)
                <span data-file-name=""></span>
            </span>
        </template>
        <template data-chat-reply="">
            message(
                from: MessageRole::Assistant,
                message_content(
                    attrs: attributes! {
                        style="display: flex; flex-direction: column; gap: var(--space-4)"
                    },
                    reasoning(
                        streaming: true,
                        attrs: attributes! { data-reply-reasoning="" },
                        reasoning_trigger(streaming: true)
                        reasoning_content(
                            response(id: "reply-thinking", source: REASONING)
                        )
                    )
                    response(
                        id: "reply",
                        source: "",
                        streaming: true,
                        citations: &citations
                    )
                    <p
                        data-reply-stopped=""
                        hidden=""
                        style="color: var(--color-text-muted); font-size: var(--font-size-sm); font-style: italic"
                    >
                        "Response stopped."
                    </p>
                    <div data-reply-sources="" hidden="">
                        sources(
                            sources_trigger(count: 3)
                            sources_content(
                                source(
                                    index: 1,
                                    id: "reply-source-1",
                                    focusable: true,
                                    title: "Parental leave policy 2025",
                                    href: "https://hr.example.com/policies/parental-leave",
                                    description: "Sixteen weeks of fully paid leave for every new parent."
                                )
                                source(
                                    index: 2,
                                    id: "reply-source-2",
                                    focusable: true,
                                    title: "Leave FAQ",
                                    meta: "HR help centre"
                                )
                                source(
                                    index: 3,
                                    id: "reply-source-3",
                                    focusable: true,
                                    title: "Requesting parental leave.pdf",
                                    meta: "p. 2",
                                    icon_name: Icon::File
                                )
                            )
                        )
                    </div>
                )
                message_actions(
                    attrs: attributes! { data-reply-actions="" hidden="" },
                    message_copy_action(value: ANSWER)
                    message_action(
                        label: "Regenerate",
                        attrs: attributes! { data-reply-regenerate="" },
                        icon(name: Icon::RefreshCw)
                    )
                    message_action(
                        label: "Good response",
                        pressed: false,
                        exclusive: true,
                        icon(name: Icon::ThumbsUp)
                    )
                    message_action(
                        label: "Bad response",
                        pressed: false,
                        exclusive: true,
                        icon(name: Icon::ThumbsDown)
                    )
                )
            )
        </template>
        for (n, stage) in stages.iter().enumerate() {
            <template data-chat-stage=(n) data-len=(stage.len)>
                for (index, block) in &stage.changed {
                    <template data-index=(*index)>
                        response_block(block: block, citations: &citations)
                    </template>
                }
            </template>
        }
        <p
            style="margin-block-start: var(--space-4); color: var(--color-text-muted); font-size: var(--font-size-sm)"
        >
            "Start with a suggestion or type a question. Press Stop at any point."
        </p>
    })
}
