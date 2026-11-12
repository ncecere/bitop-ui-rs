use bitop_ui::components::badge::badge;
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
        page_header(slug: "sidebar")
        example(
            title: "This page's shell",
            <p>
                "The gallery is built on the sidebar. Use the button in the header to collapse it; "
                "narrow the window to turn it into a drawer."
            </p>
            badge("sidebar_layout")
            badge("sidebar")
            badge("sidebar_toggle")
        )
    })
}
