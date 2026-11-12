//! A hex colour with a native colour picker, optional preset swatches and a WCAG
//! contrast check against the text colour that will be drawn on it.
//!
//! The hex input is the field's control: `id` goes on it, so a
//! [`field_label`](super::field::field_label) can point at it, and its value is
//! submitted with the form under `name` as `#rrggbb`, or `""` for the default colour.
//!
//! The component has no colours of its own: `default_color`, used while the value is
//! empty, and `contrast_with` are the caller's. The contrast line describes the hex
//! input (`aria-describedby`) and says pass or fail in words with the ratio, never by
//! colour alone. Ratios are rounded down, so 4.46 never reads as a passing "4.5".
//!
//! The server renders the first state; the library script then keeps the swatch, the
//! picker, the presets and the contrast line in step with what is typed or picked.
//!
//! ```ignore
//! view! {
//!     field(
//!         field_label(target: &id, "Accent colour")
//!         color_field(id: &id, name: "accent", value: "#1d4ed8",
//!             default_color: "#1d4ed8", contrast_with: "#ffffff", contrast_label: "white text",
//!             presets: &[ColorPreset { value: "#1d4ed8", label: "Blue" }]
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::color_field as css;

/// A quick pick in a [`color_field`]: a hex colour and its accessible name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPreset<'a> {
    /// The colour, as `#rrggbb`.
    pub value: &'a str,
    /// The name of the colour, such as "Blue".
    pub label: &'a str,
}

/// `#rrggbb` in lower case for `#RGB`, `rrggbb` or `#RRGGBB`; `None` when `value` is
/// not a colour.
#[must_use]
pub fn normalize_hex(value: &str) -> Option<String> {
    let hex = value.trim();
    let hex = hex.strip_prefix('#').unwrap_or(hex).to_ascii_lowercase();
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    match hex.len() {
        6 => Some(format!("#{hex}")),
        3 => Some(
            hex.chars()
                .fold(String::from("#"), |out, c| format!("{out}{c}{c}")),
        ),
        _ => None,
    }
}

/// The relative luminance of a normalized hex colour.
fn luminance(hex: &str) -> f64 {
    let channel = |index: usize| {
        let value = f64::from(u8::from_str_radix(&hex[index..index + 2], 16).unwrap_or(0)) / 255.0;
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5)
}

/// The WCAG 2.1 contrast ratio of two hex colours, from 1 to 21; `None` when either
/// is not a colour.
#[must_use]
pub fn contrast_ratio(a: &str, b: &str) -> Option<f64> {
    let a = luminance(&normalize_hex(a)?);
    let b = luminance(&normalize_hex(b)?);
    Some((a.max(b) + 0.05) / (a.min(b) + 0.05))
}

/// A ratio as text, such as `4.5:1`, rounded down to one decimal so that 4.46 never
/// reads as a passing "4.5".
#[must_use]
pub fn format_ratio(ratio: f64) -> String {
    format!("{:.1}:1", (ratio * 10.0).floor() / 10.0)
}

/// A colour field.
///
/// - `value` is `#rrggbb`, or `""` for the default colour.
/// - `default_color` is the hex colour used while the value is empty.
/// - `contrast_with` is the hex text colour drawn on this colour, and
///   `contrast_label` how to describe it, such as "white text". Leave
///   `contrast_with` empty for no contrast check and no sample.
/// - `min_contrast` is the ratio to meet; 4.5 is WCAG AA for body text.
/// - `presets` are quick picks.
/// - The messages are templates: `{color}` is replaced with the default colour,
///   `{label}` with `contrast_label`, `{ratio}` with the ratio and `{min}` with
///   `min_contrast`.
/// - `attrs` are forwarded to the hex `<input>`.
#[component]
pub async fn color_field(
    cx: &Cx,
    id: &str,
    name: &str,
    #[default] value: &str,
    default_color: &str,
    #[default] contrast_with: &str,
    #[default("the text colour")] contrast_label: &str,
    #[default(4.5)] min_contrast: f64,
    #[default] presets: &[ColorPreset<'_>],
    #[default] disabled: bool,
    #[default("Pick a colour")] picker_label: &str,
    #[default("Preset colours")] presets_label: &str,
    #[default("Aa")] sample: &str,
    #[default("{color} (default)")] placeholder: &str,
    #[default("Enter a hex colour such as {color}.")] invalid_message: &str,
    #[default("Contrast with {label} {ratio}: meets WCAG AA ({min}:1).")] pass_message: &str,
    #[default("Contrast with {label} {ratio}: below {min}:1. Choose a darker colour.")]
    darker_message: &str,
    #[default("Contrast with {label} {ratio}: below {min}:1. Choose a lighter colour.")]
    lighter_message: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = value.trim().to_ascii_lowercase();
    let hex = normalize_hex(&value);
    let fallback = normalize_hex(default_color).unwrap_or_else(|| default_color.to_owned());
    let shown = hex.clone().unwrap_or_else(|| fallback.clone());
    let invalid = !value.is_empty() && hex.is_none();
    let text_color = normalize_hex(contrast_with);
    let ratio = contrast_ratio(&shown, contrast_with);
    let passes = ratio.is_some_and(|ratio| ratio >= min_contrast);
    // Light text needs a darker background to pass, dark text a lighter one.
    let light_text = text_color
        .as_deref()
        .is_none_or(|text| luminance(text) > 0.18);

    let min = min_contrast.to_string();
    let template = |text: &str| {
        text.replace("{color}", &fallback)
            .replace("{label}", contrast_label)
            .replace("{min}", &min)
    };
    let placeholder = template(placeholder);
    let invalid_message = template(invalid_message);
    let pass_message = template(pass_message);
    let fail_message = template(if light_text {
        darker_message
    } else {
        lighter_message
    });
    let message = match ratio {
        _ if invalid => invalid_message.clone(),
        Some(ratio) if passes => pass_message.replace("{ratio}", &format_ratio(ratio)),
        Some(ratio) => fail_message.replace("{ratio}", &format_ratio(ratio)),
        None => String::new(),
    };
    // Wide enough for the placeholder or a full "#rrggbb"; the font is monospace.
    let hex_chars = placeholder.chars().count().max(7);
    let contrast_id = format!("{id}-contrast");
    let presets: Vec<(ColorPreset<'_>, bool)> = presets
        .iter()
        .map(|preset| (*preset, hex.is_some() && normalize_hex(preset.value) == hex))
        .collect();

    Ok(view! {
        <div
            class=(css::ROOT)
            data-color-field=""
            data-default=(&fallback)
            data-contrast-with=(text_color.clone())
            data-min-contrast=(&min)
        >
            <div class=(css::ROW)>
                <span
                    class=(css::SWATCH)
                    data-color-shown=""
                    style=(format!("background-color: {shown}"))
                >
                    <input
                        type="color"
                        aria-label=(picker_label)
                        class=(css::PICKER)
                        value=(&shown)
                        disabled=(disabled)
                    >
                </span>
                <input
                    type="text"
                    id=(id)
                    name=(name)
                    value=(&value)
                    placeholder=(&placeholder)
                    spellcheck="false"
                    autocomplete="off"
                    autocapitalize="off"
                    maxlength="7"
                    aria-describedby=(&contrast_id)
                    aria-invalid=(invalid.then_some("true"))
                    disabled=(disabled)
                    style=(format!("--color-field-hex-chars: {hex_chars}"))
                    class=(class!(css::HEX, attrs.remove("class")))
                    (attrs)
                >
                if let Some(text) = &text_color {
                    <span
                        aria-hidden="true"
                        class=(css::SAMPLE)
                        data-color-shown=""
                        style=(format!("background-color: {shown}; color: {text}"))
                    >
                        (sample)
                    </span>
                }
            </div>
            if !presets.is_empty() {
                <div role="group" aria-label=(presets_label) class=(css::PRESETS)>
                    for (preset, selected) in presets {
                        <button
                            type="button"
                            class=(css::PRESET)
                            style=(format!("background-color: {}", preset.value))
                            aria-label=(preset.label)
                            aria-pressed=(if selected { "true" } else { "false" })
                            title=(preset.label)
                            data-value=(normalize_hex(preset.value))
                            disabled=(disabled)
                        >
                            icon(name: Icon::Check)
                        </button>
                    }
                </div>
            }
            <p
                id=(&contrast_id)
                class=(css::CONTRAST)
                data-pass=(passes && !invalid)
                data-invalid-message=(&invalid_message)
                data-pass-message=(&pass_message)
                data-fail-message=(&fail_message)
            >
                icon(name: Icon::Check)
                icon(name: Icon::CircleAlert)
                <span data-color-message="">(message)</span>
            </p>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::{contrast_ratio, format_ratio, normalize_hex};

    #[test]
    fn normalizes_hex() {
        assert_eq!(normalize_hex("#1D4ED8").as_deref(), Some("#1d4ed8"));
        assert_eq!(normalize_hex(" abc ").as_deref(), Some("#aabbcc"));
        assert_eq!(normalize_hex("1d4ed8").as_deref(), Some("#1d4ed8"));
        assert_eq!(normalize_hex("#12"), None);
        assert_eq!(normalize_hex("#gggggg"), None);
        assert_eq!(normalize_hex(""), None);
    }

    #[test]
    fn contrast_ratios() {
        assert_eq!(
            format_ratio(contrast_ratio("#000", "#fff").unwrap()),
            "21.0:1"
        );
        assert_eq!(
            format_ratio(contrast_ratio("#fff", "#ffffff").unwrap()),
            "1.0:1"
        );
        assert_eq!(contrast_ratio("blue", "#fff"), None);
    }

    #[test]
    fn ratios_round_down() {
        assert_eq!(format_ratio(4.46), "4.4:1");
        assert_eq!(format_ratio(4.5), "4.5:1");
    }
}
