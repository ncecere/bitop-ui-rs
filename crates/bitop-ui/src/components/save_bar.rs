//! The actions of a long form (Save, Discard), stuck to the bottom of the scrolling
//! area while the form has unsaved changes, so that they stay in reach.
//!
//! Put it last inside the form, or inside the element that scrolls with it: it uses
//! `position: sticky`, so it rests at the end of the form once that is in view. The
//! message is a polite live region, so "Unsaved changes" is announced when the bar
//! appears. While closed, the bar takes no room and its actions are hidden.
//!
//! Differences from upstream: upstream shows the bar from React state
//! (`open={dirty}`). Here the server renders `open` for a form it knows to hold
//! unsaved values, and with `auto` the library script opens the bar at the first
//! `input` or `change` event in its form and closes it when the form is reset. A
//! script on the page can also call `bitopUi.setSaveBar(bar, open)`. Either way the
//! bar sends a `bitop:toggle` event when it opens or closes.
//!
//! ```ignore
//! view! {
//!     <form method="post">
//!         // fields
//!         save_bar(auto: true,
//!             button(variant: ButtonVariant::Ghost, attrs: attributes! { type="reset" }, "Discard")
//!             button(attrs: attributes! { type="submit" }, "Save")
//!         )
//!     </form>
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::save_bar as css;

/// The save bar. `child` is its actions, such as Discard and Save buttons.
///
/// - `open` shows the bar from the start.
/// - `auto` lets the script open it when a field of the form around it changes, and
///   close it when the form is reset.
/// - `message` is the status text, announced when the bar opens.
#[component]
pub async fn save_bar(
    cx: &Cx,
    #[default] open: bool,
    #[default] auto: bool,
    #[default("Unsaved changes")] message: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::BAR, attrs.remove("class")))
            data-save-bar=(if auto { "auto" } else { "" })
            data-message=(message)
            if open {
                data-open=""
            }
            (attrs)
        >
            // Empty while closed: the text is put in when the bar opens, so that it is
            // announced.
            <p role="status" class=(css::MESSAGE)>
                if open {
                    (message)
                }
            </p>
            <div class=(css::ACTIONS) hidden=(!open)>(child)</div>
        </div>
    })
}
