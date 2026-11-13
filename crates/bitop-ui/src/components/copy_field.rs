//! A read-only value, such as an API key or a command, with a copy button.
//!
//! The value is plain text that can be selected, so it can be copied by hand too. The
//! copy button is handled by the same script as
//! [`copy_button`](super::copy_button::copy_button), and the result is announced
//! through a polite live region that is always in the page.
//!
//! ```ignore
//! view! {
//!     copy_field(id: &id, label: "API key", value: "sk-live-1234",
//!         description: "Shown once. Store it somewhere safe."
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{View, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
};
use crate::css::{button as button_css, copy_button as copy_button_css, copy_field as css};

/// A copy field.
///
/// - `id` is the base for the IDs that connect the parts.
/// - `label` is the visible label. `name` is the plain name used in the button's
///   accessible name, such as "API key"; it defaults to `label`.
/// - `description` is help text under the value.
/// - `multiline` wraps a long value over several lines and keeps its line breaks,
///   for code.
/// - `sans` shows the value in the text font; the default is monospace.
/// - The other strings are templates in which `{name}` is replaced.
#[component]
pub async fn copy_field(
    cx: &Cx,
    id: &str,
    label: &str,
    value: &str,
    #[default] name: &str,
    #[default] description: &str,
    #[default] multiline: bool,
    #[default] sans: bool,
    #[default("Copy {name}")] copy_label: &str,
    #[default("Copied {name}")] copied_label: &str,
    #[default("Copy")] copy_text: &str,
    #[default("Copied")] copied_text: &str,
    #[default("{name} copied to clipboard")] copied_status: &str,
    #[default("Couldn't copy. Select the text and copy it manually.")] failed_status: &str,
) -> Result<impl View> {
    let name = if name.is_empty() { label } else { name };
    let label_id = format!("{id}-label");
    let value_id = format!("{id}-value");
    let description_id = format!("{id}-description");
    Ok(view! {
        <div
            id=(id)
            role="group"
            aria-labelledby=(&label_id)
            aria-describedby=((!description.is_empty()).then_some(&description_id))
            data-copy-root=""
            class=(css::ROOT)
        >
            <span id=(&label_id) class=(css::LABEL)>(label)</span>
            <div class=(css::BOX) data-multiline=(multiline)>
                <(if multiline { "pre" } else { "code" })
                    id=(&value_id)
                    class=(css::VALUE)
                    data-mono=(!sans)
                >
                    (value)
                </(if multiline { "pre" } else { "code" })>
                <button
                    type="button"
                    data-copy=""
                    data-copy-target=(&value_id)
                    data-copy-timeout="2500"
                    data-state="idle"
                    data-label=(copy_label.replace("{name}", name))
                    data-copied-label=(copied_label.replace("{name}", name))
                    aria-label=(copy_label.replace("{name}", name))
                    data-variant=(ButtonVariant::Secondary.as_str())
                    data-size=(ButtonSize::Sm.as_str())
                    class=(class!(button_css::ROOT, copy_button_css::BUTTON, css::BUTTON))
                >
                    icon(name: Icon::Copy)
                    icon(name: Icon::Check)
                    <span
                        data-copy-text=""
                        data-idle=(copy_text)
                        data-copied=(copied_text)
                    >
                        (copy_text)
                    </span>
                </button>
            </div>
            if !description.is_empty() {
                <p id=(&description_id) class=(css::DESCRIPTION)>(description)</p>
            }
            <span
                role="status"
                class="sr-only"
                data-copied=(copied_status.replace("{name}", name))
                data-failed=(failed_status.replace("{name}", name))
            ></span>
        </div>
    })
}
