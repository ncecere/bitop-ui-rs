use bitop_ui::components::scroll_area::scroll_area;
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
        page_header(slug: "scroll-area")
        example(
            title: "Keyboard scrollable",
            scroll_area(
                label: "Release notes",
                attrs: attributes! { style="max-block-size: 8rem; inline-size: 100%" },
                for line in 1..=12 {
                    <p>
                        "Line "
                        (line)
                        " of the release notes."
                    </p>
                }
            )
        )
    })
}
