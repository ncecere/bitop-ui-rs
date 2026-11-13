//! A button that opens a [`calendar`](super::calendar) in a popover, for choosing a
//! date or a range of dates.
//!
//! The button shows the chosen date. Its accessible name is `label` followed by that
//! value, as an `aria-label` that the library script keeps up to date. When the picker
//! also has a visible label, such as a [`field_label`](super::field::field_label)
//! pointing at `id`, pass the same text as `label`. Opening moves focus to the
//! selected day, or to today; choosing a date, or the second end of a range, closes
//! the popup and returns focus to the button. The value is submitted with the form
//! under `name` as `2026-11-12`, and a range as `2026-11-10/2026-11-12`.
//!
//! `presets` on a range picker lists preset buttons (Today, Last 7 days and so on)
//! beside the calendar; choosing one sets the range and closes the popup.
//!
//! Differences from upstream: the button is named with `aria-label`, not with
//! `aria-labelledby` pointing at the label and the value. The button's text is
//! formatted from `format` and the short month names in [`CalendarLabels`], not by
//! `Intl.DateTimeFormat`, so that the server and the script write the same text. The
//! popup opens below the button and flips above when there is no room; there is no
//! `side`. `closeOnSelect={false}` is `keep_open`. `DateRangePresets`, the toggle
//! group that keeps a preset's ID for a URL, is not ported: it is page state more
//! than a control, and a server-rendered page can build it from links.
//!
//! ```ignore
//! let id = element_id(cx);
//! let presets = date_range_presets(today);
//! view! {
//!     field(
//!         field_label(target: &id, "Start date")
//!         date_picker(id: &id, name: "start", label: "Start date")
//!     )
//!     date_picker(id: &period, name: "period", label: "Report period",
//!         mode: DatePickerMode::Range, number_of_months: 2,
//!         presets: &presets)
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    calendar::{CalendarLabels, CalendarMode, Date, calendar},
    icon::{Icon, icon},
    popover::PopupAlign,
};
use crate::css::{button as button_css, date_picker as css, popover as popover_css};

/// What a [`date_picker`] chooses. Rendered as `data-mode`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatePickerMode {
    /// One day.
    #[default]
    Single,
    /// A first and a last day.
    Range,
}

impl DatePickerMode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Range => "range",
        }
    }
}

/// The size of a [`date_picker`]'s button. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatePickerSize {
    Sm,
    #[default]
    Md,
}

impl DatePickerSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A named range offered beside the calendar of a range [`date_picker`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateRangePreset {
    /// The button's text, such as "Last 7 days".
    pub label: String,
    /// The first day of the range.
    pub from: Date,
    /// The last day of the range.
    pub to: Date,
}

impl DateRangePreset {
    /// The last `days` days up to and including `today`, labelled "Last 7 days".
    #[must_use]
    pub fn last_days(days: u32, today: Date) -> Self {
        Self {
            label: format!("Last {days} days"),
            from: today.add_days(1 - i64::from(days)),
            to: today,
        }
    }
}

/// Today, Last 7 days, Last 30 days and Last 90 days, counted back from `today`.
#[must_use]
pub fn date_range_presets(today: Date) -> Vec<DateRangePreset> {
    vec![
        DateRangePreset {
            label: "Today".to_owned(),
            from: today,
            to: today,
        },
        DateRangePreset::last_days(7, today),
        DateRangePreset::last_days(30, today),
        DateRangePreset::last_days(90, today),
    ]
}

/// A date as the button shows it: `format` with `{month}` (the short name), `{day}`
/// and `{year}` replaced.
fn format_date(format: &str, labels: &CalendarLabels<'_>, date: Date) -> String {
    format
        .replace(
            "{month}",
            labels.months_short[usize::from(date.month() - 1)],
        )
        .replace("{day}", &date.day().to_string())
        .replace("{year}", &date.year().to_string())
}

/// A date picker.
///
/// - `id` goes on the button; `name` on the hidden input that holds the value.
/// - `selected` is the chosen day, or the first and last day of a range.
/// - `label` names the button, together with the value it shows. Pass the text of the
///   visible label when there is one.
/// - `placeholder` is shown when nothing is chosen. It defaults to "Pick a date", or
///   "Pick a date range" for a range.
/// - `format` is how the button writes a date: `{month}` (the short name), `{day}`
///   and `{year}` are replaced. `range_separator` joins the two ends of a range.
/// - `dialog_label` names the popup. It defaults to "Choose date", or "Choose dates"
///   for a range.
/// - `block` stretches the button to the width of its container.
/// - `invalid` marks the button with `aria-invalid`.
/// - `keep_open` leaves the popup open after a date is chosen.
/// - `align` is which edge of the button the popup lines up with.
/// - `presets` are ranges offered beside the calendar of a range picker, in a group
///   named `presets_label`.
/// - `footer` is a slot under the calendar.
/// - `today`, `min`, `max`, `disabled_dates`, `disabled_weekdays`, `week_starts_on`,
///   `hide_outside_days`, `number_of_months`, `dropdown`, `year_range` and `labels`
///   are the [`calendar`]'s.
/// - `attrs` are forwarded to the button.
#[component]
pub async fn date_picker(
    cx: &Cx,
    id: &str,
    #[default] name: &str,
    #[default] mode: DatePickerMode,
    #[default] selected: &[Date],
    label: &str,
    #[default] placeholder: &str,
    #[default("{month} {day}, {year}")] format: &str,
    #[default(" – ")] range_separator: &str,
    #[default] dialog_label: &str,
    #[default] size: DatePickerSize,
    #[default] block: bool,
    #[default] disabled: bool,
    #[default] invalid: bool,
    #[default] keep_open: bool,
    #[default] align: PopupAlign,
    #[default] presets: &[DateRangePreset],
    #[default("Presets")] presets_label: &str,
    #[default]
    #[into]
    footer: Child<'_>,
    #[default] today: Option<Date>,
    #[default] min: Option<Date>,
    #[default] max: Option<Date>,
    #[default] disabled_dates: &[Date],
    #[default] disabled_weekdays: &[u8],
    #[default] week_starts_on: u8,
    #[default] hide_outside_days: bool,
    #[default(1)] number_of_months: u8,
    #[default] dropdown: bool,
    #[default] year_range: Option<(i32, i32)>,
    #[default] labels: CalendarLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let range = mode == DatePickerMode::Range;
    let placeholder = match (placeholder, range) {
        ("", true) => "Pick a date range",
        ("", false) => "Pick a date",
        (text, _) => text,
    };
    let dialog_label = match (dialog_label, range) {
        ("", true) => "Choose dates",
        ("", false) => "Choose date",
        (text, _) => text,
    };
    let mut days: Vec<Date> = selected.to_vec();
    days.truncate(if range { 2 } else { 1 });
    days.sort_unstable();
    let text = match days.as_slice() {
        [] => None,
        [day] if !range => Some(format_date(format, &labels, *day)),
        [from] => Some(format!(
            "{}{range_separator}…",
            format_date(format, &labels, *from)
        )),
        [from, to, ..] => Some(format!(
            "{}{range_separator}{}",
            format_date(format, &labels, *from),
            format_date(format, &labels, *to)
        )),
    };
    let popup_id = format!("{id}-popup");
    let calendar_id = format!("{id}-calendar");
    let value_id = format!("{id}-value");
    // The name is the label followed by the value.
    let accessible_name = format!("{label} {}", text.as_deref().unwrap_or(placeholder));
    let presets: Vec<(&str, String, String, bool)> = presets
        .iter()
        .map(|preset| {
            (
                preset.label.as_str(),
                preset.from.to_string(),
                preset.to.to_string(),
                days == [preset.from, preset.to],
            )
        })
        .collect();
    let calendar_mode = if range {
        CalendarMode::Range
    } else {
        CalendarMode::Single
    };
    let with_presets = range && !presets.is_empty();

    Ok(view! {
        <span
            class=(css::ROOT)
            data-date-picker=""
            data-mode=(mode.as_str())
            data-label=(label)
            data-block=(block)
            data-format=(format)
            data-range-separator=(range_separator)
            data-placeholder=(placeholder)
            data-keep-open=(keep_open)
        >
            <button
                type="button"
                id=(id)
                popovertarget=(&popup_id)
                aria-haspopup="dialog"
                aria-label=(&accessible_name)
                aria-invalid=(invalid.then_some("true"))
                disabled=(disabled)
                class=(class!(css::TRIGGER, attrs.remove("class")))
                data-size=(size.as_str())
                data-placeholder=(text.is_none())
                data-date-picker-trigger=""
                (attrs)
            >
                icon(name: Icon::CalendarDays, attrs: attributes! { class=(css::ICON) })
                <span id=(&value_id) class=(css::VALUE)>
                    (text.as_deref().unwrap_or(placeholder))
                </span>
            </button>
            <div
                id=(&popup_id)
                popover="auto"
                role="dialog"
                aria-label=(dialog_label)
                class=(class!(popover_css::POPUP, css::POPUP))
                data-align=(align.as_str())
            >
                <div class=(css::WITH_PRESETS)>
                    if with_presets {
                        <div
                            role="group"
                            aria-label=(presets_label)
                            class=(css::PRESETS)
                        >
                            for (label, from, to, pressed) in presets {
                                <button
                                    type="button"
                                    class=(class!(button_css::ROOT, css::PRESET))
                                    data-variant="ghost"
                                    data-size="sm"
                                    aria-pressed=(if pressed { "true" } else { "false" })
                                    data-from=(from)
                                    data-to=(to)
                                >
                                    (label)
                                </button>
                            }
                        </div>
                    }
                    calendar(
                        id: &calendar_id,
                        name: name,
                        mode: calendar_mode,
                        selected: &days,
                        today: today,
                        min: min,
                        max: max,
                        disabled_dates: disabled_dates,
                        disabled_weekdays: disabled_weekdays,
                        week_starts_on: week_starts_on,
                        hide_outside_days: hide_outside_days,
                        number_of_months: number_of_months,
                        dropdown: dropdown,
                        year_range: year_range,
                        required: true,
                        disabled: disabled,
                        labels: labels
                    )
                </div>
                <div class=(css::FOOTER)>(footer)</div>
            </div>
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::{CalendarLabels, Date, DateRangePreset, date_range_presets, format_date};

    #[test]
    fn formats_the_button_text() {
        let labels = CalendarLabels::default();
        let day = Date::new(2026, 11, 2).unwrap();
        assert_eq!(
            format_date("{month} {day}, {year}", &labels, day),
            "Nov 2, 2026"
        );
        assert_eq!(
            format_date("{day} {month} {year}", &labels, day),
            "2 Nov 2026"
        );
    }

    #[test]
    fn presets_count_back_from_today() {
        let today = Date::new(2026, 11, 12).unwrap();
        let week = DateRangePreset::last_days(7, today);
        assert_eq!(week.label, "Last 7 days");
        assert_eq!(week.from, Date::new(2026, 11, 6).unwrap());
        assert_eq!(week.to, today);
        let all = date_range_presets(today);
        assert_eq!(all.len(), 4);
        assert_eq!(all[0].from, all[0].to);
        assert_eq!(all[3].from, Date::new(2026, 8, 15).unwrap());
    }
}
