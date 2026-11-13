//! Form fields, ported from bitop-ui's `field`: a control with its label, help text
//! and error ([`field`] and its parts), related fields under a legend ([`fieldset`]),
//! and the layout of a form ([`form`], [`form_actions`]).
//!
//! The parts are connected by IDs, which the caller supplies so that the control can
//! be any element: point the label's `target` at the control's `id`, and the
//! control's `aria-describedby` at the description's and the error's IDs. With an
//! error, set `aria-invalid="true"` on the control too.
//!
//! ```ignore
//! let id = element_id(cx);
//! let help = element_id(cx);
//! let error = element_id(cx);
//! view! {
//!     form(attrs: attributes! { method="post" },
//!         fieldset(legend: "Webhook", description: "We POST a JSON payload.",
//!             field(
//!                 field_label(target: &id, hint: "Optional", "Endpoint")
//!                 input(attrs: attributes! { id=(&id) type="url" aria-invalid="true"
//!                     aria-describedby=(format!("{help} {error}")) })
//!                 field_description(id: &help, "Must use HTTPS.")
//!                 field_error(id: &error, "Enter a URL that starts with https://.")
//!             )
//!         )
//!         form_actions(button(attrs: attributes! { type="submit" }, "Save"))
//!     )
//! }
//! ```
//!
//! Differences from upstream: the field does not wire its parts itself, and there is
//! no Base UI validation: a form shows the browser's own messages for `required`,
//! `type` and `pattern`, and a server-side error is a [`field_error`] the page renders.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::field as css;

/// The wrapper that stacks a field's parts. A disabled control in it dims the label.
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
///
/// - `hint` is short muted text at the end of the row, such as "Optional".
/// - `hide` keeps the label for assistive technology only.
/// - `attrs` are forwarded to the `<label>`.
#[component]
pub async fn field_label(
    cx: &Cx,
    target: &str,
    #[default] hint: &str,
    #[default] hide: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::LABEL_ROW, "sr-only" if hide))>
            <label
                for=(target)
                class=(class!(css::LABEL, attrs.remove("class")))
                (attrs)
            >
                (child)
            </label>
            if !hint.is_empty() {
                <span class=(css::LABEL_HINT)>(hint)</span>
            }
        </div>
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

/// A validation message, after an alert icon. Point the control's `aria-describedby`
/// at `id` and set `aria-invalid="true"` on the control.
#[component]
pub async fn field_error(
    cx: &Cx,
    id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p id=(id) class=(class!(css::ERROR, attrs.remove("class"))) (attrs)>
            icon(
                name: Icon::CircleAlert,
                attrs: attributes! { class=(css::ERROR_ICON) }
            )
            <span>(child)</span>
        </p>
    })
}

/// Related fields under a legend, which names the group.
///
/// - `description` is help text under the legend.
/// - `hide_legend` keeps the legend for assistive technology only.
/// - `attrs` are forwarded to the `<fieldset>`; `disabled` there disables every
///   control in it.
#[component]
pub async fn fieldset(
    cx: &Cx,
    legend: &str,
    #[default] description: &str,
    #[default] hide_legend: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <fieldset class=(class!(css::FIELDSET, attrs.remove("class"))) (attrs)>
            <legend class=(class!(css::LEGEND, "sr-only" if hide_legend))>
                (legend)
            </legend>
            if !description.is_empty() {
                <p class=(css::DESCRIPTION)>(description)</p>
            }
            <div class=(css::FIELDSET_BODY)>(child)</div>
        </fieldset>
    })
}

/// The space between the fields of a [`form`]. Rendered as `data-gap`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormGap {
    Sm,
    #[default]
    Md,
    Lg,
}

impl FormGap {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// A `<form>` that stacks its fields. `attrs` are forwarded to it, for `action` and
/// `method`.
#[component]
pub async fn form(
    cx: &Cx,
    #[default] gap: FormGap,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <form
            class=(class!(css::FORM, attrs.remove("class")))
            data-gap=(gap.as_str())
            (attrs)
        >
            (child)
        </form>
    })
}

/// How [`form_actions`] lines up its buttons. Rendered as `data-align`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormActionsAlign {
    Start,
    #[default]
    End,
    /// Spread from the start to the end.
    Between,
}

impl FormActionsAlign {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Between => "between",
        }
    }
}

/// The row of a form's buttons, at the end by default.
#[component]
pub async fn form_actions(
    cx: &Cx,
    #[default] align: FormActionsAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::ACTIONS, attrs.remove("class")))
            data-align=(align.as_str())
            (attrs)
        >
            (child)
        </div>
    })
}
