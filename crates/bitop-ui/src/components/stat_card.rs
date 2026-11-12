//! A metric tile: a label, a big number, and optionally a change, an icon, a hint,
//! a small chart and details.
//!
//! With `href` the label becomes a link whose hit area is stretched over the whole
//! card: one tab stop, named by the label, and the `<dl>` is not nested in an `<a>`.
//!
//! ```ignore
//! view! {
//!     stat_card(
//!         label: "Requests",
//!         value: "12,480",
//!         delta: StatDelta::new("+12%", StatTrend::Up).label("vs last week"),
//!         icon: Icon::Inbox,
//!         href: "/requests",
//!     )
//! }
//! ```
//!
//! The direction of a change is shown by an arrow and spelled out for assistive
//! technology ("Increased by"), not by colour alone.
//!
//! Differences from upstream: the label, the value and the hint are text, not nodes;
//! `chart` and `details` are slots, always rendered and hidden while empty.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::stat_card as css;

/// Which way a value moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatTrend {
    Up,
    Down,
    Flat,
}

impl StatTrend {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Flat => "flat",
        }
    }

    fn icon(self) -> Icon {
        match self {
            Self::Up => Icon::ArrowUpRight,
            Self::Down => Icon::ArrowDownRight,
            Self::Flat => Icon::ArrowRight,
        }
    }
}

/// Whether a change is good. Rendered as `data-sentiment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatSentiment {
    Positive,
    Negative,
    Neutral,
}

impl StatSentiment {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Neutral => "neutral",
        }
    }
}

/// The change indicator of a [`stat_card`]: `StatDelta::new("+12%", StatTrend::Up)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatDelta<'a> {
    /// The change, such as "+12%".
    pub value: &'a str,
    /// Which way the value moved; picks the arrow.
    pub trend: StatTrend,
    /// Whether the change is good. `None` means up is positive and down negative.
    pub sentiment: Option<StatSentiment>,
    /// Context, such as "vs last week".
    pub label: &'a str,
}

impl<'a> StatDelta<'a> {
    /// A change with the default sentiment and no label.
    #[must_use]
    pub fn new(value: &'a str, trend: StatTrend) -> Self {
        Self {
            value,
            trend,
            sentiment: None,
            label: "",
        }
    }

    /// Says whether the change is good, for a metric where up is bad, such as errors.
    #[must_use]
    pub fn sentiment(mut self, sentiment: StatSentiment) -> Self {
        self.sentiment = Some(sentiment);
        self
    }

    /// Adds context, such as "vs last week".
    #[must_use]
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    /// The sentiment given, or the one the trend implies.
    #[must_use]
    pub fn resolved_sentiment(&self) -> StatSentiment {
        self.sentiment.unwrap_or(match self.trend {
            StatTrend::Up => StatSentiment::Positive,
            StatTrend::Down => StatSentiment::Negative,
            StatTrend::Flat => StatSentiment::Neutral,
        })
    }
}

/// A metric tile, with `<dl>` semantics.
///
/// - `label` names the metric and `value` is the headline number.
/// - `delta` is the change indicator. `increased_label`, `decreased_label` and
///   `unchanged_label` are what is announced before it.
/// - `icon` is a decorative icon in a tinted square.
/// - `hint` is a muted footnote beside the change.
/// - `chart` is a slot under the value, for a small chart such as a sparkline.
/// - `details` is a slot for block content, such as a breakdown.
/// - `href` makes the card a link.
#[component]
pub async fn stat_card(
    cx: &Cx,
    label: &str,
    value: &str,
    #[default]
    #[into]
    delta: Option<StatDelta<'_>>,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default] hint: &str,
    #[default]
    #[into]
    chart: Child<'_>,
    #[default]
    #[into]
    details: Child<'_>,
    #[default] href: &str,
    #[default("Increased by")] increased_label: &str,
    #[default("Decreased by")] decreased_label: &str,
    #[default("Unchanged:")] unchanged_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let name = icon;
    let linked = !href.is_empty();
    let delta_label = delta.map_or("", |delta| delta.label);
    let has_meta = delta.is_some() || !hint.is_empty();
    let trend_word = delta.map(|delta| match delta.trend {
        StatTrend::Up => increased_label,
        StatTrend::Down => decreased_label,
        StatTrend::Flat => unchanged_label,
    });
    Ok(view! {
        <div
            class=(class!(css::CARD, attrs.remove("class")))
            data-stat-card=""
            data-linked=(linked.then_some(""))
            (attrs)
        >
            if let Some(name) = name {
                <span aria-hidden="true" class=(css::ICON)>icon(name: name)</span>
            }
            <dl class=(css::LIST)>
                <dt class=(css::LABEL)>
                    if linked {
                        <a href=(href) class=(css::LINK)>(label)</a>
                    } else {
                        (label)
                    }
                </dt>
                <dd class=(css::VALUE)>(value)</dd>
                if has_meta {
                    <dd class=(css::META)>
                        if let (Some(delta), Some(word)) = (delta, trend_word) {
                            <span
                                class=(css::DELTA)
                                data-sentiment=(delta.resolved_sentiment().as_str())
                                data-trend=(delta.trend.as_str())
                            >
                                icon(name: delta.trend.icon())
                                <span class="sr-only">
                                    (word)
                                    " "
                                </span>
                                (delta.value)
                            </span>
                        }
                        if !delta_label.is_empty() {
                            <span class=(css::HINT)>(delta_label)</span>
                        }
                        if !hint.is_empty() {
                            <span class=(css::HINT)>(hint)</span>
                        }
                    </dd>
                }
                <dd class=(css::CHART)>(chart)</dd>
                <dd class=(css::DETAILS)>(details)</dd>
            </dl>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::{StatDelta, StatSentiment, StatTrend};

    #[test]
    fn sentiment_follows_the_trend_unless_given() {
        let sentiment = |trend| StatDelta::new("1", trend).resolved_sentiment();
        assert_eq!(sentiment(StatTrend::Up), StatSentiment::Positive);
        assert_eq!(sentiment(StatTrend::Down), StatSentiment::Negative);
        assert_eq!(sentiment(StatTrend::Flat), StatSentiment::Neutral);
        let errors = StatDelta::new("+4", StatTrend::Up).sentiment(StatSentiment::Negative);
        assert_eq!(errors.resolved_sentiment(), StatSentiment::Negative);
    }
}
