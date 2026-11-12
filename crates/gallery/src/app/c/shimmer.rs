use bitop_ui::components::shimmer::shimmer;
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
        page_header(slug: "shimmer")
        example(
            title: "Active",
            shimmer(attrs: attributes! { id="active" }, "Thinking…")
        )
        example(
            title: "A slower sweep, in a sentence",
            <p>
                "The assistant is "
                shimmer(
                    duration: 4.0,
                    attrs: attributes! { id="slow" style="font-weight: var(--font-weight-medium)" },
                    "searching 58 documents"
                )
                "."
            </p>
        )
        example(
            title: "Not active",
            shimmer(
                active: false,
                attrs: attributes! { id="idle" },
                "Thought for 4 seconds"
            )
        )
    })
}
