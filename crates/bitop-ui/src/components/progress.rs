//! A labelled progress bar.
//!
//! ```ignore
//! view! {
//!     progress(label: "Uploading", value: Some(40.0))
//!     progress(label: "Indexing", value: None)
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::progress as css;

/// The colour of a [`progress`] bar. Rendered as `data-tone`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressTone {
    #[default]
    Primary,
    Success,
    Danger,
}

impl ProgressTone {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Success => "success",
            Self::Danger => "danger",
        }
    }
}

/// A progress bar.
///
/// - `label` is shown above the bar and names it for assistive technology.
///   `hide_label` keeps it for assistive technology only.
/// - `value` runs from zero to `max`. `None` means the amount is not known, and the
///   bar animates.
/// - `show_value` shows the percentage beside the label.
/// - `small` makes the bar thinner.
#[component]
pub async fn progress(
    cx: &Cx,
    label: &str,
    value: Option<f64>,
    #[default(100.0)] max: f64,
    #[default] hide_label: bool,
    #[default(true)] show_value: bool,
    #[default] tone: ProgressTone,
    #[default] small: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let percent = value.map(|value| percent(value, max));
    let shown = show_value && percent.is_some();
    Ok(view! {
        <div
            role="progressbar"
            aria-label=(label)
            aria-valuemin="0"
            aria-valuemax=(max.to_string())
            if let Some(value) = value {
                aria-valuenow=(value.clamp(0.0, max).to_string())
            }
            if let Some(percent) = percent {
                aria-valuetext=(format!("{percent}%"))
            }
            data-tone=(tone.as_str())
            data-size=(if small { "sm" } else { "md" })
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <div
                class=(class!(css::HEADER, (hide_label && !shown).then_some("sr-only")))
            >
                <span class=(class!(css::LABEL, hide_label.then_some("sr-only")))>
                    (label)
                </span>
                if let (true, Some(percent)) = (show_value, percent) {
                    <span class=(css::VALUE)>
                        (percent)
                        "%"
                    </span>
                }
            </div>
            <div class=(css::TRACK)>
                if let Some(percent) = percent {
                    <span class=(css::INDICATOR) style=(format!("width: {percent}%"))></span>
                } else {
                    <span class=(css::INDICATOR) data-indeterminate=""></span>
                }
            </div>
        </div>
    })
}

/// `value` as a whole percentage of `max`, kept between 0 and 100.
fn percent(value: f64, max: f64) -> u8 {
    if max <= 0.0 || !value.is_finite() {
        return 0;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let percent = (value / max * 100.0).round().clamp(0.0, 100.0) as u8;
    percent
}

#[cfg(test)]
mod tests {
    use super::percent;

    #[test]
    fn percent_is_rounded_and_clamped() {
        assert_eq!(percent(40.0, 100.0), 40);
        assert_eq!(percent(1.0, 3.0), 33);
        assert_eq!(percent(250.0, 100.0), 100);
        assert_eq!(percent(-5.0, 100.0), 0);
        assert_eq!(percent(5.0, 0.0), 0);
        assert_eq!(percent(f64::NAN, 100.0), 0);
    }
}
