//! How much of the model's context window a conversation uses: a ring and a
//! percentage that open a card with the base set's meter, a token breakdown and the
//! cost.
//!
//! The trigger's name carries the numbers ("Context window: 21% used, 42,000 of
//! 200,000 tokens"), and the level is never shown by colour alone. The card is the
//! base set's popover: the trigger opens it on click, Enter and Space, and the library
//! script also opens it on hover after a short delay.
//!
//! ```ignore
//! view! {
//!     context(id: "context", used_tokens: 42_000, max_tokens: 200_000,
//!         usage: ContextUsage { input: Some(30_000), output: Some(9_000),
//!             ..ContextUsage::default() },
//!         cost: ContextCost { total: Some(0.084), ..ContextCost::default() },
//!         model_name: "Claude Sonnet")
//! }
//! ```
//!
//! Differences from upstream: numbers are grouped and shortened the English way
//! ([`format_number`]) rather than by `Intl` and a `locale`, and the cost is written
//! through `money_format` ("${amount}") rather than a `currency` code. The meter is
//! the base set's, whose value text also says "near limit" from 75% and "almost full"
//! from 90%.
//!
//! [`format_number`]: bitop_ui::format::format_number

use bitop_ui::{
    components::{
        meter::{MeterSize, meter},
        popover::PopupAlign,
    },
    css::popover as popover_css,
    format::format_number,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::css::context as css;

/// Tokens by kind, for a [`context`]'s breakdown. A kind left out is not shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ContextUsage {
    /// Tokens sent to the model.
    pub input: Option<u64>,
    /// Tokens the model wrote.
    pub output: Option<u64>,
    /// Tokens the model spent thinking.
    pub reasoning: Option<u64>,
    /// Input tokens read from the cache.
    pub cached: Option<u64>,
}

/// Cost by kind and in total, in the currency of the context's `money_format`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ContextCost {
    /// What the input cost.
    pub input: Option<f64>,
    /// What the output cost.
    pub output: Option<f64>,
    /// What the reasoning cost.
    pub reasoning: Option<f64>,
    /// What the cached input cost.
    pub cached: Option<f64>,
    /// The whole cost, shown on its own line.
    pub total: Option<f64>,
}

/// How full the window is. Rendered as `data-level`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContextLevel {
    /// Under 75%.
    #[default]
    Normal,
    /// From 75%.
    High,
    /// From 90%.
    Critical,
}

impl ContextLevel {
    /// The level for a share of the window, from 0 to 1.
    #[must_use]
    pub fn of(ratio: f64) -> Self {
        if ratio >= 0.9 {
            Self::Critical
        } else if ratio >= 0.75 {
            Self::High
        } else {
            Self::Normal
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

/// A token count in short form: `42K`, `1.3M`, through [`format_number`], which rounds
/// the binary value (so 1,250,000 is `1.2M`, where `Intl` says `1.3M`).
///
/// ```
/// use bitop_ui_ai::components::context::compact_tokens;
///
/// assert_eq!(compact_tokens(42_000), "42K");
/// assert_eq!(compact_tokens(1_260_000), "1.3M");
/// ```
#[must_use]
pub fn compact_tokens(n: u64) -> String {
    #[allow(clippy::cast_precision_loss)]
    format_number(n as f64, 1, true)
}

/// An amount of money with up to four decimals and at least two, written into
/// `format` at `{amount}`.
///
/// ```
/// use bitop_ui_ai::components::context::format_money;
///
/// assert_eq!(format_money(0.084, "${amount}"), "$0.084");
/// assert_eq!(format_money(1.5, "${amount}"), "$1.50");
/// assert_eq!(format_money(1234.5, "{amount} €"), "1,234.50 €");
/// ```
#[must_use]
pub fn format_money(amount: f64, format: &str) -> String {
    let mut text = format_number(amount, 4, false);
    match text.split_once('.') {
        None => text.push_str(".00"),
        Some((_, fraction)) if fraction.len() == 1 => text.push('0'),
        Some(_) => {}
    }
    format.replace("{amount}", &text)
}

/// The context indicator: a trigger and its card.
///
/// - `id` goes on the card; the trigger refers to it.
/// - `used_tokens` and `max_tokens` fill the ring and the meter.
/// - `usage` and `cost` fill the breakdown; `model_name` is shown under the meter.
/// - `label` names the card and the meter; `trigger_label` names the trigger, with
///   `{label}`, `{percent}`, `{used}` and `{max}`.
/// - `input_label`, `output_label`, `reasoning_label`, `cached_label`, `tokens_label`
///   (with `{n}`) and `total_label` are the breakdown's strings; `money_format` writes
///   an amount, with `{amount}`.
/// - `high_label`, `critical_label` and `over_label` are added to the meter's value
///   text from 75%, from 90% and above the limit.
/// - `attrs` are forwarded to the trigger.
#[component]
pub async fn context(
    cx: &Cx,
    id: &str,
    used_tokens: u64,
    max_tokens: u64,
    #[default] usage: ContextUsage,
    #[default] cost: ContextCost,
    #[default] model_name: &str,
    #[default("Context window")] label: &str,
    #[default("{label}: {percent}% used, {used} of {max} tokens")] trigger_label: &str,
    #[default("Input")] input_label: &str,
    #[default("Output")] output_label: &str,
    #[default("Reasoning")] reasoning_label: &str,
    #[default("Cached")] cached_label: &str,
    #[default("{n} tokens")] tokens_label: &str,
    #[default("Total cost")] total_label: &str,
    #[default("${amount}")] money_format: &str,
    #[default("Near limit")] high_label: &str,
    #[default("Almost full")] critical_label: &str,
    #[default("Over limit")] over_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    #[allow(clippy::cast_precision_loss)]
    let (used, max) = (used_tokens as f64, max_tokens as f64);
    let ratio = if max_tokens > 0 {
        (used / max).clamp(0.0, 1.0)
    } else {
        0.0
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let percent = (ratio * 100.0).round() as u32;
    let level = ContextLevel::of(ratio);
    let name = trigger_label
        .replace("{label}", label)
        .replace("{percent}", &percent.to_string())
        .replace("{used}", &format_number(used, 0, false))
        .replace("{max}", &format_number(max, 0, false));
    let value_text = format!(
        "{percent}% · {} / {}",
        compact_tokens(used_tokens),
        compact_tokens(max_tokens)
    );
    let rows: Vec<(String, String, String)> = [
        (input_label, usage.input, cost.input),
        (output_label, usage.output, cost.output),
        (reasoning_label, usage.reasoning, cost.reasoning),
        (cached_label, usage.cached, cost.cached),
    ]
    .into_iter()
    .filter(|(_, tokens, money)| tokens.is_some() || money.is_some())
    .map(|(text, tokens, money)| {
        (
            text.to_owned(),
            #[allow(clippy::cast_precision_loss)]
            tokens.map_or_else(String::new, |n| {
                tokens_label.replace("{n}", &format_number(n as f64, 0, false))
            }),
            money.map_or_else(String::new, |m| format_money(m, money_format)),
        )
    })
    .collect();
    let total = cost.total.map(|m| format_money(m, money_format));
    let ring_style = format!("--ratio: {ratio:.4}");
    Ok(view! {
        <button
            type="button"
            popovertarget=(id)
            aria-label=(&name)
            aria-expanded="false"
            data-context-trigger=""
            data-level=(level.as_str())
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            <svg
                aria-hidden="true"
                viewBox="0 0 20 20"
                class=(css::RING)
                style=(ring_style)
            >
                <circle cx="10" cy="10" r="8" class=(css::RING_TRACK)></circle>
                <circle cx="10" cy="10" r="8" pathLength="100" class=(css::RING_VALUE)></circle>
            </svg>
            <span aria-hidden="true">
                (percent)
                "%"
            </span>
        </button>
        <div
            id=(id)
            popover="auto"
            role="dialog"
            aria-label=(label)
            data-context-card=""
            data-align=(PopupAlign::Start.as_str())
            class=(class!(popover_css::POPUP, css::CARD))
        >
            meter(
                label: label,
                value: used,
                max: max,
                warning_at: 0.75,
                critical_at: 0.9,
                value_text: &value_text,
                show_status: false,
                size: MeterSize::Sm,
                warning_label: high_label,
                critical_label: critical_label,
                over_label: over_label,
                attrs: attributes! { data-context-meter="" }
            )
            if !model_name.is_empty() {
                <p class=(css::MODEL)>(model_name)</p>
            }
            if !rows.is_empty() {
                <dl class=(css::ROWS)>
                    for (text, tokens, money) in rows {
                        <div class=(css::ROW)>
                            <dt>(text)</dt>
                            <dd>
                                if !tokens.is_empty() {
                                    <span>(tokens)</span>
                                }
                                if !money.is_empty() {
                                    <span class=(css::COST)>(money)</span>
                                }
                            </dd>
                        </div>
                    }
                </dl>
            }
            if let Some(total) = total {
                <div class=(class!(css::ROW, css::TOTAL))>
                    <span>(total_label)</span>
                    <span>(total)</span>
                </div>
            }
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::{ContextLevel, compact_tokens, format_money};

    #[test]
    fn levels() {
        assert_eq!(ContextLevel::of(0.74), ContextLevel::Normal);
        assert_eq!(ContextLevel::of(0.75), ContextLevel::High);
        assert_eq!(ContextLevel::of(0.9), ContextLevel::Critical);
    }

    #[test]
    fn numbers() {
        assert_eq!(compact_tokens(950), "950");
        assert_eq!(compact_tokens(200_000), "200K");
        assert_eq!(format_money(0.0, "${amount}"), "$0.00");
        assert_eq!(format_money(0.000_12, "${amount}"), "$0.0001");
        assert_eq!(format_money(12.0, "${amount}"), "$12.00");
    }
}
