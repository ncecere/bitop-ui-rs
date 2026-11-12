//! Chat and agent components for Topcoat, built on [`bitop_ui`]. The Rust port of the AI
//! elements in [bitop-ui](https://github.com/ncecere/bitop-ui).
//!
//! They are SDK-agnostic: plain arguments such as who sent a message, whether it is
//! still streaming, and the state of a tool call, so any model API's output maps onto
//! them.
//!
//! Render [`bitop_ui::head`] and then [`head`] inside `<head>`. This crate adds its own
//! stylesheet and script; tokens, themes and base styles come from `bitop_ui`.

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

/// Every AI component's styles. Load after `bitop_ui::STYLESHEET`.
pub const STYLESHEET: Asset = asset!("{out_dir}/bitop-ui-ai.css");

/// Behaviour for the AI components, built from the `*.js` beside each one.
pub const SCRIPT: Asset = asset!("{out_dir}/bitop-ui-ai.js");

/// The `<link>` and `<script>` tags for this crate. Render inside `<head>`, after
/// `bitop_ui::head()`.
#[component]
pub async fn head(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <link rel="stylesheet" href=(STYLESHEET)>
        <script type="module" src=(SCRIPT)></script>
    })
}
