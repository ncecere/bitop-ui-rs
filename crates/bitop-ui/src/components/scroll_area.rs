//! A region that scrolls, with thin scrollbars that match the theme.
//!
//! It uses native scrolling. It is focusable so that keyboard users can scroll it, which
//! is why it needs a `label`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::scroll_area as css;

/// A scrolling region. Give it a size, for example with `max-block-size` in `style`.
#[component]
pub async fn scroll_area(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="region"
            aria-label=(label)
            tabindex="0"
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
