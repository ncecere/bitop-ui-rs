use bitop_ui::components::{
    badge::badge,
    breadcrumbs::{breadcrumb, breadcrumbs},
    button::{ButtonVariant, button},
    page_header::{PageHeaderLevel, page_header as header},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const WIDE: &str = "inline-size: 100%; margin-block-end: 0";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "page-header")
        example(
            title: "Everything: breadcrumbs, status, description, facts and actions",
            header(
                title: "Deployments",
                level: PageHeaderLevel::H2,
                title_id: "deployments-title",
                description: "Every release of this project, newest first. A deployment is kept for ninety days after a newer one replaces it.",
                breadcrumbs: view! {
                    breadcrumbs(
                        label: "Deployments breadcrumb",
                        breadcrumb(href: "/", label: "Acme")
                        breadcrumb(href: "/c/page-header", label: "Web")
                        breadcrumb(label: "Deployments")
                    )
                },
                meta: view! { badge("Live") },
                facts: view! {
                    <span
                        style="color: var(--color-text-muted); font-size: var(--font-size-sm)"
                    >
                        "42 deployments · last one 3 hours ago"
                    </span>
                },
                actions: view! {
                    button(variant: ButtonVariant::Outline, "Settings")
                    button("Deploy")
                },
                attrs: attributes! { data-demo="full" style=(WIDE) }
            )
        )
        example(
            title: "A title alone",
            header(
                title: "Members",
                level: PageHeaderLevel::H2,
                attrs: attributes! { data-demo="plain" style=(WIDE) }
            )
        )
        example(
            title: "A long name wraps, and the actions stay level with the title",
            header(
                title: "connection-eu-central-1-production-readonly-replica-0042",
                level: PageHeaderLevel::H3,
                description: "A read-only replica.",
                actions: view! { button(variant: ButtonVariant::Danger, "Delete") },
                attrs: attributes! {
                    data-demo="long"
                    style="inline-size: 24rem; max-inline-size: 100%; margin-block-end: 0"
                }
            )
        )
    })
}
