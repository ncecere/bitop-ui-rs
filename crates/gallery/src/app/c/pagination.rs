use bitop_ui::components::{
    button::ButtonSize,
    pagination::{
        pagination, pagination_content, pagination_ellipsis, pagination_item, pagination_link,
        pagination_next, pagination_previous, paginator,
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::{page, request::uri},
    view::{View, view},
};

use crate::app::kit::{example, page_header};

/// The `page` parameter of the request, so that the first example really pages.
fn requested_page(cx: &Cx) -> u32 {
    uri(cx)
        .query()
        .unwrap_or_default()
        .split('&')
        .find_map(|pair| pair.strip_prefix("page="))
        .and_then(|value| value.parse().ok())
        .unwrap_or(5)
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let current = requested_page(cx);

    Ok(view! {
        page_header(slug: "pagination")
        example(
            title: "Twenty pages: follow a link and the server renders the next list",
            paginator(page: current, page_count: 20, href: "/c/pagination?page={page}")
        )
        example(
            title: "The first page, and the last",
            paginator(
                page: 1,
                page_count: 10,
                href: "/c/pagination?page={page}",
                label: "First page"
            )
            paginator(
                page: 10,
                page_count: 10,
                href: "/c/pagination?page={page}",
                label: "Last page"
            )
        )
        example(
            title: "Small, with two pages on each side and two at each end",
            paginator(
                page: 10,
                page_count: 20,
                href: "/c/pagination?page={page}",
                siblings: 2,
                boundaries: 2,
                size: ButtonSize::Sm,
                label: "Small pagination"
            )
        )
        example(
            title: "Few pages, and its text in another language",
            paginator(
                page: 2,
                page_count: 3,
                href: "/c/pagination?page={page}",
                label: "Seitennavigation",
                page_label: "Seite {page}",
                previous_text: "Zurück",
                next_text: "Weiter",
                previous_label: "Zurück zur vorherigen Seite",
                next_label: "Weiter zur nächsten Seite",
                compact: true
            )
        )
        example(
            title: "Previous and next only, for data with cursors",
            pagination(
                label: "Cursor pagination",
                pagination_content(
                    pagination_item(pagination_previous(disabled: true))
                    pagination_item(pagination_next(href: "/c/pagination?after=abc"))
                )
            )
        )
        example(
            title: "Put together from the parts",
            pagination(
                label: "Composed pagination",
                pagination_content(
                    pagination_item(pagination_previous(href: "/c/pagination?page=1"))
                    pagination_item(
                        pagination_link(
                            href: "/c/pagination?page=1",
                            label: "Page 1",
                            "1"
                        )
                    )
                    pagination_item(
                        pagination_link(
                            href: "/c/pagination?page=2",
                            label: "Page 2",
                            active: true,
                            "2"
                        )
                    )
                    pagination_item(pagination_ellipsis())
                    pagination_item(
                        pagination_link(
                            href: "/c/pagination?page=9",
                            label: "Page 9",
                            "9"
                        )
                    )
                    pagination_item(pagination_next(href: "/c/pagination?page=3"))
                )
            )
        )
    })
}
