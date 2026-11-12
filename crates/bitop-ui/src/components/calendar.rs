//! A month grid for choosing a day, several days or a range of days, on the WAI-ARIA
//! date grid pattern.
//!
//! The server renders the grid, with [`Date`] doing the calendar arithmetic. The
//! library script then changes the month and the selection without a request, and
//! adds the keyboard: with focus on a day, the arrow keys move by day and by week
//! (mirrored in a right-to-left page), Home and End go to the start and end of the
//! week, PageUp and PageDown change the month and with Shift the year; Enter or Space
//! selects. Only one day is in the tab order, and the grid follows the focused day
//! into other months.
//!
//! The selection is submitted with the form under `name`, through a hidden input:
//! `2026-11-12` for one day, `2026-11-10/2026-11-12` for a range and
//! `2026-11-10,2026-11-12` for several days. A `change` event fires on that input,
//! and `bitop:change` on the calendar with `detail.values` (the ISO dates). A script
//! can read and set the selection with `bitopUi.calendar.values(element)` and
//! `bitopUi.calendar.setValue(element, ["2026-11-12"])`.
//!
//! Selection is shown by a filled day and `aria-selected`, today by bold underlined
//! text and `aria-current`, and an unavailable day is struck through and has
//! `aria-disabled`, so no state relies on colour alone.
//!
//! Differences from upstream: month and weekday names are not taken from
//! `Intl.DateTimeFormat`, because the server and the script must render the same
//! text; they are in [`CalendarLabels`], in English by default. `isDateDisabled` is a
//! function there; here it is the lists `disabled_dates` and `disabled_weekdays`.
//! `renderDay` and `renderDayContent` are not ported: the script rebuilds the days
//! when the month changes and could not rebuild custom content. When `today` is not
//! given it is the server's date in UTC; pass the person's own date if their time
//! zone is known.
//!
//! ```ignore
//! let id = element_id(cx);
//! let selected = [Date::new(2026, 11, 12).unwrap()];
//! view! {
//!     calendar(id: &id, name: "day", selected: &selected)
//!     calendar(id: &other, name: "stay", mode: CalendarMode::Range, number_of_months: 2)
//! }
//! ```

use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::{button as button_css, calendar as css};

/// A day in the proleptic Gregorian calendar, with no time and no time zone.
///
/// Dates order by time, and print and parse as ISO 8601 (`2026-11-12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    /// The date, or `None` when it does not exist, such as 30 February. Years are
    /// 1 to 9999.
    #[must_use]
    pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        let valid = (1..=9999).contains(&year)
            && (1..=12).contains(&month)
            && day >= 1
            && day <= Self::days_in_month(year, month);
        valid.then_some(Self { year, month, day })
    }

    /// Parses `YYYY-MM-DD`; `None` when the text is not a date.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let digits = |range: std::ops::Range<usize>| {
            let part = &text[range];
            part.bytes()
                .all(|b| b.is_ascii_digit())
                .then(|| part.parse::<i32>().ok())
                .flatten()
        };
        Self::new(
            digits(0..4)?,
            u8::try_from(digits(5..7)?).ok()?,
            u8::try_from(digits(8..10)?).ok()?,
        )
    }

    /// Today's date in UTC, from the system clock.
    #[must_use]
    pub fn today_utc() -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        Self::from_days(i64::try_from(seconds / 86_400).unwrap_or(0))
    }

    /// The year.
    #[must_use]
    pub fn year(self) -> i32 {
        self.year
    }

    /// The month, 1 to 12.
    #[must_use]
    pub fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, 1 to 31.
    #[must_use]
    pub fn day(self) -> u8 {
        self.day
    }

    /// Whether `year` has 29 February.
    #[must_use]
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    /// The number of days in a month; 0 when `month` is not 1 to 12.
    #[must_use]
    pub fn days_in_month(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 0,
        }
    }

    /// Days since 1 January 1970. (Howard Hinnant's `days_from_civil`.)
    fn to_days(self) -> i64 {
        let year = i64::from(self.year) - i64::from(self.month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year.rem_euclid(400);
        let month = i64::from(self.month);
        let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5
            + i64::from(self.day)
            - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// The date that is `days` after 1 January 1970, kept within years 1 to 9999.
    fn from_days(days: i64) -> Self {
        let days = days.clamp(Self::MIN.to_days(), Self::MAX.to_days()) + 719_468;
        let era = days.div_euclid(146_097);
        let day_of_era = days.rem_euclid(146_097);
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
        let month = if shifted_month < 10 {
            shifted_month + 3
        } else {
            shifted_month - 9
        };
        let year = year_of_era + era * 400 + i64::from(month <= 2);
        Self {
            year: i32::try_from(year).unwrap_or(1),
            month: u8::try_from(month).unwrap_or(1),
            day: u8::try_from(day).unwrap_or(1),
        }
    }

    const MIN: Self = Self {
        year: 1,
        month: 1,
        day: 1,
    };
    const MAX: Self = Self {
        year: 9999,
        month: 12,
        day: 31,
    };

    /// The day of the week: 0 is Sunday, 6 is Saturday.
    #[must_use]
    pub fn weekday(self) -> u8 {
        // 1 January 1970 was a Thursday.
        u8::try_from((self.to_days() + 4).rem_euclid(7)).unwrap_or(0)
    }

    /// The date `days` later, or earlier when `days` is negative.
    #[must_use]
    pub fn add_days(self, days: i64) -> Self {
        Self::from_days(self.to_days().saturating_add(days))
    }

    /// The same day `months` later, or the month's last day when it is shorter:
    /// 31 January plus one month is 28 or 29 February.
    #[must_use]
    pub fn add_months(self, months: i32) -> Self {
        let index = (self.year * 12 + i32::from(self.month) - 1).saturating_add(months);
        let year = index.div_euclid(12);
        let month = u8::try_from(index.rem_euclid(12)).unwrap_or(0) + 1;
        if year < 1 {
            return Self::MIN;
        }
        if year > 9999 {
            return Self::MAX;
        }
        Self {
            year,
            month,
            day: self.day.min(Self::days_in_month(year, month)),
        }
    }

    /// The first day of this date's month.
    #[must_use]
    pub fn start_of_month(self) -> Self {
        Self { day: 1, ..self }
    }

    /// The last day of this date's month.
    #[must_use]
    pub fn end_of_month(self) -> Self {
        Self {
            day: Self::days_in_month(self.year, self.month),
            ..self
        }
    }

    /// The first day of this date's week, when weeks start on `week_starts_on`
    /// (0 is Sunday).
    #[must_use]
    pub fn start_of_week(self, week_starts_on: u8) -> Self {
        let back = (self.weekday() + 7 - week_starts_on % 7) % 7;
        self.add_days(-i64::from(back))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// What a [`calendar`] selects. Rendered as `data-mode`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    /// One day.
    #[default]
    Single,
    /// Any number of separate days.
    Multiple,
    /// A first and a last day, and the days between them.
    Range,
}

impl CalendarMode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
            Self::Range => "range",
        }
    }

    /// What separates the dates in the submitted value.
    fn separator(self) -> &'static str {
        if self == Self::Range { "/" } else { "," }
    }
}

/// The text of a [`calendar`]: names, accessible labels and formats. The default is
/// English; build another with struct update syntax.
///
/// ```ignore
/// CalendarLabels { previous_month: "Mois précédent", ..CalendarLabels::default() }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarLabels<'a> {
    /// The name of the button that goes back a month.
    pub previous_month: &'a str,
    /// The name of the button that goes forward a month.
    pub next_month: &'a str,
    /// The name of the month select.
    pub month: &'a str,
    /// The name of the year select.
    pub year: &'a str,
    /// The months, January first.
    pub months: [&'a str; 12],
    /// The months in short, January first; the date picker's button uses them.
    pub months_short: [&'a str; 12],
    /// The days of the week, Sunday first.
    pub weekdays: [&'a str; 7],
    /// The days of the week in short, Sunday first; the column headings.
    pub weekdays_short: [&'a str; 7],
    /// The heading of a month. `{month}` and `{year}` are replaced.
    pub caption: &'a str,
    /// The accessible name of a day. `{weekday}`, `{month}`, `{day}` and `{year}` are
    /// replaced.
    pub date: &'a str,
    /// The accessible name of a selected day. `{date}` is replaced with `date`.
    pub selected: &'a str,
}

impl Default for CalendarLabels<'_> {
    fn default() -> Self {
        Self {
            previous_month: "Previous month",
            next_month: "Next month",
            month: "Month",
            year: "Year",
            months: [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ],
            months_short: [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ],
            weekdays: [
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ],
            weekdays_short: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
            caption: "{month} {year}",
            date: "{weekday}, {month} {day}, {year}",
            selected: "{date}, selected",
        }
    }
}

impl CalendarLabels<'_> {
    fn month_name(&self, date: Date) -> &str {
        self.months[usize::from(date.month() - 1)]
    }

    fn caption_for(&self, month: Date) -> String {
        self.caption
            .replace("{month}", self.month_name(month))
            .replace("{year}", &month.year().to_string())
    }

    fn date_for(&self, date: Date) -> String {
        self.date
            .replace("{weekday}", self.weekdays[usize::from(date.weekday())])
            .replace("{month}", self.month_name(date))
            .replace("{day}", &date.day().to_string())
            .replace("{year}", &date.year().to_string())
    }
}

/// The selection, as the grid shows it.
struct Selection<'a> {
    mode: CalendarMode,
    days: &'a [Date],
}

/// How one day is selected.
#[derive(Clone, Copy, Default)]
struct Picked {
    selected: bool,
    range_start: bool,
    range_end: bool,
    range_middle: bool,
}

impl Selection<'_> {
    fn of(&self, date: Date) -> Picked {
        if self.mode != CalendarMode::Range {
            return Picked {
                selected: self.days.contains(&date),
                ..Picked::default()
            };
        }
        let Some(&from) = self.days.first() else {
            return Picked::default();
        };
        let to = self.days.get(1).copied().unwrap_or(from);
        let (start, end) = (date == from, date == to);
        let middle = date > from && date < to;
        // A one-day range is drawn as a single selected day, with no band.
        let spans = from != to;
        Picked {
            selected: start || end || middle,
            range_start: start && spans,
            range_end: end && spans,
            range_middle: middle,
        }
    }
}

/// One day of the grid, as it is rendered.
struct DayView {
    iso: String,
    number: String,
    label: String,
    outside: bool,
    picked: Picked,
    today: bool,
    disabled: bool,
    tab_stop: bool,
}

/// One month of the grid, as it is rendered.
struct MonthView {
    caption_id: String,
    caption: String,
    dropdown: bool,
    /// Value, name, selected, disabled.
    month_options: Vec<(String, String, bool, bool)>,
    /// Year, selected.
    year_options: Vec<(String, bool)>,
    weeks: Vec<Vec<Option<DayView>>>,
}

/// The day that is in the tab order: the first selected day or today when the months
/// shown include it, otherwise the first day shown; kept within `min` and `max`.
fn tab_stop(
    selection: &Selection<'_>,
    today: Date,
    first: Date,
    last: Date,
    min: Option<Date>,
    max: Option<Date>,
) -> Date {
    let shown = |date: &Date| *date >= first && *date <= last;
    let clamp = |date: Date| {
        let clamped = match (min, max) {
            (Some(min), _) if date < min => min,
            (_, Some(max)) if date > max => max,
            _ => date,
        };
        if shown(&clamped) { clamped } else { date }
    };
    let selected = match selection.mode {
        CalendarMode::Range => selection.days.first().into_iter().collect::<Vec<_>>(),
        _ => selection.days.iter().collect(),
    };
    selected
        .into_iter()
        .chain([&today])
        .find(|date| shown(date))
        .map_or_else(|| clamp(first), |date| clamp(*date))
}

/// A calendar.
///
/// - `id` goes on the wrapper; `name` on the hidden input that holds the selection.
/// - `selected` is the selection: at most one day for `Single`, the first and last day
///   for `Range` (or only the first), any days for `Multiple`.
/// - `month` is any day of the first month shown. It defaults to the month of the
///   first selected day, else of `today`.
/// - `today` is the day marked as today; by default the server's date in UTC.
/// - `min` and `max` are the earliest and latest days that can be chosen.
///   `disabled_dates` and `disabled_weekdays` (0 is Sunday) mark more days
///   unavailable; they can still be focused and are announced as disabled.
/// - `week_starts_on` is the first day of the week: 0 is Sunday, 1 Monday.
/// - `hide_outside_days` leaves out the days of the previous and next month that fill
///   the first and last week.
/// - `number_of_months` shows several months side by side.
/// - `dropdown` adds month and year selects for quick navigation; `year_range` is
///   the first and last year offered, by default `min` and `max`, else 100 years back
///   and 10 ahead.
/// - `required` keeps a `Single` selection when the selected day is clicked again.
/// - `max_selected` limits a `Multiple` selection; 0 means no limit.
/// - `disabled` disables the whole calendar.
/// - `attrs` are forwarded to the wrapper.
#[component]
pub async fn calendar(
    cx: &Cx,
    id: &str,
    #[default] name: &str,
    #[default] mode: CalendarMode,
    #[default] selected: &[Date],
    #[default] month: Option<Date>,
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
    #[default] required: bool,
    #[default] max_selected: usize,
    #[default] disabled: bool,
    #[default] labels: CalendarLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let today = today.unwrap_or_else(Date::today_utc);
    let week_starts_on = week_starts_on % 7;
    let count = i32::from(number_of_months.max(1));

    // The selection in the order the grid and the form expect.
    let mut days: Vec<Date> = selected.to_vec();
    match mode {
        CalendarMode::Single => days.truncate(1),
        CalendarMode::Range => {
            days.truncate(2);
            days.sort_unstable();
        }
        CalendarMode::Multiple => {
            days.sort_unstable();
            days.dedup();
        }
    }
    let selection = Selection { mode, days: &days };
    let value = days
        .iter()
        .map(Date::to_string)
        .collect::<Vec<_>>()
        .join(mode.separator());

    let first = month
        .or_else(|| days.first().copied())
        .unwrap_or(today)
        .start_of_month();
    let last = first.add_months(count - 1).end_of_month();
    let stop = tab_stop(&selection, today, first, last, min, max);
    let is_disabled = |date: Date| {
        disabled
            || min.is_some_and(|min| date < min)
            || max.is_some_and(|max| date > max)
            || disabled_dates.contains(&date)
            || disabled_weekdays.contains(&date.weekday())
    };
    let (from_year, to_year) = year_range.unwrap_or((
        min.map_or(today.year() - 100, Date::year),
        max.map_or(today.year() + 10, Date::year),
    ));

    let months: Vec<MonthView> = (0..count)
        .map(|index| {
            let month = first.add_months(index);
            let end = month.end_of_month();
            let mut day = month.start_of_week(week_starts_on);
            let mut weeks = Vec::new();
            while day <= end {
                let week = (0..7)
                    .map(|_| {
                        let date = day;
                        day = day.add_days(1);
                        let outside = date.month() != month.month();
                        if outside && hide_outside_days {
                            return None;
                        }
                        let picked = selection.of(date);
                        let full = labels.date_for(date);
                        Some(DayView {
                            iso: date.to_string(),
                            number: date.day().to_string(),
                            label: if picked.selected {
                                labels.selected.replace("{date}", &full)
                            } else {
                                full
                            },
                            outside,
                            picked,
                            today: date == today,
                            disabled: is_disabled(date),
                            tab_stop: !outside && date == stop,
                        })
                    })
                    .collect();
                weeks.push(week);
            }
            let dropdown = dropdown && index == 0;
            let mut years: Vec<i32> = (from_year..=to_year).collect();
            if !years.contains(&month.year()) {
                years.push(month.year());
                years.sort_unstable();
            }
            MonthView {
                caption_id: format!("{id}-caption-{index}"),
                caption: labels.caption_for(month),
                dropdown,
                month_options: if dropdown {
                    (1..=12u8)
                        .map(|number| {
                            let option = Date {
                                month: number,
                                ..month
                            };
                            let out = min.is_some_and(|min| option.end_of_month() < min)
                                || max.is_some_and(|max| option > max);
                            (
                                (number - 1).to_string(),
                                labels.months[usize::from(number - 1)].to_owned(),
                                number == month.month(),
                                out,
                            )
                        })
                        .collect()
                } else {
                    Vec::new()
                },
                year_options: if dropdown {
                    years
                        .into_iter()
                        .map(|year| (year.to_string(), year == month.year()))
                        .collect()
                } else {
                    Vec::new()
                },
                weeks,
            }
        })
        .collect();
    let weekdays: Vec<(&str, &str)> = (0..7)
        .map(|offset| {
            let day = usize::from((week_starts_on + offset) % 7);
            (labels.weekdays[day], labels.weekdays_short[day])
        })
        .collect();

    let previous_disabled = disabled || min.is_some_and(|min| first.add_days(-1) < min);
    let next_disabled = disabled || max.is_some_and(|max| last.add_days(1) > max);
    let join = |dates: &[Date]| {
        (!dates.is_empty()).then(|| {
            dates
                .iter()
                .map(Date::to_string)
                .collect::<Vec<_>>()
                .join(",")
        })
    };
    let disabled_weekdays_list = (!disabled_weekdays.is_empty()).then(|| {
        disabled_weekdays
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    });

    Ok(view! {
        <div
            id=(id)
            class=(class!(css::ROOT, attrs.remove("class")))
            data-calendar=""
            data-mode=(mode.as_str())
            data-month=(format!("{:04}-{:02}", first.year(), first.month()))
            data-today=(today.to_string())
            data-min=(min.map(|date| date.to_string()))
            data-max=(max.map(|date| date.to_string()))
            data-disabled-dates=(join(disabled_dates))
            data-disabled-weekdays=(disabled_weekdays_list)
            data-week-start=(week_starts_on.to_string())
            data-hide-outside=(hide_outside_days)
            data-required=(required)
            data-max-selected=((max_selected > 0).then(|| max_selected.to_string()))
            data-disabled=(disabled)
            data-month-names=(labels.months.join("|"))
            data-month-names-short=(labels.months_short.join("|"))
            data-weekday-names=(labels.weekdays.join("|"))
            data-caption-format=(labels.caption)
            data-date-format=(labels.date)
            data-selected-format=(labels.selected)
            (attrs)
        >
            <div class=(css::NAV)>
                <button
                    type="button"
                    aria-label=(labels.previous_month)
                    class=(class!(button_css::ROOT, css::NAV_BUTTON))
                    data-variant="ghost"
                    data-size="icon"
                    data-calendar-previous=""
                    disabled=(previous_disabled)
                >
                    icon(name: Icon::ChevronLeft)
                </button>
                <button
                    type="button"
                    aria-label=(labels.next_month)
                    class=(class!(button_css::ROOT, css::NAV_BUTTON))
                    data-variant="ghost"
                    data-size="icon"
                    data-calendar-next=""
                    disabled=(next_disabled)
                >
                    icon(name: Icon::ChevronRight)
                </button>
            </div>
            <div class=(css::MONTHS)>
                for month in months {
                    <div class=(css::MONTH) data-calendar-month="">
                        <div
                            class=(css::CAPTION)
                            data-layout=(if month.dropdown {
                                "dropdown"
                            } else {
                                "label"
                            })
                        >
                            <span
                                id=(&month.caption_id)
                                aria-live="polite"
                                class=(class!(css::CAPTION_LABEL, month.dropdown.then_some("sr-only")))
                                data-calendar-caption=""
                            >
                                (month.caption)
                            </span>
                            if month.dropdown {
                                <span class=(css::SELECT_WRAP)>
                                    <select
                                        aria-label=(labels.month)
                                        class=(css::SELECT)
                                        data-calendar-select="month"
                                        disabled=(disabled)
                                    >
                                        for (value, name, selected, out) in month.month_options {
                                            <option value=(value) selected=(selected) disabled=(out)>
                                                (name)
                                            </option>
                                        }
                                    </select>
                                </span>
                                <span class=(css::SELECT_WRAP)>
                                    <select
                                        aria-label=(labels.year)
                                        class=(css::SELECT)
                                        data-calendar-select="year"
                                        disabled=(disabled)
                                    >
                                        for (year, selected) in month.year_options {
                                            <option value=(&year) selected=(selected)>(&year)</option>
                                        }
                                    </select>
                                </span>
                            }
                        </div>
                        <table
                            role="grid"
                            aria-labelledby=(&month.caption_id)
                            aria-multiselectable=((mode != CalendarMode::Single).then_some(
                                "true",
                            ))
                            aria-disabled=(disabled.then_some("true"))
                            class=(css::GRID)
                        >
                            <thead>
                                <tr>
                                    for (long, short) in &weekdays {
                                        <th scope="col" abbr=(*long) class=(css::WEEKDAY)>
                                            (*short)
                                        </th>
                                    }
                                </tr>
                            </thead>
                            <tbody>
                                for week in month.weeks {
                                    <tr>
                                        for day in week {
                                            if let Some(day) = day {
                                                <td
                                                    role="gridcell"
                                                    aria-selected=(if day.picked.selected {
                                                        "true"
                                                    } else {
                                                        "false"
                                                    })
                                                    class=(css::CELL)
                                                    data-outside=(day.outside)
                                                    data-range-start=(day.picked.range_start)
                                                    data-range-end=(day.picked.range_end)
                                                    data-range-middle=(day.picked.range_middle)
                                                >
                                                    <button
                                                        type="button"
                                                        class=(css::DAY)
                                                        data-day=((!day.outside).then_some(&day.iso))
                                                        data-date=(&day.iso)
                                                        tabindex=(if day.tab_stop { "0" } else { "-1" })
                                                        aria-label=(&day.label)
                                                        aria-current=(day.today.then_some("date"))
                                                        aria-disabled=(day.disabled.then_some("true"))
                                                        data-outside=(day.outside)
                                                    >
                                                        <span aria-hidden="true" class=(css::DAY_NUMBER)>
                                                            (&day.number)
                                                        </span>
                                                    </button>
                                                </td>
                                            } else {
                                                <td role="gridcell" class=(css::CELL)></td>
                                            }
                                        }
                                    </tr>
                                }
                            </tbody>
                        </table>
                    </div>
                }
            </div>
            <input
                type="hidden"
                name=((!name.is_empty()).then_some(name))
                value=(&value)
                disabled=(disabled)
            >
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::{CalendarLabels, CalendarMode, Date, Selection, tab_stop};

    fn date(year: i32, month: u8, day: u8) -> Date {
        Date::new(year, month, day).unwrap()
    }

    #[test]
    fn rejects_dates_that_do_not_exist() {
        assert!(Date::new(2026, 2, 28).is_some());
        assert!(Date::new(2026, 2, 29).is_none());
        assert!(Date::new(2024, 2, 29).is_some());
        assert!(Date::new(1900, 2, 29).is_none());
        assert!(Date::new(2000, 2, 29).is_some());
        assert!(Date::new(2026, 13, 1).is_none());
        assert!(Date::new(2026, 0, 1).is_none());
        assert!(Date::new(2026, 4, 31).is_none());
        assert!(Date::new(0, 1, 1).is_none());
    }

    #[test]
    fn parses_and_prints_iso() {
        assert_eq!(Date::parse("2026-11-12"), Some(date(2026, 11, 12)));
        assert_eq!(date(2026, 1, 5).to_string(), "2026-01-05");
        assert_eq!(date(987, 1, 5).to_string(), "0987-01-05");
        for bad in [
            "",
            "2026-11",
            "2026-02-30",
            "2026/11/12",
            "20261112xx",
            "2026-1-05x",
            "+026-11-12",
        ] {
            assert_eq!(Date::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn days_round_trip() {
        assert_eq!(date(1970, 1, 1).to_days(), 0);
        assert_eq!(date(1969, 12, 31).to_days(), -1);
        assert_eq!(date(2000, 3, 1).to_days(), 11_017);
        assert_eq!(date(2026, 11, 12).to_days(), 20_769);
        // Every day of 800 years, across the 400-year cycle, in order.
        let mut day = date(1600, 1, 1);
        let mut days = day.to_days();
        while day < date(2400, 1, 1) {
            assert_eq!(Date::from_days(days), day);
            assert_eq!(day.to_days(), days);
            let next = day.add_days(1);
            assert!(next > day);
            assert!(Date::new(next.year(), next.month(), next.day()).is_some());
            day = next;
            days += 1;
        }
        assert_eq!(day, date(2400, 1, 1));
    }

    #[test]
    fn weekdays() {
        assert_eq!(date(1970, 1, 1).weekday(), 4);
        assert_eq!(date(2023, 1, 1).weekday(), 0);
        assert_eq!(date(2026, 11, 12).weekday(), 4);
        assert_eq!(date(2024, 2, 29).weekday(), 4);
        assert_eq!(date(1900, 1, 1).weekday(), 1);
    }

    #[test]
    fn adds_days() {
        assert_eq!(date(2026, 12, 31).add_days(1), date(2027, 1, 1));
        assert_eq!(date(2024, 2, 28).add_days(1), date(2024, 2, 29));
        assert_eq!(date(2026, 3, 1).add_days(-1), date(2026, 2, 28));
        assert_eq!(date(2026, 11, 12).add_days(-7), date(2026, 11, 5));
        assert_eq!(date(9999, 12, 31).add_days(1), date(9999, 12, 31));
        assert_eq!(date(1, 1, 1).add_days(-1), date(1, 1, 1));
    }

    #[test]
    fn adds_months_and_clamps_the_day() {
        assert_eq!(date(2026, 1, 31).add_months(1), date(2026, 2, 28));
        assert_eq!(date(2024, 1, 31).add_months(1), date(2024, 2, 29));
        assert_eq!(date(2026, 11, 12).add_months(2), date(2027, 1, 12));
        assert_eq!(date(2026, 1, 15).add_months(-1), date(2025, 12, 15));
        assert_eq!(date(2026, 3, 31).add_months(-13), date(2025, 2, 28));
        assert_eq!(date(2026, 5, 31).add_months(12), date(2027, 5, 31));
        assert_eq!(date(9999, 6, 1).add_months(12), date(9999, 12, 31));
    }

    #[test]
    fn month_and_week_edges() {
        assert_eq!(date(2026, 11, 12).start_of_month(), date(2026, 11, 1));
        assert_eq!(date(2026, 11, 12).end_of_month(), date(2026, 11, 30));
        assert_eq!(date(2024, 2, 10).end_of_month(), date(2024, 2, 29));
        // 12 November 2026 is a Thursday.
        assert_eq!(date(2026, 11, 12).start_of_week(0), date(2026, 11, 8));
        assert_eq!(date(2026, 11, 12).start_of_week(1), date(2026, 11, 9));
        assert_eq!(date(2026, 11, 8).start_of_week(0), date(2026, 11, 8));
        assert_eq!(date(2026, 11, 1).start_of_week(1), date(2026, 10, 26));
    }

    #[test]
    fn orders_by_time() {
        assert!(date(2026, 11, 12) < date(2026, 11, 13));
        assert!(date(2026, 11, 30) < date(2026, 12, 1));
        assert!(date(2026, 12, 31) < date(2027, 1, 1));
    }

    #[test]
    fn labels_format_dates() {
        let labels = CalendarLabels::default();
        assert_eq!(labels.caption_for(date(2026, 11, 1)), "November 2026");
        assert_eq!(
            labels.date_for(date(2026, 11, 12)),
            "Thursday, November 12, 2026"
        );
    }

    #[test]
    fn range_selection() {
        let days = [date(2026, 11, 10), date(2026, 11, 12)];
        let range = Selection {
            mode: CalendarMode::Range,
            days: &days,
        };
        let start = range.of(date(2026, 11, 10));
        assert!(start.selected && start.range_start && !start.range_end && !start.range_middle);
        let middle = range.of(date(2026, 11, 11));
        assert!(middle.selected && middle.range_middle && !middle.range_start);
        let end = range.of(date(2026, 11, 12));
        assert!(end.selected && end.range_end && !end.range_start);
        assert!(!range.of(date(2026, 11, 13)).selected);

        // Only a first day: one selected day, and no band.
        let open = Selection {
            mode: CalendarMode::Range,
            days: &days[..1],
        };
        let only = open.of(date(2026, 11, 10));
        assert!(only.selected && !only.range_start && !only.range_end);
        assert!(!open.of(date(2026, 11, 11)).selected);
    }

    #[test]
    fn tab_stops() {
        let today = date(2026, 11, 12);
        let (first, last) = (date(2026, 11, 1), date(2026, 11, 30));
        let none = Selection {
            mode: CalendarMode::Single,
            days: &[],
        };
        assert_eq!(tab_stop(&none, today, first, last, None, None), today);
        let days = [date(2026, 11, 20)];
        let picked = Selection {
            mode: CalendarMode::Single,
            days: &days,
        };
        assert_eq!(tab_stop(&picked, today, first, last, None, None), days[0]);
        // Another month: the first day shown, or the first that can be chosen.
        let (first, last) = (date(2027, 1, 1), date(2027, 1, 31));
        assert_eq!(tab_stop(&none, today, first, last, None, None), first);
        assert_eq!(
            tab_stop(&none, today, first, last, Some(date(2027, 1, 10)), None),
            date(2027, 1, 10)
        );
        // Today is before the minimum: the minimum.
        let (first, last) = (date(2026, 11, 1), date(2026, 11, 30));
        assert_eq!(
            tab_stop(&none, today, first, last, Some(date(2026, 11, 15)), None),
            date(2026, 11, 15)
        );
    }
}
