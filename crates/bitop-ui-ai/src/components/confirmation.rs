//! Asks the person to approve a tool call before it runs.
//!
//! The card renders the text of every state, and shows the one that matches its
//! `state`. Its actions are a real form: each button submits `name=value` (by default
//! `decision=approve` or `decision=reject`), so a server answers by rendering the card
//! in its new state. Before the form is sent, a cancelable `bitop:confirmation` event
//! goes out from the card with `detail.name` and `detail.value`; a page that answers in
//! script cancels it and sets `data-state` on the card itself.
//!
//! When the state changes in the page, the library script announces the outcome in a
//! polite live region, and focus that was on an action button, which has now gone,
//! moves to the card, so that keyboard users keep their place.
//!
//! The card is a group named by `label`, usually the title.
//!
//! ```ignore
//! view! {
//!     confirmation(state: ConfirmationState::Requested, label: "Send the email?",
//!         confirmation_title("Send the email to 42 recipients?")
//!         confirmation_request("The agent wants to call send_email.")
//!         confirmation_accepted("Approved. Sending…")
//!         confirmation_rejected("Rejected. Nothing was sent.")
//!         confirmation_actions(action: "/tool-calls/7",
//!             <input type="hidden" name="call" value="7">
//!             confirmation_action(value: "reject", variant: ButtonVariant::Outline,
//!                 "Reject")
//!             confirmation_action(value: "approve", "Approve")
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: the parts do not read the state from the card; all are
//! rendered and CSS shows those of the current state, so that the page can change it
//! by setting one attribute. `onClick` on an action is the form, or the event above.
//! The card is named with `aria-label` rather than by its title's ID, as Playwright's
//! Firefox crashes on a group named by an element inside it.

use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    icon::{Icon, icon},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::confirmation as css;

/// Where a [`confirmation`] stands. Rendered as `data-state`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConfirmationState {
    /// Waiting for an answer: the request and the actions are shown.
    #[default]
    Requested,
    Accepted,
    Rejected,
}

impl ConfirmationState {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }
}

/// The card: a group named by `label`, with an icon for each state.
///
/// `attrs` are forwarded to the root `<div>`, which takes focus from the script.
#[component]
pub async fn confirmation(
    cx: &Cx,
    label: &str,
    #[default] state: ConfirmationState,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            tabindex="-1"
            data-confirmation=""
            data-state=(state.as_str())
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::ICON)>
                icon(
                    name: Icon::ShieldQuestion,
                    attrs: attributes! { data-when="requested" }
                )
                icon(
                    name: Icon::CircleCheck,
                    attrs: attributes! { data-when="accepted" }
                )
                icon(name: Icon::CircleX, attrs: attributes! { data-when="rejected" })
            </span>
            <div class=(css::BODY)>
                (child)
                // Always rendered: a live region must exist before its text changes.
                <span role="status" class="sr-only" data-confirmation-status=""></span>
            </div>
        </div>
    })
}

/// The question, such as "Send the email to 42 recipients?".
#[component]
pub async fn confirmation_title(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <p class=(class!(css::TITLE, attrs.remove("class"))) (attrs)>(child)</p> })
}

/// What the agent wants to do. Shown while the request waits.
#[component]
pub async fn confirmation_request(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-when="requested"
            class=(class!(css::TEXT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Shown, and announced when the state changes in the page, once approved.
#[component]
pub async fn confirmation_accepted(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-when="accepted"
            data-confirmation-outcome=""
            class=(class!(css::TEXT, css::OUTCOME, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Shown, and announced when the state changes in the page, once rejected.
#[component]
pub async fn confirmation_rejected(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-when="rejected"
            data-confirmation-outcome=""
            class=(class!(css::TEXT, css::OUTCOME, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The actions: a `<form>` shown while the request waits. Put
/// [`confirmation_action`]s in it, and hidden inputs that say which call it is.
///
/// `action` and `method` are the form's; `post` by default. `attrs` are forwarded to
/// the `<form>`.
#[component]
pub async fn confirmation_actions(
    cx: &Cx,
    #[default] action: &str,
    #[default("post")] method: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <form
            action=((!action.is_empty()).then_some(action))
            method=(method)
            data-when="requested"
            data-confirmation-actions=""
            class=(class!(css::ACTIONS, attrs.remove("class")))
            (attrs)
        >
            (child)
        </form>
    })
}

/// An action: a small submit button that sends `name=value`.
#[component]
pub async fn confirmation_action(
    cx: &Cx,
    value: &str,
    #[default("decision")] name: &str,
    #[default] variant: ButtonVariant,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    attrs.insert(cx, "type", "submit");
    attrs.insert(cx, "name", name);
    attrs.insert(cx, "value", value);
    attrs.insert(cx, "data-confirmation-action", "");
    if disabled {
        attrs.insert(cx, "disabled", "");
    }
    Ok(view! { button(variant: variant, size: ButtonSize::Sm, attrs: attrs, (child)) })
}
