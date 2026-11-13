//! Checkbox, checkbox group and switch, ported from bitop-ui's `Checkbox`,
//! `CheckboxGroup` and `Switch`, all on `<input type="checkbox">`.
//!
//! The browser supplies the state, keyboard handling and form submission. Each
//! control comes with its label (`child`) and an optional description under it; the
//! whole option is the `<label>`, so a click anywhere on it toggles the control.
//! [`checkbox_group`] is a `<fieldset>` with a legend, a description and an error.
//!
//! `indeterminate` is a property of the input, not an attribute: the server marks the
//! checkbox with `data-indeterminate`, which draws the dash at once, and the library
//! script sets the property, so that assistive technology reads it as mixed. A click
//! clears it, as in the browser.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     checkbox_group(id: &id, legend: "Scopes", description: "What this token can do.",
//!         checkbox(name: "scope", value: "read", checked: true,
//!             description: "List projects and deployments.", "Read")
//!         checkbox(name: "scope", value: "admin", disabled: true, "Admin")
//!     )
//!     switch(name: "previews", description: "Build every pull request.", checked: true,
//!         "Preview deployments")
//!     switch(label_position: SwitchLabelPosition::Start, "Settings-row layout")
//! }
//! ```
//!
//! Differences from upstream: the description is inside the option's `<label>`, so it
//! is read as part of the name rather than through `aria-describedby`, as
//! [`radio`](super::radio_group::radio) does. `onCheckedChange` and
//! `onValueChange` are the native `change` event; values are initial values.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::checkbox as css;

/// The class of a bare checkbox's box, for an `<input type="checkbox">` that has its
/// own label elsewhere, such as a table's select-all cell.
pub const CHECKBOX: &str = css::CHECKBOX;

/// A checkbox with its label (`child`).
///
/// - `description` is help text under the label.
/// - `name`, `value`, `checked` and `disabled` go on the `<input>`; an empty `name` or
///   `value` is left out (the browser then submits `on`).
/// - `indeterminate` shows the mixed state (a dash) until the checkbox is clicked.
/// - `attrs` are forwarded to the `<input>`, for `aria-invalid` or `form`.
#[component]
pub async fn checkbox(
    cx: &Cx,
    #[default] name: &str,
    #[default] value: &str,
    #[default] checked: bool,
    #[default] indeterminate: bool,
    #[default] disabled: bool,
    #[default] description: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label class=(css::ITEM)>
            <span class=(css::LABEL)>
                <input
                    type="checkbox"
                    name=((!name.is_empty()).then_some(name))
                    value=((!value.is_empty()).then_some(value))
                    checked=(checked)
                    disabled=(disabled)
                    data-indeterminate=(indeterminate)
                    class=(class!(css::CHECKBOX, attrs.remove("class")))
                    (attrs)
                >
                <span class=(css::TEXT)>(child)</span>
            </span>
            if !description.is_empty() {
                <span class=(css::DESCRIPTION)>(description)</span>
            }
        </label>
    })
}

/// A group of [`checkbox`]es with a legend.
///
/// - `id` goes on the `<fieldset>`, and is the base for the IDs that connect the
///   description and the error to it. The legend names the group.
/// - `description` is help text under the legend.
/// - `error` is a validation message, shown with an icon; when it is set the boxes
///   are drawn in the danger colour.
/// - `horizontal` lays the options out in a row.
/// - `attrs` are forwarded to the `<fieldset>`; `disabled` there disables every box.
#[component]
pub async fn checkbox_group(
    cx: &Cx,
    id: &str,
    legend: &str,
    #[default] description: &str,
    #[default] error: &str,
    #[default] horizontal: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let description_id = format!("{id}-description");
    let error_id = format!("{id}-error");
    let described_by = match (description.is_empty(), error.is_empty()) {
        (true, true) => None,
        (false, true) => Some(description_id.clone()),
        (true, false) => Some(error_id.clone()),
        (false, false) => Some(format!("{description_id} {error_id}")),
    };
    Ok(view! {
        <div class=(css::GROUP) data-invalid=(!error.is_empty())>
            <fieldset
                id=(id)
                aria-describedby=(described_by)
                class=(class!(css::FIELDSET, attrs.remove("class")))
                (attrs)
            >
                <legend class=(css::LEGEND)>(legend)</legend>
                if !description.is_empty() {
                    <p id=(&description_id) class=(css::GROUP_DESCRIPTION)>
                        (description)
                    </p>
                }
                <div
                    class=(css::OPTIONS)
                    data-orientation=(if horizontal { "horizontal" } else { "vertical" })
                >
                    (child)
                </div>
            </fieldset>
            if !error.is_empty() {
                <p id=(&error_id) class=(css::ERROR)>
                    icon(
                        name: Icon::CircleAlert,
                        attrs: attributes! { class=(css::ERROR_ICON) }
                    )
                    <span>(error)</span>
                </p>
            }
        </div>
    })
}

/// Where a [`switch`]'s label sits. Rendered as `data-label-position`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwitchLabelPosition {
    /// The label first and the switch at the end of the row: a settings row.
    Start,
    /// The switch first, then the label.
    #[default]
    End,
}

impl SwitchLabelPosition {
    /// The value of `data-label-position`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// A switch, for a setting that applies as soon as it is toggled, with its label
/// (`child`).
///
/// - `description` is help text under the label.
/// - `label_position: SwitchLabelPosition::Start` puts the label first and the
///   switch at the end of the row.
/// - `name`, `value`, `checked` and `disabled` go on the `<input>`.
/// - `attrs` are forwarded to the `<input>`.
#[component]
pub async fn switch(
    cx: &Cx,
    #[default] name: &str,
    #[default] value: &str,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] description: &str,
    #[default] label_position: SwitchLabelPosition,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            class=(css::ITEM)
            data-switch=""
            data-label-position=(label_position.as_str())
        >
            <span class=(css::LABEL)>
                <input
                    type="checkbox"
                    role="switch"
                    name=((!name.is_empty()).then_some(name))
                    value=((!value.is_empty()).then_some(value))
                    checked=(checked)
                    disabled=(disabled)
                    class=(class!(css::SWITCH, attrs.remove("class")))
                    (attrs)
                >
                <span class=(css::TEXT)>(child)</span>
            </span>
            if !description.is_empty() {
                <span class=(css::DESCRIPTION)>(description)</span>
            }
        </label>
    })
}
