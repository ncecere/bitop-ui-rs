//! A select for options with icons and hints, ported from bitop-ui's `Select`, and the
//! native select (upstream's `NativeSelect`).
//!
//! [`select`] is a button with the role of a combobox that opens a popover listbox:
//! each option can have an icon and a muted hint, and the chosen one has a check mark.
//! The chosen value is submitted with the form under `name`, through an input beside the
//! button, which also carries `required`. The library script adds the keyboard pattern
//! of a select: ArrowDown, ArrowUp, Enter and Space open it on the chosen option; the
//! arrow keys, Home, End and typeahead move through the options; Enter, Space or a
//! click chooses; Escape and Tab close. Typing on the closed button chooses the option
//! that starts with the letters, as a native select does. Choosing sends a `change`
//! event from the input and `bitop:change` from the root, with the value.
//!
//! [`native_select`] is the `<select>` element, which the browser draws: use it in
//! plain forms, and [`select`] when the options need icons or hints. Use
//! [`combobox`](super::combobox::combobox) when the list needs filtering.
//!
//! ```ignore
//! const MACHINES: [SelectItem; 2] = [
//!     SelectItem::new("standard", "Standard").icon(Icon::Cpu).hint("2 vCPU"),
//!     SelectItem::new("turbo", "Turbo").hint("8 vCPU").disabled(),
//! ];
//! view! {
//!     select(id: "machine", label: "Machine", name: "machine", items: &MACHINES,
//!         value: "standard")
//! }
//! ```
//!
//! Differences from upstream: the button and the list are named with `aria-label` (the
//! label's text), not `aria-labelledby`, for a crash in Playwright's Firefox (see the
//! contributing guide). The popup is a popover anchored below the button, not
//! placed over it; the value is an initial value and `onValueChange` is the `change`
//! event. Without the script the button does not open.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::{popover as popover_css, select as css};

/// One option of a [`select`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectItem<'a> {
    /// What is submitted.
    pub value: &'a str,
    /// What is shown, in the list and on the button.
    pub label: &'a str,
    /// A decorative icon before the label.
    pub icon: Option<Icon>,
    /// Muted text at the end of the row, such as "2 vCPU".
    pub hint: &'a str,
    /// Shown, but cannot be chosen.
    pub disabled: bool,
}

impl<'a> SelectItem<'a> {
    /// An option with a value and a label.
    #[must_use]
    pub const fn new(value: &'a str, label: &'a str) -> Self {
        Self {
            value,
            label,
            icon: None,
            hint: "",
            disabled: false,
        }
    }

    /// Adds an icon.
    #[must_use]
    pub const fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Adds a hint.
    #[must_use]
    pub const fn hint(mut self, hint: &'a str) -> Self {
        self.hint = hint;
        self
    }

    /// Makes it impossible to choose.
    #[must_use]
    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// The height of a [`select`]'s button. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectSize {
    Sm,
    #[default]
    Md,
}

impl SelectSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A select with a styled list.
///
/// - `id` goes on the button; the label, the popup and the list take IDs from it.
/// - `label` is shown above the button and names it; `hide_label` keeps it for
///   assistive technology only.
/// - `value` is the chosen option's value; `placeholder` shows when there is none.
/// - `name`, `required` and `disabled` are for the form.
/// - `attrs` are forwarded to the button, for `aria-describedby` or `aria-invalid`.
#[component]
pub async fn select(
    cx: &Cx,
    id: &str,
    label: &str,
    items: &[SelectItem<'_>],
    #[default] name: &str,
    #[default] value: &str,
    #[default("Select…")] placeholder: &str,
    #[default] hide_label: bool,
    #[default] size: SelectSize,
    #[default] disabled: bool,
    #[default] required: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let label_id = format!("{id}-label");
    let popup_id = format!("{id}-popup");
    let list_id = format!("{id}-list");
    let chosen = items
        .iter()
        .find(|item| item.value == value && !value.is_empty());
    let shown = chosen.map_or(placeholder, |item| item.label).to_owned();
    let value = chosen.map_or("", |item| item.value);
    let options: Vec<(SelectItem<'_>, bool)> = items
        .iter()
        .map(|item| (*item, chosen.is_some_and(|c| c.value == item.value)))
        .collect();
    let trigger_class = class!(css::TRIGGER, attrs.remove("class"));
    Ok(Child::new(view! {
        <div class=(css::ROOT) data-select="">
            <label
                id=(&label_id)
                for=(id)
                class=(if hide_label { "sr-only" } else { css::LABEL })
            >
                (label)
            </label>
            <button
                type="button"
                id=(id)
                role="combobox"
                aria-haspopup="listbox"
                aria-expanded="false"
                aria-controls=(&list_id)
                aria-label=(label)
                popovertarget=(&popup_id)
                data-size=(size.as_str())
                disabled=(disabled)
                class=(trigger_class)
                (attrs)
            >
                <span
                    class=(css::VALUE)
                    data-placeholder=(chosen.is_none().then_some(""))
                >
                    (shown)
                </span>
                icon(
                    name: Icon::ChevronsUpDown,
                    attrs: attributes! { class=(css::ICON) }
                )
            </button>
            if required {
                <input
                    tabindex="-1"
                    aria-hidden="true"
                    name=((!name.is_empty()).then_some(name))
                    value=(value)
                    required=(true)
                    disabled=(disabled)
                    class=(css::INPUT)
                    data-select-value=""
                >
            } else {
                <input
                    type="hidden"
                    name=((!name.is_empty()).then_some(name))
                    value=(value)
                    disabled=(disabled)
                    data-select-value=""
                >
            }
            <div
                id=(&popup_id)
                popover="auto"
                data-width="trigger"
                class=(class!(popover_css::SURFACE, css::POPUP))
                data-select-popup=""
            >
                <div
                    id=(&list_id)
                    role="listbox"
                    tabindex="-1"
                    aria-label=(label)
                    class=(css::LIST)
                >
                    for (item, selected) in &options {
                        <div
                            role="option"
                            tabindex="-1"
                            aria-selected=(if *selected { "true" } else { "false" })
                            aria-disabled=(item.disabled.then_some("true"))
                            data-value=(item.value)
                            data-label=(item.label)
                            class=(class!(popover_css::ITEM, css::ITEM))
                        >
                            if let Some(name) = item.icon {
                                <span class=(css::ITEM_ICON)>icon(name: name)</span>
                            }
                            <span class=(css::ITEM_TEXT)>(item.label)</span>
                            if !item.hint.is_empty() {
                                <span class=(css::HINT)>(item.hint)</span>
                            }
                            <span aria-hidden="true" class=(css::INDICATOR)>
                                icon(name: Icon::Check)
                            </span>
                        </div>
                    }
                </div>
            </div>
        </div>
    }))
}

/// A native `<select>`. `attrs` are forwarded to it; `child` is its `<option>`s.
#[component]
pub async fn native_select(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(css::NATIVE)>
            <select class=(class!(css::CONTROL, attrs.remove("class"))) (attrs)>
                (child)
            </select>
        </span>
    })
}
