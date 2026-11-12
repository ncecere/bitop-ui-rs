//! A text input that filters a list of options, for choosing one value from many.
//!
//! The server renders every option. The library script filters them as the person
//! types and adds the ARIA combobox keyboard pattern: arrow keys move through the
//! options, Enter chooses, Escape closes. The chosen option's value is submitted with
//! the form under `name`, through a hidden input.
//!
//! For a short list that needs no filtering, use [`select`](super::select::select).
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     field(
//!         field_label(target: &id, "Model")
//!         combobox(id: &id, name: "model", label: "Models", placeholder: "Choose a model",
//!             combobox_option(value: "a", "Model A")
//!             combobox_option(value: "b", "Model B")
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::{combobox as css, input as input_css, popover as popover_css};

/// A combobox.
///
/// - `id` goes on the text input, so a label can point at it.
/// - `label` names the list of options for assistive technology.
/// - `value` and `display` are the chosen option's value and text, if one is chosen.
/// - `empty` is shown when nothing matches.
/// - `attrs` are forwarded to the text input.
#[component]
pub async fn combobox(
    cx: &Cx,
    id: &str,
    name: &str,
    label: &str,
    #[default] value: &str,
    #[default] display: &str,
    #[default] placeholder: &str,
    #[default("No results")] empty: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let list = format!("{id}-list");
    Ok(view! {
        <div class=(css::ROOT) data-combobox="">
            <input
                id=(id)
                type="text"
                role="combobox"
                aria-controls=(&list)
                aria-expanded="false"
                aria-autocomplete="list"
                autocomplete="off"
                spellcheck="false"
                value=(display)
                placeholder=(placeholder)
                class=(class!(input_css::ROOT, css::INPUT, attrs.remove("class")))
                (attrs)
            >
            <input type="hidden" name=(name) value=(value)>
            <div popover="manual" class=(class!(popover_css::POPUP, css::POPUP))>
                <div id=(&list) role="listbox" aria-label=(label) class=(css::LIST)>
                    (child)
                </div>
                <div class=(css::EMPTY) data-combobox-empty="" hidden=(true)>
                    (empty)
                </div>
            </div>
        </div>
    })
}

/// One option in a [`combobox`]. `child` is its text.
#[component]
pub async fn combobox_option(
    cx: &Cx,
    value: &str,
    #[default] selected: bool,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="option"
            data-value=(value)
            aria-selected=(if selected { "true" } else { "false" })
            if disabled {
                aria-disabled="true"
            }
            class=(class!(css::OPTION, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
