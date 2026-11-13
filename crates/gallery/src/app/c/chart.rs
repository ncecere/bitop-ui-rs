use bitop_ui::components::chart::{
    ChartPattern, ChartPoint, ChartSeries, ChartSwatch, ChartTone, chart_data, chart_legend,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let tones = [
        ChartSeries::new("Primary"),
        ChartSeries::new("Info"),
        ChartSeries::new("Success"),
        ChartSeries::new("Warning"),
        ChartSeries::new("Danger"),
        ChartSeries::new("Neutral"),
        ChartSeries::new("Accent"),
    ];
    let lines = [
        ChartSeries::new("Answers"),
        ChartSeries::new("Conversations"),
        ChartSeries::new("Escalations"),
    ];
    let own = [
        ChartSeries::new("Budget")
            .tone(ChartTone::Neutral)
            .pattern(ChartPattern::Dotted),
        ChartSeries::new("Spend")
            .tone(ChartTone::Danger)
            .pattern(ChartPattern::Solid),
    ];
    let days = [
        ChartPoint::new("Sep 1", [61.0, 20.0, 3.0]),
        ChartPoint::new("Sep 2", [74.0, f64::NAN, 5.0]),
        ChartPoint::new("Sep 3", [1204.5, 31.0]),
    ];
    let pairs = [
        ChartPoint::new("Mo", [1.0, 2.0]),
        ChartPoint::new("Di", [f64::NAN, 4.0]),
    ];
    let pair = [ChartSeries::new("Antworten"), ChartSeries::new("Gespräche")];
    Ok(view! {
        page_header(slug: "chart")
        example(
            title: "Tones, by position",
            chart_legend(series: &tones, attrs: attributes! { aria-label="Tones" })
        )
        example(
            title: "Line swatches: solid, dashed and dotted",
            chart_legend(
                series: &lines,
                swatch: ChartSwatch::Line,
                attrs: attributes! { aria-label="Line patterns" }
            )
        )
        example(
            title: "A tone and a pattern of its own",
            chart_legend(
                series: &own,
                swatch: ChartSwatch::Line,
                attrs: attributes! { aria-label="Own tones" }
            )
        )
        example(
            title: "The data table",
            <div style="inline-size: 100%">
                chart_data(
                    caption: "Answers per day",
                    series: &lines,
                    data: &days,
                    open: true
                )
            </div>
        )
        example(
            title: "Every string is an argument",
            <div style="inline-size: 100%">
                chart_data(
                    caption: "Antworten pro Tag",
                    series: &pair,
                    data: &pairs,
                    label_header: "Tag",
                    toggle_label: "Daten zeigen",
                    missing_label: "Keine Daten"
                )
            </div>
        )
    })
}
