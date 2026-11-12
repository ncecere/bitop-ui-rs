use bitop_ui::components::{
    icon::{Icon, icon},
    marker::{MarkerVariant, marker, marker_content, marker_icon},
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
        page_header(slug: "marker")
        example(
            title: "Variants",
            <div style="display: grid; gap: var(--space-4); inline-size: 100%">
                marker(
                    marker_icon(icon(name: Icon::Search))
                    marker_content("Explored 4 files")
                )
                marker(
                    variant: MarkerVariant::Border,
                    marker_icon(icon(name: Icon::Check))
                    marker_content("Ran the checks")
                )
                marker(variant: MarkerVariant::Separator, marker_content("Today"))
            </div>
        )
        example(
            title: "A link, and a status",
            <div style="display: grid; gap: var(--space-4); inline-size: 100%">
                marker(
                    href: "#run-42",
                    marker_icon(icon(name: Icon::ArrowUpRight))
                    marker_content("Open run 42")
                )
                marker(
                    attrs: attributes! { role="status" },
                    marker_icon(icon(name: Icon::Info))
                    marker_content(
                        "Compacting the conversation. "
                        <a href="#details">"Details"</a>
                    )
                )
            </div>
        )
    })
}
