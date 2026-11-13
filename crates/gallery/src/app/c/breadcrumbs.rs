use bitop_ui::components::{
    breadcrumbs::{breadcrumb, breadcrumb_collapsed, breadcrumb_collapsed_item, breadcrumbs},
    icon::Icon,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let hidden_id = element_id(cx);

    Ok(view! {
        page_header(slug: "breadcrumbs")
        example(
            title: "A trail, the first crumb with an icon",
            breadcrumbs(
                breadcrumb(href: "/", label: "Acme", icon_name: Icon::Boxes)
                breadcrumb(href: "/c/breadcrumbs?p=projects", label: "Projects")
                breadcrumb(href: "/c/breadcrumbs?p=web", label: "Web")
                breadcrumb(label: "v2.1")
            )
        )
        example(
            title: "The middle collapsed into a menu",
            breadcrumbs(
                label: "Collapsed breadcrumb",
                breadcrumb(href: "/", label: "Acme", icon_name: Icon::Boxes)
                breadcrumb_collapsed(
                    id: &hidden_id,
                    label: "Show 3 hidden levels",
                    title: "Projects › Web › Releases",
                    breadcrumb_collapsed_item(
                        href: "/c/breadcrumbs?p=projects",
                        "Projects"
                    )
                    breadcrumb_collapsed_item(href: "/c/breadcrumbs?p=web", "Web")
                    breadcrumb_collapsed_item(
                        href: "/c/breadcrumbs?p=releases",
                        "Releases"
                    )
                )
                breadcrumb(label: "v2.1")
            )
        )
        example(
            title: "Too long for its room: the current page is cut first, then the others",
            <div style="inline-size: 22rem; max-inline-size: 100%">
                breadcrumbs(
                    label: "Long breadcrumb",
                    breadcrumb(href: "/", label: "Org")
                    breadcrumb(
                        href: "/c/breadcrumbs?p=team",
                        label: "Demo team with a long name"
                    )
                    breadcrumb(href: "/c/breadcrumbs?p=conn", label: "Connections")
                    breadcrumb(
                        label: "connection-eu-central-1-production-readonly-replica"
                    )
                )
            </div>
        )
        example(
            title: "Wrapping onto more lines",
            <div style="inline-size: 22rem; max-inline-size: 100%">
                breadcrumbs(
                    label: "Wrapping breadcrumb",
                    wrap: true,
                    attrs: attributes! { data-demo="wrap" },
                    breadcrumb(href: "/", label: "Org")
                    breadcrumb(
                        href: "/c/breadcrumbs?p=team",
                        label: "Demo team with a long name"
                    )
                    breadcrumb(href: "/c/breadcrumbs?p=conn", label: "Connections")
                    breadcrumb(label: "connection-eu-central-1")
                )
            </div>
        )
    })
}
