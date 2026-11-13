//! Text inputs, ported from bitop-ui's `Input` and `Textarea`: `<input>` and
//! `<textarea>` with the library's styling. The native select is
//! [`native_select`](super::select::native_select).
//!
//! Put them in a [`field`](super::field::field) for a label, help text and an error.
//! Mark a failed value with `aria-invalid="true"`; the border turns to the danger
//! colour.
//!
//! ```ignore
//! view! {
//!     input(start_icon: Icon::Search, attrs: attributes! { type="search" aria-label="Search" })
//!     input(size: InputSize::Sm, attrs: attributes! { id=(&id) placeholder="32px" })
//!     textarea(attrs: attributes! { id=(&notes) name="notes" })
//! }
//! ```
//!
//! Differences from upstream: `onValueChange` is the native `input` event.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::input as css;

/// The class for an element styled as a text input.
pub const INPUT: &str = css::ROOT;

/// The height of an [`input`]: 32 or 36 pixels. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
}

impl InputSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A single-line input. `attrs` are forwarded to the `<input>`.
///
/// `start_icon` is a decorative icon inside the start of the input; the input is
/// then wrapped in a `<span>`, which takes the caller's `class`.
#[component]
pub async fn input(
    cx: &Cx,
    #[default] size: InputSize,
    #[default]
    #[into]
    start_icon: Option<Icon>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let extra = attrs.remove("class");
    Ok(view! {
        if let Some(start_icon) = start_icon {
            <span class=(class!(css::ADORNED, extra))>
                <span class=(css::ADORNMENT) aria-hidden="true">
                    icon(name: start_icon)
                </span>
                <input class=(css::ROOT) data-size=(size.as_str()) (attrs)>
            </span>
        } else {
            <input class=(class!(css::ROOT, extra)) data-size=(size.as_str()) (attrs)>
        }
    })
}

/// A multi-line input, three rows high unless `attrs` set `rows`. `child` is the
/// initial text.
#[component]
pub async fn textarea(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    if !attrs.contains_key("rows") {
        attrs.insert(cx, "rows", "3");
    }
    Ok(view! {
        <textarea
            class=(class!(css::ROOT, css::TEXTAREA, attrs.remove("class")))
            (attrs)
        >
            (child)
        </textarea>
    })
}
