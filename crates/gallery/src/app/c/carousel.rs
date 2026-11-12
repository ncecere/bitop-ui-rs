use bitop_ui::components::{
    button::button,
    carousel::{
        CarouselOrientation, carousel, carousel_content, carousel_indicators, carousel_item,
        carousel_next, carousel_previous,
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

const SLIDE: &str = "display: grid; place-items: center; block-size: 10rem; border: 1px solid var(--color-border); border-radius: var(--radius-card); background: var(--color-surface-sunken); font-size: var(--font-size-2xl); font-weight: var(--font-weight-semibold)";
const CARD: &str = "display: grid; place-items: center; gap: var(--space-2); block-size: 8rem; border: 1px solid var(--color-border); border-radius: var(--radius-card); background: var(--color-surface-sunken)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let basic = element_id(cx);
    let several = element_id(cx);
    let sized = element_id(cx);
    let vertical = element_id(cx);

    Ok(view! {
        page_header(slug: "carousel")
        example(
            title: "One slide at a time, with buttons and dots",
            carousel(
                label: "Featured templates",
                attrs: attributes! { data-demo="basic" },
                carousel_content(
                    id: &basic,
                    for number in 1..=5 {
                        carousel_item(<div style=(SLIDE)>(number)</div>)
                    }
                )
                carousel_previous(target: &basic)
                carousel_next(target: &basic)
                carousel_indicators(target: &basic, count: 5)
            )
        )
        example(
            title: "Three at a time, starting at the third, with controls inside the slides",
            carousel(
                label: "Plans",
                default_index: 2,
                attrs: attributes! { data-demo="several" },
                carousel_content(
                    id: &several,
                    slides_per_view: 3,
                    for name in [
                        "Free",
                        "Team",
                        "Business",
                        "Scale",
                        "Enterprise",
                        "Custom",
                    ] {
                        carousel_item(<div style=(CARD)>(name)</div>)
                    }
                )
                carousel_previous(target: &several)
                carousel_next(target: &several)
            )
        )
        example(
            title: "Slides with a size of their own, and a field inside one",
            carousel(
                label: "Recent projects",
                slide_label: "Project {index} of {count}",
                announcement: "Project {index} of {count}",
                attrs: attributes! { data-demo="sized" },
                carousel_content(
                    id: &sized,
                    carousel_item(
                        basis: "14rem",
                        <div style=(CARD)>
                            <label>
                                "Rename "
                                <input name="project" value="Web" size="8">
                            </label>
                        </div>
                    )
                    carousel_item(
                        basis: "14rem",
                        <div style=(CARD)>button("Open API")</div>
                    )
                    carousel_item(basis: "14rem", <div style=(CARD)>"Docs"</div>)
                    carousel_item(
                        basis: "14rem",
                        label: "The last project",
                        <div style=(CARD)>"Design"</div>
                    )
                )
                carousel_previous(target: &sized, label: "Previous project")
                carousel_next(target: &sized, label: "Next project")
            )
        )
        example(
            title: "Vertical",
            carousel(
                label: "Announcements",
                orientation: CarouselOrientation::Vertical,
                attrs: attributes! { data-demo="vertical" style="block-size: 12rem" },
                carousel_content(
                    id: &vertical,
                    for text in [
                        "Version 2 is out",
                        "New region: Frankfurt",
                        "Maintenance on Sunday",
                    ] {
                        carousel_item(
                            <div
                                style="display: grid; place-items: center; block-size: 12rem; border: 1px solid var(--color-border); border-radius: var(--radius-card); background: var(--color-surface-sunken)"
                            >
                                (text)
                            </div>
                        )
                    }
                )
                carousel_previous(target: &vertical)
                carousel_next(target: &vertical)
            )
        )
    })
}
