//! A panel for something the assistant made (a document, a chart, generated code), with
//! a header, actions and content that scrolls.
//!
//! It is a `<section>` named by `label`, usually the title. The content takes focus, so
//! that keyboard users can scroll it; the section already names it.
//!
//! Actions are icon buttons with tooltips. With `name`, one is a submit button that
//! sends `name=value` with the form around it. Without it, the library script sends a
//! `bitop:artifact-action` event from the artifact, with the action's `id` in
//! `detail.action`; [`artifact_close`] sends `bitop:artifact-close`. A close button in
//! a dialog can close it instead, with `commandfor` and `command="close"` in `attrs`.
//!
//! ```ignore
//! view! {
//!     artifact(label: "Q3 summary",
//!         artifact_header(
//!             <div>
//!                 artifact_title("Q3 summary")
//!                 artifact_description("Markdown · 2 min ago")
//!             </div>
//!             artifact_actions(
//!                 artifact_action(label: "Download", id: "download",
//!                     tooltip: "download-tip", icon(name: Icon::Upload))
//!                 artifact_close(tooltip: "close-tip")
//!             )
//!         )
//!         artifact_content(response(source: &markdown))
//!     )
//! }
//! ```
//!
//! Differences from upstream: `onClick` is the submit button or the events above, and a
//! tooltip needs an ID, as the base set's tooltips do. The section is named with
//! `aria-label` rather than by its title's ID, as Playwright's Firefox crashes on a
//! region named by an element inside it.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::{button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::artifact as css;

/// The panel: a `<section>` named by `label`.
#[component]
pub async fn artifact(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section
            aria-label=(label)
            data-artifact=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </section>
    })
}

/// The header row. Its first child takes the room: put the title and description in
/// one element, then [`artifact_actions`].
#[component]
pub async fn artifact_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The title, cut with an ellipsis when long. `level` (2 to 6) makes it a heading;
/// otherwise it is a `<p>`.
#[component]
pub async fn artifact_title(
    cx: &Cx,
    #[default]
    #[into]
    level: Option<u8>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::TITLE, attrs.remove("class"));
    Ok(view! {
        match level {
            Some(2) => <h2 class=(class) (attrs)>(child)</h2>,
            Some(3) => <h3 class=(class) (attrs)>(child)</h3>,
            Some(4) => <h4 class=(class) (attrs)>(child)</h4>,
            Some(5) => <h5 class=(class) (attrs)>(child)</h5>,
            Some(6) => <h6 class=(class) (attrs)>(child)</h6>,
            _ => <p class=(class) (attrs)>(child)</p>,
        }
    })
}

/// A line under the title, such as the format and when it was made.
#[component]
pub async fn artifact_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::DESCRIPTION, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// The row of [`artifact_action`]s at the end of the header.
#[component]
pub async fn artifact_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// An action: an icon button named by `label`. `child` is the icon.
///
/// - `tooltip` is an ID for a tooltip with `tooltip_text`, or the label.
/// - `name` and `value` make it a submit button; otherwise `id` is sent in the
///   `bitop:artifact-action` event.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn artifact_action(
    cx: &Cx,
    label: &str,
    #[default] id: &str,
    #[default] tooltip: &str,
    #[default] tooltip_text: &str,
    #[default] name: &str,
    #[default] value: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
    let tip = if tooltip_text.is_empty() {
        label
    } else {
        tooltip_text
    };
    let submits = !name.is_empty();
    Ok(view! {
        <button
            type=(if submits { "submit" } else { "button" })
            name=(submits.then_some(name))
            value=(submits.then_some(value))
            aria-label=(label)
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            disabled=(disabled)
            data-artifact-action=(id)
            data-variant="ghost"
            data-size="icon"
            class=(class!(button_css::ROOT, css::ACTION, attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
        if has_tooltip {
            <span
                id=(tooltip)
                role="tooltip"
                popover="manual"
                class=(tooltip_css::POPUP)
            >
                (tip)
            </span>
        }
    })
}

/// A close button: an [`artifact_action`] with an X that sends `bitop:artifact-close`
/// unless `name` makes it a submit button.
#[component]
pub async fn artifact_close(
    cx: &Cx,
    #[default("Close")] label: &str,
    #[default] tooltip: &str,
    #[default] name: &str,
    #[default] value: &str,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        artifact_action(
            label: label,
            id: "close",
            tooltip: tooltip,
            name: name,
            value: value,
            attrs: attrs,
            icon(name: Icon::X)
        )
    })
}

/// The body, which scrolls and takes focus. `flush` removes the padding, for previews
/// and code that run to the edges.
#[component]
pub async fn artifact_content(
    cx: &Cx,
    #[default] flush: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            tabindex="0"
            data-flush=(flush.then_some(""))
            class=(class!(css::CONTENT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
