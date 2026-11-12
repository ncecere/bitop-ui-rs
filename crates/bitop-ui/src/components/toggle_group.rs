//! A row or column of toggles that share their pressed state.
//!
//! One item can be pressed at a time unless `multiple` is set. The group has
//! `role="group"` and must be named with `label`. The library script adds the
//! keyboard pattern: the group is one tab stop, which is the pressed item (the first
//! pressed one with `multiple`), and the arrow keys, Home and End move focus between
//! items. Space and Enter press the focused item.
//!
//! With a `name` on the items, each pressed item submits `name=value` with its form.
//!
//! Differences from upstream: an item takes its variant and size from the group
//! through CSS, not through context. A joined group with `scroll` scrolls sideways
//! when it is too wide, but does not fade the edge where more items are hidden.
//!
//! ```ignore
//! view! {
//!     toggle_group(label: "Text alignment",
//!         toggle_group_item(value: "left", name: "align", pressed: true, "Left")
//!         toggle_group_item(value: "center", name: "align", "Centre")
//!         toggle_group_item(value: "right", name: "align", "Right")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::toggle::{ToggleSize, ToggleVariant};
use crate::css::{toggle as toggle_css, toggle_group as css};

/// A group of [`toggle_group_item`]s. `label` names the group.
///
/// - `variant` and `size` apply to every item.
/// - `multiple` lets several items be pressed at once.
/// - `joined` draws the items as one segmented control.
/// - `scroll` makes a joined group that is too wide scroll sideways instead of
///   wrapping onto more lines.
/// - `vertical` stacks the items; the up and down arrow keys then move between them.
#[component]
pub async fn toggle_group(
    cx: &Cx,
    label: &str,
    #[default] variant: ToggleVariant,
    #[default] size: ToggleSize,
    #[default] multiple: bool,
    #[default] joined: bool,
    #[default] scroll: bool,
    #[default] vertical: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-toggle-group=""
            data-multiple=(multiple)
            data-orientation=(if vertical { "vertical" } else { "horizontal" })
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            data-joined=(joined)
            data-overflow-mode=(joined.then_some(if scroll { "scroll" } else { "wrap" }))
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A toggle in a [`toggle_group`]. `value` identifies it.
///
/// - `pressed` is the initial state.
/// - `icon_only` makes it square, for a single icon; give it an `aria-label` in `attrs`.
/// - `name` makes a pressed item submit `name=value` with its form.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn toggle_group_item(
    cx: &Cx,
    value: &str,
    #[default] pressed: bool,
    #[default] icon_only: bool,
    #[default] disabled: bool,
    #[default] name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // No tabindex here: the script sets it, to make the group one tab stop.
    Ok(view! {
        <button
            type="button"
            aria-pressed=(if pressed { "true" } else { "false" })
            disabled=(disabled)
            data-toggle=""
            data-value=(value)
            data-icon-only=(icon_only)
            class=(class!(toggle_css::TOGGLE, css::ITEM, attrs.remove("class")))
            (attrs)
        >
            (child)
            if !name.is_empty() {
                <input type="hidden" name=(name) value=(value) disabled=(!pressed)>
            }
        </button>
    })
}
