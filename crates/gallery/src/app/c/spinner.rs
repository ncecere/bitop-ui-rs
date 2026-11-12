use bitop_ui::components::spinner::{SpinnerSize, loading, spinner};
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
        page_header(slug: "spinner")
        example(
            title: "Sizes",
            spinner(size: SpinnerSize::Sm)
            spinner()
            spinner(size: SpinnerSize::Lg, label: "Saving")
        )
        example(title: "Loading block", <div style="inline-size: 100%">loading()</div>)
    })
}
