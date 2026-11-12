use bitop_ui::components::separator::{SeparatorSpacing, separator};
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
        page_header(slug: "separator")
        example(
            title: "Horizontal",
            <div style="inline-size: 100%">
                <p>"Above"</p>
                separator()
                <p>"Below"</p>
            </div>
        )
        example(
            title: "With a label",
            <div style="inline-size: 100%">separator(label: "or")</div>
        )
        example(
            title: "Vertical",
            <div style="display: flex; align-items: center; block-size: 1.5rem">
                <span>"Docs"</span>
                separator(vertical: true, spacing: SeparatorSpacing::Sm)
                <span>"Support"</span>
            </div>
        )
    })
}
