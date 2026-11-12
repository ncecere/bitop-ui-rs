use bitop_ui::components::{icon::Icon, input::textarea};
use bitop_ui_ai::components::suggestion::{
    SuggestionsAlign, SuggestionsLayout, suggestion, suggestions,
};
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
        page_header(slug: "suggestion")
        example(
            title: "Wrapping, centred",
            <div data-example="wrap" style="inline-size: 100%">
                suggestions(
                    suggestion(suggestion: "What is our parental leave policy?")
                    suggestion(suggestion: "How do I request time off?")
                    suggestion(suggestion: "Summarise the travel policy")
                    suggestion(
                        suggestion: "Draft a polite reply to a customer who asked for a refund after the thirty-day window had closed, offering store credit instead",
                        text: "Draft a polite reply to a customer who asked for a refund after the thirty-day window had closed, offering store credit instead"
                    )
                )
            </div>
        )
        example(
            title: "From the start, with icons",
            <div data-example="start" style="inline-size: 100%">
                suggestions(
                    align: SuggestionsAlign::Start,
                    label: "Ideas",
                    suggestion(
                        suggestion: "Search the handbook",
                        icon_name: Icon::Search
                    )
                    suggestion(
                        suggestion: "Explain the expense report rules for contractors",
                        text: "Expense rules",
                        icon_name: Icon::Info
                    )
                    suggestion(suggestion: "Not available yet", disabled: true)
                )
            </div>
        )
        example(
            title: "One row that scrolls",
            <div data-example="scroll" style="inline-size: 18rem">
                suggestions(
                    layout: SuggestionsLayout::Scroll,
                    suggestion(suggestion: "Plan a team offsite")
                    suggestion(suggestion: "Write a status update")
                    suggestion(suggestion: "Compare two vendors")
                    suggestion(suggestion: "Prepare interview questions")
                )
            </div>
        )
        example(
            title: "Fill a text box, or fill it and send",
            <form
                id="fill-form"
                action="/c/suggestion"
                data-example="fill"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                textarea(
                    attrs: attributes! { id="fill-box" name="prompt" aria-label="Prompt" rows="2" }
                )
                suggestions(
                    align: SuggestionsAlign::Start,
                    suggestion(
                        suggestion: "Translate this into French",
                        target: "fill-box"
                    )
                    suggestion(
                        suggestion: "Make it shorter",
                        target: "fill-box",
                        send: true
                    )
                )
            </form>
        )
        example(
            title: "A submit button, without a script",
            <form
                action="/c/suggestion"
                data-example="submit"
                style="inline-size: 100%"
            >
                suggestions(
                    align: SuggestionsAlign::Start,
                    suggestion(suggestion: "Tell me a joke", name: "message")
                    suggestion(suggestion: "Write a haiku", name: "message")
                )
            </form>
        )
    })
}
