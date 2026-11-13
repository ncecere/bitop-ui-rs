//! A small bar chart: one slot per point, one bar per series, overlapping or
//! stacked. Drawn on the server in SVG, sized to its container.
//!
//! ```ignore
//! let series = [ChartSeries::new("Answers"), ChartSeries::new("Escalations")];
//! let days = [
//!     ChartPoint::new("Sep 1", [61.0, 4.0]),
//!     ChartPoint::new("Sep 2", [74.0, 7.0]),
//! ];
//! view! {
//!     bar_chart(
//!         summary: "Answers per day, Sep 1 to 2: 135 in total, most on Sep 2 (74)",
//!         series: &series,
//!         data: &days,
//!         data_table: ChartTable::new("Answers per day"),
//!     )
//! }
//! ```
//!
//! To assistive technology the plot is one image named by `summary`: say what the
//! chart shows (the range, the total, the extremes). `data_table` adds a "Show data"
//! disclosure with every number in a table. The plot is also a tab stop: the arrow
//! keys, Home and End move between points, and the library script shows the values
//! of the point under the keyboard or the pointer in a readout over the plot, which
//! is announced politely.
//!
//! With `layout` [`BarLayout::Overlap`] the series share each slot, the first at the
//! back in its paler colour, so put the largest series first. With
//! [`BarLayout::Stack`] they add up. Bars are solid, in their series' colour, and fill
//! their slot with 2px between neighbours, as upstream's.
//!
//! The scale runs from 0 to the largest value, or stack total. `max` fixes the top,
//! such as 100 for a percentage; taller bars are drawn full height and stacks are cut
//! there. Negative values are drawn as empty bars; NaN and infinities are no data,
//! left out of the scale and drawn as an empty bar marked `data-missing`. The
//! readout and the table show the real values.
//!
//! Differences from upstream: upstream draws the bars with CSS boxes and shows the
//! values in a `title`; here they are SVG rectangles, and the readout shows on
//! hover and on keyboard focus. In right-to-left the time axis runs from the right, as
//! upstream's does. An empty chart says "No data".

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::chart::{
    ChartFormat, ChartPoint, ChartSeries, ChartSize, ChartSwatch, ChartTable, chart_data,
    chart_legend, chart_value, coordinate, format_chart_value, point_text, series_tone,
};
use crate::css::{bar_chart as css, chart as chart_css};

/// How the series of a [`bar_chart`] share a slot. Rendered as `data-layout`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BarLayout {
    /// Bars share the slot, the first series at the back.
    #[default]
    Overlap,
    /// Bars add up.
    Stack,
}

impl BarLayout {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Overlap => "overlap",
            Self::Stack => "stack",
        }
    }
}

/// A bar's box, in percentages of the plot: `x` and `y` from the top left.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BarRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// The top of the scale: `max` if given, else the largest value (overlap) or stack
/// total (stack), and never below 0. Values with no data and negative values add
/// nothing.
#[must_use]
pub fn bar_peak(values: &[Vec<Option<f64>>], layout: BarLayout, max: Option<f64>) -> f64 {
    if let Some(max) = max {
        return max;
    }
    values
        .iter()
        .map(|point| {
            let positive = point.iter().map(|value| value.unwrap_or(0.0).max(0.0));
            match layout {
                BarLayout::Overlap => positive.fold(0.0, f64::max),
                BarLayout::Stack => positive.sum(),
            }
        })
        .fold(0.0, f64::max)
}

/// The bars of every point, one per series, in percentages of the plot. `values`
/// holds one list per point, of one value per series.
#[must_use]
pub fn bar_rects(values: &[Vec<Option<f64>>], layout: BarLayout, peak: f64) -> Vec<Vec<BarRect>> {
    #[allow(clippy::cast_precision_loss)]
    let slot = 100.0 / values.len().max(1) as f64;
    let scale = |value: f64| {
        if peak > 0.0 {
            value.clamp(0.0, peak) / peak * 100.0
        } else {
            0.0
        }
    };
    values
        .iter()
        .enumerate()
        .map(|(index, point)| {
            #[allow(clippy::cast_precision_loss)]
            let x = index as f64 * slot;
            let width = slot;
            let mut base = 0.0;
            point
                .iter()
                .map(|value| {
                    let value = value.unwrap_or(0.0).max(0.0);
                    let (bottom, top) = match layout {
                        BarLayout::Overlap => (0.0, scale(value)),
                        BarLayout::Stack => {
                            let bottom = scale(base);
                            base += value;
                            (bottom, scale(base))
                        }
                    };
                    BarRect {
                        x,
                        y: 100.0 - top,
                        width,
                        height: top - bottom,
                    }
                })
                .collect()
        })
        .collect()
}

struct Bar {
    rect: BarRect,
    missing: bool,
    top: bool,
}

struct Series {
    tone: &'static str,
    bars: Vec<Bar>,
}

/// A bar chart with a legend, a readout and an optional data table.
///
/// - `series` are the bars of each slot, and `data` the slots, in order.
/// - `summary` names the plot for assistive technology. Required.
/// - `layout` overlaps or stacks the bars.
/// - `size` is the plot's height.
/// - `format_value` writes the peak label, the readout and the table's numbers.
/// - `legend` shows the legend, and `axis` the first and last labels and the peak.
/// - `max` fixes the top of the scale.
/// - `data_table` adds the "Show data" disclosure.
/// - `toggle_label`, `missing_label` and `empty_label` are the strings.
#[component]
pub async fn bar_chart(
    cx: &Cx,
    series: &[ChartSeries<'_>],
    data: &[ChartPoint<'_>],
    summary: &str,
    #[default] layout: BarLayout,
    #[default] size: ChartSize,
    #[default(format_chart_value)] format_value: ChartFormat,
    #[default(true)] legend: bool,
    #[default(true)] axis: bool,
    #[default]
    #[into]
    max: Option<f64>,
    #[default]
    #[into]
    data_table: Option<ChartTable<'_>>,
    #[default("Show data")] toggle_label: &str,
    #[default("No data")] missing_label: &str,
    #[default("No data")] empty_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let values = data
        .iter()
        .map(|point| {
            (0..series.len())
                .map(|index| chart_value(point, index))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let peak = bar_peak(&values, layout, max);
    let rects = bar_rects(&values, layout, peak);
    let groups = series
        .iter()
        .enumerate()
        .map(|(index, one)| {
            let bars = rects
                .iter()
                .zip(&values)
                .map(|(rects, point)| {
                    // The top of a stack is the last bar with height; only it is rounded.
                    let top = match layout {
                        BarLayout::Overlap => true,
                        BarLayout::Stack => rects
                            .iter()
                            .rposition(|rect| rect.height > 0.0)
                            .is_some_and(|last| last == index),
                    };
                    Bar {
                        rect: rects[index],
                        missing: point[index].is_none(),
                        top,
                    }
                })
                .collect();
            Series {
                tone: series_tone(one, index).as_str(),
                bars,
            }
        })
        .collect::<Vec<_>>();
    let slots = data
        .iter()
        .map(|point| point_text(point, series, format_value, missing_label))
        .collect::<Vec<_>>();
    let first = data.first().map(|point| point.label);
    let last = (data.len() > 1)
        .then(|| data.last().map(|point| point.label))
        .flatten();
    let peak_text = format_value(peak);
    let empty = data.is_empty();
    Ok(view! {
        <figure
            class=(class!(chart_css::ROOT, css::ROOT, attrs.remove("class")))
            data-size=(size.as_str())
            data-layout=(layout.as_str())
            data-chart="bar"
            (attrs)
        >
            if legend {
                chart_legend(series: series, swatch: ChartSwatch::Square)
            }
            <div class=(chart_css::FRAME)>
                <div
                    class=(chart_css::PLOT)
                    role="img"
                    aria-label=(summary)
                    tabindex=((!empty).then_some("0"))
                    data-chart-plot=""
                >
                    if axis {
                        <span class=(chart_css::PEAK)>(peak_text)</span>
                    }
                    <div class=(chart_css::AREA)>
                        <div class=(chart_css::SLOTS)>
                            for text in slots {
                                <span
                                    class=(chart_css::SLOT)
                                    data-chart-slot=""
                                    data-chart-value=(text)
                                ></span>
                            }
                        </div>
                        if empty {
                            <span class=(chart_css::EMPTY)>(empty_label)</span>
                        }
                        <svg
                            aria-hidden="true"
                            focusable="false"
                            class=(class!(chart_css::SVG, css::SVG))
                        >
                            for group in groups {
                                <g
                                    class=(chart_css::TONE)
                                    data-tone=(group.tone)
                                    data-series=""
                                >
                                    for bar in group.bars {
                                        <rect
                                            class=(css::BAR)
                                            x=(format!("{}%", coordinate(bar.rect.x)))
                                            y=(format!("{}%", coordinate(bar.rect.y)))
                                            width=(format!("{}%", coordinate(bar.rect.width)))
                                            height=(format!("{}%", coordinate(bar.rect.height)))
                                            data-missing=(bar.missing.then_some(""))
                                            data-top=(bar.top.then_some(""))
                                        ></rect>
                                    }
                                </g>
                            }
                        </svg>
                    </div>
                </div>
                <div
                    class=(chart_css::READOUT)
                    data-chart-readout=""
                    aria-live="polite"
                ></div>
            </div>
            if axis && !empty {
                <div aria-hidden="true" class=(chart_css::AXIS)>
                    <span>(first)</span>
                    if let Some(last) = last {
                        <span>(last)</span>
                    }
                </div>
            }
            if let Some(table) = data_table {
                chart_data(
                    caption: table.caption,
                    label_header: table.label_header,
                    open: table.open,
                    series: series,
                    data: data,
                    format_value: format_value,
                    toggle_label: toggle_label,
                    missing_label: missing_label
                )
            }
        </figure>
    })
}

#[cfg(test)]
mod tests {
    use super::{BarLayout, BarRect, bar_peak, bar_rects};

    fn heights(rects: &[Vec<BarRect>]) -> Vec<Vec<f64>> {
        rects
            .iter()
            .map(|point| point.iter().map(|rect| rect.height).collect())
            .collect()
    }

    #[test]
    fn peaks() {
        let values = vec![
            vec![Some(10.0), Some(30.0)],
            vec![Some(40.0), None],
            vec![Some(-5.0), Some(20.0)],
        ];
        assert_eq!(bar_peak(&values, BarLayout::Overlap, None), 40.0);
        assert_eq!(bar_peak(&values, BarLayout::Stack, None), 40.0);
        assert_eq!(
            bar_peak(&[vec![Some(10.0), Some(35.0)]], BarLayout::Stack, None),
            45.0
        );
        assert_eq!(bar_peak(&values, BarLayout::Overlap, Some(100.0)), 100.0);
        assert_eq!(bar_peak(&[], BarLayout::Overlap, None), 0.0);
        assert_eq!(bar_peak(&[vec![Some(-3.0)]], BarLayout::Overlap, None), 0.0);
    }

    #[test]
    fn overlapping_bars_are_proportional() {
        let values = vec![vec![Some(50.0), Some(25.0)], vec![Some(100.0), None]];
        let rects = bar_rects(&values, BarLayout::Overlap, 100.0);
        assert_eq!(heights(&rects), vec![vec![50.0, 25.0], vec![100.0, 0.0]]);
        assert_eq!(rects[0][0].y, 50.0);
        // Two slots of 50%; each bar fills its slot (CSS clips 1px from each side).
        assert_eq!(rects[0][0].x, 0.0);
        assert_eq!(rects[1][1].x, 50.0);
        assert_eq!(rects[0][0].width, 50.0);
    }

    #[test]
    fn stacks_add_up_and_are_cut_at_the_top() {
        let values = vec![vec![Some(20.0), Some(30.0)], vec![Some(80.0), Some(40.0)]];
        let rects = bar_rects(&values, BarLayout::Stack, 100.0);
        assert_eq!(heights(&rects), vec![vec![20.0, 30.0], vec![80.0, 20.0]]);
        assert_eq!(rects[0][1].y, 50.0);
        assert_eq!(rects[1][1].y, 0.0);
    }

    #[test]
    fn negative_missing_and_flat_values() {
        let values = vec![vec![Some(-10.0)], vec![None], vec![Some(10.0)]];
        assert_eq!(
            heights(&bar_rects(&values, BarLayout::Overlap, 10.0)),
            vec![vec![0.0], vec![0.0], vec![100.0]]
        );
        // All values equal: every bar reaches the top.
        let flat = vec![vec![Some(5.0)], vec![Some(5.0)]];
        let peak = bar_peak(&flat, BarLayout::Overlap, None);
        assert_eq!(
            heights(&bar_rects(&flat, BarLayout::Overlap, peak)),
            vec![vec![100.0], vec![100.0]]
        );
        // An empty scale draws nothing.
        assert_eq!(
            heights(&bar_rects(&[vec![Some(0.0)]], BarLayout::Overlap, 0.0)),
            vec![vec![0.0]]
        );
    }

    #[test]
    fn a_single_point_fills_the_width() {
        let rects = bar_rects(&[vec![Some(3.0)]], BarLayout::Overlap, 3.0);
        assert_eq!(
            (rects[0][0].x, rects[0][0].width, rects[0][0].height),
            (0.0, 100.0, 100.0)
        );
    }

    #[test]
    fn an_explicit_domain_clamps() {
        let values = vec![vec![Some(150.0)], vec![Some(25.0)]];
        let peak = bar_peak(&values, BarLayout::Overlap, Some(100.0));
        assert_eq!(
            heights(&bar_rects(&values, BarLayout::Overlap, peak)),
            vec![vec![100.0], vec![25.0]]
        );
    }
}
