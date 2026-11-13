use bitop_ui::components::{
    button::{ButtonSize, button},
    icon::Icon,
    query_state::{QueryStatus, query_state},
    skeleton::skeleton_text,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    // `?state=` picks what the first example shows, as a real page would from its data.
    let live = match query_param(cx, "state").as_deref() {
        Some("ready") => QueryStatus::Ready,
        Some("empty") => QueryStatus::Empty,
        Some("loading") => QueryStatus::Loading,
        _ => QueryStatus::Error,
    };
    Ok(view! {
        page_header(slug: "query-state")
        example(
            title: "A state the server chooses",
            <div data-example="live" style="inline-size: 100%">
                query_state(
                    state: live,
                    error_status: 503,
                    retry_href: "/c/query-state?state=ready",
                    empty_title: "No projects yet",
                    <ul data-content="">
                        <li>"Apollo"</li>
                        <li>"Gemini"</li>
                    </ul>
                )
            </div>
        )
        example(
            title: "Loading",
            <div data-example="loading" style="inline-size: 100%">
                query_state(state: QueryStatus::Loading, <p>"Content"</p>)
            </div>
        )
        example(
            title: "Loading, with skeleton rows",
            <div data-example="skeleton" style="inline-size: 100%">
                query_state(
                    state: QueryStatus::Loading,
                    loading_label: "Loading projects…",
                    loading: view! { skeleton_text(lines: 3) },
                    <p>"Content"</p>
                )
            </div>
        )
        example(
            title: "Error, keeping the last good content",
            <div data-example="stale" style="inline-size: 100%">
                query_state(
                    state: QueryStatus::Error,
                    error_message: "Could not refresh the list.",
                    keep_content: true,
                    attrs: attributes! { style="--query-state-padding: 0 0 var(--space-3)" },
                    <p data-content="">"Apollo, Gemini (as of 5 minutes ago)"</p>
                )
            </div>
        )
        example(
            title: "Empty",
            <div data-example="empty" style="inline-size: 100%">
                query_state(
                    state: QueryStatus::Empty,
                    empty_title: "No projects yet",
                    empty_description: "Create one to see it here.",
                    empty_icon: Icon::Inbox,
                    empty_action: view! { button(size: ButtonSize::Sm, "New project") },
                    <p>"Content"</p>
                )
            </div>
        )
        example(
            title: "Empty, with its own content",
            <div data-example="custom-empty" style="inline-size: 100%">
                query_state(
                    state: QueryStatus::Empty,
                    empty: view! { <p>"No results match the filters."</p> },
                    <p>"Content"</p>
                )
            </div>
        )
        example(
            title: "Ready",
            <div data-example="ready" style="inline-size: 100%">
                query_state(state: QueryStatus::Ready, <p>"Three projects."</p>)
            </div>
        )
    })
}
