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
pub mod markdown;

/// The state of a chat, shared by
/// [`conversation_announcer`](components::conversation::conversation_announcer) and
/// [`prompt_input`](components::prompt_input::prompt_input). Rendered as
/// `data-status`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChatStatus {
    /// Waiting for the person.
    #[default]
    Ready,
    /// A message was sent; no answer has arrived yet.
    Submitted,
    /// The answer is arriving.
    Streaming,
    /// The answer failed.
    Error,
}

impl ChatStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Submitted => "submitted",
            Self::Streaming => "streaming",
            Self::Error => "error",
        }
    }

    /// Whether an answer is on its way.
    #[must_use]
    pub fn is_busy(self) -> bool {
        matches!(self, Self::Submitted | Self::Streaming)
    }
}

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
