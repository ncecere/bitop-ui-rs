//! A labelled group of radio buttons, on `<fieldset>` and `<input type="radio">`.
//!
//! The browser supplies the state and form submission, and the group is one tab
//! stop. Give every [`radio`] in a group the same `name`. The library script handles
//! the arrow keys, which move between the radios and select them, so that they wrap
//! at the ends and follow the visual order in a right-to-left page in every browser.
//!
//! Upstream takes the options as a list. Here each option is a [`radio`] part, so
//! that its label can be any markup.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     radio_group(id: &id, legend: "Plan",
//!         radio(name: "plan", value: "free", checked: true, "Free")
//!         radio(name: "plan", value: "team", description: "For up to 20 people.", "Team")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::radio_group as css;

/// How the options of a [`radio_group`] look. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioGroupVariant {
    /// A radio button with its label beside it.
    #[default]
    Default,
    /// Each option is a tile that can be selected anywhere on its surface.
    Card,
}

impl RadioGroupVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Card => "card",
        }
    }
}

/// A group of [`radio`]s with a legend.
///
/// - `id` goes on the `<fieldset>`, and is the base for the IDs that connect the
///   description and the error to it. The legend names the group.
/// - `description` is help text under the legend.
/// - `error` is a validation message; when it is set the group is marked invalid.
/// - `horizontal` lays the options out in a row.
/// - `attrs` are forwarded to the `<fieldset>`; `disabled` there disables every radio.
#[component]
pub async fn radio_group(
    cx: &Cx,
    id: &str,
    legend: &str,
    #[default] description: &str,
    #[default] error: &str,
    #[default] horizontal: bool,
    #[default] variant: RadioGroupVariant,
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
        <div class=(css::GROUP)>
            <fieldset
                id=(id)
                role="radiogroup"
                aria-describedby=(described_by)
                aria-invalid=((!error.is_empty()).then_some("true"))
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
                    data-variant=(variant.as_str())
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

/// One option in a [`radio_group`]. `child` is its label.
///
/// `description` is help text under the label; it is part of the option's label, so
/// it is read out with it. `attrs` are forwarded to the `<input>`.
#[component]
pub async fn radio(
    cx: &Cx,
    name: &str,
    value: &str,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] description: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label class=(css::ITEM)>
            <span class=(css::LABEL)>
                <input
                    type="radio"
                    name=(name)
                    value=(value)
                    checked=(checked)
                    disabled=(disabled)
                    class=(class!(css::RADIO, attrs.remove("class")))
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
