//! A component library for Topcoat in the style of Base UI. The Rust port of
//! [bitop-ui](https://github.com/ncecere/bitop-ui), sharing its tokens and themes.
//!
//! - **Parts, not monoliths.** A dialog is `dialog_trigger`, `dialog`, `dialog_title`, and so
//!   on. Each part renders one element and forwards `attrs` to it.
//! - **State in attributes.** Variants and state are `data-*` and ARIA attributes, so styles
//!   select on them and never on generated class names.
//! - **Platform first.** Overlays use `<dialog>`, the Popover API, invoker commands and CSS
//!   anchor positioning. [`SCRIPT`] adds only what the platform lacks.
//! - **CSS Modules and variables.** Each component has a `*.module.css` file compiled by
//!   `build.rs`. Class names are constants in [`css`]; colours and sizes are CSS variables
//!   defined in `styles/tokens.css` and a theme file.
//!
//! Load the stylesheet and scripts in the document head with [`head`].

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    view::{View, component, view},
};

pub mod components;

/// Scoped class names generated from the `*.module.css` files.
pub mod css {
    include!(concat!(env!("OUT_DIR"), "/css_modules.rs"));
}

/// Tokens, the default theme, base styles and every component's styles.
pub const STYLESHEET: Asset = asset!("{out_dir}/bitop-ui.css");

/// Keyboard and pointer behaviour the platform does not provide, such as arrow keys in
/// menus. One file, built from the `*.js` beside each component.
pub const SCRIPT: Asset = asset!("{out_dir}/bitop-ui.js");

/// Sets `data-theme` on `<html>` from the stored colour mode or the operating system,
/// before the page paints.
pub const COLOR_MODE_SCRIPT: Asset = asset!("./color-mode.js");

/// The `<link>` and `<script>` tags for the library. Render inside `<head>`.
#[component]
pub async fn head(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <link rel="stylesheet" href=(STYLESHEET)>
        <script src=(COLOR_MODE_SCRIPT)></script>
        <script type="module" src=(SCRIPT)></script>
    })
}
