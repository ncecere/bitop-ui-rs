//! Messages waiting to be sent while the assistant is busy, or an agent's to-do list,
//! in a card that opens and closes.
//!
//! [`queue`] is a native `<details>` and [`queue_header`] its `<summary>`, with the
//! title and a count ("2 items" to assistive technology). Completed items are struck
//! through and "(done)" is read after them. Row actions show on hover and on focus, so
//! the keyboard reaches them, and always on touch screens.
//!
//! A [`queue_item_action`] with a `name` is a submit button that sends `name=value`
//! with the form around it, so that a server can drop or edit the item. Without one it
//! is a plain button for the page's own script.
//!
//! ```ignore
//! view! {
//!     queue(open: true,
//!         queue_header(title: "Queued", count: 2)
//!         queue_content(label: "Queued messages",
//!             queue_item(
//!                 actions: view! {
//!                     queue_item_action(label: "Remove", name: "remove", value: "q-1",
//!                         icon(name: Icon::X))
//!                 },
//!                 "Also compare with last year"
//!             )
//!             queue_item(completed: true, "Read the policy")
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream is Base UI's collapsible; a `<summary>` is
//! announced the same way. There is no `onOpenChange`: listen for the native `toggle`
//! event. `onClick` on an action is the submit button or the page's script, and an
//! action's tooltip needs an ID, as the base set's tooltips do.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::{button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::queue as css;

/// The card: a `<details>`. Put a [`queue_header`] first, then [`queue_content`].
/// `open` opens it; `attrs` are forwarded to the `<details>`.
#[component]
pub async fn queue(
    cx: &Cx,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-queue=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>`: a chevron, `title` and, with `count`, a count. `item_one` and
/// `item_other` are read after the count.
#[component]
pub async fn queue_header(
    cx: &Cx,
    title: &str,
    #[default]
    #[into]
    count: Option<usize>,
    #[default("item")] item_one: &str,
    #[default("items")] item_other: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <summary class=(class!(css::TRIGGER, attrs.remove("class"))) (attrs)>
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
            <span class=(css::TITLE)>(title)</span>
            " "
            if let Some(count) = count {
                <span class=(css::COUNT) data-queue-count="">
                    (count)
                    " "
                    <span class="sr-only">
                        (if count == 1 { item_one } else { item_other })
                    </span>
                </span>
            }
        </summary>
    })
}

/// The items: a `<ul>`, named by `label` when given.
#[component]
pub async fn queue_content(
    cx: &Cx,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            aria-label=((!label.is_empty()).then_some(label))
            class=(class!(css::LIST, attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One item: a marker, the text, an optional description and row actions.
///
/// - `completed` checks the marker, strikes the text through and reads `done_label`
///   after it.
/// - `actions` is a slot for [`queue_item_action`]s.
#[component]
pub async fn queue_item(
    cx: &Cx,
    #[default] description: &str,
    #[default] completed: bool,
    #[default("(done)")] done_label: &str,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-completed=(completed.then_some(""))
            class=(class!(css::ITEM, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::MARKER)>
                if completed {
                    icon(name: Icon::Check)
                }
            </span>
            <span class=(css::TEXT)>
                <span class=(css::LABEL)>
                    (child)
                    if completed {
                        " "
                        <span class="sr-only">(done_label)</span>
                    }
                </span>
                if !description.is_empty() {
                    <span class=(css::DESCRIPTION)>(description)</span>
                }
            </span>
            <span class=(css::ACTIONS)>(actions)</span>
        </li>
    })
}

/// A row action: an icon button named by `label`. `child` is the icon.
///
/// - `tooltip` is an ID for a tooltip with the label.
/// - `name` and `value` make it a submit button.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn queue_item_action(
    cx: &Cx,
    label: &str,
    #[default] tooltip: &str,
    #[default] name: &str,
    #[default] value: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
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
            data-queue-action=""
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
                (label)
            </span>
        }
    })
}
