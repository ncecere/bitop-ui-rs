//! Onboarding steps with done states: "Getting started", an admin setup list.
//!
//! A titled section with a progress summary ("2 of 4 done") and an ordered list of
//! steps. Each step has a title, an optional description and an action. The first
//! open step is the current one: its action is the solid button.
//!
//! ```ignore
//! let steps = [
//!     ChecklistStep::new("Add a data source").done(true),
//!     ChecklistStep::new("Create a knowledge base").action("Create", "/kbs/new"),
//! ];
//! view! { checklist(title: "Get started", steps: &steps, dismissible: true) }
//! ```
//!
//! The done state is shown by an icon and spelled out for assistive technology
//! ("Done:" or "To do:" before the title), not by colour.
//!
//! Differences from upstream: a step's action is a link (a label and a URL), not a
//! click handler or an arbitrary node, and the action of a step that is not current
//! is an outline button, since this library's button has no secondary variant.
//! Upstream's `onDismiss` is `dismissible` here: the library script removes the
//! checklist after sending a cancelable `bitop-dismiss` event from it, which is where
//! an application remembers the choice. The section is named with `aria-label`, not
//! `aria-labelledby` pointing at its own title; the name is the same.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::{
    icon::{Icon, icon},
    progress::{ProgressTone, progress},
};
use crate::css::{button as button_css, checklist as css};

/// One step of a [`checklist`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChecklistStep<'a> {
    /// What to do.
    pub title: &'a str,
    /// A sentence of help under the title.
    pub description: &'a str,
    /// Whether the step is done.
    pub done: bool,
    /// The text of the link that does the step. Hidden once the step is done.
    pub action_label: &'a str,
    /// Where that link goes.
    pub action_href: &'a str,
}

impl<'a> ChecklistStep<'a> {
    /// An open step with no description and no action.
    #[must_use]
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            ..Self::default()
        }
    }

    /// Adds a sentence of help.
    #[must_use]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Marks the step done or open.
    #[must_use]
    pub fn done(mut self, done: bool) -> Self {
        self.done = done;
        self
    }

    /// Adds the link that does the step.
    #[must_use]
    pub fn action(mut self, label: &'a str, href: &'a str) -> Self {
        self.action_label = label;
        self.action_href = href;
        self
    }
}

/// The heading level of a [`checklist`]'s title.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChecklistLevel {
    #[default]
    H2,
    H3,
    H4,
}

/// How many steps are done, and the index of the first open one.
#[must_use]
pub fn checklist_progress(steps: &[ChecklistStep<'_>]) -> (usize, Option<usize>) {
    (
        steps.iter().filter(|step| step.done).count(),
        steps.iter().position(|step| !step.done),
    )
}

/// A checklist.
///
/// - `title` is the heading, and names the section.
/// - `description` is a sentence under the title.
/// - `steps` are the steps, in order.
/// - `level` is the heading level of the title.
/// - `dismissible` adds a button that removes the checklist; see the module
///   documentation. `dismiss_label` is its name, followed by the title.
/// - `complete` is shown instead of the progress summary when every step is done.
/// - `hide_progress` hides the progress bar; the summary text stays.
/// - `summary_label` is the summary, with `{done}` and `{total}` for the counts.
/// - `done_label` and `todo_label` are announced before each step's title.
#[component]
pub async fn checklist(
    cx: &Cx,
    title: &str,
    steps: &[ChecklistStep<'_>],
    #[default] description: &str,
    #[default] level: ChecklistLevel,
    #[default] dismissible: bool,
    #[default] complete: &str,
    #[default] hide_progress: bool,
    #[default("{done} of {total} done")] summary_label: &str,
    #[default("Done")] done_label: &str,
    #[default("To do")] todo_label: &str,
    #[default("Dismiss")] dismiss_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let (done, current) = checklist_progress(steps);
    let total = steps.len();
    let all_done = total > 0 && done == total;
    let summary = summary_label
        .replace("{done}", &done.to_string())
        .replace("{total}", &total.to_string());
    #[allow(clippy::cast_precision_loss)]
    let percent = if total == 0 {
        0.0
    } else {
        done as f64 / total as f64 * 100.0
    };
    let dismiss = format!("{dismiss_label} {title}");
    Ok(view! {
        <section
            aria-label=(title)
            class=(class!(css::ROOT, attrs.remove("class")))
            data-checklist=""
            data-complete=(all_done.then_some(""))
            (attrs)
        >
            <div class=(css::HEADER)>
                <div class=(css::HEADING)>
                    match level {
                        ChecklistLevel::H2 => {
                            <h2 class=(css::TITLE)>(title)</h2>
                        }
                        ChecklistLevel::H3 => {
                            <h3 class=(css::TITLE)>(title)</h3>
                        }
                        ChecklistLevel::H4 => {
                            <h4 class=(css::TITLE)>(title)</h4>
                        }
                    }
                    if !description.is_empty() {
                        <p class=(css::DESCRIPTION)>(description)</p>
                    }
                </div>
                if dismissible {
                    <button
                        type="button"
                        aria-label=(dismiss)
                        class=(class!(button_css::ROOT, css::DISMISS))
                        data-variant="ghost"
                        data-size="icon"
                        data-checklist-dismiss=""
                    >
                        icon(name: Icon::X)
                    </button>
                }
            </div>
            if all_done && !complete.is_empty() {
                <div class=(css::COMPLETE)>(complete)</div>
            } else if hide_progress {
                <p class=(css::SUMMARY)>(summary)</p>
            } else {
                progress(
                    label: &summary,
                    value: Some(percent),
                    show_value: false,
                    small: true,
                    tone: if all_done {
                        ProgressTone::Success
                    } else {
                        ProgressTone::Primary
                    }
                )
            }
            <ol class=(css::STEPS)>
                for (index, step) in steps.iter().enumerate() {
                    <li
                        class=(css::STEP)
                        data-done=(step.done.then_some(""))
                        data-current=((current == Some(index)).then_some(""))
                    >
                        <span aria-hidden="true" class=(css::MARKER)>
                            if step.done {
                                icon(name: Icon::Check)
                            } else {
                                (index + 1)
                            }
                        </span>
                        <div class=(css::BODY)>
                            <p class=(css::STEP_TITLE)>
                                <span class="sr-only">
                                    (if step.done { done_label } else { todo_label })
                                    ": "
                                </span>
                                (step.title)
                            </p>
                            if !step.description.is_empty() {
                                <p class=(css::STEP_DESCRIPTION)>(step.description)</p>
                            }
                        </div>
                        if !step.done && !step.action_label.is_empty() {
                            <div class=(css::ACTION)>
                                <a
                                    href=(step.action_href)
                                    class=(button_css::ROOT)
                                    data-variant=(if current == Some(index) {
                                        "primary"
                                    } else {
                                        "secondary"
                                    })
                                    data-size="sm"
                                >
                                    (step.action_label)
                                </a>
                            </div>
                        }
                    </li>
                }
            </ol>
        </section>
    })
}

#[cfg(test)]
mod tests {
    use super::{ChecklistStep, checklist_progress};

    #[test]
    fn counts_done_steps_and_finds_the_current_one() {
        let steps = [
            ChecklistStep::new("a").done(true),
            ChecklistStep::new("b"),
            ChecklistStep::new("c").done(true),
            ChecklistStep::new("d"),
        ];
        assert_eq!(checklist_progress(&steps), (2, Some(1)));
        assert_eq!(checklist_progress(&steps[..1]), (1, None));
        assert_eq!(checklist_progress(&[]), (0, None));
    }
}
