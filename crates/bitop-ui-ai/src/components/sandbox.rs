//! Code the assistant ran in a sandbox: a card that opens to show the code and its
//! output in tabs.
//!
//! [`sandbox`] is a native `<details>`, open by default, and [`sandbox_header`] its
//! `<summary>`, so the browser opens and closes it on click, Enter and Space. The
//! header shows the title, the state as a badge with a word ("Running"), never by
//! colour alone, and an optional summary ("exit code 1"); all of it is the trigger's
//! name. The tabs are the base set's [`tabs`], so the arrow keys, Home and End move
//! between them; the code is a [`code_block`] with a copy button.
//!
//! ```ignore
//! view! {
//!     sandbox(
//!         sandbox_header(title: "analyze.py", state: SandboxState::Completed,
//!             summary: "1.2 s")
//!         sandbox_content(
//!             sandbox_tabs(
//!                 sandbox_tabs_list(label: "Sandbox",
//!                     sandbox_tab(panel: "run-code", selected: true, "Code")
//!                     sandbox_tab(panel: "run-output", "Output"))
//!                 sandbox_tab_panel(id: "run-code", selected: true,
//!                     sandbox_code(code: SOURCE, language: "python"))
//!                 sandbox_tab_panel(id: "run-output", sandbox_output(output: &stdout))
//!             )
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: tabs are paired with their panels by the panel's ID, as
//! the base tabs are, rather than by a `value`. There is no `onOpenChange`: listen for
//! the native `toggle` event. The error and the summary are text.
//!
//! [`tabs`]: bitop_ui::components::tabs::tabs
//! [`code_block`]: super::code_block::code_block

use bitop_ui::{
    components::{
        icon::{Icon, icon},
        tabs::{tab, tab_list, tab_panel, tabs},
    },
    css::badge as badge_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::code_block::code_block;
use crate::css::{sandbox as css, tool as tool_css};

/// The state of a run. Rendered as `data-state`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SandboxState {
    /// Waiting to run.
    #[default]
    Pending,
    Running,
    Completed,
    Error,
}

impl SandboxState {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Error => "error",
        }
    }

    /// The base badge's colour; running has its own, as in the tool card.
    fn badge(self) -> &'static str {
        match self {
            Self::Pending | Self::Running => "neutral",
            Self::Completed => "success",
            Self::Error => "danger",
        }
    }
}

/// The card: a `<details>`, open unless `open` is `false`. Put a [`sandbox_header`]
/// first, then [`sandbox_content`].
#[component]
pub async fn sandbox(
    cx: &Cx,
    #[default(true)] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-sandbox=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>`: an icon, the title, the state and a summary.
///
/// - `title` is the file's name or "Python sandbox".
/// - `state` picks the badge; `state_label` replaces its word. `pending_label`,
///   `running_label`, `completed_label` and `error_label` are the words.
/// - `summary` is short text after the badge, such as "exit code 1" or "2.3 s".
/// - `icon_name` replaces the terminal icon.
#[component]
pub async fn sandbox_header(
    cx: &Cx,
    title: &str,
    state: SandboxState,
    #[default] state_label: &str,
    #[default] summary: &str,
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
            SandboxState::Pending => pending_label,
            SandboxState::Running => running_label,
            SandboxState::Completed => completed_label,
            SandboxState::Error => error_label,
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
                icon(name: icon_name.unwrap_or(Icon::SquareTerminal))
            </span>
            <span class=(css::TITLE)>(title)</span>
            " "
            <span
                data-tone=(state.badge())
                data-state=(state.as_str())
                class=(class!(badge_css::ROOT, tool_css::BADGE, css::BADGE))
            >
                <span aria-hidden="true" class=(tool_css::DOT)></span>
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

/// The panel under the header, for the [`sandbox_tabs`].
#[component]
pub async fn sandbox_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The base [`tabs`](bitop_ui::components::tabs::tabs), without their gap.
#[component]
pub async fn sandbox_tabs(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        tabs(
            attrs: attributes! { class=(class!(css::TABS, attrs.remove("class"))) (attrs) },
            (child)
        )
    })
}

/// The bar of tabs, named `label`, with `actions` (such as a "Run again" button) at
/// its end.
#[component]
pub async fn sandbox_tabs_list(
    cx: &Cx,
    #[default("Sandbox")] label: &str,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(css::BAR)>
            tab_list(
                label: label,
                attrs: attributes! { class=(class!(css::LIST, attrs.remove("class"))) (attrs) },
                (child)
            )
            <div class=(css::ACTIONS)>(actions)</div>
        </div>
    })
}

/// A tab that shows the [`sandbox_tab_panel`] whose ID is `panel`.
#[component]
pub async fn sandbox_tab(
    cx: &Cx,
    panel: &str,
    #[default] selected: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        tab(
            panel: panel,
            selected: selected,
            attrs: attributes! { class=(class!(css::TAB, attrs.remove("class"))) (attrs) },
            (child)
        )
    })
}

/// The content of one tab, hidden unless `selected`.
#[component]
pub async fn sandbox_tab_panel(
    cx: &Cx,
    id: &str,
    #[default] selected: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        tab_panel(
            id: id,
            selected: selected,
            attrs: attributes! {
                class=(class!(css::TAB_PANEL, attrs.remove("class")))
                (attrs)
            },
            (child)
        )
    })
}

/// The code that ran: a [`code_block`] with a copy button, up to `max_height`.
#[component]
pub async fn sandbox_code(
    cx: &Cx,
    code: &str,
    #[default] language: &str,
    #[default] filename: &str,
    #[default] show_line_numbers: bool,
    #[default("24rem")] max_height: &str,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        code_block(
            code: code,
            language: language,
            filename: filename,
            show_line_numbers: show_line_numbers,
            max_height: max_height,
            attrs: attrs
        )
    })
}

/// What the program printed, and an error.
///
/// - `output` is shown as text in a [`code_block`] named `output_label`, whose copy
///   button says what it copies with `copy_label` ("Copy output").
/// - `error_text` is shown as an error, read after `error_label`.
/// - Without either, `empty_text` says there is nothing yet.
#[component]
pub async fn sandbox_output(
    cx: &Cx,
    #[default] output: &str,
    #[default] error_text: &str,
    #[default("No output")] empty_text: &str,
    #[default("Output")] output_label: &str,
    #[default("Error: ")] error_label: &str,
    #[default("output")] copy_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let has_output = !output.is_empty();
    let has_error = !error_text.is_empty();
    Ok(view! {
        <div
            class=(class!(css::OUTPUT, attrs.remove("class")))
            data-sandbox-output=""
            (attrs)
        >
            if has_output {
                code_block(
                    code: output,
                    language: "text",
                    filename: output_label,
                    copy_label: copy_label,
                    max_height: "24rem"
                )
            }
            if has_error {
                <div dir="ltr" class=(css::ERROR)>
                    icon(
                        name: Icon::CircleAlert,
                        attrs: attributes! { class=(css::ERROR_ICON) }
                    )
                    <div>
                        <span class="sr-only">(error_label)</span>
                        (error_text)
                    </div>
                </div>
            }
            if !has_output && !has_error {
                <p class=(css::EMPTY)>(empty_text)</p>
            }
        </div>
    })
}
