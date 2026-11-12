//! Shimmering placeholders for content that is loading.
//!
//! Skeletons are hidden from assistive technology. Pair a region of them with one
//! [`loading`](super::spinner::loading) or a visually hidden `role="status"` message.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::skeleton as css;

/// The shape of a [`skeleton`]. Rendered as `data-shape`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkeletonShape {
    #[default]
    Rect,
    /// One line of text.
    Text,
    Circle,
}

impl SkeletonShape {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rect => "rect",
            Self::Text => "text",
            Self::Circle => "circle",
        }
    }
}

/// One placeholder. `width` and `height` are CSS lengths such as `"12rem"` or `"60%"`.
#[component]
pub async fn skeleton(
    cx: &Cx,
    #[default] shape: SkeletonShape,
    #[default] width: &str,
    #[default] height: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let mut style = String::new();
    if !width.is_empty() {
        style.push_str(&format!("width: {width};"));
    }
    if !height.is_empty() {
        style.push_str(&format!("height: {height};"));
    }
    Ok(view! {
        <span
            aria-hidden="true"
            data-shape=(shape.as_str())
            class=(class!(css::SKELETON, attrs.remove("class")))
            if !style.is_empty() {
                style=(style)
            }
            (attrs)
        ></span>
    })
}

/// A paragraph of placeholder lines. The last line is shorter.
#[component]
pub async fn skeleton_text(
    cx: &Cx,
    #[default(3)] lines: usize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            aria-hidden="true"
            class=(class!(css::TEXT, attrs.remove("class")))
            (attrs)
        >
            for line in 0..lines {
                <span
                    aria-hidden="true"
                    data-shape="text"
                    class=(css::SKELETON)
                    style=(if lines > 1 && line == lines - 1 {
                        "width: 60%"
                    } else {
                        "width: 100%"
                    })
                ></span>
            }
        </span>
    })
}
