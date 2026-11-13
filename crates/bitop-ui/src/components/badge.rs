//! Small labels in five tones, ported from bitop-ui's `Badge`, `StatusBadge` and
//! `StatusDot`.
//!
//! A badge is a soft tint with a thin ring in the tone's colour, or, with
//! `variant: BadgeVariant::Outline`, the surface with a hairline. [`status_badge`]
//! leads with a status dot, which pulses for work in progress; [`status_dot`] is the
//! dot on its own. Colour alone does not carry meaning: say the status in the text, or
//! give a lone dot a `label`. The pulse is off under reduced motion.
//!
//! ```ignore
//! view! {
//!     badge(tone: BadgeTone::Info, variant: BadgeVariant::Outline, "Beta")
//!     status_badge(tone: BadgeTone::Info, pulse: true, "Building")
//!     status_dot(tone: BadgeTone::Danger, pulse: true, label: "Gateway down")
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::badge as css;

/// The colour of a [`badge`] or a [`status_dot`]. Rendered as `data-tone`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeTone {
    #[default]
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}

impl BadgeTone {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

/// The fill of a [`badge`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    /// A tinted fill with a ring in the tone's colour.
    #[default]
    Soft,
    /// The surface with a hairline ring, and the tone's text colour.
    Outline,
}

impl BadgeVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Soft => "soft",
            Self::Outline => "outline",
        }
    }
}

/// The height and padding of a [`badge`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    /// 1.125rem high, with 11px text.
    Sm,
    /// 1.375rem high.
    #[default]
    Md,
}

impl BadgeSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A small pill label. `dot` leads it with a status dot, and `pulse` makes that dot
/// pulse (and shows it). Text colours meet 4.5:1 on their fills.
#[component]
pub async fn badge(
    cx: &Cx,
    #[default] tone: BadgeTone,
    #[default] variant: BadgeVariant,
    #[default] size: BadgeSize,
    #[default] dot: bool,
    #[default] pulse: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            class=(class!(css::ROOT, attrs.remove("class")))
            data-tone=(tone.as_str())
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            if dot || pulse {
                <span
                    aria-hidden="true"
                    class=(css::DOT)
                    data-pulse=(pulse.then_some(""))
                ></span>
            }
            (child)
        </span>
    })
}

/// A [`badge`] with a status dot, such as "● Ready", or "● Processing" with `pulse`.
#[component]
pub async fn status_badge(
    cx: &Cx,
    #[default] tone: BadgeTone,
    #[default] variant: BadgeVariant,
    #[default] size: BadgeSize,
    #[default] pulse: bool,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        badge(
            tone: tone,
            variant: variant,
            size: size,
            dot: true,
            pulse: pulse,
            attrs: attrs,
            (child)
        )
    })
}

/// A status dot on its own. Without a `label` it is decorative (hidden from assistive
/// technology), and the status must be said in the text beside it; with one it is an
/// image named by the label.
#[component]
pub async fn status_dot(
    cx: &Cx,
    #[default] tone: BadgeTone,
    #[default] pulse: bool,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let labelled = !label.is_empty();
    Ok(view! {
        <span
            role=(labelled.then_some("img"))
            aria-label=(labelled.then_some(label))
            aria-hidden=((!labelled).then_some("true"))
            class=(class!(css::STANDALONE_DOT, attrs.remove("class")))
            data-tone=(tone.as_str())
            (attrs)
        >
            <span class=(css::DOT) data-pulse=(pulse.then_some(""))></span>
        </span>
    })
}
