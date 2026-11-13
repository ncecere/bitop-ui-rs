//! A card with the steps an agent proposes, which can stream in and be collapsed.
//!
//! The card is a `<section>` named by `label`, usually its title. The toggle in the header is a button
//! with `aria-expanded` that shows and hides the list of steps; each step's status is
//! read after it ("(done)", "(in progress)", "(to do)"), and the active step is the
//! list's `aria-current` step.
//!
//! While `streaming`, the title and the description shimmer and the card is
//! `aria-busy`. The library script follows the root's `data-streaming` attribute, which
//! an app that streams HTML into the page sets and removes.
//!
//! The parts refer to each other by ID: pass the same `id` to [`plan`],
//! [`plan_header`] and [`plan_content`].
//!
//! ```ignore
//! view! {
//!     plan(id: "migration", label: "Migrate the billing service", streaming: true,
//!         plan_header(id: "migration", streaming: true,
//!             title: "Migrate the billing service", description: "4 steps")
//!         plan_content(id: "migration",
//!             plan_step(status: PlanStepStatus::Complete, "Snapshot the database")
//!             plan_step(status: PlanStepStatus::Active, "Run the migration")
//!         )
//!         plan_footer(button("Approve plan"))
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream is Base UI's collapsible; here the toggle needs
//! the library script, and without it the steps stay as rendered. There is no
//! `onOpenChange`: a `bitop:plan-toggle` event goes out from the card with
//! `detail.open`. The title and description are text. The card is named with
//! `aria-label` rather than by its title's ID, as Playwright's Firefox crashes on a
//! region named by an element inside it.

use bitop_ui::components::{
    icon::{Icon, icon},
    shimmer::shimmer,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::plan as css;

/// The state of a [`plan_step`]. Rendered as `data-status`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlanStepStatus {
    Complete,
    Active,
    #[default]
    Pending,
}

impl PlanStepStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Active => "active",
            Self::Pending => "pending",
        }
    }
}

/// The card: a `<section>` named by `label`, usually the header's title.
///
/// - `id` is shared with the header and the content.
/// - `streaming` makes it `aria-busy`; `open` (the default) shows the steps.
/// - `attrs` are forwarded to the `<section>`.
#[component]
pub async fn plan(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] streaming: bool,
    #[default(true)] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section
            id=(id)
            aria-label=(label)
            aria-busy=(streaming.then_some("true"))
            data-plan=""
            data-streaming=(streaming.then_some(""))
            data-open=(open.then_some(""))
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </section>
    })
}

/// The title, a description, actions and the toggle.
///
/// - `id` is the card's, for the panel the toggle controls; `open` and `streaming`
///   the same as the card's.
/// - `actions` is a slot for buttons before the toggle.
/// - `toggle_label` names the toggle.
#[component]
pub async fn plan_header(
    cx: &Cx,
    id: &str,
    title: &str,
    #[default] description: &str,
    #[default] streaming: bool,
    #[default(true)] open: bool,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default("Show steps")] toggle_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-plan-header=""
            class=(class!(css::HEADER, attrs.remove("class")))
            (attrs)
        >
            <div class=(css::HEADINGS)>
                <p class=(css::TITLE)>shimmer(active: streaming, (title))</p>
                if !description.is_empty() {
                    <p class=(css::DESCRIPTION)>
                        shimmer(active: streaming, (description))
                    </p>
                }
            </div>
            <div class=(css::ACTIONS)>
                (actions)
                <button
                    type="button"
                    aria-label=(toggle_label)
                    aria-expanded=(if open { "true" } else { "false" })
                    aria-controls=(format!("{id}-steps"))
                    data-plan-toggle=""
                    class=(css::TOGGLE)
                >
                    icon(name: Icon::ChevronsUpDown)
                </button>
            </div>
        </div>
    })
}

/// The steps: an `<ol>` named by `label`, in the panel the toggle shows and hides.
/// `id` is the card's; `open` the same as the card's.
#[component]
pub async fn plan_content(
    cx: &Cx,
    id: &str,
    #[default(true)] open: bool,
    #[default("Steps")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(format!("{id}-steps"))
            hidden=(!open)
            class=(class!(css::PANEL, attrs.remove("class")))
            (attrs)
        >
            <ol aria-label=(label) class=(css::STEPS)>(child)</ol>
        </div>
    })
}

/// One step. `complete_label`, `active_label` and `pending_label` are read after it.
#[component]
pub async fn plan_step(
    cx: &Cx,
    #[default] status: PlanStepStatus,
    #[default("done")] complete_label: &str,
    #[default("in progress")] active_label: &str,
    #[default("to do")] pending_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let said = match status {
        PlanStepStatus::Complete => complete_label,
        PlanStepStatus::Active => active_label,
        PlanStepStatus::Pending => pending_label,
    };
    Ok(view! {
        <li
            data-status=(status.as_str())
            aria-current=((status == PlanStepStatus::Active).then_some("step"))
            class=(class!(css::STEP, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::MARKER)>
                match status {
                    PlanStepStatus::Complete => icon(name: Icon::Check),
                    PlanStepStatus::Active => <span class=(css::DOT)></span>,
                    PlanStepStatus::Pending => icon(name: Icon::Circle),
                }
            </span>
            <span class=(css::STEP_TEXT)>
                (child)
                " "
                <span class="sr-only">
                    "("
                    (said)
                    ")"
                </span>
            </span>
        </li>
    })
}

/// Actions under the steps, such as "Approve plan". Hidden while empty.
#[component]
pub async fn plan_footer(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::FOOTER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}
