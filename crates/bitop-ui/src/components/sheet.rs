//! A modal dialog attached to an edge of the screen, for secondary tasks such as
//! filters, details and settings that keep the page in view.
//!
//! It is a native `<dialog>`, like [`dialog`](super::dialog): the browser traps focus,
//! closes it on Escape and on a click on the backdrop, and returns focus to the
//! trigger. The title names it and the description describes it. Use
//! [`drawer`](super::drawer) for a panel that can be swiped away.
//!
//! `size: SheetSize::Full` covers the viewport, for a side panel's stand-in on a
//! phone; its close button then comes first, like a page's back button. A sheet of
//! text to read can take focus on its title (`focus_title`). When the body scrolls and
//! holds nothing focusable, the library script makes it focusable (a region named by
//! the title), so that the keyboard can scroll it.
//!
//! Differences from upstream: the trigger and the close button are parts
//! ([`sheet_trigger`], [`sheet_close`]) connected by the sheet's ID, not props. The
//! sides are physical, as upstream: a right sheet stays on the right in a
//! right-to-left page, and its contents mirror. `initialFocus` is `focus_title` or an
//! `autofocus` attribute on a control in the body; `finalFocus` is always the trigger.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     sheet_trigger(target: &id, "Filters")
//!     sheet(
//!         id: &id,
//!         title: "Filters",
//!         description: "Narrow the list of deployments.",
//!         footer: view! {
//!             sheet_close(target: &id, "Cancel")
//!             button("Apply")
//!         },
//!         <p>"Fields"</p>
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
};
use crate::css::{button as button_css, sheet as css};

/// The edge a [`sheet`] slides in from. Rendered as `data-side`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

impl SheetSide {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Left => "left",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// The width of a left or right [`sheet`]; top and bottom sheets span the viewport.
/// `Full` covers the whole screen from any side. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Full,
}

impl SheetSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Full => "full",
        }
    }
}

/// A button that opens the sheet whose ID is `target`.
#[component]
pub async fn sheet_trigger(
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

/// The sheet. `child` is its body.
///
/// - `title` names the sheet and `description` describes it.
/// - `footer` is a slot pinned to the end, usually buttons; use [`sheet_close`] for
///   Cancel.
/// - `hide_close` removes the × button; Escape still closes the sheet.
/// - `close_label` and `close_icon` are the × button's name and icon, such as an
///   arrow back for a full-screen sheet.
/// - `focus_title` moves focus to the title when the sheet opens, not to the first
///   control.
/// - `attrs` are forwarded to the `<dialog>`.
#[component]
pub async fn sheet(
    cx: &Cx,
    id: &str,
    title: &str,
    description: &str,
    #[default] side: SheetSide,
    #[default] size: SheetSize,
    #[default] hide_close: bool,
    #[default("Close")] close_label: &str,
    #[default(Icon::X)] close_icon: Icon,
    #[default] focus_title: bool,
    #[default]
    #[into]
    footer: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let title_id = format!("{id}-title");
    let description_id = format!("{id}-description");
    let full = size == SheetSize::Full;
    Ok(view! {
        <dialog
            id=(id)
            aria-labelledby=(&title_id)
            aria-describedby=(&description_id)
            closedby="any"
            class=(class!(css::POPUP, attrs.remove("class")))
            data-sheet=""
            data-side=(side.as_str())
            data-size=(size.as_str())
            data-closable=(!hide_close)
            (attrs)
        >
            // A full-screen sheet's close button comes first, where it shows.
            if full && !hide_close {
                <button
                    type="button"
                    commandfor=(id)
                    command="close"
                    aria-label=(close_label)
                    class=(class!(button_css::ROOT, css::CLOSE))
                    data-variant="ghost"
                    data-size="icon"
                >
                    icon(name: close_icon)
                </button>
            }
            <div class=(css::HEADER)>
                <h2
                    id=(&title_id)
                    class=(css::TITLE)
                    if focus_title {
                        tabindex="-1"
                        autofocus=(true)
                    }
                >
                    (title)
                </h2>
                <p id=(&description_id) class=(css::DESCRIPTION)>(description)</p>
            </div>
            <div class=(css::BODY) data-sheet-body="" data-sheet-title=(&title_id)>
                (child)
            </div>
            <div class=(css::FOOTER)>(footer)</div>
            // Otherwise last, so that focus lands on the first field and not on ×.
            if !full && !hide_close {
                <button
                    type="button"
                    commandfor=(id)
                    command="close"
                    aria-label=(close_label)
                    class=(class!(button_css::ROOT, css::CLOSE))
                    data-variant="ghost"
                    data-size="icon"
                >
                    icon(name: close_icon)
                </button>
            }
        </dialog>
    })
}

/// A button that closes the sheet whose ID is `target`.
#[component]
pub async fn sheet_close(
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
