//! One tool call, in a card that opens to show its input and its output.
//!
//! [`tool`] is a native `<details>` and [`tool_header`] its `<summary>`, so the browser
//! opens and closes it on click, Enter and Space. The header shows the tool's name or a
//! title, the state as a badge with a word ("Running"), never by colour alone, and an
//! optional summary ("5 results"); all of it is the trigger's name.
//!
//! [`tool_input`] and [`tool_output`] show the arguments and the result in a
//! [`code_block`]; a failed call shows its error instead.
//!
//! ```ignore
//! view! {
//!     tool(
//!         tool_header(name: "search_documents", state: ToolState::Completed,
//!             summary: "5 results")
//!         tool_content(
//!             tool_input(input: r#"{ "query": "refund policy" }"#)
//!             tool_output(output: &result_json)
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: input and output are text, which the caller formats
//! (as JSON, say); upstream's `formatToolValue` turns any value into JSON in the
//! browser. A result that is not code is the output's child instead of a React node.
//! There is no `onOpenChange`: listen for the native `toggle` event.
//!
//! [`code_block`]: super::code_block::code_block

use bitop_ui::{
    components::icon::{Icon, icon},
    css::badge as badge_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::code_block::code_block;
use crate::css::tool as css;

/// The state of a tool call. Rendered as `data-state`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolState {
    /// Called, not started.
    #[default]
    Pending,
    Running,
    Completed,
    Error,
}

impl ToolState {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Error => "error",
        }
    }

    /// The base badge's colour for the state; running has its own.
    fn badge(self) -> &'static str {
        match self {
            Self::Pending | Self::Running => "neutral",
            Self::Completed => "success",
            Self::Error => "danger",
        }
    }
}

/// The card: a `<details>`. Put a [`tool_header`] first, then [`tool_content`].
/// `open` opens it; `attrs` are forwarded to the `<details>`.
#[component]
pub async fn tool(
    cx: &Cx,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-tool=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>`: an icon, the name, the state and a summary.
///
/// - `name` is the tool's name, shown as code; `title` replaces it with words
///   ("Searched the knowledge base").
/// - `state` picks the badge; `state_label` replaces its word. `pending_label`,
///   `running_label`, `completed_label` and `error_label` are the words.
/// - `summary` is short text after the badge, such as "5 results".
/// - `icon_name` replaces the wrench.
#[component]
pub async fn tool_header(
    cx: &Cx,
    name: &str,
    state: ToolState,
    #[default] title: &str,
    #[default] summary: &str,
    #[default] state_label: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default("Pending")] pending_label: &str,
    #[default("Running")] running_label: &str,
    #[default("Completed")] completed_label: &str,
    #[default("Error")] error_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let word = if state_label.is_empty() {
        match state {
            ToolState::Pending => pending_label,
            ToolState::Running => running_label,
            ToolState::Completed => completed_label,
            ToolState::Error => error_label,
        }
    } else {
        state_label
    };
    Ok(view! {
        <summary
            data-state=(state.as_str())
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::ICON)>
                icon(name: icon_name.unwrap_or(Icon::Wrench))
            </span>
            <span class=(css::NAME)>
                if title.is_empty() {
                    <code>(name)</code>
                } else {
                    (title)
                }
            </span>
            " "
            <span
                data-tone=(state.badge())
                data-state=(state.as_str())
                class=(class!(badge_css::ROOT, css::BADGE))
            >
                <span aria-hidden="true" class=(css::DOT)></span>
                (word)
            </span>
            if !summary.is_empty() {
                " "
                <span class=(css::SUMMARY)>(summary)</span>
            }
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
        </summary>
    })
}

/// The panel under the header, for [`tool_input`] and [`tool_output`].
#[component]
pub async fn tool_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The arguments: `input` as code, under `label`. `language` is the code's, `json` by
/// default.
#[component]
pub async fn tool_input(
    cx: &Cx,
    input: &str,
    #[default("json")] language: &str,
    #[default("Parameters")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <section class=(class!(css::SECTION, attrs.remove("class"))) (attrs)>
            <p class=(css::SECTION_LABEL)>(label)</p>
            code_block(code: input, language: language, hide_header: true)
        </section>
    })
}

/// The result.
///
/// - `error_text` shows the error under `error_label`, instead of the result.
/// - Otherwise `output` is shown as code (`language`, `json` by default) under
///   `label`, up to `max_height`.
/// - Without either, `child` is the result, for one that is not code.
#[component]
pub async fn tool_output(
    cx: &Cx,
    #[default] output: &str,
    #[default] error_text: &str,
    #[default("json")] language: &str,
    #[default("Result")] label: &str,
    #[default("Error")] error_label: &str,
    #[default("20rem")] max_height: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let failed = !error_text.is_empty();
    Ok(view! {
        <section
            data-tool-output=(if failed { "error" } else { "" })
            class=(class!(css::SECTION, attrs.remove("class")))
            (attrs)
        >
            <p class=(css::SECTION_LABEL)>(if failed { error_label } else { label })</p>
            if failed {
                <div class=(css::ERROR)>
                    icon(
                        name: Icon::CircleAlert,
                        attrs: attributes! { class=(css::ERROR_ICON) }
                    )
                    <div>(error_text)</div>
                </div>
            } else if !output.is_empty() {
                code_block(
                    code: output,
                    language: language,
                    hide_header: true,
                    max_height: max_height
                )
            } else {
                (child)
            }
        </section>
    })
}
