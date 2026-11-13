//! A button that copies text to the clipboard.
//!
//! The library script writes the text, swaps the icon for a check mark for a moment
//! and announces the result through a polite live region that is always in the page.
//! Where the Clipboard API is missing or refuses, it falls back to
//! `document.execCommand("copy")`.
//!
//! Any button can copy: give it `data-copy` and either `data-copy-value`, the text,
//! or `data-copy-target`, the ID of the element whose text (or value, for a form
//! control) is copied when the button is pressed. Scripts can call
//! `bitopUi.copy(text)`, which returns a promise.
//!
//! Differences from upstream: the text is a string or another element's text, not a
//! function; there are no `onCopy` and `onError` callbacks, but the button's
//! `data-state` changes to `copied` or `failed`; and the tooltip is shown only when
//! `tooltip` gives it an ID.
//!
//! ```ignore
//! view! {
//!     copy_button(value: "npm install bitop-ui", label: "command")
//!     copy_button(target: &code_id, label: "code", show_text: true)
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
};
use crate::css::{button as button_css, copy_button as css, tooltip as tooltip_css};

/// A copy button.
///
/// - `value` is the text to copy. Or `target` is the ID of the element whose text is
///   copied, read when the button is pressed.
/// - `label` says what is copied, for the accessible name: "Copy code", "Copied code".
/// - `show_text` shows "Copy" and "Copied" beside the icon.
/// - `tooltip` is an ID for a tooltip with the same words, for an icon-only button.
/// - `timeout_ms` is how long the copied state lasts.
/// - The other strings are templates in which `{label}` is replaced.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn copy_button(
    cx: &Cx,
    #[default] value: &str,
    #[default] target: &str,
    #[default("text")] label: &str,
    #[default] show_text: bool,
    #[default] tooltip: &str,
    #[default(2000)] timeout_ms: u32,
    #[default(ButtonVariant::Ghost)] variant: ButtonVariant,
    #[default("Copy {label}")] copy_label: &str,
    #[default("Copied {label}")] copied_label: &str,
    #[default("Copy")] copy_text: &str,
    #[default("Copied")] copied_text: &str,
    #[default("Copied {label} to clipboard")] copied_status: &str,
    #[default("Couldn't copy. Select the text and copy it manually.")] failed_status: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty() && !show_text;
    let tooltip_id = has_tooltip.then_some(tooltip);
    Ok(view! {
        <span class=(css::ROOT) data-copy-root="">
            <button
                type="button"
                data-copy=""
                data-copy-value=(target.is_empty().then_some(value))
                data-copy-target=((!target.is_empty()).then_some(target))
                data-copy-timeout=(timeout_ms)
                data-state="idle"
                data-label=(copy_label.replace("{label}", label))
                data-copied-label=(copied_label.replace("{label}", label))
                aria-label=(copy_label.replace("{label}", label))
                aria-describedby=(tooltip_id)
                data-tooltip=(tooltip_id)
                data-variant=(variant.as_str())
                data-size=(ButtonSize::Sm.as_str())
                data-icon-only=((!show_text).then_some(""))
                class=(class!(button_css::ROOT, css::BUTTON, attrs.remove("class")))
                (attrs)
            >
                icon(name: Icon::Copy)
                icon(name: Icon::Check)
                if show_text {
                    <span
                        aria-hidden="true"
                        data-copy-text=""
                        data-idle=(copy_text)
                        data-copied=(copied_text)
                    >
                        (copy_text)
                    </span>
                }
            </button>
            if has_tooltip {
                <span
                    id=(tooltip)
                    role="tooltip"
                    popover="manual"
                    class=(tooltip_css::POPUP)
                    data-copy-text=""
                    data-idle=(copy_text)
                    data-copied=(copied_text)
                >
                    (copy_text)
                </span>
            }
            <span
                role="status"
                class="sr-only"
                data-copied=(copied_status.replace("{label}", label))
                data-failed=(failed_status.replace("{label}", label))
            ></span>
        </span>
    })
}
