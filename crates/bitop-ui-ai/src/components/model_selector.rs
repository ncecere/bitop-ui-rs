//! A searchable model picker, grouped by provider, with capability badges.
//!
//! The trigger is a button that opens a popover (the base set's popup) with a search
//! box over a list of models. The library script adds the pattern of a searchable
//! select: typing filters by every word, the arrow keys move through the models, Enter
//! chooses, and Escape closes and returns focus to the trigger. The chosen model's
//! `id` is submitted with the form under `name`, through a hidden input, and a
//! `change` event is sent from that input.
//!
//! ```ignore
//! let models = [
//!     ModelOption::new("claude-sonnet", "Claude Sonnet", "Anthropic")
//!         .capabilities(&["vision", "tools"]).context_window(200_000),
//!     ModelOption::new("gpt-mini", "GPT mini", "OpenAI"),
//! ];
//! view! {
//!     model_selector(id: "model", name: "model", label: "Model", models: &models,
//!         value: "claude-sonnet")
//! }
//! ```
//!
//! Differences from upstream: upstream is Base UI's combobox with the input in the
//! popup; this is the Popover API, a listbox and the library script, with the same
//! keyboard pattern. The value is submitted with a form instead of reported through
//! `onValueChange`. Provider icons are not ported: a model's icon is decorative, and
//! the icon set has no brand marks.

use bitop_ui::{
    components::{
        badge::{BadgeVariant, badge},
        icon::{Icon, icon},
    },
    css::popover as popover_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::model_selector as css;

/// One model in a [`model_selector`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModelOption<'a> {
    /// What is submitted.
    pub id: &'a str,
    /// What is shown.
    pub name: &'a str,
    /// The group it is listed under, such as "Anthropic".
    pub provider: &'a str,
    /// A short line under the name.
    pub description: &'a str,
    /// Capability ids, such as `vision` or `tools`, shown as badges.
    pub capabilities: &'a [&'a str],
    /// The context window in tokens, shown as "200K context".
    pub context_window: Option<u64>,
}

impl<'a> ModelOption<'a> {
    /// A model with an id, a name and a provider.
    #[must_use]
    pub fn new(id: &'a str, name: &'a str, provider: &'a str) -> Self {
        Self {
            id,
            name,
            provider,
            ..Self::default()
        }
    }

    /// Adds a description.
    #[must_use]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Adds capability badges.
    #[must_use]
    pub fn capabilities(mut self, capabilities: &'a [&'a str]) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Adds the context window, in tokens.
    #[must_use]
    pub fn context_window(mut self, tokens: u64) -> Self {
        self.context_window = Some(tokens);
        self
    }
}

/// The size of a [`model_selector`]'s trigger. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModelSelectorSize {
    Sm,
    #[default]
    Md,
}

impl ModelSelectorSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A number of tokens in short form: `200K`, `1M`, `1.5M`, `32.8K`, one decimal at
/// most.
///
/// ```
/// use bitop_ui_ai::components::model_selector::compact_number;
///
/// assert_eq!(compact_number(200_000), "200K");
/// assert_eq!(compact_number(1_048_576), "1M");
/// ```
#[must_use]
pub fn compact_number(n: u64) -> String {
    let (value, unit) = match n {
        0..=999 => return n.to_string(),
        1_000..=999_999 => (n, ("K", 1_000)),
        1_000_000..=999_999_999 => (n, ("M", 1_000_000)),
        _ => (n, ("B", 1_000_000_000)),
    };
    #[allow(clippy::cast_precision_loss)]
    let scaled = value as f64 / unit.1 as f64;
    let rounded = (scaled * 10.0).round() / 10.0;
    if rounded.fract() == 0.0 {
        format!("{rounded:.0}{}", unit.0)
    } else {
        format!("{rounded:.1}{}", unit.0)
    }
}

/// A capability id in title case, such as `Vision` for `vision`.
fn title_case(id: &str) -> String {
    let mut chars = id.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

struct Row {
    id: String,
    option_id: String,
    name: String,
    meta: String,
    badges: Vec<(String, BadgeVariant)>,
    search: String,
    selected: bool,
}

struct Group {
    provider: String,
    rows: Vec<Row>,
}

/// A model selector.
///
/// - `id` goes on the popup; the trigger and the list are named after it.
/// - `name` is the form field; `value` the chosen model's id.
/// - `label` names the trigger ("Model: Claude Sonnet"), the popup and the list.
/// - `placeholder` is shown while no model is chosen; `search_placeholder` and
///   `search_label` are the search box's; `empty` is shown when nothing matches.
/// - `capability_labels` names capability ids (`("vision", "Vision")`); others are
///   shown in title case. `capability_tones` gives a capability a badge colour, for
///   one that is a warning rather than a feature.
/// - `context_label` is the context window's text, with `{tokens}`.
/// - `size` is the trigger's size; `disabled` disables it.
#[component]
pub async fn model_selector(
    cx: &Cx,
    id: &str,
    name: &str,
    label: &str,
    models: &[ModelOption<'_>],
    #[default] value: &str,
    #[default("Select a model")] placeholder: &str,
    #[default("Search models…")] search_placeholder: &str,
    #[default("Search models")] search_label: &str,
    #[default("No models found.")] empty: &str,
    #[default] capability_labels: &[(&str, &str)],
    #[default] capability_tones: &[(&str, BadgeVariant)],
    #[default("{tokens} context")] context_label: &str,
    #[default] size: ModelSelectorSize,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let selected = models.iter().find(|model| model.id == value);
    let shown = selected.map_or(placeholder, |model| model.name);
    let trigger_label = format!("{label}: {shown}");
    let list_id = format!("{id}-list");
    let mut groups: Vec<Group> = Vec::new();
    for (position, model) in models.iter().enumerate() {
        let meta = [
            model.description.to_owned(),
            model
                .context_window
                .map(|tokens| context_label.replace("{tokens}", &compact_number(tokens)))
                .unwrap_or_default(),
        ]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" · ");
        let badges: Vec<(String, BadgeVariant)> = model
            .capabilities
            .iter()
            .map(|capability| {
                let text = capability_labels
                    .iter()
                    .find(|(key, _)| key == capability)
                    .map_or_else(|| title_case(capability), |(_, text)| (*text).to_owned());
                let tone = capability_tones
                    .iter()
                    .find(|(key, _)| key == capability)
                    .map_or(BadgeVariant::Neutral, |(_, tone)| *tone);
                (text, tone)
            })
            .collect();
        let search = [
            model.name,
            model.provider,
            model.id,
            model.description,
            &model.capabilities.join(" "),
        ]
        .join(" ")
        .to_lowercase();
        let row = Row {
            id: model.id.to_owned(),
            option_id: format!("{id}-option-{position}"),
            name: model.name.to_owned(),
            meta,
            badges,
            search,
            selected: model.id == value,
        };
        match groups
            .iter_mut()
            .find(|group| group.provider == model.provider)
        {
            Some(group) => group.rows.push(row),
            None => groups.push(Group {
                provider: model.provider.to_owned(),
                rows: vec![row],
            }),
        }
    }
    let placeholder_shown = selected.is_none();
    // Boxed: a long list of options makes a deep view type.
    Ok(Child::new(view! {
        <div
            class=(class!(css::ROOT, attrs.remove("class")))
            data-model-selector=""
            (attrs)
        >
            <button
                type="button"
                popovertarget=(id)
                aria-label=(&trigger_label)
                aria-haspopup="dialog"
                data-label=(label)
                data-size=(size.as_str())
                disabled=(disabled)
                class=(css::TRIGGER)
                data-model-trigger=""
            >
                <span
                    class=(css::VALUE)
                    data-placeholder=(placeholder_shown.then_some(""))
                >
                    (shown)
                </span>
                icon(
                    name: Icon::ChevronsUpDown,
                    attrs: attributes! { class=(css::CHEVRON) }
                )
            </button>
            <input type="hidden" name=(name) value=(value) disabled=(disabled)>
            <div
                id=(id)
                popover="auto"
                role="dialog"
                aria-label=(label)
                data-align="start"
                class=(class!(popover_css::POPUP, css::POPUP))
            >
                <div class=(css::SEARCH)>
                    icon(
                        name: Icon::Search,
                        attrs: attributes! { class=(css::SEARCH_ICON) }
                    )
                    <input
                        type="text"
                        role="combobox"
                        aria-controls=(&list_id)
                        aria-expanded="true"
                        aria-autocomplete="list"
                        aria-label=(search_label)
                        autocomplete="off"
                        spellcheck="false"
                        placeholder=(search_placeholder)
                        class=(css::INPUT)
                        data-model-search=""
                    >
                </div>
                <div class=(css::EMPTY) data-model-empty="" hidden=(true)>(empty)</div>
                <div id=(&list_id) role="listbox" aria-label=(label) class=(css::LIST)>
                    for group in &groups {
                        <div
                            role="group"
                            aria-label=(&group.provider)
                            class=(css::GROUP)
                        >
                            <div aria-hidden="true" class=(css::GROUP_LABEL)>
                                (&group.provider)
                            </div>
                            for row in &group.rows {
                                <div
                                    role="option"
                                    id=(&row.option_id)
                                    data-value=(&row.id)
                                    data-name=(&row.name)
                                    data-search=(&row.search)
                                    aria-selected=(if row.selected { "true" } else { "false" })
                                    class=(css::ITEM)
                                >
                                    <span class=(css::ITEM_TEXT)>
                                        <span class=(css::ITEM_NAME)>(&row.name)</span>
                                        if !row.meta.is_empty() {
                                            <span class=(css::ITEM_META)>(&row.meta)</span>
                                        }
                                    </span>
                                    if !row.badges.is_empty() {
                                        <span class=(css::BADGES)>
                                            for (text, tone) in &row.badges {
                                                badge(variant: *tone, (text))
                                            }
                                        </span>
                                    }
                                    <span class=(css::CHECK)>icon(name: Icon::Check)</span>
                                </div>
                            }
                        </div>
                    }
                </div>
            </div>
        </div>
    }))
}

#[cfg(test)]
mod tests {
    use super::{compact_number, title_case};

    #[test]
    fn compact() {
        assert_eq!(compact_number(512), "512");
        assert_eq!(compact_number(8_192), "8.2K");
        assert_eq!(compact_number(32_768), "32.8K");
        assert_eq!(compact_number(128_000), "128K");
        assert_eq!(compact_number(1_000_000), "1M");
        assert_eq!(compact_number(1_500_000), "1.5M");
        assert_eq!(compact_number(2_000_000_000), "2B");
        assert_eq!(title_case("vision"), "Vision");
        assert_eq!(title_case(""), "");
    }
}
