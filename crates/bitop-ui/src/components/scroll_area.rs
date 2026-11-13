//! A region that scrolls, with slim scrollbars that match the theme, ported from
//! bitop-ui's `ScrollArea`.
//!
//! It uses native scrolling, so the wheel, touch, the keyboard and find-in-page keep
//! working. The scrollbars are the browser's, styled as a slim rounded thumb that shows
//! while the pointer is over the area. It is focusable so that keyboard users can scroll
//! it, which is why it needs a `label`.
//!
//! ```ignore
//! view! {
//!     scroll_area(label: "Release notes", max_height: "14rem", (notes))
//!     scroll_area(label: "Project tags", orientation: ScrollOrientation::Horizontal, (chips))
//! }
//! ```
//!
//! Unlike upstream, the area is a focusable region whether or not its content
//! overflows: knowing that needs a script, and an extra tab stop is the safer mistake.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::{css::scroll_area as css, merge_style};

/// The directions a [`scroll_area`] scrolls in. Rendered as `data-orientation`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollOrientation {
    /// Up and down; long lines wrap.
    #[default]
    Vertical,
    /// Sideways, for a row such as chips.
    Horizontal,
    /// Both ways, for wide content such as a table or code.
    Both,
}

impl ScrollOrientation {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both => "both",
        }
    }
}

/// A scrolling region. `max_height` (a CSS length, such as `"20rem"`) caps its height;
/// or size it with `style` or a class.
#[component]
pub async fn scroll_area(
    cx: &Cx,
    label: &str,
    #[default] orientation: ScrollOrientation,
    #[default]
    #[into]
    max_height: Option<&str>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let own = max_height.map_or_else(String::new, |h| format!("max-block-size: {h}"));
    let style = merge_style(&mut attrs, &own);
    Ok(view! {
        <div
            role="region"
            aria-label=(label)
            tabindex="0"
            class=(class!(css::ROOT, attrs.remove("class")))
            data-orientation=(orientation.as_str())
            style=(style)
            (attrs)
        >
            (child)
        </div>
    })
}
