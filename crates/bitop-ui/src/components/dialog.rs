//! Dialog and alert dialog, ported from bitop-ui's `Dialog`, `DialogClose` and
//! `AlertDialog`, on the native `<dialog>` element.
//!
//! The browser supplies the focus trap, Escape, the backdrop and focus return. The
//! trigger and close buttons use invoker commands (`commandfor`), so no script is
//! needed to open or close a dialog. The title names the dialog and the description
//! describes it. The × button comes last, so that focus lands on the first field when
//! the dialog opens, not on ×.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     dialog_trigger(target: &id, variant: ButtonVariant::Primary, "New project")
//!     dialog(
//!         id: &id,
//!         title: "New project",
//!         description: "Projects group deployments, domains and environment variables.",
//!         footer: view! {
//!             dialog_close(target: &id, "Cancel")
//!             button(attrs: attributes! { type="submit" form="new-project" }, "Create")
//!         },
//!         <form id="new-project" method="post">"Fields"</form>
//!     )
//! }
//! ```
//!
//! An alert dialog asks to confirm a consequential action. Its confirm button does
//! what `confirm_attrs` say, such as submitting a form:
//!
//! ```ignore
//! view! {
//!     dialog_trigger(target: &id, variant: ButtonVariant::Danger, "Delete project…")
//!     alert_dialog(
//!         id: &id,
//!         title: "Delete “marketing-site”?",
//!         description: "Its 42 deployments will be removed. This can't be undone.",
//!         confirm_label: "Delete project",
//!         confirm_attrs: attributes! { type="submit" form="delete" },
//!     )
//! }
//! ```
//!
//! In the browser, `bitopUi.dialog.setBusy(dialog, true)` puts the confirm button in
//! its loading state and `bitopUi.dialog.setError(dialog, "message")` shows an error
//! (an empty message hides it), for an action that runs in the page.
//!
//! Differences from upstream: the trigger and the close button are parts
//! ([`dialog_trigger`], [`dialog_close`]) connected by the dialog's ID, not props.
//! `initialFocus` is an `autofocus` attribute on a control in the body, which the
//! browser honours. `finalFocus` is `final_focus`, the ID of the element to focus when
//! the dialog closes; the library script moves focus there. `onConfirm` is
//! `confirm_attrs`, and `error` is a message rather than any thrown value.
//! `modal_only` keeps a click on the backdrop from closing a dialog.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    alert::{AlertTone, alert},
    button::{ButtonSize, ButtonVariant, button, icon_button},
    icon::{Icon, icon},
};
use crate::css::{button as button_css, dialog as css};

/// The width of a [`dialog`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogSize {
    /// 26rem, for short confirmations or single inputs.
    Sm,
    /// 32rem.
    #[default]
    Md,
    /// 42rem, for tables and forms in columns.
    Lg,
    /// 56rem.
    Xl,
}

impl DialogSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

/// The variant of an [`alert_dialog`]'s confirm button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertDialogTone {
    /// For destructive actions.
    #[default]
    Danger,
    Primary,
}

impl AlertDialogTone {
    fn variant(self) -> ButtonVariant {
        match self {
            Self::Danger => ButtonVariant::Danger,
            Self::Primary => ButtonVariant::Primary,
        }
    }
}

/// A button that opens the dialog whose ID is `target`.
#[component]
pub async fn dialog_trigger(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Secondary)] variant: ButtonVariant,
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

/// A modal dialog. `child` is its body.
///
/// - `title` names the dialog and `description` describes it.
/// - `footer` is a slot for the buttons in the tinted band at the end; use
///   [`dialog_close`] for Cancel.
/// - `size` sets the width.
/// - `hide_close` removes the × button; Escape still closes the dialog. `close_label`
///   is its accessible name.
/// - `final_focus` is the ID of the element to focus when the dialog closes, when that
///   is not the trigger, such as a select whose change opened it.
/// - `modal_only` keeps a click on the backdrop from closing it.
/// - `attrs` are forwarded to the `<dialog>`.
#[component]
pub async fn dialog(
    cx: &Cx,
    id: &str,
    title: &str,
    #[default] description: &str,
    #[default] size: DialogSize,
    #[default] hide_close: bool,
    #[default("Close")] close_label: &str,
    #[default] final_focus: &str,
    #[default] modal_only: bool,
    #[default]
    #[into]
    footer: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let title_id = format!("{id}-title");
    let description_id = format!("{id}-description");
    Ok(view! {
        <dialog
            id=(id)
            aria-labelledby=(&title_id)
            aria-describedby=((!description.is_empty()).then_some(
                description_id.as_str(),
            ))
            closedby=(if modal_only { "closerequest" } else { "any" })
            class=(class!(css::POPUP, attrs.remove("class")))
            data-size=(size.as_str())
            data-closable=((!hide_close).then_some(""))
            data-final-focus=((!final_focus.is_empty()).then_some(final_focus))
            (attrs)
        >
            <div class=(css::HEADER)>
                <div class=(css::HEADING)>
                    <h2 id=(&title_id) class=(css::TITLE)>(title)</h2>
                    if !description.is_empty() {
                        <p id=(&description_id) class=(css::DESCRIPTION)>
                            (description)
                        </p>
                    }
                </div>
            </div>
            <div class=(css::BODY)>(child)</div>
            <div class=(css::FOOTER)>(footer)</div>
            // Last, so that focus lands on the first field and not on ×.
            if !hide_close {
                icon_button(
                    label: close_label,
                    size: ButtonSize::Sm,
                    attrs: attributes! { class=(css::CLOSE) commandfor=(id) command="close" },
                    icon(name: Icon::X)
                )
            }
        </dialog>
    })
}

/// A button that closes the dialog whose ID is `target`. Secondary by default.
#[component]
pub async fn dialog_close(
    cx: &Cx,
    target: &str,
    #[default(ButtonVariant::Secondary)] variant: ButtonVariant,
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

/// A dialog that asks to confirm a consequential action, such as a deletion.
///
/// It has the `alertdialog` role and closes on Escape and Cancel, never on a click on
/// the backdrop. Cancel comes first, so it has focus when the dialog opens and Enter
/// never confirms by accident.
///
/// - `confirm_label` is the confirm button's text, a specific verb such as "Delete
///   project". `confirm_attrs` are forwarded to it: give it `type="submit"` and a
///   `form`, or listen for its clicks (it has `data-dialog-confirm`).
/// - `tone` is the confirm button's variant, danger by default.
/// - `busy` shows a spinner on the confirm button and disables it.
/// - `error` is shown as a danger alert above the buttons.
/// - `child` is extra content between the description and the buttons.
/// - `final_focus` is the ID of the element to focus when the dialog closes.
#[component]
pub async fn alert_dialog(
    cx: &Cx,
    id: &str,
    title: &str,
    description: &str,
    confirm_label: &str,
    #[default("Cancel")] cancel_label: &str,
    #[default] tone: AlertDialogTone,
    #[default] busy: bool,
    #[default] error: &str,
    #[default] final_focus: &str,
    #[default] mut confirm_attrs: Attributes,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    confirm_attrs.insert(cx, "data-dialog-confirm", "");
    let title_id = format!("{id}-title");
    let description_id = format!("{id}-description");
    Ok(view! {
        <dialog
            id=(id)
            role="alertdialog"
            aria-label=(title)
            aria-describedby=(&description_id)
            closedby="closerequest"
            class=(class!(css::POPUP, attrs.remove("class")))
            data-size="sm"
            data-final-focus=((!final_focus.is_empty()).then_some(final_focus))
            (attrs)
        >
            <div class=(css::HEADER)>
                <div class=(css::HEADING)>
                    <h2 id=(&title_id) class=(css::TITLE)>(title)</h2>
                    <p id=(&description_id) class=(css::DESCRIPTION)>(description)</p>
                </div>
            </div>
            <div class=(css::BODY)>
                <div class=(css::SLOT)>(child)</div>
                // Always rendered, so that a script can show an error.
                alert(
                    tone: AlertTone::Danger,
                    attrs: attributes! { data-dialog-error="" hidden=(error.is_empty()) },
                    (error)
                )
            </div>
            <div class=(css::FOOTER)>
                dialog_close(target: id, (cancel_label))
                button(
                    variant: tone.variant(),
                    loading: busy,
                    attrs: confirm_attrs,
                    (confirm_label)
                )
            </div>
        </dialog>
    })
}
