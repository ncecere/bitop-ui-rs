use bitop_ui::components::text_link::text_link;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "text-link")
        example(
            title: "In text",
            <p>
                "Read the "
                text_link(href: "/", "getting started guide")
                " or the "
                text_link(href: "https://example.com", external: true, "API reference")
                ". "
                text_link(href: "/", muted: true, "Privacy")
            </p>
        )
    })
}
