//! One control boundary around an input or textarea and its addons: icons, text such
//! as "https://" or "USD", keyboard hints and buttons.
//!
//! The group shows the focus ring and the error border for the control inside it.
//! Label the control as usual, with a [`field_label`](super::field::field_label) that
//! points at its ID. Addons are decorative unless they contain a button; the library
//! script makes a click on the rest of an addon focus the control.
//!
//! With an [`input_group_textarea`], use the block alignments for toolbars above or
//! below it.
//!
//! ```ignore
//! view! {
//!     input_group(
//!         input_group_addon(input_group_text("https://"))
//!         input_group_input(attrs: attributes! { id=(&id) name="site" placeholder="example.com" })
//!         input_group_addon(align: InputGroupAlign::End,
//!             input_group_button(icon_only: true, attrs: attributes! { aria-label="Copy URL" },
//!                 icon(name: Icon::Copy)
//!             )
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant};
use crate::css::{button as button_css, input as input_css, input_group as css};

/// The size of an [`input_group`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupSize {
    Sm,
    #[default]
    Md,
}

impl InputGroupSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// Where an [`input_group_addon`] sits. Rendered as `data-align`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupAlign {
    /// Before the control.
    #[default]
    Start,
    /// After the control.
    End,
    /// Above a textarea.
    BlockStart,
    /// Below a textarea.
    BlockEnd,
}

impl InputGroupAlign {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::BlockStart => "block-start",
            Self::BlockEnd => "block-end",
        }
    }
}

/// The boundary around a control and its addons.
#[component]
pub async fn input_group(
    cx: &Cx,
    #[default] size: InputGroupSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-input-group=""
            data-size=(size.as_str())
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A slot beside, above or below the control. It may come before or after the
/// control in the markup; `align` places it.
#[component]
pub async fn input_group_addon(
    cx: &Cx,
    #[default] align: InputGroupAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-input-group-addon=""
            data-align=(align.as_str())
            class=(class!(css::ADDON, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Static text in an addon, such as "https://", "USD" or "/ 280".
#[component]
pub async fn input_group_text(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::TEXT, attrs.remove("class"))) (attrs)>(child)</span>
    })
}

/// A compact button sized for an addon. `icon_only` makes it square; give it an
/// `aria-label` in `attrs`.
#[component]
pub async fn input_group_button(
    cx: &Cx,
    #[default(ButtonVariant::Ghost)] variant: ButtonVariant,
    #[default] icon_only: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let typed = attrs.contains_key("type");
    Ok(view! {
        <button
            if !typed {
                type="button"
            }
            class=(class!(button_css::ROOT, css::BUTTON, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(ButtonSize::Sm.as_str())
            data-icon-only=(icon_only.then_some(""))
            (attrs)
        >
            (child)
        </button>
    })
}

/// The group's text input. `attrs` are forwarded to the `<input>`.
#[component]
pub async fn input_group_input(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <input
            data-group-control=""
            class=(class!(input_css::ROOT, css::CONTROL, attrs.remove("class")))
            (attrs)
        >
    })
}

/// The group's textarea. `child` is the initial text.
#[component]
pub async fn input_group_textarea(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <textarea
            data-group-control=""
            class=(class!(
                input_css::ROOT,
                input_css::TEXTAREA,
                css::CONTROL,
                css::TEXTAREA,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </textarea>
    })
}
