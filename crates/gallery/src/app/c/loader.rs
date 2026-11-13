use bitop_ui::components::loader::{LoaderSize, loader};
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
        page_header(slug: "loader")
        example(
            title: "Sizes",
            loader(size: LoaderSize::Sm, label: "Loading, small")
            loader(label: "Loading")
            loader(size: LoaderSize::Lg, label: "Loading, large")
        )
        example(
            title: "With a visible label",
            loader(label: "Thinking", show_label: true)
        )
        example(
            title: "Decorative",
            <p
                role="status"
                style="display: flex; align-items: center; gap: var(--space-2)"
            >
                loader()
                "Waiting for the first token"
            </p>
        )
    })
}
