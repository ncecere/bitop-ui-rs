//! An agent's visible steps ("Searched the handbook", "Read 3 documents", "Drafting
//! the answer") as a timeline that opens and closes.
//!
//! [`chain_of_thought`] is a native `<details>` and [`chain_of_thought_header`] its
//! `<summary>`, so the browser opens and closes it on click, Enter and Space. The steps
//! are an ordered list; each step's status is read after its label ("(complete)", "(in
//! progress)", "(not started)"), the active step is `aria-current` and its label
//! shimmers.
//!
//! ```ignore
//! view! {
//!     chain_of_thought(open: true,
//!         chain_of_thought_header()
//!         chain_of_thought_content(
//!             chain_of_thought_step(label: "Searched the handbook",
//!                 chain_of_thought_search_results(
//!                     chain_of_thought_search_result("handbook.pdf")
//!                 )
//!             )
//!             chain_of_thought_step(label: "Drafting the answer",
//!                 status: StepStatus::Active)
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream is Base UI's collapsible; a `<summary>` is
//! announced the same way. There is no `onOpenChange`: listen for the native `toggle`
//! event. Labels and descriptions are text, and icons are from the icon set.

use bitop_ui::components::{
    icon::{Icon, icon},
    shimmer::shimmer,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::chain_of_thought as css;

/// The state of a [`chain_of_thought_step`]. Rendered as `data-status`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepStatus {
    #[default]
    Complete,
    Active,
    Pending,
}

impl StepStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Active => "active",
            Self::Pending => "pending",
        }
    }
}

/// The timeline: a `<details>`. Put a [`chain_of_thought_header`] first, then
/// [`chain_of_thought_content`]. `open` opens it; `attrs` are forwarded to the
/// `<details>`.
#[component]
pub async fn chain_of_thought(
    cx: &Cx,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-chain-of-thought=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>`: an icon, `label` and a chevron. `icon_name` replaces the tree icon.
#[component]
pub async fn chain_of_thought_header(
    cx: &Cx,
    #[default("Chain of thought")] label: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <summary class=(class!(css::TRIGGER, attrs.remove("class"))) (attrs)>
            <span aria-hidden="true" class=(css::TRIGGER_ICON)>
                icon(name: icon_name.unwrap_or(Icon::ListTree))
            </span>
            <span>(label)</span>
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
        </summary>
    })
}

/// The steps: an `<ol>` named by `label`. `attrs` are forwarded to the panel.
#[component]
pub async fn chain_of_thought_content(
    cx: &Cx,
    #[default("Steps")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::PANEL, attrs.remove("class"))) (attrs)>
            <ol aria-label=(label) class=(css::STEPS)>(child)</ol>
        </div>
    })
}

/// One step: a marker, the label, an optional description, and `child` under them,
/// such as search results.
///
/// - `status` sets the marker and what is read after the label: `complete_label`,
///   `active_label` or `pending_label`.
/// - `icon_name` replaces the marker's icon.
#[component]
pub async fn chain_of_thought_step(
    cx: &Cx,
    label: &str,
    #[default] description: &str,
    #[default] status: StepStatus,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default("complete")] complete_label: &str,
    #[default("in progress")] active_label: &str,
    #[default("not started")] pending_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let said = match status {
        StepStatus::Complete => complete_label,
        StepStatus::Active => active_label,
        StepStatus::Pending => pending_label,
    };
    let active = status == StepStatus::Active;
    Ok(view! {
        <li
            data-status=(status.as_str())
            aria-current=(active.then_some("step"))
            class=(class!(css::STEP, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::MARKER)>
                if let Some(name) = icon_name {
                    icon(name: name)
                } else if status == StepStatus::Complete {
                    icon(name: Icon::Check)
                } else if active {
                    <span class=(css::DOT)></span>
                } else {
                    icon(name: Icon::Circle)
                }
            </span>
            <div class=(css::BODY)>
                <span class=(css::LABEL)>
                    if active {
                        shimmer((label))
                    } else {
                        (label)
                    }
                    " "
                    <span class="sr-only">
                        "("
                        (said)
                        ")"
                    </span>
                </span>
                if !description.is_empty() {
                    <span class=(css::DESCRIPTION)>(description)</span>
                }
                (child)
            </div>
        </li>
    })
}

/// A row of [`chain_of_thought_search_result`] chips.
#[component]
pub async fn chain_of_thought_search_results(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::RESULTS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// One result, such as a file name or a site, as a chip.
#[component]
pub async fn chain_of_thought_search_result(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::RESULT, attrs.remove("class"))) (attrs)>(child)</span>
    })
}
