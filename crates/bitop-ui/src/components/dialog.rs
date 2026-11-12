//! A modal dialog on the native `<dialog>` element.
//!
//! The browser supplies the focus trap, Escape, the backdrop and focus return. The
//! trigger and close buttons use invoker commands (`commandfor`), so no script runs.
//!
//! ```ignore
//! let id = element_id(cx);
//! let title = element_id(cx);
//! view! {
//!     dialog_trigger(target: &id, "Delete workspace")
//!     dialog(id: &id, labelled_by: &title,
//!         dialog_title(id: &title, "Delete workspace?")
//!         dialog_description("This cannot be undone.")
//!         dialog_actions(
//!             dialog_close(target: &id, "Cancel")
//!             button(variant: ButtonVariant::Danger, "Delete")
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
use crate::css::{button as button_css, dialog as css};

/// A button that opens the dialog whose ID is `target`.
#[component]
pub async fn dialog_trigger(
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
            commandfor=(target)
            command="show-modal"
            aria-haspopup="dialog"
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}

/// The dialog popup.
///
/// `labelled_by` is the ID of its [`dialog_title`]. Clicking the backdrop closes the
/// dialog unless `modal_only` is set, which suits confirmations that need an answer.
#[component]
pub async fn dialog(
    cx: &Cx,
    id: &str,
    labelled_by: &str,
    #[default] modal_only: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <dialog
            id=(id)
            aria-labelledby=(labelled_by)
            closedby=(if modal_only { "closerequest" } else { "any" })
            class=(class!(css::POPUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </dialog>
    })
}

/// A dialog that interrupts with a question that must be answered, such as confirming
/// a deletion.
///
/// It has the `alertdialog` role and closes only through its buttons or Escape, never
/// by a click on the backdrop. `described_by` is the ID of its [`dialog_description`].
/// Put the least destructive button first so that it receives focus.
#[component]
pub async fn alert_dialog(
    cx: &Cx,
    id: &str,
    labelled_by: &str,
    described_by: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <dialog
            id=(id)
            role="alertdialog"
            aria-labelledby=(labelled_by)
            aria-describedby=(described_by)
            closedby="closerequest"
            class=(class!(css::POPUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </dialog>
    })
}

/// The dialog's heading, an `<h2>`.
#[component]
pub async fn dialog_title(
    cx: &Cx,
    id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2 id=(id) class=(class!(css::TITLE, attrs.remove("class"))) (attrs)>
            (child)
        </h2>
    })
}

/// Supporting text under the title.
#[component]
pub async fn dialog_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::DESCRIPTION, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// The row of buttons at the end of the dialog.
#[component]
pub async fn dialog_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A button that closes the dialog whose ID is `target`.
#[component]
pub async fn dialog_close(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Ghost)] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            commandfor=(target)
            command="close"
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </button>
    })
}
