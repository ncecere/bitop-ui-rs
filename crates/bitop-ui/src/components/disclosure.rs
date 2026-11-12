//! A button that shows and hides a panel, for optional settings such as "Advanced
//! options".
//!
//! It is a native `<details>`: the browser opens and closes it on click, Enter and
//! Space, exposes the state to assistive technology, and opens the panel when
//! find-in-page finds text in it. No script is involved.
//!
//! The panel's height is animated where the browser can style the content box of a
//! `<details>` (`::details-content`) and run a height to `auto` (`interpolate-size`);
//! elsewhere the panel opens at once.
//!
//! Differences from upstream: upstream is Base UI's collapsible, a `<button>` with
//! `aria-expanded`; a `<summary>` is announced the same way. There is no
//! `onOpenChange`; a script can listen for the native `toggle` event. `keepMounted`
//! is not needed: the closed panel is always in the document, where find-in-page
//! reaches it. `name` is an addition: disclosures that share one are exclusive.
//!
//! ```ignore
//! view! {
//!     disclosure(title: "Advanced options", summary: "Depth 2 · 200 pages",
//!         <p>"Fields"</p>
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::disclosure as css;

/// A disclosure. `child` is the panel's content.
///
/// - `title` is the trigger's text, and its accessible name.
/// - `summary` is muted text after the title, such as the current values; it is cut
///   with an ellipsis when long and shows in full on hover.
/// - `open` is the initial state.
/// - `name` groups disclosures: of those with the same name, one is open at a time.
/// - `attrs` are forwarded to the `<details>`.
#[component]
pub async fn disclosure(
    cx: &Cx,
    title: &str,
    #[default] summary: &str,
    #[default] open: bool,
    #[default] name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            name=((!name.is_empty()).then_some(name))
            class=(class!(css::ROOT, attrs.remove("class")))
            data-disclosure=""
            (attrs)
        >
            <summary class=(css::TRIGGER)>
                icon(name: Icon::ChevronRight, attrs: attributes! { class=(css::ICON) })
                <span class=(css::TITLE)>(title)</span>
                <span
                    class=(css::SUMMARY)
                    title=((!summary.is_empty()).then_some(summary))
                >
                    (summary)
                </span>
            </summary>
            <div class=(css::PANEL)><div class=(css::CONTENT)>(child)</div></div>
        </details>
    })
}
