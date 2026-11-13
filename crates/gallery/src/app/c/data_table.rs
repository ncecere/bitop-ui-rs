use bitop_ui::{
    components::{
        alert::error_alert,
        badge::{BadgeTone, status_badge},
        button::{ButtonSize, ButtonVariant, button},
        data_table::{
            DataTableColumn, DataTableSort, SortDirection, cell_text, data_table,
            data_table_message, data_table_more, data_table_row_link, data_table_select,
            data_table_skeleton, page_rows,
        },
        icon::Icon,
        input::{InputSize, input},
        pagination::{
            pagination, pagination_content, pagination_item, pagination_next, pagination_previous,
            paginator,
        },
        table::{TableDensity, table_actions, table_empty, td, th, tr},
    },
    format::format_number,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param, query_string};

struct Invoice {
    id: &'static str,
    number: &'static str,
    customer: &'static str,
    email: &'static str,
    cents: u32,
    paid: bool,
    due: &'static str,
}

const fn invoice(
    id: &'static str,
    number: &'static str,
    customer: &'static str,
    email: &'static str,
    cents: u32,
    paid: bool,
    due: &'static str,
) -> Invoice {
    Invoice {
        id,
        number,
        customer,
        email,
        cents,
        paid,
        due,
    }
}

const INVOICES: [Invoice; 12] = [
    invoice(
        "i1",
        "INV-001",
        "Acme",
        "billing@acme.example",
        120_000,
        true,
        "1 Oct",
    ),
    invoice(
        "i2",
        "INV-002",
        "Globex",
        "ap@globex.example",
        48_050,
        false,
        "3 Oct",
    ),
    invoice(
        "i3",
        "INV-003",
        "Initech",
        "pay@initech.example",
        310_000,
        true,
        "4 Oct",
    ),
    invoice(
        "i4",
        "INV-004",
        "Umbrella",
        "ar@umbrella.example",
        9_900,
        false,
        "7 Oct",
    ),
    invoice(
        "i5",
        "INV-005",
        "Hooli",
        "money@hooli.example",
        76_400,
        true,
        "9 Oct",
    ),
    invoice(
        "i6",
        "INV-006",
        "Stark",
        "ap@stark.example",
        990_000,
        false,
        "11 Oct",
    ),
    invoice(
        "i7",
        "INV-007",
        "Wayne",
        "billing@wayne.example",
        15_000,
        true,
        "12 Oct",
    ),
    invoice(
        "i8",
        "INV-008",
        "Wonka",
        "pay@wonka.example",
        2_500,
        true,
        "15 Oct",
    ),
    invoice(
        "i9",
        "INV-009",
        "Tyrell",
        "ap@tyrell.example",
        640_000,
        false,
        "18 Oct",
    ),
    invoice(
        "i10",
        "INV-010",
        "Soylent",
        "ar@soylent.example",
        33_300,
        true,
        "21 Oct",
    ),
    invoice(
        "i11",
        "INV-011",
        "Cyberdyne",
        "billing@cyberdyne.example",
        71_000,
        false,
        "25 Oct",
    ),
    invoice(
        "i12",
        "INV-012",
        "Aperture",
        "pay@aperture.example",
        4_200,
        true,
        "28 Oct",
    ),
];

const PAGE_SIZE: u32 = 5;

/// Whole dollars, or dollars and two digits of cents: "$1,200", "$480.50".
/// `format_number` drops trailing zeros, which suits counts but not money.
fn money(cents: u32) -> String {
    let dollars = format_number(f64::from(cents / 100), 0, false);
    match cents % 100 {
        0 => format!("${dollars}"),
        rest => format!("${dollars}.{rest:02}"),
    }
}

/// One row, with the strings the view needs.
struct Row {
    id: &'static str,
    number: &'static str,
    customer: &'static str,
    email: &'static str,
    amount: String,
    paid: bool,
    due: &'static str,
    href: String,
    refund: String,
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    // The server sorts, filters and pages, from the query string.
    let sort_column = query_param(cx, "sort").unwrap_or_default();
    let direction = query_param(cx, "dir").and_then(|dir| SortDirection::parse(&dir));
    let search = query_param(cx, "q").unwrap_or_default().to_lowercase();
    let action = query_param(cx, "action").unwrap_or_default();
    let query = query_string(cx);
    let chosen: Vec<&str> = query
        .split('&')
        .filter_map(|pair| pair.strip_prefix("selected="))
        .collect();

    let mut invoices: Vec<&Invoice> = INVOICES
        .iter()
        .filter(|invoice| {
            search.is_empty()
                || invoice.customer.to_lowercase().contains(&search)
                || invoice.number.to_lowercase().contains(&search)
        })
        .collect();
    let sort = match (sort_column.as_str(), direction) {
        (column @ ("number" | "customer" | "amount"), Some(direction)) => {
            invoices.sort_by(|a, b| match column {
                "customer" => a.customer.cmp(b.customer),
                "amount" => a.cents.cmp(&b.cents),
                _ => a.number.cmp(b.number),
            });
            if direction == SortDirection::Descending {
                invoices.reverse();
            }
            Some(DataTableSort {
                column: match column {
                    "customer" => "customer",
                    "amount" => "amount",
                    _ => "number",
                },
                direction,
            })
        }
        _ => None,
    };
    let total = invoices.len();
    let page_count = u32::try_from(total).unwrap_or(0).div_ceil(PAGE_SIZE).max(1);
    let current = query_param(cx, "page")
        .and_then(|page| page.parse::<u32>().ok())
        .unwrap_or(1)
        .clamp(1, page_count);
    let (first, last) = page_rows(current, PAGE_SIZE, total as u64);
    let summary = format!("Rows {first}–{last} of {total}");
    let rows: Vec<Row> = invoices
        .iter()
        .skip(((current - 1) * PAGE_SIZE) as usize)
        .take(PAGE_SIZE as usize)
        .map(|invoice| Row {
            id: invoice.id,
            number: invoice.number,
            customer: invoice.customer,
            email: invoice.email,
            amount: money(invoice.cents),
            paid: invoice.paid,
            due: invoice.due,
            href: format!("#{}", invoice.number),
            refund: format!("Refund {}", invoice.number),
        })
        .collect();
    let no_rows = rows.is_empty();

    let keep = match sort {
        Some(sort) => format!("sort={}&dir={}&", sort.column, sort.direction.as_param()),
        None => String::new(),
    };
    let page_href = format!("/c/data-table?{keep}page={{page}}");
    let status = if search.is_empty() {
        String::new()
    } else {
        format!(
            "{total} {} the filter",
            if total == 1 {
                "row matches"
            } else {
                "rows match"
            }
        )
    };
    let result = if action.is_empty() {
        String::new()
    } else {
        let numbers: Vec<&str> = INVOICES
            .iter()
            .filter(|invoice| chosen.contains(&invoice.id))
            .map(|invoice| invoice.number)
            .collect();
        format!("{action}: {}", numbers.join(", "))
    };

    let columns = [
        DataTableColumn::new("number", "Invoice").sortable().fixed(),
        DataTableColumn::new("customer", "Customer").sortable(),
        DataTableColumn::new("amount", "Amount")
            .sortable()
            .numeric(),
        DataTableColumn::new("status", "Status"),
        DataTableColumn::new("due", "Due").hidden(),
    ];
    let plain = [
        DataTableColumn::new("name", "Name"),
        DataTableColumn::new("role", "Role"),
        DataTableColumn::new("seen", "Last seen")
            .hide_label()
            .width("8rem"),
    ];

    Ok(view! {
        page_header(slug: "data-table")
        example(
            title: "Sorted and paged by the server, with selection and a columns menu",
            <form method="get" action="/c/data-table" style="inline-size: 100%">
                if !result.is_empty() {
                    <p data-result="" style="margin-block-end: var(--space-3)">
                        (result)
                    </p>
                }
                data_table(
                    id: "invoices",
                    caption: "Invoices",
                    columns: &columns,
                    sort: sort,
                    sort_href: "/c/data-table?sort={column}&dir={direction}",
                    sort_clear_href: "/c/data-table",
                    selectable: true,
                    row_count: total,
                    row_actions: true,
                    columns_menu: true,
                    columns_storage_key: "gallery-invoices-columns",
                    status: &status,
                    summary: &summary,
                    search: view! {
                        <label class="sr-only" for="invoice-search">
                            "Filter invoices"
                        </label>
                        // Enter submits the search form, which has no other field.
                        input(
                            size: InputSize::Sm,
                            start_icon: Icon::Search,
                            attrs: attributes! {
                                id="invoice-search"
                                type="search"
                                name="q"
                                value=(search.clone())
                                placeholder="Filter invoices…"
                                form="invoice-search-form"
                            }
                        )
                    },
                    bulk_actions: view! {
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            attrs: attributes! { type="submit" name="action" value="Archived" },
                            "Archive"
                        )
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Danger,
                            attrs: attributes! { type="submit" name="action" value="Deleted" },
                            "Delete"
                        )
                    },
                    footer: view! {
                        if page_count > 1 {
                            paginator(
                                page: current,
                                page_count: page_count,
                                href: &page_href,
                                size: ButtonSize::Sm,
                                label: "Invoices pages",
                                compact: true
                            )
                        }
                    },
                    if no_rows {
                        table_empty(
                            columns: 7,
                            data_table_message("No invoices match the filter.")
                        )
                    }
                    for row in rows {
                        tr(
                            attrs: attributes! { data-clickable="" },
                            data_table_select(value: row.id, label: row.number)
                            th(data_table_row_link(href: &row.href, (row.number)))
                            td(cell_text(primary: row.customer, secondary: row.email))
                            td(numeric: true, (row.amount))
                            td(
                                status_badge(
                                    tone: if row.paid {
                                        BadgeTone::Success
                                    } else {
                                        BadgeTone::Warning
                                    },
                                    (if row.paid { "Paid" } else { "Open" })
                                )
                            )
                            td(muted: true, (row.due))
                            td(
                                nowrap: true,
                                sticky_end: true,
                                table_actions(
                                    button(
                                        size: ButtonSize::Sm,
                                        variant: ButtonVariant::Ghost,
                                        attrs: attributes! { aria-label=(row.refund) },
                                        "Refund"
                                    )
                                )
                            )
                        )
                    }
                )
            </form>
            <form
                id="invoice-search-form"
                method="get"
                action="/c/data-table"
                role="search"
                aria-label="Invoices"
            ></form>
        )
        example(
            title: "Cursor paging, with a row selected from the start",
            data_table(
                id: "members",
                caption: "Members",
                columns: &plain,
                selectable: true,
                selected_count: 1,
                row_count: 40,
                density: TableDensity::Compact,
                summary: "Showing 1–3",
                footer: view! {
                    pagination(
                        label: "Members pages",
                        compact: true,
                        pagination_content(
                            pagination_item(
                                pagination_previous(
                                    disabled: true,
                                    size: ButtonSize::Sm,
                                    compact: true
                                )
                            )
                            pagination_item(
                                pagination_next(
                                    href: "/c/data-table?after=m3",
                                    size: ButtonSize::Sm,
                                    compact: true
                                )
                            )
                        )
                    )
                },
                tr(
                    data_table_select(
                        value: "m1",
                        label: "Ada Lovelace",
                        name: "member"
                    )
                    th("Ada Lovelace")
                    td("Owner")
                    td(muted: true, "Today")
                )
                tr(
                    selected: true,
                    data_table_select(
                        value: "m2",
                        label: "Grace Hopper",
                        name: "member",
                        checked: true
                    )
                    th("Grace Hopper")
                    td("Admin")
                    td(muted: true, "Yesterday")
                )
                tr(
                    data_table_select(
                        value: "m3",
                        label: "Alan Turing",
                        name: "member",
                        disabled: true
                    )
                    th("Alan Turing")
                    td("Member")
                    td(muted: true, "Last week")
                )
            )
        )
        example(
            title: "Loading",
            data_table(
                id: "loading",
                caption: "Loading members",
                columns: &plain,
                loading: true,
                status: "Loading rows…",
                data_table_skeleton(columns: 3, rows: 3)
            )
        )
        example(
            title: "Refreshing, with an error above the rows it still has",
            data_table(
                id: "stale",
                caption: "Stale members",
                columns: &plain,
                loading: true,
                stale: true,
                error: view! {
                    error_alert(
                        status: 503,
                        title: "Couldn't load rows",
                        retry_href: "/c/data-table",
                        retry_label: "Retry"
                    )
                },
                more: view! { data_table_more(href: "/c/data-table?more=1") },
                tr(
                    th("Ada Lovelace")
                    td("Owner")
                    td("Today")
                )
                tr(
                    th("Grace Hopper")
                    td("Admin")
                    td("Yesterday")
                )
            )
        )
        example(
            title: "Empty",
            data_table(
                id: "empty",
                caption: "No members",
                columns: &plain,
                table_empty(columns: 3, data_table_message("No rows."))
            )
        )
    })
}
