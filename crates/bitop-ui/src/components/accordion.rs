//! A stack of headings that each show and hide a panel.
//!
//! Each item is a native `<details>`: the browser opens and closes it on click, Enter
//! and Space, exposes the state to assistive technology, and opens a closed panel when
//! find-in-page finds text in it. No script is involved. Items that share a `name`
//! are exclusive: opening one closes the others. Tab moves between the triggers.
//!
//! The panel's height is animated where the browser can style the content box of a
//! `<details>` (`::details-content`) and run a height to `auto` (`interpolate-size`);
//! elsewhere the panel opens at once.
//!
//! Differences from upstream: upstream is Base UI's accordion, a `<button>` inside a
//! heading with a region beside it. Here the trigger is a `<summary>` with the heading
//! inside it, so some screen readers announce the trigger as a button without its
//! heading level. One panel at a time is not the default: give the items of a group
//! the same `name`. The heading level is an argument of each trigger, since a part
//! cannot read it from the accordion around it. There is no `value` and no
//! `onValueChange`; the server renders `open`, and a script can listen for the native
//! `toggle` event.
//!
//! ```ignore
//! view! {
//!     accordion(
//!         accordion_item(name: "faq", open: true,
//!             accordion_trigger("Billing")
//!             accordion_panel("Invoices are sent on the first of the month.")
//!         )
//!         accordion_item(name: "faq",
//!             accordion_trigger(meta: "3", "Members")
//!             accordion_panel("Owners can invite people.")
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::accordion as css;

/// The look of an [`accordion`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionVariant {
    /// Items separated by hairlines.
    #[default]
    Plain,
    /// Items inside a card.
    Outline,
}

impl AccordionVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Outline => "outline",
        }
    }
}

/// The heading level of an [`accordion_trigger`]. It should fit the page outline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionHeadingLevel {
    H2,
    #[default]
    H3,
    H4,
    H5,
    H6,
}

/// The accordion. Pass [`accordion_item`]s as children.
#[component]
pub async fn accordion(
    cx: &Cx,
    #[default] variant: AccordionVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::ROOT, attrs.remove("class")))
            data-accordion=""
            data-variant=(variant.as_str())
            (attrs)
        >
            (child)
        </div>
    })
}

/// One item: an [`accordion_trigger`] and an [`accordion_panel`].
///
/// - `open` is the initial state.
/// - `name` groups items: of the items with the same name, one is open at a time.
///   Leave it out to let any number be open.
#[component]
pub async fn accordion_item(
    cx: &Cx,
    #[default] open: bool,
    #[default] name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            name=((!name.is_empty()).then_some(name))
            class=(class!(css::ITEM, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The heading of an item, which opens and closes it. `child` is its title.
///
/// - `level` is the heading level.
/// - `meta` is muted text after the title, such as a count or the current value.
/// - `disabled` keeps the item as it is: the trigger cannot be focused or clicked.
#[component]
pub async fn accordion_trigger(
    cx: &Cx,
    #[default] level: AccordionHeadingLevel,
    #[default] meta: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            class=(class!(css::TRIGGER, attrs.remove("class")))
            if disabled {
                aria-disabled="true"
                tabindex="-1"
            }
            (attrs)
        >
            match level {
                AccordionHeadingLevel::H2 => {
                    <h2 class=(css::HEADER)><span class=(css::TITLE)>(child)</span></h2>
                }
                AccordionHeadingLevel::H3 => {
                    <h3 class=(css::HEADER)><span class=(css::TITLE)>(child)</span></h3>
                }
                AccordionHeadingLevel::H4 => {
                    <h4 class=(css::HEADER)><span class=(css::TITLE)>(child)</span></h4>
                }
                AccordionHeadingLevel::H5 => {
                    <h5 class=(css::HEADER)><span class=(css::TITLE)>(child)</span></h5>
                }
                AccordionHeadingLevel::H6 => {
                    <h6 class=(css::HEADER)><span class=(css::TITLE)>(child)</span></h6>
                }
            }
            <span class=(css::META)>(meta)</span>
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::ICON) })
        </summary>
    })
}

/// The content of an item, shown while it is open. `attrs` are forwarded to the
/// padded inner wrapper.
#[component]
pub async fn accordion_panel(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(css::PANEL)>
            <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>
                (child)
            </div>
        </div>
    })
}
