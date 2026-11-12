//! A form field: a control with its label, help text and error.
//!
//! The parts are connected by IDs, which the caller supplies so that the control can be
//! any element.
//!
//! ```ignore
//! let id = element_id(cx);
//! let help = element_id(cx);
//! view! {
//!     field(
//!         field_label(target: &id, "Email")
//!         input(attrs: attributes! { id=(&id) type="email" aria-describedby=(&help) })
//!         field_description(id: &help, "Used to sign in.")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::field as css;

/// The wrapper that stacks a field's parts.
#[component]
pub async fn field(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ROOT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The label for the control whose ID is `target`.
#[component]
pub async fn field_label(
    cx: &Cx,
    target: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label for=(target) class=(class!(css::LABEL, attrs.remove("class"))) (attrs)>
            (child)
        </label>
    })
}

/// Help text. Point the control's `aria-describedby` at `id`.
#[component]
pub async fn field_description(
    cx: &Cx,
    id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p id=(id) class=(class!(css::DESCRIPTION, attrs.remove("class"))) (attrs)>
            (child)
        </p>
    })
}

/// A validation message. Point the control's `aria-describedby` at `id` and set
/// `aria-invalid="true"` on the control.
#[component]
pub async fn field_error(
    cx: &Cx,
    id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p id=(id) class=(class!(css::ERROR, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// A label that wraps a checkbox or switch together with its text, so that clicking
/// the text toggles the control.
#[component]
pub async fn field_choice(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label class=(class!(css::CHOICE, attrs.remove("class"))) (attrs)>
            (child)
        </label>
    })
}
