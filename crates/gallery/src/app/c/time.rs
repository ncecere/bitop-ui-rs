use bitop_ui::{
    components::time::{TimeFormat, time},
    format,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

/// 2026-09-26T12:00:00Z.
const NOON: i64 = 1_790_424_000;

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let now = format::now();
    Ok(view! {
        page_header(slug: "time")
        example(
            title: "Formats",
            <ul
                data-example="formats"
                style="display: grid; gap: var(--space-2); padding: 0; list-style: none"
            >
                <li>time(timestamp: NOON)</li>
                <li>time(timestamp: NOON, format: TimeFormat::Date)</li>
                <li>time(timestamp: NOON, format: TimeFormat::Time)</li>
                <li>time(timestamp: NOON, format: TimeFormat::Long)</li>
                <li>time(iso: "2026-09-26T14:00:00+02:00")</li>
            </ul>
        )
        example(
            title: "Relative, from a fixed time",
            <ul
                data-example="fixed"
                style="display: grid; gap: var(--space-2); padding: 0; list-style: none"
            >
                <li>
                    time(
                        timestamp: NOON - 3 * 3600,
                        format: TimeFormat::Relative,
                        now: NOON
                    )
                </li>
                <li>
                    time(
                        timestamp: NOON - 14 * 3600,
                        format: TimeFormat::Relative,
                        now: NOON
                    )
                </li>
                <li>
                    time(
                        timestamp: NOON + 14 * 86_400,
                        format: TimeFormat::Relative,
                        now: NOON
                    )
                </li>
                <li>time(timestamp: NOON, format: TimeFormat::Relative, now: NOON)</li>
                <li>
                    time(
                        timestamp: NOON - 90,
                        format: TimeFormat::Relative,
                        now: NOON,
                        title: "When the build started"
                    )
                </li>
            </ul>
        )
        example(
            title: "Relative, kept fresh",
            <ul
                data-example="live"
                style="display: grid; gap: var(--space-2); padding: 0; list-style: none"
            >
                <li>
                    "This page was rendered "
                    time(timestamp: now - 2, format: TimeFormat::Relative)
                    "."
                </li>
                <li>
                    "The deploy finished "
                    time(timestamp: now - 3 * 3600, format: TimeFormat::Relative)
                    "."
                </li>
            </ul>
        )
        example(
            title: "A locale and a time zone",
            <ul
                data-example="locale"
                style="display: grid; gap: var(--space-2); padding: 0; list-style: none"
            >
                <li>
                    time(timestamp: NOON, locale: "de-DE", time_zone: "Europe/Berlin")
                </li>
                <li>
                    time(
                        timestamp: NOON - 3 * 3600,
                        format: TimeFormat::Relative,
                        now: NOON,
                        locale: "fr-FR",
                        time_zone: "Asia/Tokyo"
                    )
                </li>
                <li>
                    time(
                        timestamp: NOON,
                        time_zone: "Mars/Olympus",
                        attrs: attributes! { id="bad-zone" }
                    )
                </li>
            </ul>
        )
        example(
            title: "A missing value",
            <p data-example="missing">
                "Last sign-in: "
                time(iso: "not a date", <span>"never"</span>)
                time(format: TimeFormat::Date)
            </p>
            <p data-example="dash">
                "Expires: "
                time(format: TimeFormat::Date, "—")
            </p>
        )
    })
}
