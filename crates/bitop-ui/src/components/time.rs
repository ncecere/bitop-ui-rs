//! A `<time datetime="…">` with formatted text.
//!
//! ```ignore
//! view! {
//!     time(timestamp: row.created_at)                              // "Sep 26, 2026, 12:00 PM"
//!     time(timestamp: row.created_at, format: TimeFormat::Date)    // "Sep 26, 2026"
//!     time(iso: "2026-09-26T09:00:00Z", format: TimeFormat::Relative) // "3 hours ago"
//! }
//! ```
//!
//! The server does not know the visitor's locale or time zone. It renders the
//! machine-readable `datetime` and English text in UTC (marked "UTC" where the text
//! has a time of day), from [`crate::format`]. The library script then rewrites the
//! text with `Intl` in the visitor's locale and time zone, or in the `locale` and
//! `time_zone` given. A relative time has the full date as its `title`, and the script
//! keeps it fresh while it is on the page: every second under a minute, every 15
//! seconds under an hour, every 5 minutes under a day, then hourly.
//!
//! Differences from upstream: the value is a Unix timestamp in seconds or an ISO 8601
//! string, not a JavaScript `Date`. Without a script the text stays in English and
//! UTC, and a relative time is as old as the page.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::{
    css::time as css,
    format::{self, DateStyle},
};

/// How a [`time`] shows its value. Rendered as `data-time`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimeFormat {
    /// "Sep 26, 2026"
    Date,
    /// "Sep 26, 2026, 12:00 PM"
    #[default]
    Datetime,
    /// "12:00 PM"
    Time,
    /// "Saturday, September 26, 2026 at 12:00:00 PM UTC"
    Long,
    /// "3 hours ago", with the full date in the title.
    Relative,
}

impl TimeFormat {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Datetime => "datetime",
            Self::Time => "time",
            Self::Long => "long",
            Self::Relative => "relative",
        }
    }
}

/// The text the server renders for `timestamp`, in English and UTC: what a visitor
/// sees before the script runs. `now` is the reference for [`TimeFormat::Relative`].
#[must_use]
pub fn time_text(timestamp: i64, format: TimeFormat, now: i64) -> String {
    match format {
        TimeFormat::Date => format::format_date(timestamp, DateStyle::Date),
        TimeFormat::Datetime => {
            format!(
                "{} UTC",
                format::format_date(timestamp, DateStyle::Datetime)
            )
        }
        TimeFormat::Time => format!("{} UTC", format::format_date(timestamp, DateStyle::Time)),
        TimeFormat::Long => format::format_date(timestamp, DateStyle::Long),
        TimeFormat::Relative => format::format_relative_time(timestamp, now),
    }
}

/// A time.
///
/// - `timestamp` is a Unix timestamp in seconds. `iso` is an ISO 8601 date or
///   instant, used when there is no `timestamp`; see [`format::parse_iso`].
/// - `format` is how to show it.
/// - `locale` is a BCP 47 tag such as `"de-DE"`, and `time_zone` an IANA zone such as
///   `"Europe/Berlin"`, for the script; the default is the visitor's own.
/// - `now` is a fixed reference for a relative time, as a Unix timestamp. With it the
///   time does not tick. The default is the server's clock, then the visitor's.
/// - `title` replaces the title; by default a relative time has its full date there.
/// - `child` is rendered instead of the `<time>` when the value is missing or is not
///   a date. The default is nothing, as upstream's; pass `"—"` where an empty table
///   cell would look like a mistake.
#[component]
pub async fn time(
    cx: &Cx,
    #[default]
    #[into]
    timestamp: Option<i64>,
    #[default] iso: &str,
    #[default] format: TimeFormat,
    #[default] locale: &str,
    #[default] time_zone: &str,
    #[default]
    #[into]
    now: Option<i64>,
    #[default] title: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let value = timestamp.or_else(|| format::parse_iso(iso));
    let rendered = value.map(|value| {
        let text = time_text(value, format, now.unwrap_or_else(format::now));
        let own_title = if !title.is_empty() {
            Some(title.to_owned())
        } else if format == TimeFormat::Relative {
            Some(format::format_date(value, DateStyle::Long))
        } else {
            None
        };
        (format::to_iso(value), text, own_title)
    });
    Ok(view! {
        if let Some((datetime, text, own_title)) = rendered {
            <time
                datetime=(datetime)
                title=(own_title)
                data-time=(format.as_str())
                data-locale=((!locale.is_empty()).then_some(locale))
                data-time-zone=((!time_zone.is_empty()).then_some(time_zone))
                data-now=(now.map(|now| now.to_string()))
                data-title=((!title.is_empty()).then_some("fixed"))
                class=(class!(css::TIME, attrs.remove("class")))
                (attrs)
            >
                (text)
            </time>
        } else {
            (child)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{TimeFormat, time_text};

    const NOON: i64 = 1_790_424_000;

    #[test]
    fn server_text_is_utc_and_says_so() {
        assert_eq!(time_text(NOON, TimeFormat::Date, NOON), "Sep 26, 2026");
        assert_eq!(
            time_text(NOON, TimeFormat::Datetime, NOON),
            "Sep 26, 2026, 12:00 PM UTC"
        );
        assert_eq!(time_text(NOON, TimeFormat::Time, NOON), "12:00 PM UTC");
        assert_eq!(
            time_text(NOON, TimeFormat::Long, NOON),
            "Saturday, September 26, 2026 at 12:00:00 PM UTC"
        );
        assert_eq!(
            time_text(NOON - 3 * 3600, TimeFormat::Relative, NOON),
            "3 hours ago"
        );
    }
}
