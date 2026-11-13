use bitop_ui::components::scroll_area::{ScrollOrientation, scroll_area};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const VERSIONS: [&str; 24] = [
    "v2.24.0", "v2.23.0", "v2.22.0", "v2.21.0", "v2.20.0", "v2.19.0", "v2.18.0", "v2.17.0",
    "v2.16.0", "v2.15.0", "v2.14.0", "v2.13.0", "v2.12.0", "v2.11.0", "v2.10.0", "v2.9.0",
    "v2.8.0", "v2.7.0", "v2.6.0", "v2.5.0", "v2.4.0", "v2.3.0", "v2.2.0", "v2.1.0",
];

const TAGS: [&str; 12] = [
    "design",
    "frontend",
    "backend",
    "infra",
    "docs",
    "billing",
    "security",
    "mobile",
    "analytics",
    "growth",
    "support",
    "research",
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "scroll-area")
        example(
            title: "Vertical, with max_height",
            scroll_area(
                label: "Release notes",
                max_height: "14rem",
                attrs: attributes! { style="inline-size: 100%" },
                <ul
                    style="margin: 0; padding: 0.75rem 1rem; list-style: none; display: grid; gap: 0.5rem"
                >
                    for version in VERSIONS {
                        <li>
                            <strong>(version)</strong>
                            " — bug fixes and performance improvements."
                        </li>
                    }
                </ul>
            )
        )
        example(
            title: "Horizontal",
            scroll_area(
                label: "Project tags",
                orientation: ScrollOrientation::Horizontal,
                attrs: attributes! { style="inline-size: 100%" },
                <ul
                    style="display: flex; gap: 0.5rem; margin: 0; padding: 0.5rem 0 0.75rem; list-style: none; inline-size: max-content"
                >
                    for tag in TAGS {
                        <li
                            style="padding: 0.25rem 0.75rem; border-radius: 999px; box-shadow: var(--shadow-control)"
                        >
                            (tag)
                        </li>
                    }
                </ul>
            )
        )
    })
}
