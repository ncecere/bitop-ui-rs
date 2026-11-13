//! What the charts share: series colours and line patterns, the legend, the "Show
//! data" table, and the arithmetic for missing values.
//!
//! [`bar_chart`](super::bar_chart::bar_chart), [`line_chart`](super::line_chart::line_chart)
//! and [`sparkline`](super::sparkline::sparkline) are built on it. Use the parts here
//! directly for a legend or a data table beside a chart of your own.
//!
//! ```ignore
//! let series = [ChartSeries::new("Answers"), ChartSeries::new("Conversations")];
//! let days = [
//!     ChartPoint::new("Sep 1", [61.0, 20.0]),
//!     ChartPoint::new("Sep 2", [74.0, f64::NAN]),
//! ];
//! view! {
//!     chart_legend(series: &series, swatch: ChartSwatch::Line)
//!     chart_data(caption: "Answers per day", series: &series, data: &days)
//! }
//! ```
//!
//! Colours come from the `--color-chart-*` tokens, which are at least 3:1 on the
//! surface and the page in every theme. Lines also differ by pattern (solid, dashed or
//! dotted), shown in their legend swatches; bars and square swatches are solid, as
//! upstream draws them.
//!
//! Values: NaN and infinities are "no data". Charts leave them out of the scale and
//! draw a gap, and the table shows "No data". A point with fewer values than there
//! are series counts the missing ones as 0.
//!
//! Differences from upstream: series are matched to values by position, not by key,
//! and a series label is text. `formatValue` is a function pointer, `format_value`.
//! The data table's
//! disclosure has no `onOpenChange`; listen for the native `toggle` event.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use super::{
    disclosure::disclosure,
    table::{TableDensity, table, table_column, td, th, tr},
};
use crate::{css::chart as css, format::format_number};

/// The colour of a series, from the `--color-chart-*` tokens. Rendered as `data-tone`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartTone {
    #[default]
    Primary,
    Info,
    Success,
    Warning,
    Danger,
    Neutral,
    /// Magenta: a seventh hue, apart from primary and info.
    Accent,
}

impl ChartTone {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Neutral => "neutral",
            Self::Accent => "accent",
        }
    }
}

/// The tones in the order series without a tone of their own take them.
pub const CHART_TONES: [ChartTone; 7] = [
    ChartTone::Primary,
    ChartTone::Info,
    ChartTone::Success,
    ChartTone::Warning,
    ChartTone::Danger,
    ChartTone::Neutral,
    ChartTone::Accent,
];

/// How a series is drawn besides its colour. Rendered as `data-pattern`.
///
/// A line, and its legend swatch, is solid, dashed or dotted. Bars and square
/// swatches are always solid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartPattern {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl ChartPattern {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }
}

const PATTERNS: [ChartPattern; 3] = [
    ChartPattern::Solid,
    ChartPattern::Dashed,
    ChartPattern::Dotted,
];

/// One series of a chart: `ChartSeries::new("Answers").tone(ChartTone::Info)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartSeries<'a> {
    /// Names the series in the legend, the hover readout and the data table.
    pub label: &'a str,
    /// Its colour. `None` takes the next of [`CHART_TONES`] by position.
    pub tone: Option<ChartTone>,
    /// Its pattern. `None` takes solid, dashed, dotted by position.
    pub pattern: Option<ChartPattern>,
}

impl<'a> ChartSeries<'a> {
    /// A series with the tone and pattern of its position.
    #[must_use]
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            tone: None,
            pattern: None,
        }
    }

    /// Sets the colour.
    #[must_use]
    pub fn tone(mut self, tone: ChartTone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// Sets the pattern.
    #[must_use]
    pub fn pattern(mut self, pattern: ChartPattern) -> Self {
        self.pattern = Some(pattern);
        self
    }
}

/// One point of a chart: a label, such as a date, and a value for each series, in
/// the order of the series. NaN means no data.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartPoint<'a> {
    /// Shown on the axis, in the hover readout and as the table's row header.
    pub label: &'a str,
    /// One value per series. A missing value counts as 0.
    pub values: Vec<f64>,
}

impl<'a> ChartPoint<'a> {
    /// A point with its values in the order of the series.
    #[must_use]
    pub fn new(label: &'a str, values: impl Into<Vec<f64>>) -> Self {
        Self {
            label,
            values: values.into(),
        }
    }
}

/// The "Show data" table of a chart: `ChartTable::new("Answers per day")`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartTable<'a> {
    /// Names the table, such as "Answers per day".
    pub caption: &'a str,
    /// The header of the label column.
    pub label_header: &'a str,
    /// Whether the disclosure starts open.
    pub open: bool,
}

impl<'a> ChartTable<'a> {
    /// A closed table whose label column is headed "Date".
    #[must_use]
    pub fn new(caption: &'a str) -> Self {
        Self {
            caption,
            label_header: "Date",
            open: false,
        }
    }

    /// Sets the header of the label column.
    #[must_use]
    pub fn label_header(mut self, label_header: &'a str) -> Self {
        self.label_header = label_header;
        self
    }

    /// Starts the disclosure open.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

/// The height of a bar or line chart. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartSize {
    /// 6rem.
    Sm,
    /// 10rem.
    #[default]
    Md,
    /// 14rem.
    Lg,
}

impl ChartSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// The shape of a legend swatch. Rendered as `data-shape`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartSwatch {
    /// A square, for bars.
    #[default]
    Square,
    /// A short stroke, for lines.
    Line,
}

impl ChartSwatch {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Square => "square",
            Self::Line => "line",
        }
    }
}

/// Turns a value into text for the peak label, the readout and the data table.
pub type ChartFormat = fn(f64) -> String;

/// The default [`ChartFormat`]: grouped the English way, up to three decimals.
#[must_use]
pub fn format_chart_value(value: f64) -> String {
    format_number(value, 3, false)
}

/// The tone of the series at `index`: its own, else the next of [`CHART_TONES`].
#[must_use]
pub fn series_tone(series: &ChartSeries<'_>, index: usize) -> ChartTone {
    series
        .tone
        .unwrap_or(CHART_TONES[index % CHART_TONES.len()])
}

/// The pattern of the series at `index`: its own, else solid, dashed, dotted.
#[must_use]
pub fn series_pattern(series: &ChartSeries<'_>, index: usize) -> ChartPattern {
    series.pattern.unwrap_or(PATTERNS[index % PATTERNS.len()])
}

/// A point's value for the series at `index`: the number if it is finite, 0 when the
/// point has no value there, and `None` (no data) for NaN or an infinity.
#[must_use]
pub fn chart_value(point: &ChartPoint<'_>, index: usize) -> Option<f64> {
    let value = point.values.get(index).copied().unwrap_or(0.0);
    value.is_finite().then_some(value)
}

/// The values of the series at `index`, one per point.
#[must_use]
pub fn series_values(data: &[ChartPoint<'_>], index: usize) -> Vec<Option<f64>> {
    data.iter().map(|point| chart_value(point, index)).collect()
}

/// Runs of consecutive points with data, as `(index, value)` pairs. A line is drawn
/// per run, so a missing value leaves a gap.
#[must_use]
pub fn chart_runs(values: &[Option<f64>]) -> Vec<Vec<(usize, f64)>> {
    let mut runs = Vec::new();
    let mut run = Vec::new();
    for (index, value) in values.iter().enumerate() {
        match value {
            Some(value) => run.push((index, *value)),
            None if !run.is_empty() => runs.push(std::mem::take(&mut run)),
            None => {}
        }
    }
    if !run.is_empty() {
        runs.push(run);
    }
    runs
}

/// What a point holds, for the hover readout: "Sep 1: 61 Answers, 20 Conversations",
/// or "Sep 2: Conversations: No data".
#[must_use]
pub fn point_text(
    point: &ChartPoint<'_>,
    series: &[ChartSeries<'_>],
    format_value: ChartFormat,
    missing_label: &str,
) -> String {
    let values = series
        .iter()
        .enumerate()
        .map(|(index, series)| match chart_value(point, index) {
            Some(value) => format!("{} {}", format_value(value), series.label),
            None => format!("{}: {missing_label}", series.label),
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}: {values}", point.label)
}

/// A coordinate for an SVG attribute or a style: two decimals, as upstream writes.
pub(crate) fn coordinate(n: f64) -> String {
    let text = format!("{n:.2}");
    if text == "-0.00" {
        "0.00".to_owned()
    } else {
        text
    }
}

/// The series legend: a list of swatches and labels.
///
/// `swatch` is a square for bars and a line for lines. Each swatch shows the series'
/// colour; a line also shows its pattern.
#[component]
pub async fn chart_legend(
    cx: &Cx,
    series: &[ChartSeries<'_>],
    #[default] swatch: ChartSwatch,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let items = series
        .iter()
        .enumerate()
        .map(|(index, series)| {
            (
                series.label.to_owned(),
                series_tone(series, index).as_str(),
                (swatch == ChartSwatch::Line).then(|| series_pattern(series, index).as_str()),
            )
        })
        .collect::<Vec<_>>();
    Ok(view! {
        if !items.is_empty() {
            <ul
                class=(class!(css::LEGEND, attrs.remove("class")))
                data-chart-legend=""
                (attrs)
            >
                for (label, tone, pattern) in items {
                    <li class=(css::LEGEND_ITEM)>
                        <span
                            aria-hidden="true"
                            class=(class!(css::TONE, css::SWATCH))
                            data-tone=(tone)
                            data-shape=(swatch.as_str())
                            data-pattern=(pattern)
                        ></span>
                        (label)
                    </li>
                }
            </ul>
        }
    })
}

/// A "Show data" disclosure with a chart's numbers in a table, the accessible
/// alternative to the plot for anyone who needs exact values.
///
/// - `caption` names the table, such as "Answers per day".
/// - `series` are its columns and `data` its rows.
/// - `label_header` heads the column of point labels.
/// - `format_value` writes each number.
/// - `toggle_label` is the disclosure's text and `missing_label` the text of a cell
///   with no data.
/// - `open` starts the disclosure open.
#[component]
pub async fn chart_data(
    cx: &Cx,
    caption: &str,
    series: &[ChartSeries<'_>],
    data: &[ChartPoint<'_>],
    #[default("Date")] label_header: &str,
    #[default(format_chart_value)] format_value: ChartFormat,
    #[default("Show data")] toggle_label: &str,
    #[default("No data")] missing_label: &str,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let headers = series
        .iter()
        .map(|series| series.label.to_owned())
        .collect::<Vec<_>>();
    let rows = data
        .iter()
        .map(|point| {
            let cells = (0..series.len())
                .map(|index| {
                    chart_value(point, index).map_or_else(|| missing_label.to_owned(), format_value)
                })
                .collect::<Vec<_>>();
            (point.label.to_owned(), cells)
        })
        .collect::<Vec<_>>();
    let label_header = label_header.to_owned();
    Ok(view! {
        disclosure(
            title: toggle_label,
            open: open,
            attrs: attributes! {
                class=(class!(css::DATA, attrs.remove("class")))
                data-chart-data=""
                (attrs)
            },
            table(
                caption: caption,
                density: TableDensity::Compact,
                max_height: "16rem",
                sticky_header: true,
                columns: view! {
                    table_column((label_header))
                    for header in headers {
                        table_column(numeric: true, (header))
                    }
                },
                for (label, cells) in rows {
                    tr(
                        th((label))
                        for cell in cells {
                            td(numeric: true, (cell))
                        }
                    )
                }
            )
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{
        ChartPattern, ChartPoint, ChartSeries, ChartTone, chart_runs, chart_value, coordinate,
        format_chart_value, point_text, series_pattern, series_tone, series_values,
    };

    #[test]
    fn values_missing_and_not_finite() {
        let point = ChartPoint::new("a", [3.0, f64::NAN, f64::INFINITY, -2.0]);
        assert_eq!(chart_value(&point, 0), Some(3.0));
        assert_eq!(chart_value(&point, 1), None);
        assert_eq!(chart_value(&point, 2), None);
        assert_eq!(chart_value(&point, 3), Some(-2.0));
        // A point with fewer values than series: the rest count as 0.
        assert_eq!(chart_value(&point, 4), Some(0.0));
    }

    #[test]
    fn runs_split_at_gaps() {
        assert_eq!(chart_runs(&[]), Vec::<Vec<(usize, f64)>>::new());
        assert_eq!(chart_runs(&[Some(1.0)]), vec![vec![(0, 1.0)]]);
        assert_eq!(
            chart_runs(&[
                Some(1.0),
                Some(2.0),
                None,
                Some(4.0),
                None,
                None,
                Some(6.0),
                Some(7.0)
            ]),
            vec![
                vec![(0, 1.0), (1, 2.0)],
                vec![(3, 4.0)],
                vec![(6, 6.0), (7, 7.0)]
            ]
        );
        assert_eq!(chart_runs(&[None, None]), Vec::<Vec<(usize, f64)>>::new());
        let data = [
            ChartPoint::new("a", [1.0]),
            ChartPoint::new("b", [f64::NAN]),
            ChartPoint::new("c", [3.0]),
        ];
        assert_eq!(
            chart_runs(&series_values(&data, 0)),
            vec![vec![(0, 1.0)], vec![(2, 3.0)]]
        );
    }

    #[test]
    fn tones_and_patterns_by_position() {
        let plain = ChartSeries::new("a");
        assert_eq!(series_tone(&plain, 0), ChartTone::Primary);
        assert_eq!(series_tone(&plain, 1), ChartTone::Info);
        assert_eq!(series_tone(&plain, 7), ChartTone::Primary);
        assert_eq!(series_pattern(&plain, 0), ChartPattern::Solid);
        assert_eq!(series_pattern(&plain, 1), ChartPattern::Dashed);
        assert_eq!(series_pattern(&plain, 2), ChartPattern::Dotted);
        assert_eq!(series_pattern(&plain, 3), ChartPattern::Solid);
        let own = plain.tone(ChartTone::Accent).pattern(ChartPattern::Dotted);
        assert_eq!(series_tone(&own, 0), ChartTone::Accent);
        assert_eq!(series_pattern(&own, 0), ChartPattern::Dotted);
    }

    #[test]
    fn point_texts() {
        let series = [ChartSeries::new("Answers"), ChartSeries::new("Chats")];
        let point = ChartPoint::new("Sep 1", [1204.0, f64::NAN]);
        assert_eq!(
            point_text(&point, &series, format_chart_value, "No data"),
            "Sep 1: 1,204 Answers, Chats: No data"
        );
    }

    #[test]
    fn coordinates() {
        assert_eq!(coordinate(12.346), "12.35");
        assert_eq!(coordinate(-0.0001), "0.00");
        assert_eq!(coordinate(100.0), "100.00");
    }
}
