//! Text inputs: `<input>` and `<textarea>` with the library's styling.
//!
//! Mark a failed value with `aria-invalid="true"`; the border turns to the danger colour.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::input as css;

/// The class for an element styled as a text input.
pub const INPUT: &str = css::ROOT;

/// A single-line input. `attrs` are forwarded to the `<input>`.
#[component]
pub async fn input(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! { <input class=(class!(css::ROOT, attrs.remove("class"))) (attrs)> })
}

/// A multi-line input that grows with its content. `child` is the initial text.
#[component]
pub async fn textarea(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <textarea
            class=(class!(css::ROOT, css::TEXTAREA, attrs.remove("class")))
            (attrs)
        >
            (child)
        </textarea>
    })
}
