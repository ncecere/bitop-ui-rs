//! A checkbox and a switch, both on `<input type="checkbox">`.
//!
//! The browser supplies the state, keyboard handling and form submission. Wrap the
//! control and its text in [`field_choice`](super::field::field_choice), or give it an
//! `aria-label`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::checkbox as css;

/// A checkbox. `attrs` are forwarded to the `<input>`; pass `name`, `value` and
/// `checked` there.
#[component]
pub async fn checkbox(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <input
            type="checkbox"
            class=(class!(css::CHECKBOX, attrs.remove("class")))
            (attrs)
        >
    })
}

/// A switch, for a setting that applies as soon as it is toggled.
#[component]
pub async fn switch(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <input
            type="checkbox"
            role="switch"
            class=(class!(css::SWITCH, attrs.remove("class")))
            (attrs)
        >
    })
}
