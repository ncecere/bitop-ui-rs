use bitop_ui::{
    components::{
        chart::{ChartPoint, ChartSeries, ChartSize, ChartTable, ChartTone},
        line_chart::{ChartDomain, LineVariant, line_chart},
    },
    format::format_number,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

fn percent(value: f64) -> String {
    format!("{}%", format_number(value, 1, false))
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let traffic = [
        ChartSeries::new("Answers"),
        ChartSeries::new("Conversations"),
        ChartSeries::new("Escalations"),
    ];
    let week = [
        ChartPoint::new("Sep 1", [61.0, 40.0, 8.0]),
        ChartPoint::new("Sep 2", [74.0, 52.0, 12.0]),
        ChartPoint::new("Sep 3", [50.0, 38.0, 6.0]),
        ChartPoint::new("Sep 4", [96.0, 60.0, 20.0]),
        ChartPoint::new("Sep 5", [80.0, 66.0, 14.0]),
        ChartPoint::new("Sep 6", [32.0, 30.0, 4.0]),
        ChartPoint::new("Sep 7", [48.0, 44.0, 9.0]),
    ];
    let latency = [
        ChartSeries::new("p50").tone(ChartTone::Success),
        ChartSeries::new("p95").tone(ChartTone::Warning),
    ];
    let minutes = [
        ChartPoint::new("12:00", [120.0, 340.0]),
        ChartPoint::new("12:05", [140.0, 360.0]),
        ChartPoint::new("12:10", [f64::NAN, f64::NAN]),
        ChartPoint::new("12:15", [130.0, 420.0]),
        ChartPoint::new("12:20", [f64::NAN, 390.0]),
        ChartPoint::new("12:25", [110.0, 300.0]),
        ChartPoint::new("12:30", [125.0, 310.0]),
    ];
    let uptime = [ChartSeries::new("Uptime").tone(ChartTone::Info)];
    let days = [
        ChartPoint::new("Mon", [99.2]),
        ChartPoint::new("Tue", [97.5]),
        ChartPoint::new("Wed", [99.9]),
        ChartPoint::new("Thu", [92.0]),
        ChartPoint::new("Fri", [98.4]),
    ];
    let ticks = [95.0];
    let score = [ChartSeries::new("Score").tone(ChartTone::Accent)];
    let scores = [
        ChartPoint::new("Run 1", [40.0]),
        ChartPoint::new("Run 2", [-10.0]),
        ChartPoint::new("Run 3", [55.0]),
        ChartPoint::new("Run 4", [55.0]),
    ];
    let percent_ticks = [0.0, 25.0, 50.0, 75.0];
    let one = [ChartPoint::new("Today", [42.0, 30.0, 5.0])];
    let none: [ChartPoint<'_>; 0] = [];
    let german = [ChartSeries::new("Antworten")];
    let tage = [
        ChartPoint::new("Mo", [3.0]),
        ChartPoint::new("Di", [f64::NAN]),
        ChartPoint::new("Mi", [5.0]),
        ChartPoint::new("Do", [4.0]),
    ];
    Ok(view! {
        page_header(slug: "line-chart")
        example(
            title: "Three series, with the data table",
            line_chart(
                summary: "Traffic per day, Sep 1 to 7: answers peaked at 96 on Sep 4",
                series: &traffic,
                data: &week,
                data_table: ChartTable::new("Traffic per day")
            )
        )
        example(
            title: "Area",
            line_chart(
                summary: "Answers and conversations per day, Sep 1 to 7, as areas",
                series: &traffic[..2],
                data: &week,
                variant: LineVariant::Area,
                size: ChartSize::Lg
            )
        )
        example(
            title: "Gaps, and a lone point between them",
            line_chart(
                summary: "Latency in milliseconds, 12:00 to 12:30: no data at 12:10, p95 peaked at 420",
                series: &latency,
                data: &minutes,
                data_table: ChartTable::new("Latency in milliseconds").label_header(
                    "Time",
                )
            )
        )
        example(
            title: "A domain with a floor, and a tick",
            line_chart(
                summary: "Uptime per day, Mon to Fri: lowest on Thu (92%)",
                series: &uptime,
                data: &days,
                domain: ChartDomain::new(90.0, 100.0),
                ticks: &ticks,
                format_value: percent,
                points: true,
                data_table: ChartTable::new("Uptime per day")
                    .label_header("Day")
                    .open(true)
            )
        )
        example(
            title: "Percent ticks, a negative and two equal values",
            line_chart(
                summary: "Score per run: from 40 to 55, with -10 on run 2",
                series: &score,
                data: &scores,
                domain: ChartDomain::max(100.0),
                ticks: &percent_ticks,
                data_table: ChartTable::new("Score per run").label_header("Run")
            )
        )
        example(
            title: "One point, small",
            line_chart(
                summary: "Traffic today: 42 answers, 30 conversations, 5 escalations",
                series: &traffic,
                data: &one,
                size: ChartSize::Sm
            )
        )
        example(
            title: "Empty",
            line_chart(
                summary: "Traffic per day: nothing yet",
                series: &traffic,
                data: &none
            )
        )
        example(
            title: "Every string is an argument",
            line_chart(
                summary: "Antworten pro Tag",
                series: &german,
                data: &tage,
                data_table: ChartTable::new("Antworten pro Tag").label_header("Tag"),
                toggle_label: "Daten zeigen",
                missing_label: "Keine Daten",
                empty_label: "Keine Daten"
            )
        )
    })
}
