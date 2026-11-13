//! One or more series over an ordered set of points, as lines or filled areas.
//! Drawn on the server in SVG, stretched to its container.
//!
//! ```ignore
//! let series = [ChartSeries::new("Answers"), ChartSeries::new("Conversations")];
//! let days = [
//!     ChartPoint::new("Sep 1", [61.0, 20.0]),
//!     ChartPoint::new("Sep 2", [74.0, 31.0]),
//!     ChartPoint::new("Sep 3", [96.0, 28.0]),
//! ];
//! view! {
//!     line_chart(
//!         summary: "Answers per day, Sep 1 to 3: 231 in total, rising from 61 to 96",
//!         series: &series,
//!         data: &days,
//!         variant: LineVariant::Area,
//!         data_table: ChartTable::new("Answers per day"),
//!     )
//! }
//! ```
//!
//! To assistive technology the plot is one image named by `summary`: say what the
//! chart shows (the range, the total, the trend and the extremes). `data_table` adds
//! a "Show data" disclosure with every number in a table. The plot is also a tab
//! stop: the arrow keys, Home and End move between points, and the library script
//! shows the values of the point under the keyboard or the pointer in a readout
//! over the plot, which is announced politely.
//!
//! Series differ by colour and line pattern (solid, dashed, dotted), shown in the
//! legend. The scale starts at 0 and tops out at the largest value, shown as the peak
//! line. Negative values are drawn at 0. NaN and infinities are no data: left out of
//! the scale, with a gap in the line; a lone point between gaps gets a dot, and so
//! does a chart of one point. The readout and the table show the real values.
//!
//! `domain` fixes the scale instead of fitting it to the data, so that charts of a
//! bounded measure stay comparable: `ChartDomain::new(0.0, 100.0)` keeps 50% half way
//! up. A `min` other than 0 is labelled at the bottom. Values outside the domain are
//! drawn at its edge. `ticks` labels more values on the scale, each on a faint line;
//! ticks outside the scale are left out, and the top is always labelled.
//!
//! The time axis runs from left to right in either direction, as upstream's line
//! does.
//!
//! Differences from upstream: the values show in a readout on hover and keyboard
//! focus, not a `title`. Upstream's hover columns and axis labels follow the page
//! direction while the line does not; here all three run left to right. The legend
//! shows a line in the series' pattern for the area variant too, since the pattern
//! is what tells the lines apart. An empty chart says "No data".

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::chart::{
    ChartFormat, ChartPoint, ChartSeries, ChartSize, ChartSwatch, ChartTable, chart_data,
    chart_legend, chart_runs, coordinate, format_chart_value, point_text, series_pattern,
    series_tone, series_values,
};
use crate::css::{chart as chart_css, line_chart as css};

/// Lines alone, or lines over a soft fill. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineVariant {
    #[default]
    Line,
    Area,
}

impl LineVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Area => "area",
        }
    }
}

/// A fixed scale for a [`line_chart`]. An end left as `None` fits the data: 0 at the
/// bottom, the largest value at the top.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChartDomain {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl ChartDomain {
    /// A scale from `min` to `max`.
    #[must_use]
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }

    /// A scale from 0, or the data, to `max`.
    #[must_use]
    pub fn max(max: f64) -> Self {
        Self {
            min: None,
            max: Some(max),
        }
    }
}

/// The width of the plot's coordinate system. The SVG stretches to its box, and
/// strokes do not scale.
pub const LINE_WIDTH: f64 = 1000.0;
/// The height of the plot's coordinate system.
pub const LINE_HEIGHT: f64 = 100.0;

/// The ends of the scale, `(floor, peak)`: the domain's, or 0 and the largest value.
/// Values with no data count as 0.
#[must_use]
pub fn line_scale(values: &[Vec<Option<f64>>], domain: ChartDomain) -> (f64, f64) {
    let floor = domain.min.unwrap_or(0.0);
    let peak = domain.max.unwrap_or_else(|| {
        values
            .iter()
            .flatten()
            .map(|value| value.unwrap_or(0.0))
            .fold(floor.max(0.0), f64::max)
    });
    (floor, peak)
}

/// The horizontal position of point `index` of `count`; a lone point is centred.
#[must_use]
pub fn line_x(index: usize, count: usize) -> f64 {
    if count <= 1 {
        LINE_WIDTH / 2.0
    } else {
        #[allow(clippy::cast_precision_loss)]
        let x = index as f64 / (count - 1) as f64 * LINE_WIDTH;
        x
    }
}

/// The vertical position of `value` on the scale, from the top. Values outside the
/// scale are drawn at its edge; an empty scale draws everything at the bottom.
#[must_use]
pub fn line_y(value: f64, floor: f64, peak: f64) -> f64 {
    if peak > floor {
        LINE_HEIGHT - (value.clamp(floor, peak) - floor) / (peak - floor) * LINE_HEIGHT
    } else {
        LINE_HEIGHT
    }
}

/// The `d` of a series' line: one subpath per run of points with data, so that a
/// missing value leaves a gap.
#[must_use]
pub fn line_path(values: &[Option<f64>], floor: f64, peak: f64) -> String {
    let count = values.len();
    chart_runs(values)
        .iter()
        .map(|run| {
            run.iter()
                .enumerate()
                .map(|(step, (index, value))| {
                    format!(
                        "{}{},{}",
                        if step == 0 { "M" } else { "L" },
                        coordinate(line_x(*index, count)),
                        coordinate(line_y(*value, floor, peak))
                    )
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The `d` of a series' area: each run of two or more points closed down to the
/// bottom of the plot.
#[must_use]
pub fn area_path(values: &[Option<f64>], floor: f64, peak: f64) -> String {
    let count = values.len();
    let bottom = coordinate(LINE_HEIGHT);
    chart_runs(values)
        .iter()
        .filter(|run| run.len() > 1)
        .map(|run| {
            let points = run
                .iter()
                .map(|(index, value)| {
                    format!(
                        "L{},{}",
                        coordinate(line_x(*index, count)),
                        coordinate(line_y(*value, floor, peak))
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "M{},{bottom} {points} L{},{bottom} Z",
                coordinate(line_x(run[0].0, count)),
                coordinate(line_x(run[run.len() - 1].0, count))
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The ticks to label: finite, each once, from the floor up to below the peak (which
/// has its own label). None on an empty scale.
#[must_use]
pub fn tick_values(ticks: &[f64], floor: f64, peak: f64) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();
    if peak > floor {
        for &tick in ticks {
            if tick.is_finite() && tick >= floor && tick < peak && !out.contains(&tick) {
                out.push(tick);
            }
        }
    }
    out
}

struct Line {
    tone: &'static str,
    pattern: &'static str,
    line: String,
    fill: String,
}

struct Dot {
    tone: &'static str,
    style: String,
}

/// A line or area chart with a legend, a readout and an optional data table.
///
/// - `series` are the lines, and `data` the points, in order.
/// - `summary` names the plot for assistive technology. Required.
/// - `variant` draws lines, or lines over a soft fill.
/// - `size` is the plot's height.
/// - `format_value` writes the scale's labels, the readout and the table's numbers.
/// - `legend` shows the legend, and `axis` the first and last labels and the peak.
/// - `points` marks every point with a dot; by default only a chart of one point
///   has dots.
/// - `domain` fixes the scale, and `ticks` labels more values on it.
/// - `data_table` adds the "Show data" disclosure.
/// - `toggle_label`, `missing_label` and `empty_label` are the strings.
#[component]
pub async fn line_chart(
    cx: &Cx,
    series: &[ChartSeries<'_>],
    data: &[ChartPoint<'_>],
    summary: &str,
    #[default] variant: LineVariant,
    #[default] size: ChartSize,
    #[default(format_chart_value)] format_value: ChartFormat,
    #[default(true)] legend: bool,
    #[default(true)] axis: bool,
    #[default]
    #[into]
    points: Option<bool>,
    #[default] domain: ChartDomain,
    #[default] ticks: &[f64],
    #[default]
    #[into]
    data_table: Option<ChartTable<'_>>,
    #[default("Show data")] toggle_label: &str,
    #[default("No data")] missing_label: &str,
    #[default("No data")] empty_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let count = data.len();
    let values = (0..series.len())
        .map(|index| series_values(data, index))
        .collect::<Vec<_>>();
    let (floor, peak) = line_scale(&values, domain);
    let show_points = points.unwrap_or(count == 1);
    let lines = series
        .iter()
        .zip(&values)
        .enumerate()
        .filter(|_| count > 1)
        .map(|(index, (one, values))| Line {
            tone: series_tone(one, index).as_str(),
            pattern: series_pattern(one, index).as_str(),
            line: line_path(values, floor, peak),
            fill: if variant == LineVariant::Area {
                area_path(values, floor, peak)
            } else {
                String::new()
            },
        })
        .filter(|line| !line.line.is_empty())
        .collect::<Vec<_>>();
    // Dots on every point when asked, and on lone points between gaps.
    let dots = series
        .iter()
        .zip(&values)
        .enumerate()
        .flat_map(|(index, (one, values))| {
            let lone = chart_runs(values)
                .into_iter()
                .filter(|run| run.len() == 1)
                .map(|run| run[0].0)
                .collect::<Vec<_>>();
            let tone = series_tone(one, index).as_str();
            values
                .iter()
                .enumerate()
                .filter_map(move |(point, value)| {
                    let value = (*value)?;
                    (show_points || (count > 1 && lone.contains(&point))).then(|| Dot {
                        tone,
                        style: format!(
                            "--x: {}%; --y: {}%",
                            coordinate(line_x(point, count) / LINE_WIDTH * 100.0),
                            coordinate(line_y(value, floor, peak) / LINE_HEIGHT * 100.0)
                        ),
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ticks = tick_values(ticks, floor, peak)
        .into_iter()
        .map(|tick| {
            (
                format_value(tick),
                format!(
                    "--y: {}%",
                    coordinate(line_y(tick, floor, peak) / LINE_HEIGHT * 100.0)
                ),
                tick == floor,
            )
        })
        .collect::<Vec<_>>();
    let floor_text =
        (axis && floor != 0.0 && !ticks.iter().any(|tick| tick.2)).then(|| format_value(floor));
    let slots = data
        .iter()
        .map(|point| point_text(point, series, format_value, missing_label))
        .collect::<Vec<_>>();
    let first = data.first().map(|point| point.label);
    let last = (count > 1)
        .then(|| data.last().map(|point| point.label))
        .flatten();
    let peak_text = format_value(peak);
    let empty = data.is_empty();
    let swatch = ChartSwatch::Line;
    let view_box = format!("0 0 {LINE_WIDTH} {LINE_HEIGHT}");
    Ok(view! {
        <figure
            class=(class!(chart_css::ROOT, css::ROOT, attrs.remove("class")))
            data-size=(size.as_str())
            data-variant=(variant.as_str())
            data-chart="line"
            data-ltr-axis=""
            (attrs)
        >
            if legend {
                chart_legend(series: series, swatch: swatch)
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
                        if let Some(floor_text) = floor_text {
                            <span class=(css::FLOOR)>(floor_text)</span>
                        }
                        if axis {
                            for (text, style, at_floor) in ticks {
                                <span
                                    class=(css::TICK)
                                    data-floor=(at_floor.then_some(""))
                                    data-tick=""
                                    style=(style)
                                >
                                    (text)
                                </span>
                            }
                        }
                        if empty {
                            <span class=(chart_css::EMPTY)>(empty_label)</span>
                        }
                        <svg
                            aria-hidden="true"
                            focusable="false"
                            class=(chart_css::SVG)
                            viewBox=(view_box)
                            preserveAspectRatio="none"
                        >
                            for line in lines {
                                <g
                                    class=(chart_css::TONE)
                                    data-tone=(line.tone)
                                    data-series=""
                                >
                                    if !line.fill.is_empty() {
                                        <path class=(css::FILL) d=(line.fill)></path>
                                    }
                                    <path
                                        class=(css::LINE)
                                        data-pattern=(line.pattern)
                                        d=(line.line)
                                        vector-effect="non-scaling-stroke"
                                    ></path>
                                </g>
                            }
                        </svg>
                        for dot in dots {
                            <span
                                aria-hidden="true"
                                class=(class!(chart_css::TONE, css::POINT))
                                data-tone=(dot.tone)
                                data-point=""
                                style=(dot.style)
                            ></span>
                        }
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
    use super::{ChartDomain, area_path, line_path, line_scale, line_x, line_y, tick_values};

    #[test]
    fn scales_fit_the_data_or_the_domain() {
        let values = vec![vec![Some(10.0), None, Some(40.0)], vec![Some(-5.0)]];
        assert_eq!(line_scale(&values, ChartDomain::default()), (0.0, 40.0));
        assert_eq!(
            line_scale(&values, ChartDomain::new(0.0, 100.0)),
            (0.0, 100.0)
        );
        assert_eq!(line_scale(&values, ChartDomain::max(50.0)), (0.0, 50.0));
        // A floor above the data, and no data at all.
        assert_eq!(
            line_scale(
                &values,
                ChartDomain {
                    min: Some(60.0),
                    max: None
                }
            ),
            (60.0, 60.0)
        );
        assert_eq!(line_scale(&[], ChartDomain::default()), (0.0, 0.0));
        // Only negative values: the scale is empty.
        assert_eq!(
            line_scale(&[vec![Some(-3.0)]], ChartDomain::default()),
            (0.0, 0.0)
        );
    }

    #[test]
    fn positions() {
        assert_eq!(line_x(0, 1), 500.0);
        assert_eq!(line_x(0, 3), 0.0);
        assert_eq!(line_x(1, 3), 500.0);
        assert_eq!(line_x(2, 3), 1000.0);
        assert_eq!(line_y(50.0, 0.0, 100.0), 50.0);
        assert_eq!(line_y(0.0, 0.0, 100.0), 100.0);
        // Outside the scale, at its edge; negative values at 0.
        assert_eq!(line_y(150.0, 0.0, 100.0), 0.0);
        assert_eq!(line_y(-10.0, 0.0, 100.0), 100.0);
        assert_eq!(line_y(75.0, 50.0, 100.0), 50.0);
        // An empty scale (all values equal to the floor) draws at the bottom.
        assert_eq!(line_y(0.0, 0.0, 0.0), 100.0);
    }

    #[test]
    fn paths_split_at_gaps() {
        let values = [Some(0.0), Some(50.0), None, Some(100.0), Some(50.0)];
        assert_eq!(
            line_path(&values, 0.0, 100.0),
            "M0.00,100.00 L250.00,50.00 M750.00,0.00 L1000.00,50.00"
        );
        assert_eq!(
            area_path(&values, 0.0, 100.0),
            "M0.00,100.00 L0.00,100.00 L250.00,50.00 L250.00,100.00 Z \
             M750.00,100.00 L750.00,0.00 L1000.00,50.00 L1000.00,100.00 Z"
        );
        // A lone point is a move with no line, and no area.
        let lone = [Some(10.0), None, Some(20.0), None];
        assert_eq!(line_path(&lone, 0.0, 20.0), "M0.00,50.00 M666.67,0.00");
        assert_eq!(area_path(&lone, 0.0, 20.0), "");
        assert_eq!(line_path(&[None, None], 0.0, 1.0), "");
    }

    #[test]
    fn flat_and_single_values() {
        assert_eq!(
            line_path(&[Some(5.0), Some(5.0)], 0.0, 5.0),
            "M0.00,0.00 L1000.00,0.00"
        );
        assert_eq!(line_path(&[Some(5.0)], 0.0, 5.0), "M500.00,0.00");
    }

    #[test]
    fn ticks_inside_the_scale() {
        assert_eq!(
            tick_values(&[0.0, 50.0, 50.0, 100.0, 150.0, -1.0, f64::NAN], 0.0, 100.0),
            vec![0.0, 50.0]
        );
        assert_eq!(tick_values(&[60.0, 80.0], 60.0, 100.0), vec![60.0, 80.0]);
        assert!(tick_values(&[0.0], 0.0, 0.0).is_empty());
    }
}
