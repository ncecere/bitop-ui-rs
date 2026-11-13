use bitop_ui::components::{
    avatar::AvatarShape,
    icon::{Icon, icon},
};
use bitop_ui_ai::components::message::{
    MessageActionsVisibility, MessageContentVariant, MessageRole, message, message_action,
    message_actions, message_avatar, message_branch, message_branch_content,
    message_branch_selector, message_content, message_copy_action,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_string};

const ANSWER: &str = "Eligible employees get sixteen weeks of fully paid parental leave.";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let query = query_string(cx);
    Ok(view! {
        page_header(slug: "message")
        example(
            title: "A user's bubble and an assistant's answer",
            <div
                data-example="turns"
                style="display: grid; gap: var(--space-6); inline-size: 100%"
            >
                message(
                    from: MessageRole::User,
                    message_content(
                        "What is our parental leave policy?\nAnd can I split it?"
                    )
                )
                message(
                    from: MessageRole::Assistant,
                    message_content(<p>(ANSWER)</p>)
                    message_actions(
                        message_copy_action(value: ANSWER, tooltip: "copy-tip")
                        message_action(
                            label: "Regenerate",
                            tooltip: "regenerate-tip",
                            icon(name: Icon::RefreshCw)
                        )
                        message_action(
                            label: "Good response",
                            tooltip: "good-tip",
                            pressed: false,
                            exclusive: true,
                            icon(name: Icon::ThumbsUp)
                        )
                        message_action(
                            label: "Bad response",
                            tooltip: "bad-tip",
                            pressed: false,
                            exclusive: true,
                            icon(name: Icon::ThumbsDown)
                        )
                    )
                )
            </div>
        )
        example(
            title: "Avatars, and actions shown on hover",
            <div
                data-example="avatars"
                style="display: grid; gap: var(--space-6); inline-size: 100%"
            >
                message(
                    from: MessageRole::User,
                    message_avatar(name: "Ada Lovelace")
                    message_content("Summarise the travel policy.")
                )
                message(
                    from: MessageRole::Assistant,
                    label: "Handbook assistant said",
                    message_avatar(
                        name: "Handbook",
                        color: "var(--color-primary)",
                        shape: AvatarShape::Square
                    )
                    message_content(
                        <p>
                            "Book through the travel portal, economy for flights under six hours."
                        </p>
                    )
                    message_actions(
                        visibility: MessageActionsVisibility::Hover,
                        message_copy_action(value: "Book through the travel portal.")
                    )
                )
                message(
                    from: MessageRole::Assistant,
                    message_content(
                        variant: MessageContentVariant::Bubble,
                        "An assistant's message drawn as a bubble."
                    )
                )
            </div>
        )
        example(
            title: "Feedback that is sent with a form",
            <form action="/c/message" data-example="feedback">
                message(
                    from: MessageRole::Assistant,
                    message_content(<p>"Was this helpful?"</p>)
                    message_actions(
                        label: "Feedback",
                        message_action(
                            label: "Helpful",
                            pressed: true,
                            exclusive: true,
                            name: "vote",
                            value: "up",
                            icon(name: Icon::ThumbsUp)
                        )
                        message_action(
                            label: "Not helpful",
                            pressed: false,
                            exclusive: true,
                            name: "vote",
                            value: "down",
                            icon(name: Icon::ThumbsDown)
                        )
                        message_action(
                            label: "Regenerate",
                            disabled: true,
                            icon(name: Icon::RefreshCw)
                        )
                    )
                )
                <output data-query="">(query)</output>
            </form>
        )
        example(
            title: "Versions of a response",
            <div data-example="branches" style="inline-size: 100%">
                message_branch(
                    index: 1,
                    attrs: attributes! { id="branch" },
                    message_branch_content(
                        message(
                            from: MessageRole::Assistant,
                            message_content(<p>"First version: sixteen weeks."</p>)
                        )
                        message(
                            from: MessageRole::Assistant,
                            message_content(
                                <p>"Second version: sixteen weeks, fully paid."</p>
                            )
                        )
                        message(
                            from: MessageRole::Assistant,
                            message_content(
                                <p>
                                    "Third version: sixteen weeks, fully paid, in up to three blocks."
                                </p>
                            )
                        )
                    )
                    message_actions(message_branch_selector(index: 1, count: 3))
                )
            </div>
        )
        example(
            title: "One version, and an index past the end",
            <div
                data-example="clamp"
                style="display: grid; gap: var(--space-4); inline-size: 100%"
            >
                message_branch(
                    message_branch_content(
                        message(
                            from: MessageRole::Assistant,
                            message_content(<p>"Only version."</p>)
                        )
                    )
                    message_actions(message_branch_selector(index: 0, count: 1))
                )
                message_branch(
                    index: 4,
                    attrs: attributes! { id="clamped-branch" },
                    message_branch_content(
                        message(
                            from: MessageRole::Assistant,
                            message_content(<p>"Version A."</p>)
                        )
                        message(
                            from: MessageRole::Assistant,
                            message_content(<p>"Version B."</p>)
                        )
                    )
                    message_actions(message_branch_selector(index: 4, count: 2))
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="display: grid; gap: var(--space-4); inline-size: 100%"
            >
                message(
                    from: MessageRole::User,
                    user_label: "Sie sagten",
                    message_content("Hallo")
                )
                message_branch(
                    message_branch_content(
                        message(
                            from: MessageRole::Assistant,
                            assistant_label: "Der Assistent sagte",
                            message_content(<p>"Antwort eins."</p>)
                        )
                        message(
                            from: MessageRole::Assistant,
                            assistant_label: "Der Assistent sagte",
                            message_content(<p>"Antwort zwei."</p>)
                        )
                    )
                    message_actions(
                        label: "Aktionen",
                        message_copy_action(
                            value: "Antwort eins.",
                            label: "Nachricht",
                            copy_label: "{label} kopieren",
                            copied_label: "{label} kopiert",
                            copied_status: "{label} kopiert"
                        )
                        message_branch_selector(
                            index: 0,
                            count: 2,
                            label: "Antwortversionen",
                            previous_label: "Vorherige Antwort",
                            next_label: "Nächste Antwort",
                            status_label: "Antwort {index} von {count}"
                        )
                    )
                )
            </div>
        )
    })
}
