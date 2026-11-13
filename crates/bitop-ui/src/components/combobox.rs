//! A filterable select for long lists of options, ported from bitop-ui's `Combobox`.
//!
//! The server renders every option. The library script filters them by their label as
//! the person types (ignoring case and accents) and adds the ARIA combobox keyboard
//! pattern: ArrowDown and ArrowUp move through the options and, past either end, back
//! to the text; Enter chooses; Escape closes. Enter never submits the form around it,
//! unless `submit_on_enter` is set and no option is highlighted. Options that share a
//! `group` are listed under its heading; an option can have an icon and a muted hint.
//!
//! - One value: the input shows the chosen option's label, and its value is submitted
//!   under `name` through a hidden input. Focusing the input selects its text, so that
//!   typing searches afresh.
//! - `multiple`: the chosen options are chips inside the control, each with a remove
//!   button; each submits `name=value`. Backspace in the empty input removes the last
//!   chip; ArrowLeft at the start of the input moves onto the chips, where Backspace or
//!   Delete removes the one with focus. The list stays open while choosing.
//! - `free_text`: any text is the value, submitted under `name` by the input itself,
//!   and the options are suggestions; choosing one fills in its value.
//!
//! `clearable` adds a button that clears the choice, shown while there is one.
//! `auto_highlight` highlights the first match while typing, so that Enter picks it.
//! `limit` shows at most that many matches at once. Choosing sends a `change` event
//! from the hidden input (or the text input with `free_text`) and `bitop:change` from
//! the root, whose `detail.value` is the value, or the list of values.
//!
//! For a short list that needs no filtering, use [`select`](super::select::select).
//!
//! ```ignore
//! const ZONES: [ComboboxOption; 2] = [
//!     ComboboxOption::new("Europe/Berlin", "Berlin").hint("UTC+1"),
//!     ComboboxOption::new("Asia/Tokyo", "Tokyo").hint("UTC+9"),
//! ];
//! let id = element_id(cx);
//! view! {
//!     field(
//!         field_label(target: &id, "Time zone")
//!         combobox(id: &id, name: "zone", items: &ZONES, value: "Europe/Berlin",
//!             placeholder: "Search cities…", clearable: true)
//!     )
//! }
//! ```
//!
//! Differences from upstream: values are initial values and `onValueChange` is the
//! `change` event; there is no `filter` function, and no `filter={null}` for results
//! the server filtered (render the server's results as the items instead), and no
//! controlled `open`. With `multiple`, `required` is not supported.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::{combobox as css, popover as popover_css};

/// One option of a [`combobox`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboboxOption<'a> {
    /// What is submitted.
    pub value: &'a str,
    /// What is shown and searched.
    pub label: &'a str,
    /// A decorative icon before the label.
    pub icon: Option<Icon>,
    /// Muted text at the end of the row.
    pub hint: &'a str,
    /// The heading it is listed under. Options with the same group are listed together.
    pub group: &'a str,
    /// Shown, but cannot be chosen.
    pub disabled: bool,
}

impl<'a> ComboboxOption<'a> {
    /// An option with a value and a label.
    #[must_use]
    pub const fn new(value: &'a str, label: &'a str) -> Self {
        Self {
            value,
            label,
            icon: None,
            hint: "",
            group: "",
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

    /// Lists it under a group heading.
    #[must_use]
    pub const fn group(mut self, group: &'a str) -> Self {
        self.group = group;
        self
    }

    /// Makes it impossible to choose.
    #[must_use]
    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// The height of a [`combobox`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComboboxSize {
    Sm,
    #[default]
    Md,
}

impl ComboboxSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

struct Row<'a> {
    option: ComboboxOption<'a>,
    option_id: String,
    selected: bool,
}

struct Group<'a> {
    label: &'a str,
    rows: Vec<Row<'a>>,
}

/// A combobox.
///
/// - `id` goes on the text input, so that a label can point at it.
/// - `value` is the chosen option's value, or the text with `free_text`; `values` are
///   the chosen options' values with `multiple`.
/// - `empty` is shown, and announced, when nothing matches. With `free_text` it is
///   empty by default, and the list closes instead.
/// - `label` names the input when no label points at it (its `aria-label`).
/// - `list_label` names the list; by default `label`, else the placeholder, else
///   "Options". `chips_label` names the chips.
/// - `clear_label` and `trigger_label` name the clear and open buttons; by default
///   "Clear selection" and "Show options", or "Clear text" and "Show suggestions"
///   with `free_text`. `remove_label` names a chip's remove button; `{label}` is
///   replaced by the chip's text.
/// - `attrs` are forwarded to the text input, for `aria-describedby` or
///   `aria-invalid`.
#[component]
pub async fn combobox(
    cx: &Cx,
    id: &str,
    items: &[ComboboxOption<'_>],
    #[default] name: &str,
    #[default] value: &str,
    #[default] values: &[&str],
    #[default] multiple: bool,
    #[default] free_text: bool,
    #[default] label: &str,
    #[default] placeholder: &str,
    #[default] empty: Option<&str>,
    #[default] clearable: bool,
    #[default] clear_label: &str,
    #[default] trigger_label: &str,
    #[default] list_label: &str,
    #[default("Selected")] chips_label: &str,
    #[default("Remove {label}")] remove_label: &str,
    #[default] auto_highlight: bool,
    #[default] limit: Option<usize>,
    #[default] submit_on_enter: bool,
    #[default] size: ComboboxSize,
    #[default] disabled: bool,
    #[default] required: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let multiple = multiple && !free_text;
    let list_id = format!("{id}-list");
    let empty = empty.unwrap_or(if free_text { "" } else { "No results." });
    let clear_label = match (clear_label, free_text) {
        ("", true) => "Clear text",
        ("", false) => "Clear selection",
        (label, _) => label,
    };
    let trigger_label = match (trigger_label, free_text) {
        ("", true) => "Show suggestions",
        ("", false) => "Show options",
        (label, _) => label,
    };
    let list_label = if !list_label.is_empty() {
        list_label.to_owned()
    } else if !label.is_empty() {
        label.to_owned()
    } else if !placeholder.is_empty() {
        placeholder.to_owned()
    } else if free_text {
        "Suggestions".to_owned()
    } else {
        "Options".to_owned()
    };
    let is_chosen = |option: &ComboboxOption<'_>| {
        if multiple {
            values.contains(&option.value)
        } else {
            !free_text && !value.is_empty() && option.value == value
        }
    };
    let single = items.iter().find(|option| !multiple && is_chosen(option));
    let text = if free_text {
        value.to_owned()
    } else {
        single
            .map(|option| option.label.to_owned())
            .unwrap_or_default()
    };
    let hidden_value = single.map_or("", |option| option.value).to_owned();
    // The chips follow the order of `values`.
    let chips: Vec<(String, String, String)> = values
        .iter()
        .filter(|_| multiple)
        .filter_map(|chosen| items.iter().find(|option| option.value == *chosen))
        .map(|option| {
            (
                option.value.to_owned(),
                option.label.to_owned(),
                remove_label.replace("{label}", option.label),
            )
        })
        .collect();
    let has_value = if free_text {
        !value.is_empty()
    } else if multiple {
        !chips.is_empty()
    } else {
        single.is_some()
    };
    let grouped = items.iter().any(|option| !option.group.is_empty());
    let mut groups: Vec<Group<'_>> = Vec::new();
    for (position, option) in items.iter().enumerate() {
        let row = Row {
            option: *option,
            option_id: format!("{id}-option-{position}"),
            selected: is_chosen(option),
        };
        match groups.iter_mut().find(|group| group.label == option.group) {
            Some(group) => group.rows.push(row),
            None => groups.push(Group {
                label: option.group,
                rows: vec![row],
            }),
        }
    }
    let input_class = class!(
        css::INPUT,
        multiple.then_some(css::CHIP_INPUT),
        attrs.remove("class")
    );
    let input_name = (free_text && !name.is_empty()).then_some(name);
    let input_list = list_id.clone();
    let input_placeholder = (!placeholder.is_empty() && chips.is_empty()).then_some(placeholder);
    let input = view! {
        <input
            id=(id)
            type="text"
            role="combobox"
            aria-controls=(&input_list)
            aria-expanded="false"
            aria-haspopup="listbox"
            aria-autocomplete="list"
            autocomplete="off"
            spellcheck="false"
            aria-label=((!label.is_empty()).then_some(label))
            name=(input_name)
            value=(&text)
            placeholder=(input_placeholder)
            data-placeholder=((!placeholder.is_empty()).then_some(placeholder))
            disabled=(disabled)
            required=(required && !multiple)
            class=(input_class)
            (attrs)
        >
    };
    // Boxed: a long list of options makes a deep view type.
    Ok(Child::new(view! {
        <div
            class=(css::ROOT)
            data-combobox=""
            data-mode=(if multiple {
                "multiple"
            } else if free_text {
                "free-text"
            } else {
                "single"
            })
            data-auto-highlight=(auto_highlight.then_some(""))
            data-submit-on-enter=((submit_on_enter || free_text).then_some(""))
            data-limit=(limit.map(|limit| limit.to_string()))
            data-empty=(empty)
            data-remove-label=(remove_label)
        >
            <div
                class=(css::CONTROL)
                data-size=(size.as_str())
                data-disabled=(disabled.then_some(""))
                data-combobox-control=""
            >
                if multiple {
                    <div
                        class=(css::CHIPS)
                        role=((!chips.is_empty()).then_some("toolbar"))
                        aria-label=((!chips.is_empty()).then_some(chips_label))
                        data-chips-label=(chips_label)
                        data-combobox-chips=""
                    >
                        for (chip_value, chip_label, chip_remove) in &chips {
                            <span
                                class=(css::CHIP)
                                tabindex="-1"
                                data-value=(chip_value)
                            >
                                <span class=(css::CHIP_TEXT)>(chip_label)</span>
                                <button
                                    type="button"
                                    tabindex="-1"
                                    aria-label=(chip_remove)
                                    disabled=(disabled)
                                    class=(css::CHIP_REMOVE)
                                    data-combobox-remove=""
                                >
                                    icon(name: Icon::X)
                                </button>
                                <input
                                    type="hidden"
                                    name=((!name.is_empty()).then_some(name))
                                    value=(chip_value)
                                    disabled=(disabled)
                                >
                            </span>
                        }
                        (input)
                    </div>
                    <template data-combobox-chip="">
                        <span class=(css::CHIP) tabindex="-1">
                            <span class=(css::CHIP_TEXT)></span>
                            <button
                                type="button"
                                tabindex="-1"
                                class=(css::CHIP_REMOVE)
                                data-combobox-remove=""
                            >
                                icon(name: Icon::X)
                            </button>
                            <input
                                type="hidden"
                                name=((!name.is_empty()).then_some(name))
                            >
                        </span>
                    </template>
                } else {
                    (input)
                }
                <div class=(css::ACTIONS)>
                    if clearable {
                        <button
                            type="button"
                            tabindex="-1"
                            aria-label=(clear_label)
                            hidden=(!has_value)
                            disabled=(disabled)
                            class=(css::ACTION)
                            data-combobox-clear=""
                        >
                            icon(name: Icon::X)
                        </button>
                    }
                    <button
                        type="button"
                        tabindex="-1"
                        aria-label=(trigger_label)
                        aria-haspopup="listbox"
                        aria-expanded="false"
                        disabled=(disabled)
                        class=(css::ACTION)
                        data-combobox-toggle=""
                    >
                        icon(name: Icon::ChevronsUpDown)
                    </button>
                </div>
            </div>
            if !multiple && !free_text {
                <input
                    type="hidden"
                    name=((!name.is_empty()).then_some(name))
                    value=(&hidden_value)
                    disabled=(disabled)
                    data-combobox-value=""
                >
            }
            <div popover="manual" class=(class!(popover_css::SURFACE, css::POPUP))>
                <div
                    role="status"
                    aria-live="polite"
                    aria-atomic="true"
                    class=(css::EMPTY)
                    data-combobox-empty=""
                ></div>
                <div
                    id=(&list_id)
                    role="listbox"
                    aria-label=(&list_label)
                    aria-multiselectable=(multiple.then_some("true"))
                    class=(css::LIST)
                >
                    for group in &groups {
                        <div
                            role=(grouped.then_some("group"))
                            aria-label=((grouped && !group.label.is_empty()).then_some(
                                group.label,
                            ))
                            class=(css::GROUP)
                        >
                            if !group.label.is_empty() {
                                <div aria-hidden="true" class=(popover_css::GROUP_LABEL)>
                                    (group.label)
                                </div>
                            }
                            for row in &group.rows {
                                <div
                                    role="option"
                                    id=(&row.option_id)
                                    data-value=(row.option.value)
                                    data-label=(row.option.label)
                                    aria-selected=(if row.selected { "true" } else { "false" })
                                    aria-disabled=(row.option.disabled.then_some("true"))
                                    class=(class!(popover_css::ITEM, css::ITEM))
                                >
                                    if let Some(name) = row.option.icon {
                                        <span aria-hidden="true" class=(css::ITEM_ICON)>
                                            icon(name: name)
                                        </span>
                                    }
                                    <span class=(css::ITEM_TEXT)>(row.option.label)</span>
                                    if !row.option.hint.is_empty() {
                                        <span class=(css::HINT)>(row.option.hint)</span>
                                    }
                                    if !free_text {
                                        <span aria-hidden="true" class=(css::INDICATOR)>
                                            icon(name: Icon::Check)
                                        </span>
                                    }
                                </div>
                            }
                        </div>
                    }
                </div>
            </div>
        </div>
    }))
}
