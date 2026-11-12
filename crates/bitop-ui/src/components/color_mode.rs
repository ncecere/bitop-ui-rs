//! Controls for the colour mode: light, dark, or the operating system's.
//!
//! The themes read `data-theme` on `<html>`. The script that [`head`](crate::head)
//! loads sets it before the page paints, from the stored mode or the system setting,
//! and follows changes to both, including a mode set in another tab. These parts
//! change the mode with `bitopUi.setColorMode()`, and the library script keeps them
//! in step with it.
//!
//! Differences from upstream: `useColorMode` is `bitopUi.colorMode()` and
//! `bitopUi.setColorMode(mode)`, and a `bitop:color-mode` event on the document with
//! `detail.mode` and `detail.theme`. [`color_mode_select`] is new here; it is how a
//! person gets back to the system setting.
//!
//! ```ignore
//! view! {
//!     color_mode_toggle()
//!     color_mode_select()
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
    select::select,
};
use crate::css::{button as button_css, color_mode as css};

/// An icon button that switches between light and dark. `aria-pressed` says whether
/// dark mode is on; `label` is its accessible name.
#[component]
pub async fn color_mode_toggle(
    cx: &Cx,
    #[default("Dark mode")] label: &str,
    #[default(ButtonVariant::Ghost)] variant: ButtonVariant,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            aria-pressed="false"
            data-color-mode-toggle=""
            class=(class!(button_css::ROOT, css::TOGGLE, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size=(ButtonSize::Icon.as_str())
            (attrs)
        >
            icon(name: Icon::Sun)
            icon(name: Icon::Moon)
        </button>
    })
}

/// A select with the three modes. `label` is its accessible name; the other strings
/// are its options. `attrs` are forwarded to the `<select>`.
#[component]
pub async fn color_mode_select(
    cx: &Cx,
    #[default("Color mode")] label: &str,
    #[default("System")] system: &str,
    #[default("Light")] light: &str,
    #[default("Dark")] dark: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = attrs.remove("class");
    Ok(view! {
        select(
            attrs: attributes! {
                aria-label=(label)
                data-color-mode-select=""
                class=(class)
                (attrs)
            },
            <option value="system">(system)</option>
            <option value="light">(light)</option>
            <option value="dark">(dark)</option>
        )
    })
}
