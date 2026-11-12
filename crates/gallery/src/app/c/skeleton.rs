use bitop_ui::components::skeleton::{SkeletonShape, skeleton, skeleton_text};
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
        page_header(slug: "skeleton")
        example(
            title: "Shapes",
            skeleton(shape: SkeletonShape::Circle, width: "2.5rem", height: "2.5rem")
            skeleton(width: "10rem", height: "2.5rem")
        )
        example(
            title: "Text",
            <div style="inline-size: 100%">skeleton_text(lines: 3)</div>
        )
    })
}
