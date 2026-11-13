use bitop_ui::components::{
    chart::ChartTone,
    line_chart::LineVariant,
    sparkline::{SparklineBaseline, SparklineSize, sparkline},
    stat_card::stat_card,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let rising = [61.0, 70.0, 66.0, 74.0, 72.0, 88.0, 96.0];
    let gaps = [12.0, 18.0, f64::NAN, 15.0, f64::NAN, 9.0, 14.0, f64::NAN];
    let signed = [4.0, -2.0, -6.0, 1.0, 5.0, -3.0, 2.0];
    let flat = [5.0, 5.0, 5.0, 5.0];
    let narrow = [96.0, 97.0, 96.5, 98.0, 97.2, 99.0];
    let one = [42.0];
    let none: [f64; 0] = [];
    let cell = "display: grid; grid-template-columns: 9rem 1fr; gap: var(--space-2) var(--space-4); align-items: center; inline-size: 100%";
    Ok(view! {
        page_header(slug: "sparkline")
        example(
            title: "Tones",
            <div style=(cell)>
                <span>"Primary"</span>
                sparkline(values: &rising, label: "Primary: rising from 61 to 96")
                <span>"Info"</span>
                sparkline(
                    values: &rising,
                    tone: ChartTone::Info,
                    label: "Info: rising from 61 to 96"
                )
                <span>"Success"</span>
                sparkline(
                    values: &rising,
                    tone: ChartTone::Success,
                    label: "Success: rising from 61 to 96"
                )
                <span>"Warning"</span>
                sparkline(
                    values: &rising,
                    tone: ChartTone::Warning,
                    label: "Warning: rising from 61 to 96"
                )
                <span>"Danger"</span>
                sparkline(
                    values: &rising,
                    tone: ChartTone::Danger,
                    label: "Danger: rising from 61 to 96"
                )
                <span>"Neutral"</span>
                sparkline(
                    values: &rising,
                    tone: ChartTone::Neutral,
                    label: "Neutral: rising from 61 to 96"
                )
                <span>"Accent"</span>
                sparkline(
                    values: &rising,
                    tone: ChartTone::Accent,
                    label: "Accent: rising from 61 to 96"
                )
            </div>
        )
        example(
            title: "Area, small, and without the last point",
            <div style=(cell)>
                <span>"Area"</span>
                sparkline(
                    values: &rising,
                    variant: LineVariant::Area,
                    label: "Answers per day: rising from 61 to 96"
                )
                <span>"Small"</span>
                sparkline(
                    values: &rising,
                    size: SparklineSize::Sm,
                    label: "Small: rising from 61 to 96"
                )
                <span>"No last point"</span>
                sparkline(
                    values: &rising,
                    show_last: false,
                    label: "No last point: rising"
                )
            </div>
        )
        example(
            title: "Gaps, negative values and a minimum baseline",
            <div style=(cell)>
                <span>"Gaps"</span>
                sparkline(
                    values: &gaps,
                    variant: LineVariant::Area,
                    label: "Builds per day, with three days missing"
                )
                <span>"Negative"</span>
                sparkline(
                    values: &signed,
                    tone: ChartTone::Info,
                    label: "Net change per day: between -6 and 5"
                )
                <span>"From zero"</span>
                sparkline(values: &narrow, label: "Uptime from zero: 96 to 99")
                <span>"From the minimum"</span>
                sparkline(
                    values: &narrow,
                    baseline: SparklineBaseline::Min,
                    label: "Uptime from the minimum: 96 to 99"
                )
            </div>
        )
        example(
            title: "Flat, one value, and none",
            <div style=(cell)>
                <span>"Flat"</span>
                sparkline(values: &flat, label: "Flat at 5")
                <span>"One value"</span>
                sparkline(values: &one, label: "One value: 42")
                <span>"No values"</span>
                sparkline(values: &none, label: "No values yet")
            </div>
        )
        example(
            title: "Decorative, beside the same trend in words",
            <div style="display: grid; gap: var(--space-2); inline-size: 16rem">
                <p>"Answers rose from 61 to 96 this week."</p>
                sparkline(values: &rising, decorative: true)
            </div>
        )
        example(
            title: "In a stat card",
            <div style="inline-size: 16rem">
                stat_card(
                    label: "Answers",
                    value: "527",
                    chart: view! {
                        sparkline(
                            values: &rising,
                            variant: LineVariant::Area,
                            label: "Answers per day, last 7 days: rising from 61 to 96"
                        )
                    }
                )
            </div>
        )
    })
}
