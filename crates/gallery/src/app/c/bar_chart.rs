use bitop_ui::{
    components::{
        bar_chart::{BarLayout, bar_chart},
        chart::{ChartPoint, ChartSeries, ChartSize, ChartTable, ChartTone},
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
    let answers = [ChartSeries::new("Answers"), ChartSeries::new("Escalations")];
    let week = [
        ChartPoint::new("Sep 1", [61.0, 12.0]),
        ChartPoint::new("Sep 2", [74.0, 20.0]),
        ChartPoint::new("Sep 3", [50.0, 8.0]),
        ChartPoint::new("Sep 4", [96.0, 30.0]),
        ChartPoint::new("Sep 5", [80.0, 16.0]),
        ChartPoint::new("Sep 6", [32.0, 4.0]),
        ChartPoint::new("Sep 7", [48.0, 10.0]),
    ];
    let channels = [
        ChartSeries::new("Web"),
        ChartSeries::new("Slack").tone(ChartTone::Success),
        ChartSeries::new("Email").tone(ChartTone::Accent),
    ];
    let quarters = [
        ChartPoint::new("Q1", [40.0, 20.0, 10.0]),
        ChartPoint::new("Q2", [50.0, 30.0, 20.0]),
        ChartPoint::new("Q3", [30.0, 25.0, 5.0]),
        ChartPoint::new("Q4", [60.0, 35.0, 25.0]),
    ];
    let usage = [ChartSeries::new("CPU").tone(ChartTone::Warning)];
    let hours = [
        ChartPoint::new("09:00", [42.0]),
        ChartPoint::new("10:00", [87.5]),
        ChartPoint::new("11:00", [120.0]),
        ChartPoint::new("12:00", [64.0]),
    ];
    let errors = [ChartSeries::new("Errors").tone(ChartTone::Danger)];
    let gaps = [
        ChartPoint::new("Mon", [4.0]),
        ChartPoint::new("Tue", [f64::NAN]),
        ChartPoint::new("Wed", [-2.0]),
        ChartPoint::new("Thu", [8.0]),
        ChartPoint::new("Fri", [2.0]),
    ];
    let one = [ChartPoint::new("Today", [18.0])];
    let none: [ChartPoint<'_>; 0] = [];
    let german = [ChartSeries::new("Antworten")];
    let tage = [
        ChartPoint::new("Mo", [3.0]),
        ChartPoint::new("Di", [f64::NAN]),
        ChartPoint::new("Mi", [5.0]),
    ];
    Ok(view! {
        page_header(slug: "bar-chart")
        example(
            title: "Overlapping series, with the data table",
            bar_chart(
                summary: "Answers per day, Sep 1 to 7: 441 in total, most on Sep 4 (96)",
                series: &answers,
                data: &week,
                data_table: ChartTable::new("Answers per day")
            )
        )
        example(
            title: "Stacked",
            bar_chart(
                summary: "Conversations by channel per quarter: 350 in total, most in Q4 (120)",
                series: &channels,
                data: &quarters,
                layout: BarLayout::Stack,
                size: ChartSize::Lg,
                data_table: ChartTable::new("Conversations by channel").label_header(
                    "Quarter",
                )
            )
        )
        example(
            title: "A fixed scale, in percent",
            bar_chart(
                summary: "CPU use per hour, 09:00 to 12:00: peaking at 120% at 11:00",
                series: &usage,
                data: &hours,
                max: 100.0,
                format_value: percent,
                data_table: ChartTable::new("CPU use per hour")
                    .label_header("Hour")
                    .open(true)
            )
        )
        example(
            title: "No data and a negative value",
            bar_chart(
                summary: "Errors per day, Mon to Fri: no data on Tue, a correction of -2 on Wed",
                series: &errors,
                data: &gaps,
                data_table: ChartTable::new("Errors per day").label_header("Day")
            )
        )
        example(
            title: "One point, small, without legend or axis",
            <div style="display: grid; gap: var(--space-5); inline-size: 100%">
                bar_chart(
                    summary: "Errors today: 18",
                    series: &errors,
                    data: &one,
                    size: ChartSize::Sm
                )
                bar_chart(
                    summary: "Answers per day, without legend or axis",
                    series: &answers,
                    data: &week,
                    size: ChartSize::Sm,
                    legend: false,
                    axis: false
                )
            </div>
        )
        example(
            title: "Empty",
            bar_chart(
                summary: "Answers per day: no answers yet",
                series: &answers,
                data: &none
            )
        )
        example(
            title: "Every string is an argument",
            bar_chart(
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
