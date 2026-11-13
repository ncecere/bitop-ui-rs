use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    icon::Icon,
};
use bitop_ui_ai::{
    ChatStatus,
    components::{
        conversation::{
            EmptyTitle, conversation, conversation_announcer, conversation_content,
            conversation_empty_state, conversation_scroll_button,
        },
        message::{MessageRole, message, message_avatar, message_content},
        suggestion::{suggestion, suggestions},
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const TURNS: [(&str, &str); 4] = [
    (
        "What is our parental leave policy?",
        "Eligible employees get sixteen weeks of fully paid leave, to be taken within twelve months of the child's arrival.",
    ),
    (
        "Can I split it?",
        "Yes: into up to three blocks, each of at least two weeks.",
    ),
    (
        "How much notice do I give?",
        "Submit the request in the HR portal at least eight weeks before you start.",
    ),
    (
        "Who approves it?",
        "Your manager first, then HR, usually within five working days.",
    ),
];

const REPLY: &str = "Here is a longer answer that arrives a word at a time, as a model's reply does, so that you can watch the log follow it to the bottom, or scroll up while it streams and see that it stays where you put it. The button at the bottom brings you back to the latest message.";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "conversation")
        example(
            title: "A log that sticks to the bottom",
            <div
                data-example="log"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                <div
                    style="display: flex; block-size: 22rem; border: 1px solid var(--color-border); border-radius: var(--radius-card)"
                >
                    conversation(
                        label: "Chat with the handbook assistant",
                        viewport_attrs: attributes! { id="log" },
                        conversation_content(
                            attrs: attributes! { id="log-content" },
                            for (question, answer) in TURNS {
                                message(
                                    from: MessageRole::User,
                                    message_content((question))
                                )
                                message(
                                    from: MessageRole::Assistant,
                                    message_content(<p>(answer)</p>)
                                )
                            }
                            message(
                                from: MessageRole::Assistant,
                                message_content(<p id="streamed"></p>)
                            )
                        )
                        conversation_scroll_button(tooltip: "scroll-tip")
                    )
                </div>
                conversation_announcer(attrs: attributes! { id="announcer" })
                <div style="display: flex; flex-wrap: wrap; gap: var(--space-2)">
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Secondary,
                        attrs: attributes! {
                            data-gallery-stream="streamed"
                            data-gallery-announcer="announcer"
                            data-gallery-words=(REPLY)
                        },
                        "Stream a reply"
                    )
                </div>
            </div>
        )
        example(
            title: "Empty, with suggestions",
            <div
                data-example="empty"
                style="display: flex; block-size: 22rem; inline-size: 100%; border: 1px solid var(--color-border); border-radius: var(--radius-card)"
            >
                conversation(
                    stick_to_bottom: false,
                    conversation_empty_state(
                        title: "Ask the handbook",
                        description: "Answers come from your company's policies, with sources you can check.",
                        icon_name: Icon::BookOpenText,
                        title_as: EmptyTitle::H3,
                        suggestions(
                            suggestion(suggestion: "What is our parental leave policy?")
                            suggestion(suggestion: "How do I request time off?")
                        )
                    )
                )
            </div>
        )
        example(
            title: "An empty state with an avatar, and the default title",
            <div
                data-example="media"
                style="display: flex; block-size: 16rem; inline-size: 100%; border: 1px solid var(--color-border); border-radius: var(--radius-card)"
            >
                conversation(
                    stick_to_bottom: false,
                    conversation_empty_state(
                        icon_name: Icon::BookOpenText,
                        media: view! { message_avatar(name: "Handbook") }
                    )
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
                <div
                    style="display: flex; block-size: 10rem; border: 1px solid var(--color-border); border-radius: var(--radius-card)"
                >
                    conversation(
                        label: "Unterhaltung",
                        conversation_content(
                            for (question, answer) in TURNS {
                                message(
                                    from: MessageRole::User,
                                    user_label: "Sie sagten",
                                    message_content((question))
                                )
                                message(
                                    from: MessageRole::Assistant,
                                    assistant_label: "Der Assistent sagte",
                                    message_content(<p>(answer)</p>)
                                )
                            }
                        )
                        conversation_scroll_button(label: "Zur neuesten Nachricht")
                    )
                </div>
                conversation_announcer(
                    status: ChatStatus::Streaming,
                    submitted: "Nachricht gesendet",
                    streaming: "Der Assistent antwortet",
                    complete: "Antwort fertig",
                    error: "Die Antwort ist fehlgeschlagen",
                    attrs: attributes! { id="german-announcer" }
                )
            </div>
        )
    })
}
