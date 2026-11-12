use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    calendar::Date,
    filter_bar::{
        Facet, FacetOption, FilterBarSize, facet_counts, filter_bar, filter_rows,
        filter_values_from_query, parse_query,
    },
    input::input,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_string};

struct Doc {
    name: &'static str,
    status: &'static str,
    kinds: &'static [&'static str],
    updated: &'static str,
}

const DOCS: [Doc; 6] = [
    Doc {
        name: "handbook.pdf",
        status: "ready",
        kinds: &["pdf"],
        updated: "2026-09-26",
    },
    Doc {
        name: "pricing.docx",
        status: "ready",
        kinds: &["doc"],
        updated: "2026-09-22",
    },
    Doc {
        name: "wifi-setup.pdf",
        status: "failed",
        kinds: &["pdf"],
        updated: "2026-09-10",
    },
    Doc {
        name: "blog/launch",
        status: "failed",
        kinds: &["web"],
        updated: "2026-06-01",
    },
    Doc {
        name: "wifi-faq",
        status: "skipped",
        kinds: &["web", "doc"],
        updated: "2026-09-25",
    },
    Doc {
        name: "notes.docx",
        status: "ready",
        kinds: &["doc"],
        updated: "2026-08-30",
    },
];

const STATUSES: [FacetOption<'static>; 3] = [
    FacetOption::new("ready", "Ready"),
    FacetOption::new("failed", "Failed"),
    FacetOption::new("skipped", "Skipped"),
];
const KINDS: [FacetOption<'static>; 3] = [
    FacetOption::new("pdf", "PDF"),
    FacetOption::new("doc", "Word"),
    FacetOption::new("web", "Web page"),
];
const OWNERS: [FacetOption<'static>; 3] = [
    FacetOption::new("ada", "Ada Lovelace"),
    FacetOption::new("grace", "Grace Hopper"),
    FacetOption::new("alan", "Alan Turing"),
];
const PLANS: [FacetOption<'static>; 2] = [
    FacetOption::new("free", "Free"),
    FacetOption::new("team", "Team"),
];
const TAGS: [FacetOption<'static>; 3] = [
    FacetOption::new("new", "New"),
    FacetOption::new("pinned", "Pinned"),
    FacetOption::new("shared", "Shared"),
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    // A fixed "today", so that the date presets give the same rows on any day.
    let today = Date::new(2026, 9, 26).unwrap_or_else(Date::today_utc);
    let query = query_string(cx);
    let facets = [
        Facet::toggle("status", "Status", &STATUSES).all_label("All"),
        Facet::select("kind", "Kind", &KINDS).multiple(),
        Facet::select("owner", "Owner", &OWNERS),
        Facet::date_range("updated", "Updated"),
    ];
    let values = filter_values_from_query(&facets, &query);
    let search = parse_query(&query)
        .into_iter()
        .find(|(name, _)| name == "q")
        .map(|(_, value)| value.trim().to_lowercase())
        .unwrap_or_default();
    let access = |doc: &Doc, facet: &Facet<'_>| match facet.id {
        "status" => vec![doc.status.to_owned()],
        "kind" => doc.kinds.iter().map(|kind| (*kind).to_owned()).collect(),
        "updated" => vec![doc.updated.to_owned()],
        _ => Vec::new(),
    };
    let searched: Vec<&Doc> = DOCS
        .iter()
        .filter(|doc| search.is_empty() || doc.name.contains(&search))
        .collect();
    // The owner facet has no data behind it here; it only shows a plain select.
    let filtering = [facets[0], facets[1], facets[3]];
    let counts = facet_counts(&searched, &filtering, &values, today, |doc, facet| {
        access(doc, facet)
    });
    let rows = filter_rows(&searched, &filtering, &values, today, |doc, facet| {
        access(doc, facet)
    });
    let names: Vec<&str> = rows.iter().map(|doc| doc.name).collect();
    let status = format!(
        "{} {}",
        names.len(),
        if names.len() == 1 {
            "document"
        } else {
            "documents"
        }
    );

    let more = [
        Facet::toggle("plan", "Plan", &PLANS).chip(true),
        Facet::toggle("tag", "Tags", &TAGS)
            .multiple()
            .all_label("Any"),
    ];
    let more_values = filter_values_from_query(&more, &query);

    Ok(view! {
        page_header(slug: "filter-bar")
        example(
            title: "Filters in the URL, applied by the server",
            <div style="display: grid; gap: var(--space-4); inline-size: 100%">
                filter_bar(
                    id: "doc-filters",
                    action: "/c/filter-bar",
                    query: &query,
                    facets: &facets,
                    values: &values,
                    counts: &counts,
                    drop_params: &["page"],
                    search_param: "q",
                    start: view! {
                        <label class="sr-only" for="doc-search">
                            "Search documents"
                        </label>
                        input(
                            attrs: attributes! {
                                id="doc-search"
                                type="search"
                                name="q"
                                value=(search.clone())
                                placeholder="Search documents"
                                style="inline-size: 12rem"
                            }
                        )
                    },
                    end: view! {
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Outline,
                            "Export"
                        )
                    }
                )
                <p role="status" data-count="">(status)</p>
                <ul data-results="" style="padding-inline-start: var(--space-5)">
                    for name in names {
                        <li>(name)</li>
                    }
                </ul>
            </div>
        )
        example(
            title: "Medium, a choice that clears when pressed again, several choices, no chips",
            filter_bar(
                id: "more-filters",
                action: "/c/filter-bar",
                query: &query,
                facets: &more,
                values: &more_values,
                size: FilterBarSize::Md,
                chips: false,
                label: "More filters",
                apply_label: "Anwenden"
            )
        )
    })
}
