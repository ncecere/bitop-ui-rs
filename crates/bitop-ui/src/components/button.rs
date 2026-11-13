//! Button, ported from bitop-ui's `Button` and `IconButton`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::button as css;

/// The visual style of a [`button`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Filled with the primary colour, for the main action of a view.
    #[default]
    Primary,
    /// A raised control on the surface, for every other action.
    Secondary,
    /// No fill until hovered, for toolbars and quiet actions.
    Ghost,
    /// Filled with the danger colour, for actions such as deleting data.
    Danger,
    /// Underlined text, for an action that reads as a link.
    Link,
}

impl ButtonVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Danger => "danger",
            Self::Link => "link",
        }
    }
}

/// The size of a [`button`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    /// 32px tall.
    Sm,
    /// 36px tall.
    #[default]
    Md,
}

impl ButtonSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// The class for an element styled as a button. Pair it with `data-variant` and
/// `data-size`, and `data-icon-only` for a square one, to style a link as a button. A
/// link that cannot be followed takes `aria-disabled="true"`.
pub const BUTTON: &str = css::ROOT;

/// A button.
///
/// `attrs` are forwarded to the `<button>`. The type defaults to `button`, so a button
/// inside a form does not submit it unless `attrs` sets `type="submit"`.
///
/// - `loading` shows a spinner, sets `aria-busy` and disables the button. An icon-only
///   button shows the spinner in place of its icon. In the browser,
///   `bitopUi.button.setLoading(button, true)` does the same.
/// - `block` stretches it to the width of its container.
/// - `icon_only` makes it square. It has no visible text, so give it an `aria-label`,
///   or use [`icon_button`], which requires one.
///
/// ```ignore
/// view! {
///     button(variant: ButtonVariant::Danger, attrs: attributes! { type="submit" }, "Delete")
///     button(loading: true, "Saving")
/// }
/// ```
#[component]
pub async fn button(
    cx: &Cx,
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] loading: bool,
    #[default] block: bool,
    #[default] icon_only: bool,
    #[default] disabled: bool,
    /// The accessible name, as `aria-label`, for a button without visible text.
    #[default]
    label: &str,
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
            data-icon-only=(icon_only.then_some(""))
            data-loading=(loading.then_some(""))
            data-block=(block.then_some(""))
            aria-busy=(loading.then_some("true"))
            disabled=(disabled || loading)
            aria-label=((!label.is_empty()).then_some(label))
            (attrs)
        >
            (child)
        </button>
    })
}

/// A square button that shows only an icon. `label` is its accessible name, which
/// it needs because it has no visible text. The variant defaults to ghost.
///
/// ```ignore
/// view! { icon_button(label: "Copy", icon(name: Icon::Copy)) }
/// ```
#[component]
pub async fn icon_button(
    cx: &Cx,
    label: &str,
    #[default(ButtonVariant::Ghost)] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] loading: bool,
    #[default] disabled: bool,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        button(
            variant: variant,
            size: size,
            loading: loading,
            disabled: disabled,
            icon_only: true,
            label: label,
            attrs: attrs,
            (child)
        )
    })
}
