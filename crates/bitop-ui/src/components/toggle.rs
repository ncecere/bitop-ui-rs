//! A two-state button: `<button aria-pressed>`.
//!
//! The library script flips `aria-pressed` on click, which Space and Enter also
//! trigger. The pressed state uses the primary tint plus an inset ring, so it is not
//! signalled by fill colour alone.
//!
//! With a `name`, a pressed toggle submits `name=value` with its form, through a
//! hidden input inside the button; a toggle that is not pressed submits nothing, like
//! a checkbox.
//!
//! ```ignore
//! view! {
//!     toggle(icon_only: true, attrs: attributes! { aria-label="Bold" }, icon(name: Icon::Check))
//!     toggle(variant: ToggleVariant::Outline, pressed: true, name: "preview", "Preview")
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::toggle as css;

/// The visual style of a [`toggle`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    /// Transparent until hovered or pressed.
    #[default]
    Ghost,
    /// Has a control boundary.
    Outline,
}

impl ToggleVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ghost => "ghost",
            Self::Outline => "outline",
        }
    }
}

/// The size of a [`toggle`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleSize {
    Sm,
    #[default]
    Md,
}

impl ToggleSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// The class for an element styled as a toggle.
pub const TOGGLE: &str = css::TOGGLE;

/// A toggle button.
///
/// - `pressed` is the initial state.
/// - `icon_only` makes it square, for a single icon; give it an `aria-label` in `attrs`.
/// - `name` and `value` make a pressed toggle submit with its form.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn toggle(
    cx: &Cx,
    #[default] variant: ToggleVariant,
    #[default] size: ToggleSize,
    #[default] pressed: bool,
    #[default] icon_only: bool,
    #[default] disabled: bool,
    #[default] name: &str,
    #[default("on")] value: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            aria-pressed=(if pressed { "true" } else { "false" })
            disabled=(disabled)
            data-toggle=""
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            data-icon-only=(icon_only)
            class=(class!(css::TOGGLE, attrs.remove("class")))
            (attrs)
        >
            (child)
            if !name.is_empty() {
                <input type="hidden" name=(name) value=(value) disabled=(!pressed)>
            }
        </button>
    })
}
