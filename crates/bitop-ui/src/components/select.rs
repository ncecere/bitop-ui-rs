//! A select on the native `<select>` element.
//!
//! The browser draws the list, so it works with the keyboard, on touch screens and with
//! assistive technology without a script. Use
//! [`combobox`](super::combobox::combobox) when the list needs filtering.
//!
//! ```ignore
//! view! {
//!     select(attrs: attributes! { name="model" },
//!         <option value="a">"Model A"</option>
//!         <option value="b" selected=(true)>"Model B"</option>
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::select as css;

/// A select. `attrs` are forwarded to the `<select>`; `child` is its `<option>`s.
#[component]
pub async fn select(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(css::ROOT)>
            <select class=(class!(css::CONTROL, attrs.remove("class"))) (attrs)>
                (child)
            </select>
        </span>
    })
}
