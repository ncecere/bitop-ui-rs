use bitop_ui::components::{
    badge::{BadgeVariant, badge},
    button::{ButtonSize, ButtonVariant, button},
    empty_state::empty_state,
    icon::Icon,
    table::{TableDensity, TableSort, table, table_actions, table_column, table_empty, td, th, tr},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const FILES: [(&str, &str, &str); 4] = [
    ("handbook.pdf", "1.2 MB", "Ready"),
    ("pricing.xlsx", "84 kB", "Ready"),
    ("roadmap.md", "12 kB", "Indexing"),
    ("interview-notes.docx", "310 kB", "Ready"),
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "table")
        example(
            title: "Framed, with a row header, numbers and actions",
            table(
                caption: "Documents",
                framed: true,
                columns: view! {
                    table_column(sort: TableSort::Ascending, "Name")
                    table_column(numeric: true, "Size")
                    table_column("Status")
                    table_column(blank: true)
                },
                for (index, (name, size, status)) in FILES.into_iter().enumerate() {
                    tr(
                        selected: index == 1,
                        th((name))
                        td(numeric: true, (size))
                        td(
                            muted: true,
                            badge(
                                variant: if status == "Ready" {
                                    BadgeVariant::Success
                                } else {
                                    BadgeVariant::Neutral
                                },
                                (status)
                            )
                        )
                        td(
                            table_actions(
                                button(
                                    size: ButtonSize::Sm,
                                    variant: ButtonVariant::Ghost,
                                    attrs: attributes! { aria-label=(format!("Open {name}")) },
                                    "Open"
                                )
                            )
                        )
                    )
                }
            )
        )
        example(
            title: "Compact and striped, with a visible caption",
            table(
                caption: "Usage by model",
                show_caption: true,
                density: TableDensity::Compact,
                striped: true,
                columns: view! {
                    table_column(width: "50%", "Model")
                    table_column(numeric: true, "Requests")
                    table_column(numeric: true, hide_label: true, "Share")
                },
                tr(
                    td("small")
                    td(numeric: true, "8,120")
                    td(numeric: true, "65%")
                )
                tr(
                    td("medium")
                    td(numeric: true, "3,410")
                    td(numeric: true, "27%")
                )
                tr(
                    td("large")
                    td(numeric: true, "950")
                    td(numeric: true, "8%")
                )
                tr(
                    td("embedding")
                    td(numeric: true, "12")
                    td(numeric: true, "0%")
                )
            )
        )
        example(
            title: "A sticky header in a scroll area",
            table(
                caption: "Events",
                sticky_header: true,
                max_height: "12rem",
                framed: true,
                columns: view! {
                    table_column("Event")
                    table_column(numeric: true, "Count")
                },
                for n in 1..=12 {
                    tr(
                        td(
                            "Event "
                            (n)
                        )
                        td(numeric: true, (n * 7))
                    )
                }
            )
        )
        example(
            title: "Wider than its container, with the last column pinned",
            <div style="inline-size: 24rem; max-inline-size: 100%">
                table(
                    caption: "Deployments",
                    framed: true,
                    sticky_end: true,
                    columns: view! {
                        table_column("Commit")
                        table_column("Branch")
                        table_column("Author")
                        table_column("Environment")
                        table_column("Duration")
                        table_column(blank: true, sticky_end: true)
                    },
                    for n in 1..=3 {
                        tr(
                            td(
                                nowrap: true,
                                "671a1f"
                                (n)
                            )
                            td(nowrap: true, "feature/long-branch-name")
                            td(nowrap: true, "Ada Lovelace")
                            td(nowrap: true, "Production")
                            td(nowrap: true, "1 min 12 s")
                            td(
                                sticky_end: true,
                                table_actions(
                                    button(
                                        size: ButtonSize::Sm,
                                        variant: ButtonVariant::Outline,
                                        attrs: attributes! { aria-label=(format!("Roll back deployment {n}")) },
                                        "Roll back"
                                    )
                                )
                            )
                        )
                    }
                )
            </div>
        )
        example(
            title: "Wider than its container, without a pinned column",
            <div style="inline-size: 20rem; max-inline-size: 100%">
                table(
                    caption: "Regions",
                    columns: view! {
                        table_column("Region")
                        table_column("Provider")
                        table_column("Latency")
                        table_column("Status")
                    },
                    tr(
                        td(nowrap: true, "Frankfurt, Germany")
                        td(nowrap: true, "Example Cloud")
                        td(nowrap: true, "12 milliseconds")
                        td(nowrap: true, "Operational")
                    )
                )
            </div>
        )
        example(
            title: "Empty",
            table(
                caption: "Invoices",
                framed: true,
                columns: view! {
                    table_column("Number")
                    table_column("Date")
                    table_column(numeric: true, "Amount")
                },
                table_empty(
                    columns: 3,
                    empty_state(
                        title: "No invoices yet",
                        description: "They appear here after your first payment.",
                        icon: Some(Icon::Inbox),
                        compact: true
                    )
                )
            )
        )
        example(
            title: "Stacked on a narrow screen",
            table(
                caption: "Notification settings",
                stack: true,
                columns: view! {
                    table_column("Event")
                    table_column("Email")
                    table_column("Push")
                    table_column(hide_label: true, "Notes")
                },
                tr(
                    th("New comment")
                    td("On")
                    td("Off")
                    td(attrs: attributes! { data-label="Remarks" }, "Daily digest")
                )
                tr(
                    th("Mention")
                    td("On")
                    td("On")
                    td("At once")
                )
            )
        )
    })
}
