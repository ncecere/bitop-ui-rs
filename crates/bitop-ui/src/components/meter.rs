//! How much of a limit is used: storage, documents, queries today. Unlike
//! [`progress`](super::progress), it measures an amount within a range, not a task
//! that completes.
//!
//! ```ignore
//! view! {
//!     meter(label: "Storage", value: 8.6, max: 10.0, unit: "GB")
//!     meter(label: "Queries today", value: 4120.0, max: 5000.0,
//!         marker: MeterMarker { value: 4000.0, label: "Team limit" })
//!     meter(label: "Documents", value: 58.0, max: None)
//! }
//! ```
//!
//! At `warning_at` (80% of the way from `min` to `max`) the bar takes the warning
//! colour and "Near limit" is shown; at `critical_at` (100%) it takes the danger
//! colour with "At limit", or "Over limit" above `max`. The word and an icon always
//! come with the colour, and the word is part of the meter's `aria-valuetext`
//! ("4,120 of 5,000, near limit"). A value above `max` keeps its real number in the
//! text while the bar stops at full.
//!
//! `marker` draws a tick on the track, such as a soft limit below a hard ceiling; it
//! is described in text under the bar. A `max` of `None` shows the usage with "No
//! limit" and no bar.
//!
//! Differences from upstream: the label and the description are text, not nodes.
//! Upstream's `formatValue` function is a `unit` written after each number; for any
//! other wording pass `value_text`. Numbers are grouped the English way
//! ([`format_number`](crate::format::format_number)). The meter is named with
//! `aria-label`, and the description is tied to it only when `description_id` is
//! given.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::{css::meter as css, format::format_number};

/// Where a value stands against its limit. Rendered as `data-level`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MeterLevel {
    #[default]
    Normal,
    Warning,
    Critical,
    Over,
}

impl MeterLevel {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Warning => "warning",
            Self::Critical => "critical",
            Self::Over => "over",
        }
    }
}

/// The size of a [`meter`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MeterSize {
    Sm,
    #[default]
    Md,
}

impl MeterSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A tick on the track of a [`meter`], such as a soft limit below the ceiling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterMarker<'a> {
    /// Where the tick is, on the meter's scale.
    pub value: f64,
    /// Names the marker, such as "Team limit". Shown under the bar with its value.
    pub label: &'a str,
}

/// The level of `value` against `max` with the given thresholds, which are fractions
/// of the range: `(value - min) / (max - min)`. Upstream's defaults are 0.8 and 1.
///
/// With no limit, or an empty range, the level is normal. Above `max` the level is
/// [`MeterLevel::Over`], unless `critical_at` is itself above 1.
#[must_use]
pub fn meter_level(
    value: f64,
    max: Option<f64>,
    warning_at: f64,
    critical_at: f64,
    min: f64,
) -> MeterLevel {
    let Some(max) = max else {
        return MeterLevel::Normal;
    };
    if max <= min {
        return MeterLevel::Normal;
    }
    let ratio = (value - min) / (max - min);
    if ratio > 1.0 && critical_at <= 1.0 {
        MeterLevel::Over
    } else if ratio >= critical_at {
        MeterLevel::Critical
    } else if ratio >= warning_at {
        MeterLevel::Warning
    } else {
        MeterLevel::Normal
    }
}

/// How far `value` is along the range from `min` to `max`, as a percentage kept
/// between 0 and 100. An empty range gives 0.
#[must_use]
pub fn meter_position(value: f64, min: f64, max: f64) -> f64 {
    let span = max - min;
    if span > 0.0 && value.is_finite() {
        ((value - min) / span * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    }
}

/// A meter.
///
/// - `label` is shown above the bar and names the meter. `hide_label` keeps it for
///   assistive technology only.
/// - `value` is the amount used; `min` and `max` are the range. A `max` of `None`
///   means there is no limit.
/// - `warning_at` and `critical_at` are as in [`meter_level`].
/// - `marker` is a tick on the track.
/// - `unit` is written after each number: `"GB"` gives "8.6 GB of 10 GB".
/// - `value_text` replaces the "X of Y" text, shown and announced.
/// - `show_status` shows the status word and icon from the warning level up.
/// - `description` is muted text under the bar. `description_id` is its ID; give one
///   to have the description announced with the meter.
/// - `of_label`, `warning_label`, `critical_label`, `over_label` and
///   `unlimited_label` are the strings.
#[component]
pub async fn meter(
    cx: &Cx,
    label: &str,
    value: f64,
    #[into] max: Option<f64>,
    #[default] min: f64,
    #[default(0.8)] warning_at: f64,
    #[default(1.0)] critical_at: f64,
    #[default]
    #[into]
    marker: Option<MeterMarker<'_>>,
    #[default] unit: &str,
    #[default] value_text: &str,
    #[default] hide_label: bool,
    #[default(true)] show_status: bool,
    #[default] description: &str,
    #[default] description_id: &str,
    #[default] size: MeterSize,
    #[default("of")] of_label: &str,
    #[default("Near limit")] warning_label: &str,
    #[default("At limit")] critical_label: &str,
    #[default("Over limit")] over_label: &str,
    #[default("No limit")] unlimited_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let number = |n: f64| {
        let text = format_number(n, 3, false);
        if unit.is_empty() {
            text
        } else {
            format!("{text} {unit}")
        }
    };
    let level = meter_level(value, max, warning_at, critical_at, min);
    let status = match level {
        MeterLevel::Normal => "",
        MeterLevel::Warning => warning_label,
        MeterLevel::Critical => critical_label,
        MeterLevel::Over => over_label,
    };
    let status_icon = if level == MeterLevel::Warning {
        Icon::TriangleAlert
    } else {
        Icon::CircleAlert
    };
    let label_class = class!(css::LABEL, hide_label.then_some("sr-only"));
    let class = class!(css::ROOT, attrs.remove("class"));
    let text = match (value_text.is_empty(), max) {
        (false, _) => value_text.to_owned(),
        (true, Some(max)) => format!("{} {of_label} {}", number(value), number(max)),
        (true, None) => number(value),
    };
    let limit = max.unwrap_or(min);
    let value_now = format_plain(value.clamp(min, limit.max(min)));
    let value_min = format_plain(min);
    let value_max = format_plain(limit);
    let value_announced = if status.is_empty() {
        text.clone()
    } else {
        format!("{text}, {}", status.to_lowercase())
    };
    let fill = format!(
        "width: {}%",
        format_plain(meter_position(value, min, limit))
    );
    let marker_text = marker.map(|marker| format!("{}: {}", marker.label, number(marker.value)));
    let marker_style = marker.map(|marker| {
        format!(
            "--meter-marker: {}%",
            format_plain(meter_position(marker.value, min, limit))
        )
    });
    let has_description = !description.is_empty() || marker_text.is_some();
    let described_by = (has_description && !description_id.is_empty()).then_some(description_id);
    let description_id = (!description_id.is_empty()).then_some(description_id);
    Ok(view! {
        if max.is_none() {
            <div
                class=(class)
                data-size=(size.as_str())
                data-level="normal"
                data-meter=""
                (attrs)
            >
                <div class=(css::HEADER)>
                    <span class=(label_class)>(label)</span>
                    <span class=(css::VALUE)>
                        (text)
                        " · "
                        (unlimited_label)
                    </span>
                </div>
                if !description.is_empty() {
                    <p id=(description_id) class=(css::DESCRIPTION)>(description)</p>
                }
            </div>
        } else {
            <div
                role="meter"
                aria-label=(label)
                aria-valuenow=(value_now)
                aria-valuemin=(value_min)
                aria-valuemax=(value_max)
                aria-valuetext=(value_announced)
                aria-describedby=(described_by)
                class=(class)
                data-size=(size.as_str())
                data-level=(level.as_str())
                data-meter=""
                (attrs)
            >
                <div class=(css::HEADER)>
                    <span class=(label_class)>(label)</span>
                    <span aria-hidden="true" class=(css::VALUE)>
                        if show_status && !status.is_empty() {
                            <span class=(css::STATUS)>
                                icon(
                                    name: status_icon,
                                    attrs: attributes! { class=(css::STATUS_ICON) }
                                )
                                (status)
                            </span>
                        }
                        (text)
                    </span>
                </div>
                <div class=(css::TRACK)>
                    <span class=(css::INDICATOR) style=(fill)></span>
                    if let Some(marker_style) = marker_style {
                        <span
                            aria-hidden="true"
                            class=(css::MARKER)
                            style=(marker_style)
                        ></span>
                    }
                </div>
                if has_description {
                    <p id=(description_id) class=(css::DESCRIPTION)>
                        if let Some(marker_text) = &marker_text {
                            <span class=(css::MARKER_TEXT)>
                                <span aria-hidden="true" class=(css::MARKER_KEY)></span>
                                (marker_text)
                            </span>
                        }
                        if marker_text.is_some() && !description.is_empty() {
                            " · "
                        }
                        (description)
                    </p>
                }
            </div>
        }
    })
}

/// A number for an attribute or a style: no grouping, no trailing zeros.
fn format_plain(n: f64) -> String {
    let text = format!("{n:.4}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" {
        "0".to_owned()
    } else {
        text.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::{MeterLevel::*, format_plain, meter_level, meter_position};

    #[test]
    fn levels_at_the_default_thresholds() {
        let level = |value| meter_level(value, Some(100.0), 0.8, 1.0, 0.0);
        assert_eq!(level(0.0), Normal);
        assert_eq!(level(79.9), Normal);
        assert_eq!(level(80.0), Warning);
        assert_eq!(level(99.9), Warning);
        assert_eq!(level(100.0), Critical);
        assert_eq!(level(100.1), Over);
    }

    #[test]
    fn levels_follow_the_range_and_the_thresholds() {
        // The thresholds are fractions of the range, not of max.
        assert_eq!(meter_level(180.0, Some(200.0), 0.8, 1.0, 100.0), Warning);
        assert_eq!(meter_level(170.0, Some(200.0), 0.8, 1.0, 100.0), Normal);
        assert_eq!(meter_level(50.0, Some(100.0), 0.5, 0.9, 0.0), Warning);
        assert_eq!(meter_level(90.0, Some(100.0), 0.5, 0.9, 0.0), Critical);
        // A critical threshold above the limit: over the limit is only a warning.
        assert_eq!(meter_level(110.0, Some(100.0), 0.8, 1.2, 0.0), Warning);
        assert_eq!(meter_level(120.0, Some(100.0), 0.8, 1.2, 0.0), Critical);
    }

    #[test]
    fn no_limit_or_an_empty_range_is_normal() {
        assert_eq!(meter_level(1e9, None, 0.8, 1.0, 0.0), Normal);
        assert_eq!(meter_level(5.0, Some(0.0), 0.8, 1.0, 0.0), Normal);
        assert_eq!(meter_level(5.0, Some(1.0), 0.8, 1.0, 1.0), Normal);
    }

    #[test]
    fn positions_are_clamped_percentages() {
        assert_eq!(meter_position(8.6, 0.0, 10.0), 86.0);
        assert_eq!(meter_position(150.0, 100.0, 200.0), 50.0);
        assert_eq!(meter_position(12.0, 0.0, 10.0), 100.0);
        assert_eq!(meter_position(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(meter_position(1.0, 0.0, 0.0), 0.0);
        assert_eq!(meter_position(f64::NAN, 0.0, 10.0), 0.0);
    }

    #[test]
    fn plain_numbers() {
        assert_eq!(format_plain(86.0), "86");
        assert_eq!(format_plain(8.6), "8.6");
        assert_eq!(format_plain(100.0 / 3.0), "33.3333");
        assert_eq!(format_plain(-0.0), "0");
        assert_eq!(format_plain(4120.0), "4120");
    }
}
