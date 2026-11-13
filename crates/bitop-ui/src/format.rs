//! Formatting helpers for numbers, byte sizes, dates and relative times: the Rust
//! port of bitop-ui's `bitop-format`.
//!
//! Upstream formats with `Intl` in the visitor's locale. A server does not know the
//! visitor's locale or time zone, so these functions write English in UTC. They give
//! the text a page shows before a script runs, and what it keeps showing without
//! one; the [`time`](crate::components::time) part then localises its text in the
//! browser.
//!
//! ```
//! use bitop_ui::format::{DateStyle, PluralForms, format_bytes, format_date, format_number, plural};
//!
//! assert_eq!(format_number(1_234_567.0, 3, false), "1,234,567");
//! assert_eq!(format_number(1_234_567.0, 1, true), "1.2M");
//! assert_eq!(format_bytes(1_500_000.0, false, None), "1.5 MB");
//! assert_eq!(format_bytes(1536.0, true, None), "1.5 KiB");
//! assert_eq!(format_date(1_790_424_000, DateStyle::Date), "Sep 26, 2026");
//! assert_eq!(plural(3.0, &PluralForms::new("file", "files")), "3 files");
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

use crate::components::calendar::Date;

/// The shape of a formatted date. Upstream's `DateStyle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateStyle {
    /// "Sep 26, 2026"
    Date,
    /// "Sep 26, 2026, 12:00 PM"
    #[default]
    Datetime,
    /// "12:00 PM"
    Time,
    /// "Saturday, September 26, 2026 at 12:00:00 PM UTC"
    Long,
}

impl DateStyle {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Datetime => "datetime",
            Self::Time => "time",
            Self::Long => "long",
        }
    }
}

pub(crate) const MONTHS: [&str; 12] = [
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
];
const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const MINUTE: f64 = 60.0;
const HOUR: f64 = 60.0 * MINUTE;
const DAY: f64 = 24.0 * HOUR;
const WEEK: f64 = 7.0 * DAY;
/// The average Gregorian month.
const MONTH: f64 = 30.436_875 * DAY;
const YEAR: f64 = 365.2425 * DAY;

/// The current time as a Unix timestamp in seconds, from the system clock.
#[must_use]
pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| i64::try_from(elapsed.as_secs()).unwrap_or(0))
}

/// A Unix timestamp split into its UTC date and its time of day.
fn civil(timestamp: i64) -> (Date, u32, u32, u32) {
    let days = timestamp.div_euclid(86_400);
    let seconds = u32::try_from(timestamp.rem_euclid(86_400)).unwrap_or(0);
    (
        Date::from_days(days),
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60,
    )
}

/// A Unix timestamp (seconds) as an ISO 8601 instant in UTC: `2026-09-26T12:00:00Z`.
/// This is what a `<time datetime>` wants.
#[must_use]
pub fn to_iso(timestamp: i64) -> String {
    let (date, hour, minute, second) = civil(timestamp);
    format!("{date}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Parses an ISO 8601 date or instant into a Unix timestamp in seconds.
///
/// Accepts `2026-09-26`, `2026-09-26T12:00`, `2026-09-26T12:00:00`, fractions of a
/// second (dropped), and a zone of `Z`, `+02:00` or `-0530`. A space may stand for the
/// `T`. Text with no zone is read as UTC. `None` when the text is not a date.
#[must_use]
pub fn parse_iso(text: &str) -> Option<i64> {
    let text = text.trim();
    let date = Date::parse(text.get(..10)?)?;
    let mut seconds = date.to_days() * 86_400;
    let rest = &text[10..];
    if rest.is_empty() {
        return Some(seconds);
    }
    let rest = rest
        .strip_prefix('T')
        .or_else(|| rest.strip_prefix('t'))
        .or_else(|| rest.strip_prefix(' '))?;
    let zone_at = rest.find(['Z', 'z', '+', '-']).unwrap_or(rest.len());
    let (clock, zone) = rest.split_at(zone_at);
    let clock = clock.split('.').next()?;
    let mut parts = clock.split(':');
    let mut field = |required: bool, limit: i64| -> Option<i64> {
        match parts.next() {
            Some(part) if part.len() == 2 && part.bytes().all(|b| b.is_ascii_digit()) => {
                let value = part.parse::<i64>().ok()?;
                (value < limit).then_some(value)
            }
            None if !required => Some(0),
            _ => None,
        }
    };
    let hour = field(true, 24)?;
    let minute = field(true, 60)?;
    // 60 allows a leap second, which lands on the next minute.
    let second = field(false, 61)?;
    if parts.next().is_some() {
        return None;
    }
    seconds += hour * 3600 + minute * 60 + second;
    match zone {
        "" | "Z" | "z" => {}
        _ => {
            let sign = if zone.starts_with('-') { 1 } else { -1 };
            let digits: String = zone[1..].chars().filter(|c| *c != ':').collect();
            if !(digits.len() == 2 || digits.len() == 4)
                || !digits.bytes().all(|b| b.is_ascii_digit())
            {
                return None;
            }
            let hours = digits[..2].parse::<i64>().ok()?;
            let minutes = digits
                .get(2..4)
                .map_or(Some(0), |m| m.parse::<i64>().ok())?;
            if hours > 23 || minutes > 59 {
                return None;
            }
            seconds += sign * (hours * 3600 + minutes * 60);
        }
    }
    Some(seconds)
}

/// Formats a Unix timestamp (seconds) in UTC, in English, in the given style.
///
/// The text is what `Intl.DateTimeFormat("en-US", { timeZone: "UTC" })` gives for
/// upstream's styles, with a plain space before AM and PM.
#[must_use]
pub fn format_date(timestamp: i64, style: DateStyle) -> String {
    let (date, hour, minute, second) = civil(timestamp);
    let month = MONTHS[usize::from(date.month()) - 1];
    let half = if hour < 12 { "AM" } else { "PM" };
    let hour12 = if hour % 12 == 0 { 12 } else { hour % 12 };
    let short_date = format!("{} {}, {}", &month[..3], date.day(), date.year());
    let short_time = format!("{hour12}:{minute:02} {half}");
    match style {
        DateStyle::Date => short_date,
        DateStyle::Datetime => format!("{short_date}, {short_time}"),
        DateStyle::Time => short_time,
        DateStyle::Long => format!(
            "{}, {month} {}, {} at {hour12}:{minute:02}:{second:02} {half} UTC",
            WEEKDAYS[usize::from(date.weekday())],
            date.day(),
            date.year()
        ),
    }
}

/// Formats a number with grouping: "1,234,567".
///
/// - `max_fraction_digits` is how many decimals to keep at most; trailing zeros are
///   dropped. Upstream's default is 3, or 1 when `compact`.
/// - `compact` gives the short form: "1.2K", "3.4M".
///
/// NaN gives "". Infinity gives "∞".
#[must_use]
pub fn format_number(n: f64, max_fraction_digits: u8, compact: bool) -> String {
    if n.is_nan() {
        return String::new();
    }
    if n.is_infinite() {
        return if n < 0.0 { "-∞" } else { "∞" }.to_owned();
    }
    let digits = usize::from(max_fraction_digits.min(15));
    let (mut value, mut suffix) = (n.abs(), "");
    if compact {
        const UNITS: [&str; 5] = ["", "K", "M", "B", "T"];
        let mut unit = 0;
        while value >= 1000.0 && unit < UNITS.len() - 1 {
            value /= 1000.0;
            unit += 1;
        }
        // Rounding can reach the next unit ("1000K"): promote it.
        if unit < UNITS.len() - 1 && rounded(value, digits) >= 1000.0 {
            value /= 1000.0;
            unit += 1;
        }
        suffix = UNITS[unit];
    }
    let text = format!("{value:.digits$}");
    let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
    let fraction = fraction.trim_end_matches('0');
    let mut out = String::new();
    let zero = whole.bytes().all(|b| b == b'0') && fraction.is_empty();
    if n < 0.0 && !zero {
        out.push('-');
    }
    for (index, digit) in whole.chars().enumerate() {
        if index > 0 && (whole.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    if !fraction.is_empty() {
        out.push('.');
        out.push_str(fraction);
    }
    out.push_str(suffix);
    out
}

/// `value` rounded to `digits` decimals, as formatting will round it.
fn rounded(value: f64, digits: usize) -> f64 {
    format!("{value:.digits$}").parse().unwrap_or(value)
}

/// Formats a byte count: "999 B", "1.5 kB", "42 MB"; with `binary`, powers of 1024
/// and IEC units: "1.5 KiB".
///
/// `max_fraction_digits` defaults to 1 below 10 of a unit and 0 above. Whole bytes
/// never show decimals. A value that is not finite gives "".
#[must_use]
pub fn format_bytes(bytes: f64, binary: bool, max_fraction_digits: Option<u8>) -> String {
    const DECIMAL_UNITS: [&str; 7] = ["B", "kB", "MB", "GB", "TB", "PB", "EB"];
    const BINARY_UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    if !bytes.is_finite() {
        return String::new();
    }
    let base = if binary { 1024.0 } else { 1000.0 };
    let units = if binary { BINARY_UNITS } else { DECIMAL_UNITS };
    let mut value = bytes.abs();
    let mut unit = 0;
    while value >= base && unit < units.len() - 1 {
        value /= base;
        unit += 1;
    }
    let digits = if unit == 0 {
        0
    } else {
        max_fraction_digits.unwrap_or(u8::from(value < 10.0))
    };
    if unit < units.len() - 1 && rounded(value, usize::from(digits)) >= base {
        value /= base;
        unit += 1;
    }
    let digits = if unit == 0 { 0 } else { digits };
    format!(
        "{} {}",
        format_number(value.copysign(bytes), digits, false),
        units[unit]
    )
}

/// The unit of a relative time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelativeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl RelativeUnit {
    /// The English name of the unit, in the singular.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Second => "second",
            Self::Minute => "minute",
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

/// Picks the unit for a distance in seconds, negative in the past. Each unit is used
/// until the rounded value would reach the next one, so it never says "60 minutes"
/// or "12 months".
#[must_use]
pub fn relative_time_unit(seconds: f64) -> (i64, RelativeUnit) {
    let abs = seconds.abs();
    let pick = |size: f64, unit: RelativeUnit| {
        #[allow(clippy::cast_possible_truncation)]
        let value = (abs / size).round() as i64;
        (if seconds < 0.0 { -value } else { value }, unit)
    };
    if abs < MINUTE - 0.5 {
        pick(1.0, RelativeUnit::Second)
    } else if abs < HOUR - MINUTE / 2.0 {
        pick(MINUTE, RelativeUnit::Minute)
    } else if abs < DAY - HOUR / 2.0 {
        pick(HOUR, RelativeUnit::Hour)
    } else if abs < WEEK - DAY / 2.0 {
        pick(DAY, RelativeUnit::Day)
    } else if abs < MONTH {
        pick(WEEK, RelativeUnit::Week)
    } else if abs < YEAR - MONTH / 2.0 {
        pick(MONTH, RelativeUnit::Month)
    } else {
        pick(YEAR, RelativeUnit::Year)
    }
}

/// Whole calendar days from `now` to `timestamp` in UTC, negative in the past.
#[must_use]
pub fn calendar_day_diff(timestamp: i64, now: i64) -> i64 {
    timestamp.div_euclid(86_400) - now.div_euclid(86_400)
}

/// What [`format_relative_time`] says: a count of a unit, negative in the past.
///
/// From half a day to a week away it counts calendar days (in UTC), so "yesterday"
/// is always the day before today; closer, hours and minutes; further, weeks, months
/// and years.
#[must_use]
pub fn relative_time(timestamp: i64, now: i64) -> (i64, RelativeUnit) {
    #[allow(clippy::cast_precision_loss)]
    let seconds = timestamp.saturating_sub(now) as f64;
    let days = calendar_day_diff(timestamp, now);
    if seconds.abs() >= DAY / 2.0 && days != 0 && days.abs() < 7 {
        (days, RelativeUnit::Day)
    } else {
        relative_time_unit(seconds)
    }
}

/// "3 hours ago", "yesterday", "in 2 weeks": the time from `now` to `timestamp`, both
/// Unix timestamps in seconds, in English. See [`relative_time`] for the units.
#[must_use]
pub fn format_relative_time(timestamp: i64, now: i64) -> String {
    let (value, unit) = relative_time(timestamp, now);
    let name = unit.as_str();
    match (value, unit) {
        (0, RelativeUnit::Second) => "now".to_owned(),
        (0, RelativeUnit::Day) => "today".to_owned(),
        (1, RelativeUnit::Day) => "tomorrow".to_owned(),
        (-1, RelativeUnit::Day) => "yesterday".to_owned(),
        (0, _) => format!("this {name}"),
        (1, RelativeUnit::Week | RelativeUnit::Month | RelativeUnit::Year) => {
            format!("next {name}")
        }
        (-1, RelativeUnit::Week | RelativeUnit::Month | RelativeUnit::Year) => {
            format!("last {name}")
        }
        _ => {
            #[allow(clippy::cast_precision_loss)]
            let count = format_number(value.unsigned_abs() as f64, 0, false);
            let s = if value.abs() == 1 { "" } else { "s" };
            if value < 0 {
                format!("{count} {name}{s} ago")
            } else {
                format!("in {count} {name}{s}")
            }
        }
    }
}

/// The forms [`plural`] chooses between.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PluralForms<'a> {
    /// For a count of exactly one: "file".
    pub one: &'a str,
    /// For every other count: "files".
    pub other: &'a str,
    /// For a count of exactly zero, used as it is: "No files". `{count}` is still
    /// replaced.
    pub zero: Option<&'a str>,
}

impl<'a> PluralForms<'a> {
    /// The two forms English needs.
    #[must_use]
    pub fn new(one: &'a str, other: &'a str) -> Self {
        Self {
            one,
            other,
            zero: None,
        }
    }

    /// Adds the text for a count of zero.
    #[must_use]
    pub fn zero(mut self, zero: &'a str) -> Self {
        self.zero = Some(zero);
        self
    }
}

/// Picks the plural form for `count` and includes the formatted count. A form that
/// contains `{count}` is a template; otherwise the count goes in front.
///
/// The choice follows English: `one` for exactly 1, `other` for the rest. Upstream
/// asks `Intl.PluralRules` and so has the other CLDR categories (two, few, many);
/// for those languages, choose the form in the application.
///
/// ```
/// use bitop_ui::format::{PluralForms, plural};
///
/// assert_eq!(plural(1.0, &PluralForms::new("file", "files")), "1 file");
/// assert_eq!(plural(1200.0, &PluralForms::new("file", "files")), "1,200 files");
/// assert_eq!(plural(3.0, &PluralForms::new("{count} file left", "{count} files left")), "3 files left");
/// assert_eq!(plural(0.0, &PluralForms::new("file", "files").zero("No files")), "No files");
/// ```
#[must_use]
pub fn plural(count: f64, forms: &PluralForms<'_>) -> String {
    let formatted = format_number(count, 3, false);
    #[allow(clippy::float_cmp)]
    let form = match forms.zero {
        Some(zero) if count == 0.0 => return zero.replace("{count}", &formatted),
        _ if count == 1.0 => forms.one,
        _ => forms.other,
    };
    if form.contains("{count}") {
        form.replace("{count}", &formatted)
    } else {
        format!("{formatted} {form}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2026-09-26T12:00:00Z, a Saturday.
    const NOON: i64 = 1_790_424_000;

    #[test]
    fn iso_round_trips() {
        assert_eq!(to_iso(NOON), "2026-09-26T12:00:00Z");
        assert_eq!(to_iso(0), "1970-01-01T00:00:00Z");
        assert_eq!(to_iso(-1), "1969-12-31T23:59:59Z");
        for timestamp in [0, -1, NOON, 951_782_400, 4_102_444_799] {
            assert_eq!(parse_iso(&to_iso(timestamp)), Some(timestamp));
        }
    }

    #[test]
    fn parses_iso_forms() {
        assert_eq!(parse_iso("2026-09-26"), Some(NOON - 12 * 3600));
        assert_eq!(parse_iso("2026-09-26T12:00"), Some(NOON));
        assert_eq!(parse_iso("2026-09-26 12:00:00"), Some(NOON));
        assert_eq!(parse_iso("2026-09-26T12:00:00.250Z"), Some(NOON));
        assert_eq!(parse_iso("2026-09-26T14:00:00+02:00"), Some(NOON));
        assert_eq!(parse_iso("2026-09-26T06:30:00-0530"), Some(NOON));
        assert_eq!(parse_iso(" 2026-09-26T12:00:00Z "), Some(NOON));
    }

    #[test]
    fn rejects_what_is_not_a_date() {
        for text in [
            "",
            "yesterday",
            "2026-02-30",
            "2026-09-26T25:00",
            "2026-09-26T12",
            "2026-09-26T12:60",
            "2026-09-26T12:00:00+2",
            "2026-09-26T12:00:00:00",
            "2026-09-26X12:00",
        ] {
            assert_eq!(parse_iso(text), None, "{text:?}");
        }
    }

    #[test]
    fn formats_dates_in_utc() {
        assert_eq!(format_date(NOON, DateStyle::Date), "Sep 26, 2026");
        assert_eq!(
            format_date(NOON, DateStyle::Datetime),
            "Sep 26, 2026, 12:00 PM"
        );
        assert_eq!(format_date(NOON, DateStyle::Time), "12:00 PM");
        assert_eq!(
            format_date(NOON, DateStyle::Long),
            "Saturday, September 26, 2026 at 12:00:00 PM UTC"
        );
        assert_eq!(format_date(0, DateStyle::Datetime), "Jan 1, 1970, 12:00 AM");
        assert_eq!(
            format_date(NOON + 13 * 3600 + 5 * 60 + 9, DateStyle::Long),
            "Sunday, September 27, 2026 at 1:05:09 AM UTC"
        );
        assert_eq!(
            format_date(-1, DateStyle::Datetime),
            "Dec 31, 1969, 11:59 PM"
        );
    }

    #[test]
    fn formats_numbers() {
        assert_eq!(format_number(1_234_567.0, 3, false), "1,234,567");
        assert_eq!(format_number(1234.5678, 3, false), "1,234.568");
        assert_eq!(format_number(0.5, 3, false), "0.5");
        assert_eq!(format_number(-1234.0, 3, false), "-1,234");
        assert_eq!(format_number(-0.0001, 3, false), "0");
        assert_eq!(format_number(999.0, 0, false), "999");
        assert_eq!(format_number(f64::NAN, 3, false), "");
    }

    #[test]
    fn formats_compact_numbers() {
        assert_eq!(format_number(950.0, 1, true), "950");
        assert_eq!(format_number(1234.0, 1, true), "1.2K");
        assert_eq!(format_number(12_345.0, 1, true), "12.3K");
        assert_eq!(format_number(1_234_567.0, 1, true), "1.2M");
        assert_eq!(format_number(999_999.0, 1, true), "1M");
        assert_eq!(format_number(-2_500_000_000.0, 1, true), "-2.5B");
        assert_eq!(format_number(7.0e15, 1, true), "7,000T");
    }

    #[test]
    fn formats_bytes() {
        assert_eq!(format_bytes(0.0, false, None), "0 B");
        assert_eq!(format_bytes(999.0, false, None), "999 B");
        assert_eq!(format_bytes(1500.0, false, None), "1.5 kB");
        assert_eq!(format_bytes(1_500_000.0, false, None), "1.5 MB");
        assert_eq!(format_bytes(42_000_000.0, false, None), "42 MB");
        assert_eq!(format_bytes(1536.0, true, None), "1.5 KiB");
        assert_eq!(format_bytes(999_999.0, false, None), "1 MB");
        assert_eq!(format_bytes(-1500.0, false, None), "-1.5 kB");
        assert_eq!(format_bytes(1_234_567.0, false, Some(2)), "1.23 MB");
        assert_eq!(format_bytes(512.4, false, Some(2)), "512 B");
        assert_eq!(format_bytes(f64::INFINITY, false, None), "");
    }

    #[test]
    fn picks_relative_units() {
        use RelativeUnit::*;
        assert_eq!(relative_time_unit(0.0), (0, Second));
        assert_eq!(relative_time_unit(-59.4), (-59, Second));
        assert_eq!(relative_time_unit(-59.6), (-1, Minute));
        assert_eq!(relative_time_unit(-3569.0), (-59, Minute));
        assert_eq!(relative_time_unit(-3571.0), (-1, Hour));
        assert_eq!(relative_time_unit(3.0 * 3600.0), (3, Hour));
        assert_eq!(relative_time_unit(-23.6 * 3600.0), (-1, Day));
        assert_eq!(relative_time_unit(-6.6 * 86_400.0), (-1, Week));
        assert_eq!(relative_time_unit(14.0 * 86_400.0), (2, Week));
        assert_eq!(relative_time_unit(-31.0 * 86_400.0), (-1, Month));
        assert_eq!(relative_time_unit(-200.0 * 86_400.0), (-7, Month));
        assert_eq!(relative_time_unit(-355.0 * 86_400.0), (-1, Year));
        assert_eq!(relative_time_unit(800.0 * 86_400.0), (2, Year));
    }

    #[test]
    fn formats_relative_times() {
        let ago = |seconds: i64| format_relative_time(NOON - seconds, NOON);
        assert_eq!(ago(0), "now");
        assert_eq!(ago(1), "1 second ago");
        assert_eq!(ago(5), "5 seconds ago");
        assert_eq!(ago(90), "2 minutes ago");
        assert_eq!(ago(3 * 3600), "3 hours ago");
        assert_eq!(ago(-3 * 3600), "in 3 hours");
        assert_eq!(ago(-60), "in 1 minute");
        assert_eq!(ago(14 * 86_400), "2 weeks ago");
        assert_eq!(ago(-7 * 86_400), "next week");
        assert_eq!(ago(40 * 86_400), "last month");
        assert_eq!(ago(400 * 86_400), "last year");
        assert_eq!(ago(-3 * 366 * 86_400), "in 3 years");
    }

    #[test]
    fn relative_days_follow_the_calendar() {
        // Now is noon. 14 hours ago is 22:00 the day before: yesterday.
        assert_eq!(format_relative_time(NOON - 14 * 3600, NOON), "yesterday");
        // 11 hours ago is the same day, and under half a day: hours.
        assert_eq!(format_relative_time(NOON - 11 * 3600, NOON), "11 hours ago");
        // 38 hours ago is 22:00 two days before, though it is nearer one day and a half.
        assert_eq!(format_relative_time(NOON - 38 * 3600, NOON), "2 days ago");
        assert_eq!(format_relative_time(NOON + 13 * 3600, NOON), "tomorrow");
        assert_eq!(format_relative_time(NOON + 6 * 86_400, NOON), "in 6 days");
        assert_eq!(calendar_day_diff(NOON - 14 * 3600, NOON), -1);
        assert_eq!(calendar_day_diff(NOON + 11 * 3600, NOON), 0);
        assert_eq!(calendar_day_diff(-1, 0), -1);
    }

    #[test]
    fn plurals() {
        let files = PluralForms::new("file", "files");
        assert_eq!(plural(1.0, &files), "1 file");
        assert_eq!(plural(0.0, &files), "0 files");
        assert_eq!(plural(2.0, &files), "2 files");
        assert_eq!(plural(1200.0, &files), "1,200 files");
        assert_eq!(plural(1.5, &files), "1.5 files");
        assert_eq!(plural(0.0, &files.zero("No files")), "No files");
        assert_eq!(
            plural(0.0, &files.zero("{count} files, none")),
            "0 files, none"
        );
        let left = PluralForms::new("{count} file left", "{count} files left");
        assert_eq!(plural(1.0, &left), "1 file left");
        assert_eq!(plural(3.0, &left), "3 files left");
    }
}
