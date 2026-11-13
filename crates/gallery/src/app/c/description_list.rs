use bitop_ui::components::{
    badge::{BadgeVariant, badge},
    description_list::{
        DescriptionLayout, DescriptionSize, description_item, description_list, facts_line,
        facts_line_item,
    },
    icon::Icon,
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
        page_header(slug: "description-list")
        example(
            title: "Grid",
            description_list(
                attrs: attributes! { aria-label="Source" style="inline-size: 100%" },
                description_item(
                    label: "Start URL",
                    <a href="#docs">"https://docs.example.com"</a>
                )
                description_item(label: "Schedule", "Weekly, next on 3 October")
                description_item(
                    label: "Status",
                    badge(variant: BadgeVariant::Success, "Ready")
                )
                description_item(label: "Owner", blank: true)
            )
        )
        example(
            title: "Grid, small, with dividers",
            description_list(
                size: DescriptionSize::Sm,
                dividers: true,
                attrs: attributes! { aria-label="Plan" style="inline-size: 100%" },
                description_item(label: "Plan", "Team")
                description_item(label: "Seats", "12 of 20")
                description_item(label: "Renews", "1 January 2027")
            )
        )
        example(
            title: "Grid in a narrow container",
            description_list(
                attrs: attributes! { aria-label="Narrow" style="inline-size: 16rem" },
                description_item(label: "Region", "Frankfurt")
                description_item(label: "Runtime", "Rust 1.99")
            )
        )
        example(
            title: "Stacked, three columns",
            description_list(
                layout: DescriptionLayout::Stacked,
                columns: 3,
                attrs: attributes! { aria-label="Deployment" style="inline-size: 100%" },
                description_item(label: "Environment", "Production")
                description_item(label: "Branch", "main")
                description_item(label: "Commit", "671a1f5")
                description_item(label: "Duration", "1 min 12 s")
                description_item(
                    label: "Reviewer",
                    blank: true,
                    empty_text: "none",
                    empty_label: "No reviewer"
                )
            )
        )
        example(
            title: "Facts line",
            <div style="display: grid; gap: var(--space-4); inline-size: 100%">
                facts_line(
                    attrs: attributes! { aria-label="Knowledge base" },
                    facts_line_item(icon: Icon::Search, "Website")
                    facts_line_item(label: "Embedding profile", "Nomic 768")
                    facts_line_item("58 documents")
                    facts_line_item(<a href="#sync">"synced 2 h ago"</a>)
                )
                facts_line(
                    show_labels: true,
                    size: DescriptionSize::Sm,
                    attrs: attributes! { aria-label="Labelled facts" },
                    facts_line_item(label: "Profile", "Nomic 768")
                    facts_line_item(label: "Chunks", "1,204")
                )
                <div style="inline-size: 14rem">
                    facts_line(
                        attrs: attributes! { aria-label="Wrapped facts" },
                        facts_line_item("Website")
                        facts_line_item("Nomic 768")
                        facts_line_item("58 documents")
                        facts_line_item("synced 2 h ago")
                    )
                </div>
            </div>
        )
    })
}
