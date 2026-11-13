use bitop_ui::components::button::{ButtonSize, ButtonVariant, button};
use bitop_ui_ai::components::agent::{
    agent, agent_content, agent_header, agent_instructions, agent_output, agent_tool, agent_tools,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

const SEARCH_SCHEMA: &str = "{\n  \"type\": \"object\",\n  \"properties\": {\n    \"query\": { \"type\": \"string\" },\n    \"limit\": { \"type\": \"integer\", \"default\": 5 }\n  },\n  \"required\": [\"query\"]\n}";
const FETCH_SCHEMA: &str = "{\n  \"type\": \"object\",\n  \"properties\": { \"url\": { \"type\": \"string\", \"format\": \"uri\" } }\n}";
const OUTPUT: &str = "z.object({\n  answer: z.string(),\n  sources: z.array(z.string().url()),\n})";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "agent")
        example(
            title: "An agent with tools, one open at a time",
            <div data-example="full" style="inline-size: 100%">
                agent(
                    name: "Research assistant",
                    agent_header(
                        name: "Research assistant",
                        model: "claude-sonnet-4",
                        actions: view! {
                            button(
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                "Edit"
                            )
                        }
                    )
                    agent_content(
                        agent_instructions(
                            "Answer questions about company policy.\nCite every claim with a source from the handbook.\n\nIf you are not sure, say so."
                        )
                        agent_tools(
                            agent_tool(
                                name: "search_web",
                                description: "Search the web for recent pages",
                                schema: SEARCH_SCHEMA,
                                group: "research-tools",
                                open: true
                            )
                            agent_tool(
                                name: "fetch_page",
                                description: "Read one page",
                                schema: FETCH_SCHEMA,
                                group: "research-tools"
                            )
                            agent_tool(
                                name: "current_time",
                                description: "The time now, in UTC",
                                group: "research-tools"
                            )
                            agent_tool(
                                name: "delete_files",
                                description: "Not allowed for this agent",
                                disabled: true
                            )
                        )
                        agent_output(schema: OUTPUT)
                    )
                )
            </div>
        )
        example(
            title: "Name only, other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                agent(
                    name: "Übersetzer",
                    agent_header(
                        name: "Übersetzer",
                        model: "small-1",
                        model_label: "Modell: "
                    )
                    agent_content(
                        agent_instructions(
                            label: "Anweisungen",
                            "Übersetze ins Deutsche."
                        )
                        agent_tools(
                            label: "Werkzeuge",
                            agent_tool(name: "glossar", empty_label: "Keine Eingaben.")
                        )
                    )
                )
            </div>
        )
    })
}
