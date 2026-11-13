//! A non-modal popup anchored to its trigger, on the Popover API.
//!
//! The browser handles light dismiss, Escape, the top layer and focus return, and
//! anchors the popup to the button that opened it.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     popover_trigger(target: &id, "Filters")
//!     popover(id: &id, <p>"Popup content"</p>)
//! }
//! ```

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
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}

/// The popup. It opens below its trigger and flips when there is no room.
#[component]
pub async fn popover(
    cx: &Cx,
    id: &str,
    #[default] align: PopupAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            popover="auto"
            class=(class!(css::POPUP, attrs.remove("class")))
            data-align=(align.as_str())
            (attrs)
        >
            (child)
        </div>
    })
}
