//! Icons, drawn as CSS masks so that they take the colour of the text around them.
//!
//! The set is small on purpose: only what the library's own components show. The
//! drawings are from [Lucide](https://lucide.dev) (ISC licence). An icon is decorative
//! and hidden from assistive technology; put the meaning in text or an `aria-label`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, component, view},
};

/// An icon in the set. Rendered as `data-icon`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Info,
    CircleCheck,
    TriangleAlert,
    CircleAlert,
    ArrowUpRight,
    X,
    Check,
    ChevronDown,
    ChevronRight,
    Search,
    Inbox,
    Minus,
    Plus,
    Copy,
    ChevronLeft,
    Sun,
    Moon,
    CalendarDays,
    Upload,
    Ellipsis,
    GripVertical,
}

impl Icon {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::CircleCheck => "circle-check",
            Self::TriangleAlert => "triangle-alert",
            Self::CircleAlert => "circle-alert",
            Self::ArrowUpRight => "arrow-up-right",
            Self::X => "x",
            Self::Check => "check",
            Self::ChevronDown => "chevron-down",
            Self::ChevronRight => "chevron-right",
            Self::Search => "search",
            Self::Inbox => "inbox",
            Self::Minus => "minus",
            Self::Plus => "plus",
            Self::Copy => "copy",
            Self::ChevronLeft => "chevron-left",
            Self::Sun => "sun",
            Self::Moon => "moon",
            Self::CalendarDays => "calendar-days",
            Self::Upload => "upload",
            Self::Ellipsis => "ellipsis",
            Self::GripVertical => "grip-vertical",
        }
    }
}

/// An icon, one em square unless `attrs` gives it a class that sizes it.
#[component]
pub async fn icon(cx: &Cx, name: Icon, #[default] attrs: Attributes) -> Result<impl View> {
    Ok(view! { <span data-icon=(name.as_str()) aria-hidden="true" (attrs)></span> })
}
