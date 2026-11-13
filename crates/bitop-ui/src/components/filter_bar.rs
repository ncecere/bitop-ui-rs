//! Faceted filters above a list or a table, with a chip for each active filter and
//! "Clear all".
//!
//! Facets are data, so the same definitions drive the controls, the URL
//! ([`filter_values_from_query`], [`filter_values_to_query`]), filtering in memory
//! ([`filter_rows`]) and counts ([`facet_counts`]).
//!
//! ```ignore
//! let statuses = [FacetOption::new("ready", "Ready"), FacetOption::new("failed", "Failed")];
//! let facets = [
//!     Facet::toggle("status", "Status", &statuses).all_label("All"),
//!     Facet::select("kind", "Kind", &kinds).multiple(),
//!     Facet::date_range("updated", "Updated"),
//! ];
//! let values = filter_values_from_query(&facets, query);
//! view! {
//!     filter_bar(id: "doc-filters", action: "/docs", query: query,
//!         facets: &facets, values: &values, drop_params: &["page"])
//! }
//! ```
//!
//! The bar is a `GET` form, so it works without a script: choose, then press "Apply".
//! Each filter is a parameter named after its facet, repeated for several values
//! (`?status=failed&status=skipped`); a date range is a preset's ID (`7d`) or
//! `YYYY-MM-DD/YYYY-MM-DD`. Other parameters of the page, such as a sort, are kept,
//! except those in `drop_params`. With the library script a change submits at once,
//! "Apply" is hidden, and after the page comes back focus returns to the control
//! that was used, or after removing a chip to the next chip, then "Clear all", then
//! the first control.
//!
//! Facet types:
//! - toggle: radio buttons, or checkboxes with `multiple`, drawn as a joined row;
//!   `all_label` adds a first "All" item that clears the facet. Without one, pressing
//!   the chosen item again clears it (script).
//! - select: a `<select>`; with `multiple`, a dropdown of checkboxes with its own
//!   "Apply".
//! - date range: a joined row of presets (Today, Last 7, 30 and 90 days).
//!
//! Every active facet gets a chip, except a single-choice toggle, whose pressed item
//! already shows the choice. A chip is a link that removes its filter. The bar is a
//! group named "Filters"; each facet's control has the name its visible label shows,
//! and counts are read after the option ("Failed 3").
//!
//! Differences from upstream: upstream keeps the filters in React state and filters
//! in the browser; here each change is a request. Toggles are native radio buttons
//! and checkboxes, not toggle buttons, so that they submit without a script. A select
//! facet is a native `<select>`, or a dropdown of checkboxes, not a combobox, so it
//! has no type-ahead filtering of long lists, option icons or groups. A date range has
//! no "Custom" range picker, though a custom range in the URL is read, shown as a
//! chip and kept. Chips are links, not buttons.

use std::collections::{BTreeMap, BTreeSet};

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    calendar::Date,
    icon::{Icon, icon},
    select::native_select,
};
use crate::{
    css::{button as button_css, checkbox as checkbox_css, filter_bar as css},
    format::{MONTHS, format_number},
};

/// One choice of a [`Facet`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FacetOption<'a> {
    /// What the URL carries.
    pub value: &'a str,
    /// What the control shows.
    pub label: &'a str,
}

impl<'a> FacetOption<'a> {
    /// An option.
    #[must_use]
    pub const fn new(value: &'a str, label: &'a str) -> Self {
        Self { value, label }
    }
}

/// Today, Last 7 days, Last 30 days and Last 90 days: the default presets of a date
/// range facet. A preset's value is `today` or a number of days and `d`.
pub const DATE_RANGE_PRESETS: [FacetOption<'static>; 4] = [
    FacetOption::new("today", "Today"),
    FacetOption::new("7d", "Last 7 days"),
    FacetOption::new("30d", "Last 30 days"),
    FacetOption::new("90d", "Last 90 days"),
];

/// The control of a [`Facet`]. Rendered as `data-facet`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacetKind {
    /// A joined row of options.
    Toggle,
    /// A dropdown, for long lists of options.
    Select,
    /// A joined row of date range presets.
    DateRange,
}

impl FacetKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Toggle => "toggle",
            Self::Select => "select",
            Self::DateRange => "date-range",
        }
    }
}

/// One filter: what it is called, how it is chosen, and what can be chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facet<'a> {
    /// The key in [`FilterValues`] and the name of the URL parameter.
    pub id: &'a str,
    /// The visible label, which names the control.
    pub label: &'a str,
    /// The control.
    pub kind: FacetKind,
    /// The choices; for a date range, the presets.
    pub options: &'a [FacetOption<'a>],
    /// Whether several options can be chosen at once.
    pub multiple: bool,
    /// The text of a first item that clears the facet, such as "All". Empty for none.
    pub all_label: &'a str,
    /// What a select shows when nothing is chosen.
    pub placeholder: &'a str,
    /// Whether the facet gets a chip while active. `None` means every facet but a
    /// single-choice toggle.
    pub chip: Option<bool>,
}

impl<'a> Facet<'a> {
    fn new(id: &'a str, label: &'a str, kind: FacetKind, options: &'a [FacetOption<'a>]) -> Self {
        Self {
            id,
            label,
            kind,
            options,
            multiple: false,
            all_label: "",
            placeholder: "Any",
            chip: None,
        }
    }

    /// A joined row of options.
    #[must_use]
    pub fn toggle(id: &'a str, label: &'a str, options: &'a [FacetOption<'a>]) -> Self {
        Self::new(id, label, FacetKind::Toggle, options)
    }

    /// A dropdown of options.
    #[must_use]
    pub fn select(id: &'a str, label: &'a str, options: &'a [FacetOption<'a>]) -> Self {
        Self::new(id, label, FacetKind::Select, options)
    }

    /// A date range, with the default presets and "Any time" to clear it.
    #[must_use]
    pub fn date_range(id: &'a str, label: &'a str) -> Self {
        Self::new(id, label, FacetKind::DateRange, &DATE_RANGE_PRESETS).all_label("Any time")
    }

    /// Replaces the options; for a date range, the presets.
    #[must_use]
    pub fn options(mut self, options: &'a [FacetOption<'a>]) -> Self {
        self.options = options;
        self
    }

    /// Lets several options be chosen at once.
    #[must_use]
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Adds a first item that clears the facet.
    #[must_use]
    pub fn all_label(mut self, label: &'a str) -> Self {
        self.all_label = label;
        self
    }

    /// Sets what a select shows when nothing is chosen.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Says whether the facet gets a chip while active.
    #[must_use]
    pub fn chip(mut self, chip: bool) -> Self {
        self.chip = Some(chip);
        self
    }

    /// Whether the facet gets a chip while active: `chip`, else every facet but a
    /// single-choice toggle, whose pressed item already shows the choice.
    #[must_use]
    pub fn has_chip(&self) -> bool {
        self.chip
            .unwrap_or(!(self.kind == FacetKind::Toggle && !self.multiple))
    }
}

/// The chosen values, by facet ID. A missing or empty entry means no filter. A date
/// range is one value: a preset's ID, `YYYY-MM-DD/YYYY-MM-DD` or `YYYY-MM-DD`.
pub type FilterValues = BTreeMap<String, Vec<String>>;

/// For each facet ID, for each option value: how many rows match.
pub type FacetCounts = BTreeMap<String, BTreeMap<String, usize>>;

/// Whether the facet `id` has an active filter in `values`.
#[must_use]
pub fn is_facet_active(values: &FilterValues, id: &str) -> bool {
    values.get(id).is_some_and(|list| !list.is_empty())
}

/// The number of facets with an active filter.
#[must_use]
pub fn active_filter_count(values: &FilterValues) -> usize {
    values.values().filter(|list| !list.is_empty()).count()
}

/// Decodes one name or value of a query string: `+` is a space and `%XX` a byte.
/// Bytes that are not UTF-8 become U+FFFD.
#[must_use]
pub fn decode_component(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => out.push(b' '),
            b'%' => {
                let hex = bytes
                    .get(index + 1..index + 3)
                    .and_then(|pair| std::str::from_utf8(pair).ok())
                    .and_then(|pair| u8::from_str_radix(pair, 16).ok());
                if let Some(byte) = hex {
                    out.push(byte);
                    index += 2;
                } else {
                    out.push(b'%');
                }
            }
            byte => out.push(byte),
        }
        index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Encodes one name or value for a query string. Letters, digits and `-._~` are
/// kept, and so is `/`, which is safe in a query and keeps a date range readable;
/// a space becomes `+`.
#[must_use]
pub fn encode_component(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                out.push(char::from(byte));
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// The decoded pairs of a query string, in order. A leading `?` is allowed.
#[must_use]
pub fn parse_query(query: &str) -> Vec<(String, String)> {
    query
        .trim_start_matches('?')
        .split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            (decode_component(name), decode_component(value))
        })
        .collect()
}

/// A query string, without the `?`, from decoded pairs.
#[must_use]
pub fn build_query(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(name, value)| format!("{}={}", encode_component(name), encode_component(value)))
        .collect::<Vec<_>>()
        .join("&")
}

/// The first and last day of a date range value: a preset (`today`, `7d` for the last
/// seven days up to `today`) or `YYYY-MM-DD/YYYY-MM-DD`, or one day. `None` when the
/// text is none of these, or the range ends before it starts.
#[must_use]
pub fn date_range_of(value: &str, today: Date) -> Option<(Date, Date)> {
    if value == "today" {
        return Some((today, today));
    }
    if let Some(days) = value.strip_suffix('d') {
        let days = days
            .bytes()
            .all(|b| b.is_ascii_digit())
            .then(|| days.parse::<u32>().ok())
            .flatten()
            .filter(|days| (1..=36_500).contains(days))?;
        return Some((today.add_days(1 - i64::from(days)), today));
    }
    let (from, to) = match value.split_once('/') {
        Some((from, to)) => (Date::parse(from)?, Date::parse(to)?),
        None => {
            let day = Date::parse(value)?;
            (day, day)
        }
    };
    (from <= to).then_some((from, to))
}

/// Reads the filters from a query string. Options the facet does not have, and date
/// ranges that are neither one of its presets nor valid dates, are dropped; a facet
/// that is not `multiple` keeps the first value.
#[must_use]
pub fn filter_values_from_query(facets: &[Facet<'_>], query: &str) -> FilterValues {
    let pairs = parse_query(query);
    let mut values = FilterValues::new();
    for facet in facets {
        let given = pairs
            .iter()
            .filter(|(name, _)| name == facet.id)
            .map(|(_, value)| value.as_str());
        let mut list: Vec<String> = Vec::new();
        for value in given {
            let known = facet.options.iter().any(|option| option.value == value);
            let valid = match facet.kind {
                FacetKind::DateRange => {
                    known || (value.contains('-') && date_range_of(value, Date::MIN).is_some())
                }
                _ => known,
            };
            if valid && !list.iter().any(|kept| kept == value) {
                list.push(value.to_owned());
            }
        }
        if !(facet.multiple && facet.kind != FacetKind::DateRange) {
            list.truncate(1);
        }
        if !list.is_empty() {
            values.insert(facet.id.to_owned(), list);
        }
    }
    values
}

/// Writes the filters into a query string: a copy of `query` in which each facet's
/// parameters are replaced by its values. Facets with no filter are removed, and
/// other parameters are kept, except those named in `drop`.
#[must_use]
pub fn filter_values_to_query(
    facets: &[Facet<'_>],
    values: &FilterValues,
    query: &str,
    drop: &[&str],
) -> String {
    let mut pairs: Vec<(String, String)> = parse_query(query)
        .into_iter()
        .filter(|(name, _)| {
            !facets.iter().any(|facet| facet.id == name) && !drop.contains(&name.as_str())
        })
        .collect();
    for facet in facets {
        for value in values.get(facet.id).into_iter().flatten() {
            pairs.push((facet.id.to_owned(), value.clone()));
        }
    }
    build_query(&pairs)
}

/// Whether a row with `row_values` for `facet` passes the filter `chosen`. A date
/// range facet compares the row's dates (`YYYY-MM-DD`, or anything that starts with
/// one) with the range, counted from `today`.
fn matches_facet(facet: &Facet<'_>, chosen: &[String], row_values: &[String], today: Date) -> bool {
    if chosen.is_empty() {
        return true;
    }
    if facet.kind == FacetKind::DateRange {
        let Some((from, to)) = chosen.first().and_then(|value| date_range_of(value, today)) else {
            return true;
        };
        return row_values.iter().any(|value| {
            value
                .get(..10)
                .and_then(Date::parse)
                .is_some_and(|date| date >= from && date <= to)
        });
    }
    chosen.iter().any(|value| row_values.contains(value))
}

/// The rows that pass every active facet. `accessor` gives a row's values for a
/// facet: one or more option values, or for a date range the row's date as
/// `YYYY-MM-DD`. `today` is what date range presets count back from.
pub fn filter_rows<'r, T>(
    rows: &'r [T],
    facets: &[Facet<'_>],
    values: &FilterValues,
    today: Date,
    accessor: impl Fn(&T, &Facet<'_>) -> Vec<String>,
) -> Vec<&'r T> {
    let active: Vec<(&Facet<'_>, &Vec<String>)> = facets
        .iter()
        .filter_map(|facet| values.get(facet.id).map(|chosen| (facet, chosen)))
        .filter(|(_, chosen)| !chosen.is_empty())
        .collect();
    rows.iter()
        .filter(|row| {
            active
                .iter()
                .all(|(facet, chosen)| matches_facet(facet, chosen, &accessor(row, facet), today))
        })
        .collect()
}

/// Counts for the options of toggle and select facets: for each facet, the rows that
/// pass all the other facets, counted by option. So an option's count is what
/// choosing it would give.
pub fn facet_counts<T>(
    rows: &[T],
    facets: &[Facet<'_>],
    values: &FilterValues,
    today: Date,
    accessor: impl Fn(&T, &Facet<'_>) -> Vec<String>,
) -> FacetCounts {
    let mut out = FacetCounts::new();
    for facet in facets {
        if facet.kind == FacetKind::DateRange {
            continue;
        }
        let mut others = values.clone();
        others.remove(facet.id);
        let pool = filter_rows(rows, facets, &others, today, &accessor);
        let mut counts: BTreeMap<String, usize> = facet
            .options
            .iter()
            .map(|option| (option.value.to_owned(), 0))
            .collect();
        for row in pool {
            let seen: BTreeSet<String> = accessor(row, facet).into_iter().collect();
            for value in seen {
                if let Some(count) = counts.get_mut(&value) {
                    *count += 1;
                }
            }
        }
        out.insert(facet.id.to_owned(), counts);
    }
    out
}

/// One active filter, as the bar shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterChip {
    /// The facet's label, or the search's.
    pub facet: String,
    /// The chosen option's label, the range, or the search text.
    pub text: String,
    /// The query string of the page without this filter.
    pub query: String,
}

/// "Sep 1, 2026 – Sep 7, 2026", or one date when the range is a single day.
fn describe_range(from: Date, to: Date) -> String {
    let one = |date: Date| {
        format!(
            "{} {}, {}",
            &MONTHS[usize::from(date.month()) - 1][..3],
            date.day(),
            date.year()
        )
    };
    if from == to {
        one(from)
    } else {
        format!("{} – {}", one(from), one(to))
    }
}

/// The chips for the active filters: the search text first, if `search_param` names a
/// parameter with text in it, then a chip for each chosen value of each facet that
/// has chips. Each chip carries the query string that removes it.
#[must_use]
pub fn filter_chips(
    facets: &[Facet<'_>],
    values: &FilterValues,
    query: &str,
    drop: &[&str],
    search_param: &str,
    search_label: &str,
) -> Vec<FilterChip> {
    let base = filter_values_to_query(facets, values, query, drop);
    let mut chips = Vec::new();
    if !search_param.is_empty() {
        let pairs = parse_query(&base);
        let text = pairs
            .iter()
            .find(|(name, _)| name == search_param)
            .map(|(_, value)| value.trim().to_owned())
            .unwrap_or_default();
        if !text.is_empty() {
            let rest: Vec<(String, String)> = pairs
                .into_iter()
                .filter(|(name, _)| name != search_param)
                .collect();
            chips.push(FilterChip {
                facet: search_label.to_owned(),
                text,
                query: build_query(&rest),
            });
        }
    }
    for facet in facets {
        let Some(chosen) = values.get(facet.id).filter(|list| !list.is_empty()) else {
            continue;
        };
        if !facet.has_chip() {
            continue;
        }
        for value in chosen {
            let mut without = values.clone();
            without
                .entry(facet.id.to_owned())
                .and_modify(|list| list.retain(|kept| kept != value));
            let label = facet
                .options
                .iter()
                .find(|option| option.value == value)
                .map(|option| option.label.to_owned());
            let text = match (label, facet.kind) {
                (Some(label), _) => label,
                (None, FacetKind::DateRange) => date_range_of(value, Date::MIN)
                    .map_or_else(|| value.clone(), |(from, to)| describe_range(from, to)),
                (None, _) => value.clone(),
            };
            chips.push(FilterChip {
                facet: facet.label.to_owned(),
                text,
                query: filter_values_to_query(facets, &without, query, drop),
            });
        }
    }
    chips
}

/// The size of the controls of a [`filter_bar`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FilterBarSize {
    #[default]
    Sm,
    Md,
}

impl FilterBarSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// What [`filter_bar`] renders for one option.
struct OptionView {
    value: String,
    label: String,
    count: Option<String>,
    checked: bool,
}

/// What [`filter_bar`] renders for one facet.
struct FacetView {
    id: String,
    label: String,
    label_id: String,
    kind: FacetKind,
    multiple: bool,
    all_label: String,
    placeholder: String,
    active: bool,
    summary: String,
    options: Vec<OptionView>,
    /// A value in the URL that no option stands for, such as a custom date range.
    extra: Option<String>,
}

/// `path?query`, or the path alone when the query is empty.
fn url(action: &str, query: &str) -> String {
    if query.is_empty() {
        action.to_owned()
    } else {
        format!("{action}?{query}")
    }
}

/// The filter bar. `attrs` go to the group.
///
/// - `id` is the group's ID; the IDs of the selects are made from it, and the script
///   uses it to return focus after the page comes back.
/// - `action` is the path of the page, and `query` its current query string.
/// - `facets` and `values` are the filters and what is chosen; see
///   [`filter_values_from_query`].
/// - `counts` are shown beside the options; see [`facet_counts`].
/// - `drop_params` are parameters to leave out when the filters change, such as
///   `page`.
/// - `search_param` names the parameter of a search box in `start`: its text gets a
///   chip too ("Search: wifi"), labelled `search_label`, and "Clear all" clears it.
/// - `start` is a slot before the facets, inside the form, for a search input, and
///   `end` one after them, for buttons.
/// - `chips` shows the active filters and "Clear all".
/// - `size` is the size of the controls.
/// - The other arguments are the strings. `remove_label` is a chip's accessible name,
///   with `{facet}` and `{value}`; `selected_label` what a dropdown of checkboxes
///   shows for several choices, with `{count}`.
#[component]
pub async fn filter_bar(
    cx: &Cx,
    id: &str,
    action: &str,
    facets: &[Facet<'_>],
    values: &FilterValues,
    #[default] query: &str,
    #[default]
    #[into]
    counts: Option<&FacetCounts>,
    #[default] drop_params: &[&str],
    #[default] search_param: &str,
    #[default("Search")] search_label: &str,
    #[default]
    #[into]
    start: Child<'_>,
    #[default]
    #[into]
    end: Child<'_>,
    #[default(true)] chips: bool,
    #[default] size: FilterBarSize,
    #[default("Filters")] label: &str,
    #[default("Active filters")] active_label: &str,
    #[default("Clear all")] clear_label: &str,
    #[default("Apply")] apply_label: &str,
    #[default("Remove filter {facet}: {value}")] remove_label: &str,
    #[default("{count} selected")] selected_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let views: Vec<FacetView> = facets
        .iter()
        .map(|facet| {
            let chosen: &[String] = values.get(facet.id).map_or(&[], Vec::as_slice);
            let options: Vec<OptionView> = facet
                .options
                .iter()
                .map(|option| OptionView {
                    value: option.value.to_owned(),
                    label: option.label.to_owned(),
                    count: counts
                        .and_then(|counts| counts.get(facet.id))
                        .and_then(|counts| counts.get(option.value))
                        .map(|count| {
                            #[allow(clippy::cast_precision_loss)]
                            let count = *count as f64;
                            format_number(count, 0, false)
                        }),
                    checked: chosen.iter().any(|value| value == option.value),
                })
                .collect();
            let picked: Vec<&str> = options
                .iter()
                .filter(|option| option.checked)
                .map(|option| option.label.as_str())
                .collect();
            let summary = match picked.len() {
                0 => facet.placeholder.to_owned(),
                1 | 2 => picked.join(", "),
                many => selected_label.replace("{count}", &many.to_string()),
            };
            FacetView {
                id: facet.id.to_owned(),
                label: facet.label.to_owned(),
                label_id: format!("{id}-{}", facet.id),
                kind: facet.kind,
                multiple: facet.multiple && facet.kind != FacetKind::DateRange,
                all_label: facet.all_label.to_owned(),
                placeholder: facet.placeholder.to_owned(),
                active: !chosen.is_empty(),
                summary,
                extra: chosen
                    .iter()
                    .find(|value| !facet.options.iter().any(|option| option.value == *value))
                    .cloned(),
                options,
            }
        })
        .collect();

    // Parameters that are not filters ride along as hidden inputs. The search box in
    // `start` submits its own.
    let kept: Vec<(String, String)> = parse_query(query)
        .into_iter()
        .filter(|(name, _)| {
            !facets.iter().any(|facet| facet.id == name)
                && !drop_params.contains(&name.as_str())
                && name != search_param
        })
        .collect();
    let chip_list: Vec<(String, String, String, String)> = if chips {
        filter_chips(
            facets,
            values,
            query,
            drop_params,
            search_param,
            search_label,
        )
        .into_iter()
        .map(|chip| {
            let name = remove_label
                .replace("{facet}", &chip.facet)
                .replace("{value}", &chip.text);
            (
                format!("{}:", chip.facet),
                chip.text,
                url(action, &chip.query),
                name,
            )
        })
        .collect()
    } else {
        Vec::new()
    };
    let any_active = !chip_list.is_empty() || active_filter_count(values) > 0;
    let clear_href = url(action, &build_query(&kept));
    let size = size.as_str();

    Ok(view! {
        <div
            id=(id)
            role="group"
            aria-label=(label)
            class=(class!(css::ROOT, attrs.remove("class")))
            data-filter-bar=""
            data-size=(size)
            (attrs)
        >
            <form
                method="get"
                action=(action)
                class=(css::CONTROLS)
                data-filter-form=""
            >
                for (name, value) in kept {
                    <input type="hidden" name=(name) value=(value)>
                }
                <div class=(css::START)>(start)</div>
                for facet in views {
                    <div class=(css::FACET) data-facet=(facet.kind.as_str())>
                        if facet.kind == FacetKind::Select && !facet.multiple {
                            <label for=(facet.label_id.clone()) class=(css::LABEL)>
                                (facet.label.clone())
                            </label>
                            <div class=(css::SELECT)>
                                native_select(
                                    attrs: attributes! {
                                        id=(facet.label_id.clone())
                                        name=(facet.id.clone())
                                        data-size=(size)
                                    },
                                    <option value="">(facet.placeholder.clone())</option>
                                    for option in facet.options {
                                        <option
                                            value=(option.value)
                                            selected=(option.checked.then_some(""))
                                        >
                                            (option.label)
                                            if let Some(count) = option.count {
                                                " ("
                                                (count)
                                                ")"
                                            }
                                        </option>
                                    }
                                )
                            </div>
                        } else if facet.kind == FacetKind::Select {
                            <span class=(css::LABEL)>(facet.label.clone())</span>
                            <details class=(css::MULTI) data-filter-multi="">
                                <summary
                                    class=(css::MULTI_SUMMARY)
                                    data-active=(facet.active.then_some(""))
                                >
                                    <span class="sr-only">
                                        (facet.label.clone())
                                        ": "
                                    </span>
                                    <span class=(css::MULTI_VALUE)>(facet.summary)</span>
                                    icon(name: Icon::ChevronDown)
                                </summary>
                                <div class=(css::MULTI_PANEL)>
                                    <div
                                        role="group"
                                        aria-label=(facet.label.clone())
                                        class=(css::MULTI_LIST)
                                    >
                                        for option in facet.options {
                                            <label class=(css::MULTI_OPTION)>
                                                <input
                                                    type="checkbox"
                                                    class=(checkbox_css::CHECKBOX)
                                                    name=(facet.id.clone())
                                                    value=(option.value)
                                                    checked=(option.checked.then_some(""))
                                                >
                                                <span>(option.label)</span>
                                                if let Some(count) = option.count {
                                                    " "
                                                    <span class=(css::COUNT)>(count)</span>
                                                }
                                            </label>
                                        }
                                    </div>
                                    <button
                                        type="submit"
                                        class=(button_css::ROOT)
                                        data-variant="primary"
                                        data-size="sm"
                                    >
                                        (apply_label)
                                    </button>
                                </div>
                            </details>
                        } else {
                            <span class=(css::LABEL)>(facet.label.clone())</span>
                            <div
                                role=(if facet.multiple { "group" } else { "radiogroup" })
                                aria-label=(facet.label.clone())
                                class=(css::TOGGLES)
                                data-clearable=((!facet.multiple
                                        && facet.all_label.is_empty()).then_some(""))
                            >
                                if !facet.all_label.is_empty() {
                                    <label class=(css::TOGGLE)>
                                        <input
                                            type=(if facet.multiple { "checkbox" } else { "radio" })
                                            name=(facet.id.clone())
                                            value=""
                                            checked=((!facet.active).then_some(""))
                                            data-filter-all=""
                                        >
                                        <span>(facet.all_label.clone())</span>
                                    </label>
                                }
                                for option in facet.options {
                                    <label class=(css::TOGGLE)>
                                        <input
                                            type=(if facet.multiple { "checkbox" } else { "radio" })
                                            name=(facet.id.clone())
                                            value=(option.value)
                                            checked=(option.checked.then_some(""))
                                        >
                                        <span>
                                            (option.label)
                                            if let Some(count) = option.count {
                                                " "
                                                <span class=(css::COUNT)>(count)</span>
                                            }
                                        </span>
                                    </label>
                                }
                                if let Some(extra) = facet.extra {
                                    <input
                                        type="hidden"
                                        name=(facet.id.clone())
                                        value=(extra)
                                        data-filter-extra=""
                                    >
                                }
                            </div>
                        }
                    </div>
                }
                <div class=(css::END)>
                    <button
                        type="submit"
                        class=(class!(button_css::ROOT, css::APPLY))
                        data-variant="secondary"
                        data-size=(size)
                        data-filter-apply=""
                    >
                        (apply_label)
                    </button>
                    (end)
                </div>
            </form>
            if chips && any_active {
                <div class=(css::CHIPS_ROW)>
                    <ul
                        aria-label=(active_label)
                        class=(css::CHIPS)
                        data-filter-chips=""
                    >
                        for (facet, text, href, name) in chip_list {
                            <li>
                                <a
                                    href=(href)
                                    class=(css::CHIP)
                                    aria-label=(name)
                                    data-filter-chip=""
                                >
                                    <span class=(css::CHIP_FACET)>(facet)</span>
                                    " "
                                    (text)
                                    icon(
                                        name: Icon::X,
                                        attrs: attributes! { class=(css::CHIP_ICON) }
                                    )
                                </a>
                            </li>
                        }
                        <li>
                            <a
                                href=(clear_href)
                                class=(button_css::ROOT)
                                data-variant="ghost"
                                data-size="sm"
                                data-filter-clear=""
                            >
                                (clear_label)
                            </a>
                        </li>
                    </ul>
                </div>
            }
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATUSES: [FacetOption<'static>; 3] = [
        FacetOption::new("ready", "Ready"),
        FacetOption::new("failed", "Failed"),
        FacetOption::new("skipped", "Skipped"),
    ];
    const KINDS: [FacetOption<'static>; 3] = [
        FacetOption::new("pdf", "PDF"),
        FacetOption::new("doc", "Word"),
        FacetOption::new("web", "Web page"),
    ];

    fn facets() -> [Facet<'static>; 3] {
        [
            Facet::toggle("status", "Status", &STATUSES).all_label("All"),
            Facet::select("kind", "Kind", &KINDS).multiple(),
            Facet::date_range("updated", "Updated"),
        ]
    }

    fn values(pairs: &[(&str, &[&str])]) -> FilterValues {
        pairs
            .iter()
            .map(|(id, list)| {
                (
                    (*id).to_owned(),
                    list.iter().map(|value| (*value).to_owned()).collect(),
                )
            })
            .collect()
    }

    fn today() -> Date {
        Date::new(2026, 9, 26).unwrap()
    }

    #[test]
    fn components_round_trip() {
        assert_eq!(decode_component("a+b%20c%2Fd"), "a b c/d");
        assert_eq!(decode_component("100%"), "100%");
        assert_eq!(decode_component("%zz"), "%zz");
        assert_eq!(decode_component("caf%C3%A9"), "café");
        assert_eq!(encode_component("a b&c=d"), "a+b%26c%3Dd");
        assert_eq!(encode_component("café"), "caf%C3%A9");
        assert_eq!(
            encode_component("2026-09-01/2026-09-07"),
            "2026-09-01/2026-09-07"
        );
        for text in ["wi-fi & co", "a+b", "100%", "x=y?z#w", "日本"] {
            assert_eq!(decode_component(&encode_component(text)), text);
        }
    }

    #[test]
    fn queries_parse_and_build() {
        let pairs = parse_query("?q=wi+fi&status=failed&status=skipped&flag");
        assert_eq!(
            pairs,
            [
                ("q".to_owned(), "wi fi".to_owned()),
                ("status".to_owned(), "failed".to_owned()),
                ("status".to_owned(), "skipped".to_owned()),
                ("flag".to_owned(), String::new()),
            ]
        );
        assert_eq!(
            build_query(&pairs),
            "q=wi+fi&status=failed&status=skipped&flag="
        );
        assert_eq!(parse_query(""), []);
    }

    #[test]
    fn reads_filters_from_a_query() {
        let facets = facets();
        let read = filter_values_from_query(
            &facets,
            "sort=name&status=failed&status=ready&kind=pdf&kind=web&kind=pdf&kind=exe&updated=7d",
        );
        // A single-choice facet keeps the first value; unknown options are dropped;
        // repeated values are kept once.
        assert_eq!(
            read,
            values(&[
                ("status", &["failed"]),
                ("kind", &["pdf", "web"]),
                ("updated", &["7d"])
            ])
        );
        assert_eq!(
            filter_values_from_query(&facets, "status=&kind=&updated="),
            FilterValues::new()
        );
        assert_eq!(
            filter_values_from_query(&facets, "status=nope"),
            FilterValues::new()
        );
    }

    #[test]
    fn reads_date_ranges() {
        let facets = facets();
        let read = |query| filter_values_from_query(&facets, query).remove("updated");
        assert_eq!(read("updated=today"), Some(vec!["today".to_owned()]));
        assert_eq!(
            read("updated=2026-09-01/2026-09-07"),
            Some(vec!["2026-09-01/2026-09-07".to_owned()])
        );
        assert_eq!(
            read("updated=2026-09-01"),
            Some(vec!["2026-09-01".to_owned()])
        );
        // A preset the facet does not offer, dates that do not exist, a range backwards.
        assert_eq!(read("updated=14d"), None);
        assert_eq!(read("updated=2026-02-30/2026-03-01"), None);
        assert_eq!(read("updated=2026-09-07/2026-09-01"), None);
        assert_eq!(read("updated=soon"), None);
    }

    #[test]
    fn date_ranges() {
        let day = |d| Date::new(2026, 9, d).unwrap();
        assert_eq!(date_range_of("today", today()), Some((day(26), day(26))));
        assert_eq!(date_range_of("7d", today()), Some((day(20), day(26))));
        assert_eq!(date_range_of("1d", today()), Some((day(26), day(26))));
        assert_eq!(
            date_range_of("2026-09-01/2026-09-07", today()),
            Some((day(1), day(7)))
        );
        assert_eq!(date_range_of("2026-09-03", today()), Some((day(3), day(3))));
        assert_eq!(date_range_of("0d", today()), None);
        assert_eq!(date_range_of("d", today()), None);
        assert_eq!(date_range_of("-3d", today()), None);
    }

    #[test]
    fn writes_filters_into_a_query() {
        let facets = facets();
        let chosen = values(&[("status", &["failed"]), ("kind", &["pdf", "web"])]);
        assert_eq!(
            filter_values_to_query(&facets, &chosen, "sort=name&status=ready&page=3", &["page"]),
            "sort=name&status=failed&kind=pdf&kind=web"
        );
        // No filters: the facets' parameters go, the others stay.
        assert_eq!(
            filter_values_to_query(&facets, &FilterValues::new(), "kind=pdf&sort=name", &[]),
            "sort=name"
        );
        assert_eq!(
            filter_values_to_query(&facets, &FilterValues::new(), "", &[]),
            ""
        );
        // What is written is what is read.
        let query = filter_values_to_query(&facets, &chosen, "", &[]);
        assert_eq!(filter_values_from_query(&facets, &query), chosen);
    }

    #[test]
    fn counts_active_filters() {
        let chosen = values(&[("status", &["failed"]), ("kind", &[]), ("updated", &["7d"])]);
        assert_eq!(active_filter_count(&chosen), 2);
        assert!(is_facet_active(&chosen, "status"));
        assert!(!is_facet_active(&chosen, "kind"));
        assert!(!is_facet_active(&chosen, "other"));
    }

    #[test]
    fn chips() {
        let facets = facets();
        let chosen = values(&[
            ("status", &["failed"]),
            ("kind", &["pdf", "web"]),
            ("updated", &["2026-09-01/2026-09-07"]),
        ]);
        let query = "q=wi+fi&sort=name&status=failed&kind=pdf&kind=web&updated=2026-09-01/2026-09-07&page=2";
        let chips = filter_chips(&facets, &chosen, query, &["page"], "q", "Search");
        let texts: Vec<(&str, &str)> = chips
            .iter()
            .map(|chip| (chip.facet.as_str(), chip.text.as_str()))
            .collect();
        // The single-choice toggle has no chip: its pressed item shows the choice.
        assert_eq!(
            texts,
            [
                ("Search", "wi fi"),
                ("Kind", "PDF"),
                ("Kind", "Web page"),
                ("Updated", "Sep 1, 2026 – Sep 7, 2026"),
            ]
        );
        assert_eq!(
            chips[0].query,
            "sort=name&status=failed&kind=pdf&kind=web&updated=2026-09-01/2026-09-07"
        );
        assert_eq!(
            chips[1].query,
            "q=wi+fi&sort=name&status=failed&kind=web&updated=2026-09-01/2026-09-07"
        );
        assert_eq!(
            chips[3].query,
            "q=wi+fi&sort=name&status=failed&kind=pdf&kind=web"
        );
        // A preset is described by its label, and `chip` overrides the default.
        let facets = [
            Facet::toggle("status", "Status", &STATUSES).chip(true),
            facets[2],
        ];
        let chosen = values(&[("status", &["ready"]), ("updated", &["30d"])]);
        let chips = filter_chips(&facets, &chosen, "", &[], "", "Search");
        assert_eq!(chips[0].text, "Ready");
        assert_eq!(chips[1].text, "Last 30 days");
        assert_eq!(chips[1].query, "status=ready");
    }

    struct Doc {
        status: &'static str,
        kinds: &'static [&'static str],
        updated: &'static str,
    }

    const DOCS: [Doc; 5] = [
        Doc {
            status: "ready",
            kinds: &["pdf"],
            updated: "2026-09-26",
        },
        Doc {
            status: "ready",
            kinds: &["doc", "web"],
            updated: "2026-09-22",
        },
        Doc {
            status: "failed",
            kinds: &["pdf"],
            updated: "2026-09-10T08:00:00Z",
        },
        Doc {
            status: "failed",
            kinds: &["web"],
            updated: "2026-06-01",
        },
        Doc {
            status: "skipped",
            kinds: &[],
            updated: "",
        },
    ];

    fn access(doc: &Doc, facet: &Facet<'_>) -> Vec<String> {
        match facet.id {
            "status" => vec![doc.status.to_owned()],
            "kind" => doc.kinds.iter().map(|kind| (*kind).to_owned()).collect(),
            _ => vec![doc.updated.to_owned()],
        }
    }

    #[test]
    fn filters_rows() {
        let facets = facets();
        let pass = |chosen: &FilterValues| {
            let rows = filter_rows(&DOCS, &facets, chosen, today(), access);
            rows.iter()
                .map(|row| DOCS.iter().position(|doc| std::ptr::eq(doc, *row)).unwrap())
                .collect::<Vec<_>>()
        };
        assert_eq!(pass(&FilterValues::new()), [0, 1, 2, 3, 4]);
        assert_eq!(pass(&values(&[("status", &["failed"])])), [2, 3]);
        // Any of several values of one facet; all of several facets.
        assert_eq!(pass(&values(&[("kind", &["pdf", "web"])])), [0, 1, 2, 3]);
        assert_eq!(
            pass(&values(&[
                ("kind", &["pdf", "web"]),
                ("status", &["failed"])
            ])),
            [2, 3]
        );
        // Date ranges are whole days, and a row with no date does not pass.
        assert_eq!(pass(&values(&[("updated", &["today"])])), [0]);
        assert_eq!(pass(&values(&[("updated", &["7d"])])), [0, 1]);
        assert_eq!(pass(&values(&[("updated", &["30d"])])), [0, 1, 2]);
        assert_eq!(
            pass(&values(&[("updated", &["2026-06-01/2026-09-10"])])),
            [2, 3]
        );
    }

    #[test]
    fn counts_options_against_the_other_facets() {
        let facets = facets();
        let counts = facet_counts(
            &DOCS,
            &facets,
            &values(&[("status", &["failed"])]),
            today(),
            access,
        );
        // The status facet is counted without its own filter.
        assert_eq!(counts["status"]["ready"], 2);
        assert_eq!(counts["status"]["failed"], 2);
        assert_eq!(counts["status"]["skipped"], 1);
        // The kind facet is counted among the failed rows.
        assert_eq!(counts["kind"]["pdf"], 1);
        assert_eq!(counts["kind"]["web"], 1);
        assert_eq!(counts["kind"]["doc"], 0);
        assert!(!counts.contains_key("updated"));
    }
}
