use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::button as css;

/// The visual style of a [`button`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Accent-filled, for the main action.
    #[default]
    Solid,
    /// Bordered, on the surface behind it.
    Outline,
    /// No fill until hovered, for toolbars.
    Ghost,
    /// Danger-filled, for actions such as deleting data.
    Danger,
}

impl ButtonVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Danger => "danger",
        }
    }
}

/// The size of a [`button`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    /// Square, for a single icon. Give the button an `aria-label`.
    Icon,
}

impl ButtonSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Icon => "icon",
        }
    }
}

/// The class for an element styled as a button. Pair it with `data-variant` and
/// `data-size` to style a link as a button.
pub const BUTTON: &str = css::ROOT;

/// A button.
///
/// `attrs` are forwarded to the `<button>`. The type defaults to `button`, so a button
/// inside a form does not submit it unless `attrs` sets `type="submit"`.
///
/// ```ignore
/// view! {
///     button(variant: ButtonVariant::Danger, attrs: attributes! { type="submit" }, "Delete")
/// }
/// ```
#[component]
pub async fn button(
    cx: &Cx,
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let typed = attrs.contains_key("type");
    Ok(view! {
        <button
            if !typed {
                type="button"
            }
            class=(class!(css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}
