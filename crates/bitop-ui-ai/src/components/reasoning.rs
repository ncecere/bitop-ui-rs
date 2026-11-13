//! The model's thinking, in a panel that opens while it streams and closes after.
//!
//! [`reasoning`] is a native `<details>` and [`reasoning_trigger`] its `<summary>`, so the
//! browser opens and closes it on click, Enter and Space. The trigger reads
//! "Thinking…", shimmering, while the thinking streams, then "Thought for N seconds".
//!
//! The library script follows the root's `data-streaming` attribute, which an app
//! that streams HTML into the page sets and removes:
//!
//! - When it is set, the panel opens.
//! - When it is removed, the trigger says how long the thinking took (measured, or
//!   `duration`), and the panel closes once, `auto_close_delay` milliseconds later.
//! - Once the reader has opened or closed the panel, it never moves on its own again.
//! - If focus is inside the panel when it closes on its own, focus moves to the
//!   trigger instead of being lost.
//!
//! A server that renders the finished state at once passes `streaming: false`; the
//! panel is then closed unless `open` is set.
//!
//! ```ignore
//! view! {
//!     reasoning(streaming: true,
//!         reasoning_trigger(streaming: true)
//!         reasoning_content(response(source: &thinking, streaming: true))
//!     )
//! }
//! ```
//!
//! Differences from upstream: there is no `onOpenChange`; the native `toggle` event
//! says the same. The trigger takes `streaming` and `duration` too, for the label it
//! renders before the script runs. `getLabel` is three strings.

use bitop_ui::components::{
    icon::{Icon, icon},
    shimmer::shimmer,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::reasoning as css;

/// The trigger's text when not streaming: `done_one` or `done_other` with `{n}`
/// replaced by the seconds, or `done_unknown` without a duration.
///
/// ```
/// use bitop_ui_ai::components::reasoning::reasoning_label;
///
/// let label = |d| reasoning_label(d, "Thought for {n} second", "Thought for {n} seconds",
///     "Thought for a few seconds");
/// assert_eq!(label(Some(1)), "Thought for 1 second");
/// assert_eq!(label(Some(12)), "Thought for 12 seconds");
/// assert_eq!(label(None), "Thought for a few seconds");
/// ```
#[must_use]
pub fn reasoning_label(
    duration: Option<u32>,
    done_one: &str,
    done_other: &str,
    done_unknown: &str,
) -> String {
    match duration {
        Some(1) => done_one.replace("{n}", "1"),
        Some(n) => done_other.replace("{n}", &n.to_string()),
        None => done_unknown.to_owned(),
    }
}

/// The panel: a `<details>`. Put a [`reasoning_trigger`] first, then
/// [`reasoning_content`].
///
/// - `streaming` says the thinking is still arriving: the panel is open.
/// - `open` opens it when not streaming.
/// - `duration` is the thinking time in seconds, when it is known.
/// - `auto_close` closes the panel once streaming ends, after `auto_close_delay`
///   milliseconds.
/// - `attrs` are forwarded to the `<details>`.
#[component]
pub async fn reasoning(
    cx: &Cx,
    #[default] streaming: bool,
    #[default] open: bool,
    #[default]
    #[into]
    duration: Option<u32>,
    #[default(true)] auto_close: bool,
    #[default(1000)] auto_close_delay: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open || streaming)
            data-reasoning=""
            data-streaming=(streaming.then_some(""))
            data-duration=(duration)
            data-auto-close=(auto_close.then(|| auto_close_delay.to_string()))
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>` that opens and closes the panel.
///
/// - `streaming` and `duration` are the same as the panel's, for the text rendered on
///   the server; the script keeps it up to date after that.
/// - `thinking_label` is shown while streaming; `done_one`, `done_other` (with `{n}`)
///   and `done_unknown` after, as [`reasoning_label`] says.
/// - `label` replaces the text in every state.
#[component]
pub async fn reasoning_trigger(
    cx: &Cx,
    #[default] streaming: bool,
    #[default]
    #[into]
    duration: Option<u32>,
    #[default] label: &str,
    #[default("Thinking…")] thinking_label: &str,
    #[default("Thought for {n} second")] done_one: &str,
    #[default("Thought for {n} seconds")] done_other: &str,
    #[default("Thought for a few seconds")] done_unknown: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let text = if !label.is_empty() {
        label.to_owned()
    } else if streaming {
        thinking_label.to_owned()
    } else {
        reasoning_label(duration, done_one, done_other, done_unknown)
    };
    let fixed = !label.is_empty();
    Ok(view! {
        <summary class=(class!(css::TRIGGER, attrs.remove("class"))) (attrs)>
            icon(name: Icon::Brain, attrs: attributes! { class=(css::ICON) })
            <span
                class=(css::LABEL)
                data-reasoning-label=(if fixed { "fixed" } else { "" })
                data-thinking=(thinking_label)
                data-done-one=(done_one)
                data-done-other=(done_other)
                data-done-unknown=(done_unknown)
            >
                shimmer(active: streaming && !fixed, (text))
            </span>
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
        </summary>
    })
}

/// The thinking text, muted and set off by a line. `attrs` are forwarded to the
/// panel.
#[component]
pub async fn reasoning_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::PANEL, attrs.remove("class")))
            data-reasoning-panel=""
            (attrs)
        >
            <div class=(css::CONTENT)>(child)</div>
        </div>
    })
}
