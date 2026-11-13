//! A slider, or a range with two thumbs, on `<input type="range">`, ported from
//! bitop-ui's `Slider`.
//!
//! The browser supplies dragging, the keyboard pattern (arrow keys step, Page Up and
//! Page Down take larger steps, Home and End go to the ends) and form submission.
//! The library script keeps the filled part of the track and the value readout in
//! step with the thumbs.
//!
//! A range (`range: (40.0, 160.0)`) is two range inputs over one track, both
//! submitted under `name`, the minimum first. Each thumb is a tab stop of its own and
//! is named by `thumb_labels`, or "{label} minimum" and "{label} maximum". The
//! script keeps the thumbs `min_steps_between` steps apart, and a press on the track
//! moves the nearer thumb there and drags it.
//!
//! Name the slider with a visible `label`, or with an `aria-label` in `attrs`.
//!
//! ```ignore
//! view! {
//!     slider(id: &id, name: "volume", label: "Volume", value: 40.0, show_value: true)
//!     slider(id: &price, name: "price", label: "Price", range: (40.0, 160.0),
//!         max: 250.0, step: 5.0, min_steps_between: 2.0, show_value: true,
//!         format: "${value}", thumb_labels: &["Minimum price", "Maximum price"])
//! }
//! ```
//!
//! Differences from upstream: no more than two thumbs. The readout and the text for
//! assistive technology are templates, not functions, and there is no `format` with
//! `Intl` options: write the currency into the template. A thumb cannot be dragged
//! past the other one; upstream's thumbs stop there too.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::slider as css;

/// The size of a [`slider`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderSize {
    Sm,
    #[default]
    Md,
}

impl SliderSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// Where `value` lies between `min` and `max`, from 0 to 1.
fn fraction(value: f64, min: f64, max: f64) -> f64 {
    if max > min {
        (value - min) / (max - min)
    } else {
        0.0
    }
}

/// One thumb, as it is rendered.
struct Thumb {
    id: String,
    value: String,
    label: Option<String>,
    value_text: Option<String>,
}

/// A slider, or a range when `range` is set.
///
/// - `id` goes on the `<input>` (on the first one of a range; the second is
///   `{id}-end`); the label and the readout point at it.
/// - `value` is the initial value, between `min` (default 0) and `max` (default 100),
///   in steps of `step` (default 1).
/// - `range` makes two thumbs, with these initial values; `min_steps_between` is how
///   many steps they keep apart.
/// - `label` is the visible label; `hide_label` keeps it for assistive technology only.
/// - `thumb_labels` names the thumbs of a range, such as
///   `&["Minimum price", "Maximum price"]`.
/// - `show_value` shows the current value beside the label, as an `<output>`; a
///   range shows both, joined by " – ".
/// - `format` is the readout of a value, with `{value}` replaced, such as `"{value}%"`.
/// - `value_text` is what assistive technology reads for the value, with `{value}`
///   replaced, such as `"{value} percent"`. By default it is the number alone.
/// - `vertical` turns the slider upright, with the maximum at the top.
/// - `attrs` are forwarded to the `<input>`, or to both inputs of a range.
#[component]
pub async fn slider(
    cx: &Cx,
    id: &str,
    #[default] name: &str,
    #[default] value: f64,
    #[default]
    #[into]
    range: Option<(f64, f64)>,
    #[default] min: f64,
    #[default(100.0)] max: f64,
    #[default(1.0)] step: f64,
    #[default] min_steps_between: f64,
    #[default] label: &str,
    #[default] hide_label: bool,
    #[default] thumb_labels: &[&str],
    #[default] show_value: bool,
    #[default("{value}")] format: &str,
    #[default] value_text: &str,
    #[default] size: SliderSize,
    #[default] vertical: bool,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let top = max.max(min);
    let values: Vec<f64> = match range {
        Some((start, end)) => {
            let start = start.clamp(min, top);
            vec![start, end.clamp(start, top)]
        }
        None => vec![value.clamp(min, top)],
    };
    let is_range = values.len() > 1;
    // A range's inputs carry their own names, so the caller's `aria-label` names the
    // group instead, and is the base of the thumbs' names.
    let group_label = if is_range {
        crate::take_attribute(&mut attrs, "aria-label")
            .or_else(|| (!label.is_empty()).then(|| label.to_owned()))
    } else {
        None
    };
    let base_name = group_label.as_deref();
    let thumbs: Vec<Thumb> = values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let shown = value.to_string();
            let own = thumb_labels.get(index).filter(|l| !l.is_empty());
            let label = if is_range {
                own.map(|l| (*l).to_owned()).or_else(|| {
                    base_name.map(|base| {
                        format!("{base} {}", if index == 0 { "minimum" } else { "maximum" })
                    })
                })
            } else {
                own.map(|l| (*l).to_owned())
            };
            Thumb {
                id: if index == 0 {
                    id.to_owned()
                } else {
                    format!("{id}-end")
                },
                value_text: (!value_text.is_empty()).then(|| value_text.replace("{value}", &shown)),
                value: shown,
                label,
            }
        })
        .collect();
    let readout = values
        .iter()
        .map(|value| format.replace("{value}", &value.to_string()))
        .collect::<Vec<_>>()
        .join(" – ");
    let output_for = thumbs
        .iter()
        .map(|thumb| thumb.id.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    // The filled part of the track runs from --slider-from to --slider-fill (0 to 1),
    // on the first input, whose track is the one drawn.
    let style = if is_range {
        format!(
            "--slider-from: {}; --slider-fill: {}",
            fraction(values[0], min, max),
            fraction(values[1], min, max)
        )
    } else {
        format!("--slider-fill: {}", fraction(values[0], min, max))
    };
    let style = crate::merge_style(&mut attrs, &style);
    let extra = attrs.remove("class");
    let orientation = if vertical { "vertical" } else { "horizontal" };
    let has_header = !label.is_empty() || show_value;
    let name = (!name.is_empty()).then_some(name);
    let min_steps = (min_steps_between > 0.0).then(|| min_steps_between.to_string());
    Ok(view! {
        <div
            class=(css::ROOT)
            role=(is_range.then_some("group"))
            aria-label=(group_label)
            data-slider=""
            data-size=(size.as_str())
            data-orientation=(orientation)
        >
            if has_header {
                <div class=(css::HEADER) data-hidden-label=(hide_label && !show_value)>
                    if !label.is_empty() {
                        <label
                            for=(id)
                            class=(class!(css::LABEL, "sr-only" if hide_label))
                        >
                            (label)
                        </label>
                    }
                    if show_value {
                        <output
                            for=(output_for)
                            class=(css::VALUE)
                            data-format=(format)
                        >
                            (readout)
                        </output>
                    }
                </div>
            }
            <div
                class=(css::CONTROL)
                data-orientation=(orientation)
                data-range=(is_range)
                data-min-steps=(min_steps)
            >
                for (index, thumb) in thumbs.into_iter().enumerate() {
                    <input
                        type="range"
                        id=(thumb.id)
                        name=(name)
                        min=(min.to_string())
                        max=(max.to_string())
                        step=(step.to_string())
                        value=(thumb.value)
                        disabled=(disabled)
                        aria-label=(thumb.label)
                        aria-orientation=(vertical.then_some("vertical"))
                        aria-valuetext=(thumb.value_text)
                        data-value-text=((!value_text.is_empty()).then_some(value_text))
                        data-thumb=(if !is_range {
                            None
                        } else if index == 0 {
                            Some("start")
                        } else {
                            Some("end")
                        })
                        style=(if index == 0 { style.clone() } else { None })
                        class=(class!(css::INPUT, extra.clone()))
                        (attrs.clone())
                    >
                }
            </div>
        </div>
    })
}
