//! A rich preview of a link's destination (a profile, a repository, a document),
//! shown when the link is hovered or focused.
//!
//! The card is a popover that the library script shows after a short delay on hover,
//! at once on keyboard focus, and hides a moment after the pointer leaves both the
//! link and the card, or on Escape. Any element can be a trigger: give it
//! `data-hover-card`, set to the card's ID, and `aria-expanded="false"`, which is what
//! [`hover_card_trigger`] does.
//!
//! Render the card right after its trigger. Tab then moves from the link into the
//! card, so the card can hold links and buttons; Escape returns focus to the link.
//! The card is a `<span>` and takes phrasing content, so that the pair can sit inside
//! a paragraph.
//! Hovering does not work on touch screens, so everything the card shows should also
//! be at the link's destination.
//!
//! Differences from upstream: upstream's card cannot be reached from the keyboard at
//! all; this one can. `side` is physical for left and right, as upstream; `align` is
//! logical. The gap from the trigger is the CSS variable `--hover-card-gap`, not a
//! prop.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     hover_card_trigger(target: &id, href: "/u/ada", "@ada")
//!     hover_card(id: &id, label: "Ada Lovelace",
//!         <p>"Mathematician and writer."</p>
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::popover::PopupAlign;
use crate::css::{hover_card as css, text_link as text_link_css};

/// The side of the trigger a [`hover_card`] opens on. Rendered as `data-side`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl HoverCardSide {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// A link that shows the hover card whose ID is `target`.
///
/// `delay` and `close_delay` are in milliseconds: how long the pointer rests on the
/// link before the card opens, and how long after it leaves before the card closes.
#[component]
pub async fn hover_card_trigger(
    cx: &Cx,
    target: &str,
    href: &str,
    #[default(600)] delay: u32,
    #[default(300)] close_delay: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            aria-expanded="false"
            aria-controls=(target)
            data-hover-card=(target)
            data-delay=(delay.to_string())
            data-close-delay=(close_delay.to_string())
            data-tone="default"
            class=(class!(text_link_css::LINK, attrs.remove("class")))
            (attrs)
        >
            (child)
        </a>
    })
}

/// The card. `label` names it for assistive technology, such as the name of the
/// person or page it previews.
///
/// It is a `<span>`, so that it can follow a link inside a paragraph. Its children
/// must be phrasing content for the same reason (`<span>`, `<strong>`, `<a>`, `<img>`,
/// `<button>`, not `<div>` or `<p>`); they are laid out as a column.
#[component]
pub async fn hover_card(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] side: HoverCardSide,
    #[default(PopupAlign::Center)] align: PopupAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            id=(id)
            role="group"
            aria-label=(label)
            popover="manual"
            class=(class!(css::CARD, attrs.remove("class")))
            data-hover-card-popup=""
            data-side=(side.as_str())
            data-align=(align.as_str())
            (attrs)
        >
            (child)
        </span>
    })
}
