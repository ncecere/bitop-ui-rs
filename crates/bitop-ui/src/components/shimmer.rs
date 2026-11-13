//! Text with a light sweep moving across it, for "Thinking…" and other labels of
//! work in progress.
//!
//! The resting colour is the muted text colour and the sweep brightens towards the
//! text colour, so the text is always readable. Under reduced motion the sweep is
//! removed and the text is simply muted.
//!
//! ```ignore
//! view! { shimmer("Thinking…") }
//! ```
//!
//! Differences from upstream: upstream's `render` prop, which swaps the element, is
//! not ported; the shimmer is a `<span>`, which can go inside any other element.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::shimmer as css;

/// Shimmering text.
///
/// - `duration` is the seconds one sweep takes.
/// - `active` turns the animation off, once streaming ends for example, without
///   changing the element. A script can do the same by removing `data-active`.
#[component]
pub async fn shimmer(
    cx: &Cx,
    #[default(2.0)] duration: f64,
    #[default(true)] active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let duration = if duration.is_finite() && duration > 0.0 {
        duration
    } else {
        2.0
    };
    let style = crate::merge_style(&mut attrs, &format!("--shimmer-duration: {duration}s"));
    Ok(view! {
        <span
            class=(class!(css::SHIMMER, attrs.remove("class")))
            data-shimmer=""
            data-active=(active.then_some(""))
            style=(style)
            (attrs)
        >
            (child)
        </span>
    })
}
