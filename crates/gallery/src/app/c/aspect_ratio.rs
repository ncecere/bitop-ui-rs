use bitop_ui::components::aspect_ratio::aspect_ratio;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

/// A 1x1 PNG, so the example needs no network request.
const PIXEL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "aspect-ratio")
        example(
            title: "16 by 9, framed",
            <div style="inline-size: 100%; max-inline-size: 20rem">
                aspect_ratio(
                    ratio: 16.0 / 9.0,
                    framed: true,
                    <img src=(PIXEL) alt="A blue square">
                )
            </div>
        )
        example(
            title: "Square",
            <div style="inline-size: 6rem">aspect_ratio(framed: true)</div>
        )
    })
}
