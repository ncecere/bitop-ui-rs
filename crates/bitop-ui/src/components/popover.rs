//! A non-modal floating panel anchored to its trigger, ported from bitop-ui's
//! `Popover`, on the Popover API.
//!
//! The browser handles light dismiss, Escape, the top layer and focus return, and
//! anchors the popup to the button that opened it. The popup is a dialog named by its
//! title. The library script moves focus into it when it opens and handles
//! `open_on_hover`.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     popover_trigger(target: &id, "What is a preview deployment?")
//!     popover(
//!         id: &id,
//!         title: "Preview deployments",
//!         description: "Every pull request gets its own URL.",
//!         text_link(href: "/docs/previews", "Read more")
//!     )
//! }
//! ```
//!
//! Use a [`tooltip`](super::tooltip) for short labels and a popover when the content
//! is interactive.
//!
//! Differences from upstream: the trigger is a part ([`popover_trigger`]), or any
//! button with `popovertarget`, connected by the popover's ID. There is no
//! `pointerFocus`: a click moves focus to the first control, as upstream's default.
//! Controlled `open` is the native `showPopover()` and `hidePopover()`, and
//! `onOpenChange` the native `toggle` event.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant};
use crate::css::{button as button_css, popover as css};

/// Which edge of the trigger the popup lines up with. Rendered as `data-align`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopupAlign {
    #[default]
    Start,
    Center,
    End,
}

impl PopupAlign {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

/// The side of its trigger a popup opens on. Rendered as `data-side`. Left and right
/// are physical, as upstream.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopupSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl PopupSide {
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

/// A button that toggles the popover whose ID is `target`.
#[component]
pub async fn popover_trigger(
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
            popovertarget=(target)
            aria-haspopup="dialog"
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}

/// The popup, a non-modal dialog anchored to its trigger.
///
/// - `title` is its heading, which names it; `description` describes it. `child`
///   follows them.
/// - `side` and `align` place it against the trigger; it flips to the other side when
///   there is no room.
/// - `open_on_hover` also opens it when the pointer rests on the trigger, for an
///   "infotip"; it still opens on a click, a tap and from the keyboard. Opened by
///   hover, it closes when the pointer leaves the trigger and the popup, unless it
///   was clicked meanwhile.
/// - `hover_delay` is how long the pointer rests on the trigger first, in
///   milliseconds.
///
/// Opened by a click or from the keyboard, focus moves into the popup, to its first
/// control or the popup itself; opened by hover, it stays where it was.
#[component]
pub async fn popover(
    cx: &Cx,
    id: &str,
    #[default] title: &str,
    #[default] description: &str,
    #[default(PopupSide::Bottom)] side: PopupSide,
    #[default(PopupAlign::Center)] align: PopupAlign,
    #[default] open_on_hover: bool,
    #[default(300)] hover_delay: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let description_id = format!("{id}-description");
    Ok(view! {
        <div
            id=(id)
            popover="auto"
            role="dialog"
            tabindex="-1"
            aria-label=((
                // Named by its title's text, not through aria-labelledby: a crash in
                // Playwright's Firefox (see the contributing guide).
                !title.is_empty()).then_some(title))
            aria-describedby=((!description.is_empty()).then_some(
                description_id.as_str(),
            ))
            class=(class!(css::POPUP, css::PANEL, attrs.remove("class")))
            data-popover=""
            data-side=(side.as_str())
            data-align=(align.as_str())
            data-open-on-hover=(open_on_hover.then_some(""))
            data-hover-delay=(open_on_hover.then(|| hover_delay.to_string()))
            (attrs)
        >
            if !title.is_empty() {
                <h2 class=(css::TITLE)>(title)</h2>
            }
            if !description.is_empty() {
                <p id=(&description_id) class=(css::DESCRIPTION)>(description)</p>
            }
            (child)
        </div>
    })
}
