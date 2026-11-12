//! What to show where a list, table or page has nothing in it yet.
//!
//! ```ignore
//! view! {
//!     empty_state(title: "No chats yet", description: "Start one to see it here.",
//!         icon: Some(Icon::Inbox),
//!         button("New chat")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::empty_state as css;

/// An empty state. `child` is the action, usually one button.
///
/// `compact` suits tables and small cards. `heading` renders the title as an `<h2>`
/// instead of a paragraph; use it when the empty state is the main content of the page.
#[component]
pub async fn empty_state(
    cx: &Cx,
    title: &str,
    #[default] description: &str,
    #[default] icon: Option<Icon>,
    #[default] compact: bool,
    #[default] heading: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let name = icon;
    Ok(view! {
        <div
            data-size=(if compact { "compact" } else { "md" })
            class=(class!(css::EMPTY, attrs.remove("class")))
            (attrs)
        >
            if let Some(name) = name {
                <span aria-hidden="true" class=(css::ICON)>icon(name: name)</span>
            }
            if heading {
                <h2 class=(css::TITLE)>(title)</h2>
            } else {
                <p class=(css::TITLE)>(title)</p>
            }
            if !description.is_empty() {
                <p class=(css::DESCRIPTION)>(description)</p>
            }
            <div class=(css::ACTION)>(child)</div>
        </div>
    })
}
