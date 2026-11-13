//! A short hint shown when its trigger is hovered or focused, ported from bitop-ui's
//! `Tooltip` and `TooltipText`.
//!
//! The tooltip describes the trigger; it does not name it. An icon-only button still
//! needs an `aria-label`. Do not put anything interactive in a tooltip, because it
//! cannot be reached.
//!
//! The library script shows the tooltip after a short delay on hover, at once on
//! keyboard focus, and hides it on Escape, when the trigger is pressed, and when the
//! pointer leaves. Any element can be a trigger: give it `aria-describedby` and
//! `data-tooltip`, both set to the tooltip's ID, which is what [`tooltip_trigger`] and
//! [`tooltip_text`] do. Tooltips do not open on touch, so keep what they say
//! supplementary; use a [`popover`](super::popover) with `open_on_hover` for anything
//! essential.
//!
//! Differences from upstream: the tooltip is a part beside its trigger, connected by
//! its ID, not a wrapper. There is no `TooltipProvider`; each tooltip has its own
//! `delay`.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     tooltip_trigger(target: &id, "Share")
//!     tooltip(id: &id, "Anyone with the link can read this chat")
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    popover::{PopupAlign, PopupSide},
};
use crate::css::{button as button_css, tooltip as css};

/// A button described by the tooltip whose ID is `target`.
#[component]
pub async fn tooltip_trigger(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Secondary)] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            aria-describedby=(target)
            data-tooltip=(target)
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}

/// The tooltip popup. Keep `child` to a few words of plain text.
///
/// - `side` and `align` place it against the trigger; it flips to the other side when
///   there is no room.
/// - `shortcut` is a keyboard shortcut shown after the text, such as "⌘C".
/// - `delay` is how long the pointer rests on the trigger before it opens, in
///   milliseconds. Keyboard focus opens it at once.
#[component]
pub async fn tooltip(
    cx: &Cx,
    id: &str,
    #[default(PopupSide::Top)] side: PopupSide,
    #[default(PopupAlign::Center)] align: PopupAlign,
    #[default] shortcut: &str,
    #[default(600)] delay: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            role="tooltip"
            popover="manual"
            class=(class!(css::POPUP, attrs.remove("class")))
            data-side=(side.as_str())
            data-align=(align.as_str())
            data-delay=(delay.to_string())
            (attrs)
        >
            (child)
            if !shortcut.is_empty() {
                <span class=(css::SHORTCUT)>(shortcut)</span>
            }
        </div>
    })
}

/// Plain text that has a tooltip, such as a score in a table cell, without making it
/// a button: a dotted underline and a tab stop, so that the keyboard can open the
/// tooltip too. The tooltip whose ID is `target` describes the text.
///
/// ```ignore
/// view! {
///     tooltip_text(target: &id, "82%")
///     tooltip(id: &id, "Recall@5: the share of questions whose document was in the top 5.")
/// }
/// ```
#[component]
pub async fn tooltip_text(
    cx: &Cx,
    target: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            tabindex="0"
            aria-describedby=(target)
            data-tooltip=(target)
            class=(class!(css::TEXT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}
