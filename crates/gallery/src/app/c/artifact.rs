use bitop_ui::components::icon::{Icon, icon};
use bitop_ui_ai::components::{
    artifact::{
        artifact, artifact_action, artifact_actions, artifact_close, artifact_content,
        artifact_description, artifact_header, artifact_title,
    },
    code_block::code_block,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

const CODE: &str =
    "fn main() {\n    let total: u32 = (1..=10).sum();\n    println!(\"{total}\");\n}\n";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let action = query_param(cx, "artifact");
    Ok(view! {
        page_header(slug: "artifact")
        example(
            title: "A document, with actions",
            <form
                data-example="document"
                method="get"
                action="/c/artifact"
                style="inline-size: 100%"
            >
                artifact(
                    label: "Q3 summary",
                    attrs: attributes! { id="document-artifact" style="max-block-size: 16rem" },
                    artifact_header(
                        <div>
                            artifact_title("Q3 summary")
                            artifact_description("Markdown · 2 min ago")
                        </div>
                        artifact_actions(
                            artifact_action(
                                label: "Open in a new tab",
                                id: "open",
                                tooltip: "artifact-open-tip",
                                icon(name: Icon::ExternalLink)
                            )
                            artifact_action(
                                label: "Save a copy",
                                tooltip: "artifact-save-tip",
                                name: "artifact",
                                value: "save",
                                icon(name: Icon::Copy)
                            )
                            artifact_close(tooltip: "artifact-close-tip")
                        )
                    )
                    artifact_content(
                        <h3 style="margin-block-end: var(--space-2)">"Revenue"</h3>
                        <p>
                            "Revenue rose 12% on the quarter, led by the new team plan."
                        </p>
                        <p style="margin-block-start: var(--space-3)">
                            "Churn held at 2.1%. Support tickets fell by a fifth after the help centre moved to the new search."
                        </p>
                        <p style="margin-block-start: var(--space-3)">
                            "Hiring stayed on plan: four engineers and two designers joined."
                        </p>
                        <p style="margin-block-start: var(--space-3)">
                            "Next quarter: the European data centre, and the annual price review."
                        </p>
                        <p style="margin-block-start: var(--space-3)">
                            "Risks: the payments migration slipped by two weeks."
                        </p>
                    )
                )
                if let Some(action) = action {
                    <p role="status" data-saved="">
                        "Action: "
                        (action)
                    </p>
                }
            </form>
        )
        example(
            title: "Code, flush, with a heading",
            <div data-example="code" style="inline-size: 100%">
                artifact(
                    label: "main.rs",
                    artifact_header(
                        <div>
                            artifact_title(level: 3, "main.rs")
                            artifact_description("Rust · 4 lines")
                        </div>
                    )
                    artifact_content(
                        flush: true,
                        code_block(code: CODE, language: "rust", hide_header: true)
                    )
                )
            </div>
        )
    })
}
