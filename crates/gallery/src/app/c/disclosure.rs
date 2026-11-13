use bitop_ui::components::{
    disclosure::disclosure,
    field::{field, field_label},
    input::input,
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
        page_header(slug: "disclosure")
        example(
            title: "Closed, with a summary of the values inside",
            disclosure(
                title: "Advanced options",
                summary: "Depth 2 · 200 pages",
                attrs: attributes! { data-demo="advanced" style="inline-size: 100%" },
                field(
                    field_label(target: "disclosure-depth", "Depth")
                    input(
                        attrs: attributes! { id="disclosure-depth" type="number" value="2" }
                    )
                )
                field(
                    field_label(target: "disclosure-pages", "Pages")
                    input(
                        attrs: attributes! { id="disclosure-pages" type="number" value="200" }
                    )
                )
            )
        )
        example(
            title: "Open from the start",
            disclosure(
                title: "Request headers",
                open: true,
                attrs: attributes! { data-demo="open" style="inline-size: 100%" },
                <p>"Accept: text/html"</p>
            )
        )
        example(
            title: "A long summary is cut",
            disclosure(
                title: "Filters",
                summary: "Status is live or paused · Region is Frankfurt · Owner is anyone · Created in the last ninety days",
                attrs: attributes! {
                    data-demo="long"
                    style="inline-size: 22rem; max-inline-size: 100%"
                },
                <p>"Four filters apply."</p>
            )
        )
        example(
            title: "Sharing a name: one open at a time",
            <div style="display: grid; gap: var(--space-2); inline-size: 100%">
                disclosure(
                    title: "Shipping",
                    name: "checkout",
                    open: true,
                    <p>"Arrives in three days."</p>
                )
                disclosure(
                    title: "Payment",
                    name: "checkout",
                    <p>"Card ending in 4242."</p>
                )
            </div>
        )
    })
}
