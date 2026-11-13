use bitop_ui::components::{
    icon::Icon,
    line_chart::LineVariant,
    sparkline::sparkline,
    stat_card::{StatDelta, StatSentiment, StatTrend, stat_card},
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
    Ok(view! {
        page_header(slug: "stat-card")
        example(
            title: "Trends",
            <div
                style="display: grid; grid-template-columns: repeat(auto-fill, minmax(13rem, 1fr)); gap: var(--space-4); inline-size: 100%"
            >
                stat_card(
                    label: "Requests",
                    value: "12,480",
                    delta: StatDelta::new("+12%", StatTrend::Up).label("vs last week"),
                    icon: Icon::Inbox
                )
                stat_card(
                    label: "Conversion",
                    value: "3.1%",
                    delta: StatDelta::new("-0.4%", StatTrend::Down)
                )
                stat_card(
                    label: "Latency",
                    value: "182 ms",
                    delta: StatDelta::new("0%", StatTrend::Flat),
                    hint: "p50, last hour"
                )
                stat_card(
                    label: "Errors",
                    value: "37",
                    delta: StatDelta::new("+9", StatTrend::Up)
                        .sentiment(StatSentiment::Negative)
                        .label("since deploy")
                )
            </div>
        )
        example(
            title: "A link, with a chart and details",
            <div
                style="display: grid; grid-template-columns: repeat(auto-fill, minmax(13rem, 1fr)); gap: var(--space-4); inline-size: 100%"
            >
                stat_card(
                    label: "Sources",
                    value: "58",
                    href: "#sources",
                    hint: "3 syncing",
                    details: view! {
                        "Includes "
                        <a href="#archived">"4 archived"</a>
                    }
                )
                stat_card(
                    label: "Storage",
                    value: "1.5 GB",
                    chart: view! {
                        sparkline(
                            values: &[2.0, 4.0, 3.0, 7.0, 6.0, 9.0, 10.0],
                            variant: LineVariant::Area,
                            label: "Storage over the last week"
                        )
                    }
                )
                stat_card(label: "Plain", value: "7")
            </div>
        )
    })
}
