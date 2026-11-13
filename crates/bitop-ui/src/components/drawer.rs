//! A panel that slides in from an edge and can be swiped away toward that edge.
//!
//! It is a native `<dialog>`: the browser traps focus, closes it on Escape and on a
//! click on the backdrop, and returns focus to the trigger. It is named by its
//! [`drawer_title`] and described by its [`drawer_description`]. Swiping is a shortcut
//! only: the library script lets a touch drag the drawer by its handle or header (a
//! mouse, by its handle) and closes it when it is released more than a third of the
//! way out, or flicked. If swiping is not needed, [`sheet`](super::sheet) is the
//! simpler edge-anchored dialog.
//!
//! Differences from upstream: `Drawer` and `DrawerContent` are one part, [`drawer`],
//! and the trigger and close buttons are connected to it by its ID. A drawer is always
//! modal. Snap points and the shrinking of a drawer behind a nested one are not
//! ported; both depend on Base UI measuring the drawer as it is dragged.
//!
//! ```ignore
//! let id = element_id(cx);
//! let title = element_id(cx);
//! view! {
//!     drawer_trigger(target: &id, "Open")
//!     drawer(id: &id, labelled_by: &title,
//!         drawer_header(
//!             drawer_title(id: &title, "Move goal")
//!             drawer_description("Set your daily activity goal.")
//!         )
//!         drawer_body(<p>"Content"</p>)
//!         drawer_footer(
//!             button("Submit")
//!             drawer_close(target: &id, "Cancel")
//!         )
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
use crate::css::{button as button_css, drawer as css};

/// The edge a [`drawer`] is attached to; it is swiped back toward this edge to
/// dismiss it. Rendered as `data-side`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

impl DrawerSide {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// A button that opens the drawer whose ID is `target`.
#[component]
pub async fn drawer_trigger(
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

/// The drawer panel. Pass [`drawer_header`], [`drawer_body`] and [`drawer_footer`] as
/// children.
///
/// - `labelled_by` is the ID of its [`drawer_title`]; `described_by` the ID given to
///   its [`drawer_description`], if it has one.
/// - `show_handle` shows the grab handle. It defaults to shown for bottom and top
///   drawers, where it hints that the drawer can be dragged.
/// - `show_close` adds an × button in the corner, named `close_label`.
/// - `attrs` are forwarded to the `<dialog>`.
#[component]
pub async fn drawer(
    cx: &Cx,
    id: &str,
    labelled_by: &str,
    #[default] described_by: &str,
    #[default] side: DrawerSide,
    #[default] show_handle: Option<bool>,
    #[default] show_close: bool,
    #[default("Close")] close_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let handle = show_handle.unwrap_or(matches!(side, DrawerSide::Bottom | DrawerSide::Top));
    Ok(view! {
        <dialog
            id=(id)
            aria-labelledby=(labelled_by)
            aria-describedby=((!described_by.is_empty()).then_some(described_by))
            closedby="any"
            class=(class!(css::POPUP, attrs.remove("class")))
            data-drawer=""
            data-side=(side.as_str())
            data-closable=(show_close)
            (attrs)
        >
            if handle {
                <div aria-hidden="true" class=(css::HANDLE) data-drawer-handle=""></div>
            }
            <div class=(css::CONTENT)>
                (child)
                // Last, so that focus lands on the first field and not on ×.
                if show_close {
                    <button
                        type="button"
                        commandfor=(id)
                        command="close"
                        aria-label=(close_label)
                        class=(class!(button_css::ROOT, css::CLOSE))
                        data-variant="ghost"
                        data-size="icon"
                    >
                        icon(name: Icon::X)
                    </button>
                }
            </div>
        </dialog>
    })
}

/// Groups [`drawer_title`] and [`drawer_description`]. Centred on bottom and top
/// drawers.
#[component]
pub async fn drawer_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::HEADER, attrs.remove("class")))
            data-drawer-header=""
            (attrs)
        >
            (child)
        </div>
    })
}

/// Scrollable, padded content between the header and the footer.
#[component]
pub async fn drawer_body(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::BODY, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// Actions pinned to the end of the drawer. Buttons stack at full width.
#[component]
pub async fn drawer_footer(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::FOOTER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The required heading, an `<h2>`; it names the drawer.
#[component]
pub async fn drawer_title(
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

/// Optional supporting text. To have it describe the drawer, give it an `id` in
/// `attrs` and pass the same ID to the drawer as `described_by`.
#[component]
pub async fn drawer_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::DESCRIPTION, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// A button that closes the drawer whose ID is `target`.
#[component]
pub async fn drawer_close(
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
