use bitop_ui_ai::components::open_in_chat::{ChatProvider, open_in_chat, open_in_chat_item};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

const PROMPT: &str = "Explain this stack trace: TypeError: cannot read properties of undefined (reading 'map') at render (list.tsx:12)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let every = [
        ChatProvider::CHATGPT,
        ChatProvider::CLAUDE,
        ChatProvider::V0,
        ChatProvider::CURSOR,
        ChatProvider::T3,
        ChatProvider::SCIRA,
    ];
    let campus = [
        ChatProvider::CLAUDE,
        ChatProvider::custom(
            "campus",
            "Open in Campus AI",
            "https://ai.example.edu/new?q={query}",
        ),
    ];
    let german = [
        ChatProvider::CHATGPT.titled("In ChatGPT öffnen"),
        ChatProvider::CLAUDE.titled("In Claude öffnen"),
    ];
    Ok(view! {
        page_header(slug: "open-in-chat")
        example(
            title: "The default providers",
            <div data-example="default">
                open_in_chat(id: "open-default", query: PROMPT)
            </div>
        )
        example(
            title: "Every built-in provider, under a heading",
            <div data-example="every">
                open_in_chat(
                    id: "open-every",
                    query: PROMPT,
                    providers: &every,
                    group_label: "Open this prompt in"
                )
            </div>
        )
        example(
            title: "A provider of your own, and an extra item",
            <div data-example="custom">
                open_in_chat(
                    id: "open-custom",
                    query: "Summarise chapter 3",
                    providers: &campus,
                    open_in_chat_item(
                        provider: ChatProvider::T3,
                        query: "Summarise chapter 3 in French"
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de">
                open_in_chat(
                    id: "open-german",
                    query: "Erkläre diesen Fehler",
                    providers: &german,
                    label: "Im Chat öffnen",
                    new_tab_label: "(öffnet in einem neuen Tab)"
                )
            </div>
        )
    })
}
