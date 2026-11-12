//! A box that keeps a width-to-height ratio. Images, videos and iframes inside it fill
//! it and are cropped to fit.
//!
//! ```ignore
//! view! { aspect_ratio(ratio: 16.0 / 9.0, <img src="..." alt="...">) }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::aspect_ratio as css;

/// The box. `ratio` is width divided by height. `framed` rounds the corners and shows
/// a sunken background while the media loads.
#[component]
pub async fn aspect_ratio(
    cx: &Cx,
    #[default(1.0)] ratio: f64,
    #[default] framed: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            if framed {
                data-framed=""
            }
            class=(class!(css::RATIO, attrs.remove("class")))
            style=(format!("--aspect-ratio: {ratio}"))
            (attrs)
        >
            (child)
        </div>
    })
}
