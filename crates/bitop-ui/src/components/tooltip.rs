//! A short hint shown when its trigger is hovered or focused.
//!
//! The tooltip describes the trigger; it does not name it. An icon-only button still
//! needs an `aria-label`. Do not put anything interactive in a tooltip, because it
//! cannot be reached.
//!
//! The library script shows the tooltip after a short delay on hover, at once on
//! keyboard focus, and hides it on Escape. Any element can be a trigger: give it
//! `aria-describedby` and `data-tooltip`, both set to the tooltip's ID, which is what
//! [`tooltip_trigger`] does.
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

use super::button::{ButtonSize, ButtonVariant};
use crate::css::{button as button_css, tooltip as css};

/// A button described by the tooltip whose ID is `target`.
#[component]
pub async fn tooltip_trigger(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Outline)] variant: ButtonVariant,
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
#[component]
pub async fn tooltip(
    cx: &Cx,
    id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            role="tooltip"
            popover="manual"
            class=(class!(css::POPUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
