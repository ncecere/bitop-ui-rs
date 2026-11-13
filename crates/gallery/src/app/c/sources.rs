use bitop_ui::components::icon::Icon;
use bitop_ui_ai::components::sources::{source, sources, sources_content, sources_trigger};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "sources")
        example(
            title: "Closed, with links",
            <div data-example="links" style="inline-size: 100%">
                sources(
                    sources_trigger(count: 3)
                    sources_content(
                        source(
                            index: 1,
                            title: "Parental leave policy 2025",
                            href: "https://www.example.com/policies/parental-leave",
                            description: "Eligible employees receive sixteen weeks of fully paid leave, to be taken within twelve months of the child's arrival."
                        )
                        source(
                            index: 2,
                            title: "Benefits FAQ",
                            href: "https://help.example.org/benefits#leave",
                            description: "Leave can be split into up to three blocks of at least two weeks each."
                        )
                        source(
                            index: 3,
                            title: "Requesting leave.pdf",
                            meta: "p. 4",
                            icon_name: Icon::File
                        )
                    )
                )
            </div>
        )
        example(
            title: "Open, one source",
            <div data-example="one" style="inline-size: 100%">
                sources(
                    open: true,
                    sources_trigger(count: 1)
                    sources_content(
                        source(
                            title: "Handbook, chapter 3",
                            href: "https://handbook.example.com/3"
                        )
                    )
                )
            </div>
        )
        example(
            title: "Selectable cards, which citation chips can focus",
            <div data-example="select" style="inline-size: 100%">
                sources(
                    open: true,
                    sources_trigger(count: 2)
                    sources_content(
                        source(
                            index: 1,
                            id: "select-source-1",
                            focusable: true,
                            selectable: true,
                            select_action: "Show",
                            title: "Leave policy",
                            href: "https://www.example.com/leave",
                            description: "Sixteen weeks, fully paid."
                        )
                        source(
                            id: "select-source-2",
                            focusable: true,
                            selectable: true,
                            select_label: "Open Travel policy in the side panel",
                            title: "Travel policy"
                        )
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                sources(
                    sources_trigger(
                        count: 2,
                        label_one: "{count} Quelle verwendet",
                        label_other: "{count} Quellen verwendet"
                    )
                    sources_content(
                        label: "Quellen",
                        source(
                            index: 1,
                            title: "Urlaubsregelung",
                            href: "https://intranet.example.de/urlaub",
                            source_label: "Quelle",
                            new_tab_label: "(öffnet in einem neuen Tab)"
                        )
                        source(index: 2, title: "Reisekosten", source_label: "Quelle")
                    )
                )
            </div>
        )
    })
}
