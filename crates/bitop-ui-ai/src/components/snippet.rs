//! A one-line command with a copy button, such as an install command in an answer.
//!
//! The prompt (`$`) is decorative: it is hidden from assistive technology and never
//! copied. A long command scrolls sideways, so the `<code>` is in the tab order and the
//! arrow keys scroll it. The command stays left to right in a right-to-left page; the
//! copy button follows the page.
//!
//! ```ignore
//! view! {
//!     snippet(code: "cargo add bitop-ui-ai")
//!     snippet(code: "SELECT 1;", prefix: ">", label: "query")
//! }
//! ```
//!
//! Differences from upstream: the prompt and the command sit in a left-to-right line of
//! their own, so that the prompt stays before the command in a right-to-left page.

use bitop_ui::components::copy_button::copy_button;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::css::snippet as css;

/// A snippet.
///
/// - `code` is the command, shown and copied as it is.
/// - `prefix` is the decorative prompt; `""` hides it.
/// - `label` says what is copied, for the button's name: "Copy command".
/// - `copy_label`, `copied_label`, `copied_status` and `failed_status` are the copy
///   button's strings, in which `{label}` is replaced; see [`copy_button`].
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn snippet(
    cx: &Cx,
    code: &str,
    #[default("$")] prefix: &str,
    #[default("command")] label: &str,
    #[default("Copy {label}")] copy_label: &str,
    #[default("Copied {label}")] copied_label: &str,
    #[default("Copied {label} to clipboard")] copied_status: &str,
    #[default("Couldn't copy. Select the text and copy it manually.")] failed_status: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ROOT, attrs.remove("class"))) data-snippet="" (attrs)>
            <span dir="ltr" class=(css::LINE)>
                if !prefix.is_empty() {
                    <span aria-hidden="true" class=(css::PREFIX)>(prefix)</span>
                }
                <code tabindex="0" class=(css::CODE)>(code)</code>
            </span>
            copy_button(
                value: code,
                label: label,
                copy_label: copy_label,
                copied_label: copied_label,
                copied_status: copied_status,
                failed_status: failed_status,
                attrs: attributes! { class=(css::COPY) }
            )
        </div>
    })
}
