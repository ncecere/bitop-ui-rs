use bitop_ui::components::icon::{Icon, icon};
use bitop_ui_ai::components::chain_of_thought::{
    StepStatus, chain_of_thought, chain_of_thought_content, chain_of_thought_header,
    chain_of_thought_search_result, chain_of_thought_search_results, chain_of_thought_step,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "chain-of-thought")
        example(
            title: "Every step state, with search results",
            <div data-example="open" style="inline-size: 100%">
                chain_of_thought(
                    open: true,
                    chain_of_thought_header()
                    chain_of_thought_content(
                        chain_of_thought_step(
                            label: "Searched the handbook",
                            description: "Found 3 documents about parental leave",
                            chain_of_thought_search_results(
                                chain_of_thought_search_result(
                                    icon(name: Icon::FileText)
                                    "handbook.pdf"
                                )
                                chain_of_thought_search_result("leave-policy-2025.md")
                                chain_of_thought_search_result("faq.md")
                            )
                        )
                        chain_of_thought_step(
                            label: "Checked the web",
                            icon_name: Icon::Globe
                        )
                        chain_of_thought_step(
                            label: "Drafting the answer",
                            status: StepStatus::Active
                        )
                        chain_of_thought_step(
                            label: "Checking citations",
                            status: StepStatus::Pending
                        )
                    )
                )
            </div>
        )
        example(
            title: "Closed, with its own header",
            <div data-example="closed" style="inline-size: 100%">
                chain_of_thought(
                    attrs: attributes! { id="closed-chain" },
                    chain_of_thought_header(
                        label: "How I got here",
                        icon_name: Icon::Brain
                    )
                    chain_of_thought_content(
                        chain_of_thought_step(label: "Read the question")
                        chain_of_thought_step(label: "Answered it")
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                chain_of_thought(
                    open: true,
                    chain_of_thought_header(label: "Gedankengang")
                    chain_of_thought_content(
                        label: "Schritte",
                        chain_of_thought_step(
                            label: "Handbuch durchsucht",
                            complete_label: "erledigt"
                        )
                    )
                )
            </div>
        )
    })
}
