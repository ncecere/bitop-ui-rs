//! Panels side by side, or stacked, with handles between them that resize the two
//! panels they separate.
//!
//! Sizes are percentages of the group. Panels and handles are direct children of
//! their group; nest a whole group inside a panel for a grid.
//!
//! Each handle is a focusable window splitter (WAI-ARIA): `role="separator"` with
//! `aria-valuenow`, `aria-valuemin` and `aria-valuemax` (the size of the panel before
//! it, and how far it can go) and `aria-controls` pointing at that panel. Drag it, or
//! use the arrow keys (by `keyboard_step` percent), Home for the smallest size and End
//! for the largest. In a right-to-left page the panels are in the reading order and
//! the horizontal arrow keys follow what is seen.
//!
//! The server renders each panel at its default size; the library script lays the
//! group out (clamping the sizes and making them add up to 100), sets the handles'
//! ARIA values and does the resizing. When a resize finishes, the group sends a
//! `bitop:layoutchange` event whose `detail.layout` is the array of sizes.
//!
//! Differences from upstream: upstream's `onLayoutChange` is that event, and
//! `defaultLayout` is the panels' `default_size`s: to keep a layout, store the array
//! (in a cookie or with the user) and render the sizes from it. As a shortcut that
//! needs no server code, `storage` names a `localStorage` key under which the script
//! keeps the layout and from which it restores it. A handle takes its orientation
//! from its group in the stylesheet, and the script writes its `aria-orientation`.
//! Panels added to a group after it was laid out are not sized.
//!
//! ```ignore
//! let sidebar = element_id(cx);
//! let editor = element_id(cx);
//! view! {
//!     resizable_panel_group(storage: "editor-layout",
//!         resizable_panel(id: &sidebar, default_size: 30.0, min_size: 20.0, "Sidebar")
//!         resizable_handle(label: "Resize sidebar", with_handle: true)
//!         resizable_panel(id: &editor, "Editor")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::{css::resizable as css, merge_style};

/// The direction the panels of a [`resizable_panel_group`] are laid out in. Rendered
/// as `data-orientation`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResizableOrientation {
    /// Side by side.
    #[default]
    Horizontal,
    /// Stacked.
    Vertical,
}

impl ResizableOrientation {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// The group. Pass [`resizable_panel`]s with a [`resizable_handle`] between each two.
/// It fills its parent, so give the parent a height.
///
/// - `keyboard_step` is how many percent an arrow key moves a handle.
/// - `storage` is a `localStorage` key to keep the layout under; leave it out and
///   listen for `bitop:layoutchange` to keep it elsewhere.
#[component]
pub async fn resizable_panel_group(
    cx: &Cx,
    #[default] orientation: ResizableOrientation,
    #[default(5.0)] keyboard_step: f32,
    #[default] storage: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::GROUP, attrs.remove("class")))
            data-resizable-group=""
            data-orientation=(orientation.as_str())
            data-keyboard-step=(keyboard_step.to_string())
            data-storage=((!storage.is_empty()).then_some(storage))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One panel. `id` is what the handle after it points its `aria-controls` at.
///
/// Put padding on the content, not on the panel: a panel's padding is not part of
/// what the handles share out, so the sizes would be off by it.
///
/// - `default_size` is its first size in percent; panels without one share what is
///   left.
/// - `min_size` and `max_size` are the limits a handle can move it to, in percent.
#[component]
pub async fn resizable_panel(
    cx: &Cx,
    id: &str,
    #[default]
    #[into]
    default_size: Option<f32>,
    #[default] min_size: f32,
    #[default(100.0)] max_size: f32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // Until the script has laid the group out, a panel with a default size takes that
    // share and the others grow into the rest.
    let own = default_size.map_or_else(String::new, |size| format!("--panel-default-size: {size}"));
    let style = merge_style(&mut attrs, &own);
    Ok(view! {
        <div
            id=(id)
            class=(class!(css::PANEL, attrs.remove("class")))
            style=(style)
            data-resizable-panel=""
            data-min-size=(min_size.to_string())
            data-max-size=(max_size.to_string())
            data-default-size=(default_size.map(|size| size.to_string()))
            data-size-pending=(default_size.map(|_| ""))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The handle between two panels.
///
/// - `label` is its accessible name; say what it resizes, such as "Resize sidebar".
/// - `with_handle` shows a grip in the middle of the line.
/// - `disabled` fixes the two panels' sizes; the handle cannot be focused.
#[component]
pub async fn resizable_handle(
    cx: &Cx,
    #[default("Resize panels")] label: &str,
    #[default] with_handle: bool,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="separator"
            aria-label=(label)
            tabindex=((!disabled).then_some("0"))
            aria-disabled=(disabled.then_some("true"))
            class=(class!(css::HANDLE, attrs.remove("class")))
            data-resizable-handle=""
            (attrs)
        >
            if with_handle {
                <span aria-hidden="true" class=(css::GRIP)>
                    icon(name: Icon::GripVertical)
                </span>
            }
        </div>
    })
}
