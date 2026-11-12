//! A slider, on `<input type="range">`.
//!
//! The browser supplies dragging, the keyboard pattern (arrow keys step, Page Up and
//! Page Down take larger steps, Home and End go to the ends) and form submission.
//! The library script keeps the filled part of the track and the value readout in
//! step with the thumb.
//!
//! Name the slider with a visible `label`, or with an `aria-label` in `attrs`.
//!
//! Differences from upstream: one thumb only, so no range with a minimum and a
//! maximum thumb; use two sliders for that. The readout and the text for assistive
//! technology are templates, not functions.
//!
//! ```ignore
//! view! {
//!     slider(id: &id, name: "volume", label: "Volume", value: 40.0, show_value: true)
//!     slider(id: &zoom, name: "zoom", min: 25.0, max: 400.0, step: 25.0, value: 100.0,
//!         format: "{value}%", attrs: attributes! { aria-label="Zoom" }
//!     )
//! }
//! ```

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

/// A slider.
///
/// - `id` goes on the `<input>`; the label and the readout point at it.
/// - `value` is the initial value, between `min` (default 0) and `max` (default 100),
///   in steps of `step` (default 1).
/// - `label` is the visible label; `hide_label` keeps it for assistive technology only.
/// - `show_value` shows the current value beside the label, as an `<output>`.
/// - `format` is the readout, with `{value}` replaced, such as `"{value}%"`.
/// - `value_text` is what assistive technology reads for the value, with `{value}`
///   replaced, such as `"{value} percent"`. By default it is the number alone.
/// - `vertical` turns the slider upright, with the maximum at the top.
/// - `attrs` are forwarded to the `<input>`.
#[component]
pub async fn slider(
    cx: &Cx,
    id: &str,
    #[default] name: &str,
    #[default] value: f64,
    #[default] min: f64,
    #[default(100.0)] max: f64,
    #[default(1.0)] step: f64,
    #[default] label: &str,
    #[default] hide_label: bool,
    #[default] show_value: bool,
    #[default("{value}")] format: &str,
    #[default] value_text: &str,
    #[default] size: SliderSize,
    #[default] vertical: bool,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = value.clamp(min, max.max(min));
    let fill = if max > min {
        (value - min) / (max - min)
    } else {
        0.0
    };
    let orientation = if vertical { "vertical" } else { "horizontal" };
    let has_header = !label.is_empty() || show_value;
    let shown = value.to_string();
    Ok(view! {
        <div
            class=(css::ROOT)
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
                        <output for=(id) class=(css::VALUE) data-format=(format)>
                            (format.replace("{value}", &shown))
                        </output>
                    }
                </div>
            }
            <div class=(css::CONTROL) data-orientation=(orientation)>
                <input
                    type="range"
                    id=(id)
                    name=((!name.is_empty()).then_some(name))
                    min=(min.to_string())
                    max=(max.to_string())
                    step=(step.to_string())
                    value=(&shown)
                    disabled=(disabled)
                    aria-orientation=(vertical.then_some("vertical"))
                    aria-valuetext=((!value_text.is_empty()).then(
                        || value_text.replace("{value}", &shown),
                    ))
                    data-value-text=((!value_text.is_empty()).then_some(value_text))
                    style=(format!("--slider-fill: {fill}"))
                    class=(class!(css::INPUT, attrs.remove("class")))
                    (attrs)
                >
            </div>
        </div>
    })
}
