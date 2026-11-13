//! A card that describes an agent: its name and model, its instructions, the tools it
//! can call, each with its input schema, and its output schema.
//!
//! The card is a `<section>` named by the agent's name. Each tool is a native
//! `<details>` whose `<summary>` holds a heading with the tool's name, so the browser
//! opens and closes it on click, Enter and Space. Tools that share a `group` are
//! exclusive: opening one closes the others.
//!
//! ```ignore
//! view! {
//!     agent(name: "Research assistant",
//!         agent_header(name: "Research assistant", model: "claude-sonnet-4")
//!         agent_content(
//!             agent_instructions("Answer with citations.")
//!             agent_tools(
//!                 agent_tool(name: "search_web", group: "tools",
//!                     description: "Search the web", schema: SCHEMA)
//!             )
//!             agent_output(schema: "z.object({ answer: z.string() })")
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream's tool list is Base UI's accordion, a button
//! inside a heading; here, as in the base set's accordion, the heading is inside a
//! `<summary>`. One panel at a time is the `group` of each tool rather than the
//! default, and there is no `onValueChange`: listen for the native `toggle` event.
//! Schemas are text, which the caller formats. The card and its sections are named
//! with `aria-label` rather than by an element inside them, as Playwright's Firefox
//! crashes on a region or group named that way.

use bitop_ui::components::{
    badge::badge,
    icon::{Icon, icon},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::code_block::code_block;
use crate::css::agent as css;

/// The card: a `<section>` named by `name`, the agent's name.
#[component]
pub async fn agent(
    cx: &Cx,
    name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section
            aria-label=(name)
            data-agent=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </section>
    })
}

/// The header: an icon, the name, the model as a badge and actions.
///
/// - `model` is shown in monospace, after `model_label` for assistive technology.
/// - `icon_name` replaces the robot.
/// - `actions` is a slot at the end of the row.
#[component]
pub async fn agent_header(
    cx: &Cx,
    name: &str,
    #[default] model: &str,
    #[default("Model: ")] model_label: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>
            <span aria-hidden="true" class=(css::ICON)>
                icon(name: icon_name.unwrap_or(Icon::Bot))
            </span>
            <span class=(css::NAME)>(name)</span>
            if !model.is_empty() {
                badge(
                    attrs: attributes! { class=(css::MODEL) data-agent-model="" },
                    <span class="sr-only">(model_label)</span>
                    (model)
                )
            }
            <span class=(css::ACTIONS)>(actions)</span>
        </div>
    })
}

/// The body, under the header.
#[component]
pub async fn agent_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The system prompt, in a group named `label`. Line breaks and spaces are kept, and
/// long words wrap.
#[component]
pub async fn agent_instructions(
    cx: &Cx,
    #[default("Instructions")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            class=(class!(css::SECTION, attrs.remove("class")))
            (attrs)
        >
            <p aria-hidden="true" class=(css::SECTION_LABEL)>(label)</p>
            <div class=(css::INSTRUCTIONS)>(child)</div>
        </div>
    })
}

/// The tools, in a group named `label`. Put [`agent_tool`]s in it.
#[component]
pub async fn agent_tools(
    cx: &Cx,
    #[default("Tools")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            class=(class!(css::SECTION, attrs.remove("class")))
            (attrs)
        >
            <p aria-hidden="true" class=(css::SECTION_LABEL)>(label)</p>
            <div class=(css::TOOLS)>(child)</div>
        </div>
    })
}

/// One tool: a `<details>` with the name and description in a heading, and the input
/// schema in its panel.
///
/// - `schema` is shown as code (`schema_language`, `json` by default) under the
///   heading `schema_label`; without one, `empty_label` is shown.
/// - `level` is the heading's level, 3 by default.
/// - `group` makes the tools that share it exclusive; `open` opens this one.
/// - `disabled` keeps it closed: the trigger cannot be focused or clicked.
#[component]
pub async fn agent_tool(
    cx: &Cx,
    name: &str,
    #[default] description: &str,
    #[default] schema: &str,
    #[default("json")] schema_language: &str,
    #[default("Input schema")] schema_label: &str,
    #[default("No input parameters.")] empty_label: &str,
    #[default(3)] level: u8,
    #[default] group: &str,
    #[default] open: bool,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let heading = class!(css::TOOL_HEADING);
    Ok(Child::new(view! {
        <details
            open=(open && !disabled)
            name=((!group.is_empty()).then_some(group))
            data-agent-tool=(name)
            class=(class!(css::TOOL, attrs.remove("class")))
            (attrs)
        >
            <summary
                class=(css::TOOL_TRIGGER)
                if disabled {
                    aria-disabled="true"
                    tabindex="-1"
                }
            >
                match level {
                    2 => <h2 class=(heading)>
                        <code class=(css::TOOL_NAME)>(name)</code>
                        <span class=(css::TOOL_DESCRIPTION)>(description)</span>
                    </h2>,
                    4 => <h4 class=(heading)>
                        <code class=(css::TOOL_NAME)>(name)</code>
                        <span class=(css::TOOL_DESCRIPTION)>(description)</span>
                    </h4>,
                    5 => <h5 class=(heading)>
                        <code class=(css::TOOL_NAME)>(name)</code>
                        <span class=(css::TOOL_DESCRIPTION)>(description)</span>
                    </h5>,
                    6 => <h6 class=(heading)>
                        <code class=(css::TOOL_NAME)>(name)</code>
                        <span class=(css::TOOL_DESCRIPTION)>(description)</span>
                    </h6>,
                    _ => <h3 class=(heading)>
                        <code class=(css::TOOL_NAME)>(name)</code>
                        <span class=(css::TOOL_DESCRIPTION)>(description)</span>
                    </h3>,
                }
                icon(
                    name: Icon::ChevronDown,
                    attrs: attributes! { class=(css::CHEVRON) }
                )
            </summary>
            <div class=(css::TOOL_BODY)>
                if schema.is_empty() {
                    <p class=(css::EMPTY)>(empty_label)</p>
                } else {
                    code_block(
                        code: schema,
                        language: schema_language,
                        filename: schema_label,
                        max_height: "20rem"
                    )
                }
            </div>
        </details>
    }))
}

/// The output schema as code, in a group named `label`. `language` is the schema's,
/// `typescript` by default (a Zod or TypeScript type); pass `json` for JSON Schema.
#[component]
pub async fn agent_output(
    cx: &Cx,
    schema: &str,
    #[default("typescript")] language: &str,
    #[default("Output schema")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            class=(class!(css::SECTION, attrs.remove("class")))
            (attrs)
        >
            <p aria-hidden="true" class=(css::SECTION_LABEL)>(label)</p>
            code_block(
                code: schema,
                language: language,
                hide_header: true,
                max_height: "20rem"
            )
        </div>
    })
}
