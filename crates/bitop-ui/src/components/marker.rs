//! An inline row in a conversation or an activity feed: a status update ("Explored 4
//! files"), a system note, a bordered row, or a labelled separator ("Today").
//!
//! ```ignore
//! view! {
//!     marker(
//!         marker_icon(icon(name: Icon::Search))
//!         marker_content("Explored 4 files")
//!     )
//!     marker(variant: MarkerVariant::Separator, marker_content("Today"))
//! }
//! ```
//!
//! It has no behaviour of its own. For a marker whose text changes while work is in
//! progress, pass `role="status"` in `attrs` so that updates are announced.
//!
//! Differences from upstream: upstream's `render` prop, which swaps the element, is
//! `href` here: the marker is a `<div>` or, with `href`, an `<a>`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::marker as css;

/// The look of a [`marker`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkerVariant {
    #[default]
    Default,
    /// A hairline under the row.
    Border,
    /// The text centred between two hairlines.
    Separator,
}

impl MarkerVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Border => "border",
            Self::Separator => "separator",
        }
    }
}

/// A marker row. `href` makes it a link.
#[component]
pub async fn marker(
    cx: &Cx,
    #[default] variant: MarkerVariant,
    #[default] href: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::MARKER, attrs.remove("class"));
    Ok(view! {
        if href.is_empty() {
            <div class=(class) data-variant=(variant.as_str()) (attrs)>(child)</div>
        } else {
            <a href=(href) class=(class) data-variant=(variant.as_str()) (attrs)>
                (child)
            </a>
        }
    })
}

/// A decorative icon slot, hidden from assistive technology.
#[component]
pub async fn marker_icon(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            aria-hidden="true"
            class=(class!(css::ICON, attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The text of a [`marker`].
#[component]
pub async fn marker_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</span>
    })
}
