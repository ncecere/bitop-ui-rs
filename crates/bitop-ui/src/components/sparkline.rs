//! A tiny inline trend, with no axes and no legend, for stat cards, table cells and
//! summaries. Drawn on the server in SVG, as wide as its container.
//!
//! ```ignore
//! view! {
//!     stat_card(label: "Answers", value: "1,204", chart: view! {
//!         sparkline(
//!             values: &[61.0, 70.0, 66.0, 96.0],
//!             label: "Answers per day, last 4 days: rising from 61 to 96",
//!         )
//!     })
//! }
//! ```
//!
//! It is one image named by `label`: say what the trend is, since the shape alone is
//! not available to everyone. The last point is marked. Set `decorative` when the
//! same trend is already written out beside it; the sparkline is then hidden from
//! assistive technology. A sparkline with no label is decorative too.
//!
//! NaN and infinities are no data: left out of the scale and drawn as a gap (the
//! last point is not marked when it has no data). Negative values are drawn: the
//! scale runs from the smaller of 0 and the smallest value to the larger of 0 and
//! the largest. `baseline` [`SparklineBaseline::Min`] starts it at the smallest
//! value instead, which shows small changes larger.
//!
//! The trend runs from left to right in either direction, as upstream's does. There
//! is no hover readout, as upstream has none; a sparkline is too small to point at.
//!
//! Differences from upstream: the tones are the charts' seven, accent included. A
//! missing label makes the sparkline decorative, where upstream's types require one
//! of the two.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::{
    chart::{ChartTone, chart_runs, coordinate},
    line_chart::LineVariant,
};
use crate::css::{chart as chart_css, sparkline as css};

/// The height of a [`sparkline`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SparklineSize {
    /// 1.5rem.
    Sm,
    /// 2.5rem.
    #[default]
    Md,
}

impl SparklineSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// Where the scale of a [`sparkline`] starts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SparklineBaseline {
    /// At 0, or the smallest value when that is negative.
    #[default]
    Zero,
    /// At the smallest value.
    Min,
}

const SIZE: f64 = 100.0;

/// The ends of the scale, `(min, max)`, from the finite values.
#[must_use]
pub fn sparkline_scale(values: &[f64], baseline: SparklineBaseline) -> (f64, f64) {
    let finite = values.iter().copied().filter(|value| value.is_finite());
    let max = finite.clone().fold(0.0, f64::max);
    let smallest = finite.fold(f64::INFINITY, f64::min);
    let min = match baseline {
        SparklineBaseline::Min if smallest.is_finite() => smallest,
        _ => smallest.min(0.0),
    };
    (min, max)
}

/// A point's position in the 100 by 100 box, `(x, y)` from the top left. A lone
/// point sits at the end; a flat scale draws in the middle.
#[must_use]
pub fn sparkline_point(index: usize, count: usize, value: f64, min: f64, max: f64) -> (f64, f64) {
    #[allow(clippy::cast_precision_loss)]
    let x = if count <= 1 {
        SIZE
    } else {
        index as f64 / (count - 1) as f64 * SIZE
    };
    let span = max - min;
    let y = if span > 0.0 {
        SIZE - (value - min) / span * SIZE
    } else {
        SIZE / 2.0
    };
    (x, y)
}

/// The `d` of the line and of the area under it, split at the gaps.
#[must_use]
pub fn sparkline_paths(values: &[f64], min: f64, max: f64) -> (String, String) {
    let count = values.len();
    let runs = chart_runs(
        &values
            .iter()
            .map(|value| value.is_finite().then_some(*value))
            .collect::<Vec<_>>(),
    );
    let xy = |index: usize, value: f64| {
        let (x, y) = sparkline_point(index, count, value, min, max);
        format!("{},{}", coordinate(x), coordinate(y))
    };
    let line = runs
        .iter()
        .map(|run| {
            run.iter()
                .enumerate()
                .map(|(step, (index, value))| {
                    format!(
                        "{}{}",
                        if step == 0 { "M" } else { "L" },
                        xy(*index, *value)
                    )
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" ");
    let bottom = coordinate(SIZE);
    let fill = runs
        .iter()
        .filter(|run| run.len() > 1)
        .map(|run| {
            let first = sparkline_point(run[0].0, count, 0.0, min, max).0;
            let last = sparkline_point(run[run.len() - 1].0, count, 0.0, min, max).0;
            let points = run
                .iter()
                .map(|(index, value)| format!("L{}", xy(*index, *value)))
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "M{},{bottom} {points} L{},{bottom} Z",
                coordinate(first),
                coordinate(last)
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    (line, fill)
}

/// A sparkline.
///
/// - `values` in order, oldest first.
/// - `label` names it for assistive technology; `decorative` hides it instead.
/// - `variant` draws a line, or a line over a soft fill.
/// - `tone` is its colour.
/// - `size` is its height; its width follows the container.
/// - `show_last` marks the last point.
/// - `baseline` is where the scale starts.
#[component]
pub async fn sparkline(
    cx: &Cx,
    values: &[f64],
    #[default] label: &str,
    #[default] decorative: bool,
    #[default] variant: LineVariant,
    #[default] tone: ChartTone,
    #[default] size: SparklineSize,
    #[default(true)] show_last: bool,
    #[default] baseline: SparklineBaseline,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let hidden = decorative || label.is_empty();
    let count = values.len();
    let (min, max) = sparkline_scale(values, baseline);
    let (line, fill) = sparkline_paths(values, min, max);
    let line = (count > 1 && !line.is_empty()).then_some(line);
    let fill = (count > 1 && variant == LineVariant::Area && !fill.is_empty()).then_some(fill);
    let last = values
        .last()
        .filter(|value| show_last && value.is_finite())
        .map(|value| {
            let (x, y) = sparkline_point(count - 1, count, *value, min, max);
            format!(
                "--x: {}; --y: {}",
                coordinate(x / SIZE),
                coordinate(y / SIZE)
            )
        });
    let view_box = format!("0 0 {SIZE} {SIZE}");
    Ok(view! {
        <span
            role=((!hidden).then_some("img"))
            aria-label=((!hidden).then_some(label))
            aria-hidden=(hidden.then_some("true"))
            data-tone=(tone.as_str())
            data-size=(size.as_str())
            data-variant=(variant.as_str())
            data-sparkline=""
            class=(class!(chart_css::TONE, css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <svg
                aria-hidden="true"
                focusable="false"
                class=(css::SVG)
                viewBox=(view_box)
                preserveAspectRatio="none"
            >
                if let Some(fill) = fill {
                    <path class=(css::FILL) d=(fill)></path>
                }
                if let Some(line) = line {
                    <path class=(css::LINE) d=(line) vector-effect="non-scaling-stroke"></path>
                }
            </svg>
            if let Some(style) = last {
                <span class=(css::LAST) style=(style)></span>
            }
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::{SparklineBaseline, sparkline_paths, sparkline_point, sparkline_scale};

    #[test]
    fn scales() {
        let zero = SparklineBaseline::Zero;
        assert_eq!(sparkline_scale(&[2.0, 8.0, 4.0], zero), (0.0, 8.0));
        assert_eq!(
            sparkline_scale(&[2.0, 8.0, 4.0], SparklineBaseline::Min),
            (2.0, 8.0)
        );
        // Negative values extend the scale below 0.
        assert_eq!(sparkline_scale(&[-4.0, 6.0], zero), (-4.0, 6.0));
        assert_eq!(sparkline_scale(&[-4.0, -2.0], zero), (-4.0, 0.0));
        // No data is left out.
        assert_eq!(
            sparkline_scale(&[f64::NAN, 3.0, f64::INFINITY], zero),
            (0.0, 3.0)
        );
        assert_eq!(sparkline_scale(&[], zero), (0.0, 0.0));
        assert_eq!(
            sparkline_scale(&[f64::NAN], SparklineBaseline::Min),
            (0.0, 0.0)
        );
    }

    #[test]
    fn points() {
        assert_eq!(sparkline_point(0, 3, 0.0, 0.0, 10.0), (0.0, 100.0));
        assert_eq!(sparkline_point(2, 3, 10.0, 0.0, 10.0), (100.0, 0.0));
        assert_eq!(sparkline_point(1, 3, 0.0, -10.0, 10.0), (50.0, 50.0));
        // A lone point sits at the end; a flat scale draws in the middle.
        assert_eq!(sparkline_point(0, 1, 5.0, 0.0, 5.0), (100.0, 0.0));
        assert_eq!(sparkline_point(1, 2, 3.0, 3.0, 3.0), (100.0, 50.0));
    }

    #[test]
    fn paths_split_at_gaps() {
        let (line, fill) = sparkline_paths(&[0.0, 10.0, f64::NAN, 5.0, 10.0], 0.0, 10.0);
        assert_eq!(line, "M0.00,100.00 L25.00,0.00 M75.00,50.00 L100.00,0.00");
        assert_eq!(
            fill,
            "M0.00,100.00 L0.00,100.00 L25.00,0.00 L25.00,100.00 Z \
             M75.00,100.00 L75.00,50.00 L100.00,0.00 L100.00,100.00 Z"
        );
        // All equal with the minimum baseline: a flat line in the middle.
        let (line, _) = sparkline_paths(&[4.0, 4.0], 4.0, 4.0);
        assert_eq!(line, "M0.00,50.00 L100.00,50.00");
    }
}
