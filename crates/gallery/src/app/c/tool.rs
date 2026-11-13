use bitop_ui::components::icon::Icon;
use bitop_ui_ai::components::tool::{
    ToolState, tool, tool_content, tool_header, tool_input, tool_output,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const INPUT: &str = "{\n  \"query\": \"refund policy\",\n  \"limit\": 5\n}";
const OUTPUT: &str = "[\n  { \"title\": \"Refunds\", \"path\": \"policies/refunds.md\" },\n  { \"title\": \"Returns 2025\", \"path\": \"policies/returns-2025.pdf\" }\n]";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "tool")
        example(
            title: "Completed, open",
            <div data-example="completed" style="inline-size: 100%">
                tool(
                    open: true,
                    attrs: attributes! { id="completed-tool" },
                    tool_header(
                        name: "search_documents",
                        state: ToolState::Completed,
                        summary: "2 results"
                    )
                    tool_content(
                        tool_input(input: INPUT)
                        tool_output(output: OUTPUT)
                    )
                )
            </div>
        )
        example(
            title: "Every state",
            <div
                data-example="states"
                style="display: grid; gap: var(--space-2); inline-size: 100%"
            >
                tool(
                    attrs: attributes! { id="pending-tool" },
                    tool_header(name: "send_email", state: ToolState::Pending)
                    tool_content(tool_input(input: "{ \"to\": \"team@example.com\" }"))
                )
                tool(
                    attrs: attributes! { id="running-tool" },
                    tool_header(
                        name: "web_search",
                        title: "Searching the web",
                        state: ToolState::Running,
                        icon_name: Icon::Globe
                    )
                    tool_content(
                        tool_input(input: "{ \"q\": \"parental leave 2025\" }")
                    )
                )
                tool(
                    attrs: attributes! { id="error-tool" },
                    tool_header(name: "read_file", state: ToolState::Error)
                    tool_content(
                        tool_input(input: "{ \"path\": \"/etc/secrets\" }")
                        tool_output(
                            error_text: "Permission denied: the agent may only read files in the workspace."
                        )
                    )
                )
            </div>
        )
        example(
            title: "Text input, and a result that is not code",
            <div data-example="custom" style="inline-size: 100%">
                tool(
                    open: true,
                    tool_header(
                        name: "summarise",
                        state: ToolState::Completed,
                        state_label: "Done"
                    )
                    tool_content(
                        tool_input(
                            input: "Summarise the attached policy.",
                            language: "text"
                        )
                        tool_output(
                            <p data-custom-output="">
                                "Refunds are due within 30 days of a return."
                            </p>
                        )
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                tool(
                    open: true,
                    tool_header(
                        name: "dokumente_suchen",
                        state: ToolState::Running,
                        running_label: "Läuft"
                    )
                    tool_content(tool_input(input: "{}", label: "Parameter"))
                )
            </div>
        )
    })
}
