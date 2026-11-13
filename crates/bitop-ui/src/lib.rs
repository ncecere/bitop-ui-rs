//! A component library for Topcoat in the style of Base UI. The Rust port of
//! [bitop-ui](https://github.com/ncecere/bitop-ui), sharing its tokens and themes.
//!
//! - **Parts, not monoliths.** A dialog is `dialog_trigger`, `dialog` and `dialog_close`, and
//!   so on. Each part renders one element and forwards `attrs` to it.
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
pub mod font;
pub mod format;

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

/// Joins a part's own inline style with the `style` a caller passed in `attrs`, which
/// is taken out of `attrs` so that the element gets one `style` attribute.
pub fn merge_style(attrs: &mut topcoat::view::Attributes, own: &str) -> Option<String> {
    let theirs = take_attribute(attrs, "style").unwrap_or_default();
    match (own.is_empty(), theirs.is_empty()) {
        (true, true) => None,
        (false, true) => Some(own.to_owned()),
        (true, false) => Some(theirs),
        (false, false) => Some(format!("{own}; {theirs}")),
    }
}

/// Takes the attribute `key` out of `attrs`, as text. `None` if it is not there or
/// has no text, such as a boolean attribute.
pub(crate) fn take_attribute(attrs: &mut topcoat::view::Attributes, key: &str) -> Option<String> {
    use topcoat::view::AttributeValue;
    match attrs.remove(key)? {
        AttributeValue::PromotedStr { value, .. } => Some((*value).to_owned()),
        AttributeValue::StaticStr { value, .. } => Some(value.to_owned()),
        AttributeValue::String { value, .. } => Some(value),
        _ => None,
    }
}

/// The `<link>` and `<script>` tags for the library, and its font. Render inside
/// `<head>`. Only the Latin part of the font is preloaded; the browser fetches the
/// others when a page uses their characters.
#[component]
pub async fn head(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <link
            rel="preload"
            href=(font::INTER_LATIN)
            as="font"
            type="font/woff2"
            crossorigin="anonymous"
        >
        topcoat::font::link(font: font::INTER, preload: false)
        <link rel="stylesheet" href=(STYLESHEET)>
        <script src=(COLOR_MODE_SCRIPT)></script>
        <script type="module" src=(SCRIPT)></script>
    })
}
