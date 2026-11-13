use bitop_ui_ai::components::{
    inline_citation::{CitationSide, CitationSource, CitationVerification, inline_citation},
    sources::{source, sources, sources_content, sources_trigger},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let report = [CitationSource::new("Annual report 2024")
        .href("https://www.example.com/annual-report-2024")
        .description("Revenue, margins and headcount for the fiscal year.")
        .quote("Revenue grew 12% year over year, driven by subscriptions.")];
    let several = [
        CitationSource::new("Annual report 2024")
            .href("https://www.example.com/annual-report-2024")
            .description("Revenue, margins and headcount for the fiscal year."),
        CitationSource::new("Q4 earnings call")
            .site_name("Investor relations")
            .description("The chief financial officer's remarks on subscriptions."),
        CitationSource::new("Analyst note").href("https://research.example.org/notes/42"),
    ];
    let unnumbered = [
        CitationSource::new("Climate data portal").href("https://data.example.net/climate"),
        CitationSource::new("Weather service archive").href("https://archive.example.gov/"),
    ];
    let listed = [
        CitationSource::new("Leave policy 2025")
            .href("https://hr.example.com/leave")
            .target("listed-source-1"),
        CitationSource::new("Benefits FAQ").target("listed-source-2"),
    ];
    let first_listed = [listed[0]];
    let second_listed = [listed[1]];
    Ok(view! {
        page_header(slug: "inline-citation")
        example(
            title: "One source, and several",
            <p data-example="basic" style="max-inline-size: 36rem; line-height: 1.65">
                "Revenue grew 12% last year"
                inline_citation(id: "cite-one", index: &[1], sources: &report)
                ", while three outlets reported the same figure"
                inline_citation(
                    id: "cite-several",
                    index: &[1, 2, 3],
                    sources: &several
                )
                ". Without numbers the chip names the site"
                inline_citation(
                    id: "cite-site",
                    sources: &unnumbered,
                    side: CitationSide::Bottom
                )
                "."
            </p>
        )
        example(
            title: "Checked claims",
            <p
                data-example="verified"
                style="max-inline-size: 36rem; line-height: 1.65"
            >
                "Supported"
                inline_citation(
                    id: "cite-verified",
                    index: &[1],
                    sources: &report,
                    verification: CitationVerification::Verified
                )
                ", not supported"
                inline_citation(
                    id: "cite-unsupported",
                    index: &[2],
                    sources: &report,
                    verification: CitationVerification::Unsupported
                )
                ", and contradicted"
                inline_citation(
                    id: "cite-contradicted",
                    index: &[3],
                    sources: &report,
                    verification: CitationVerification::Contradicted,
                    verification_label: "The report gives 9%, not 12%"
                )
                "."
            </p>
        )
        example(
            title: "Going to the sources listed below",
            <div
                data-example="listed"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                <p style="max-inline-size: 36rem; line-height: 1.65">
                    "Employees get sixteen weeks of paid leave"
                    inline_citation(
                        id: "cite-jump",
                        index: &[1],
                        sources: &first_listed,
                        activate_target: "listed-source-1"
                    )
                    " and can split it into blocks"
                    inline_citation(
                        id: "cite-action",
                        index: &[2],
                        sources: &second_listed,
                        source_action: "Show source {index} below"
                    )
                    "."
                </p>
                sources(
                    sources_trigger(count: 2)
                    sources_content(
                        source(
                            index: 1,
                            id: "listed-source-1",
                            focusable: true,
                            title: "Leave policy 2025",
                            href: "https://hr.example.com/leave"
                        )
                        source(
                            index: 2,
                            id: "listed-source-2",
                            focusable: true,
                            title: "Benefits FAQ"
                        )
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <p
                data-example="strings"
                lang="de"
                style="max-inline-size: 36rem; line-height: 1.65"
            >
                "Der Umsatz stieg um 12 %"
                inline_citation(
                    id: "cite-german",
                    index: &[1, 2, 3],
                    sources: &several,
                    source_label: "Quelle",
                    sources_label: "Quellen",
                    more_label: " und {count} weitere",
                    previous_label: "Vorherige Quelle",
                    next_label: "Nächste Quelle",
                    page_label: "{page} von {count}",
                    new_tab_label: "(öffnet in einem neuen Tab)"
                )
                "."
            </p>
        )
    })
}
