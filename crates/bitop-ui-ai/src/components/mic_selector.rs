//! Picks an audio input from `navigator.mediaDevices`, in a native `<select>` that is
//! sent with its form.
//!
//! Browsers hide device names, and sometimes the devices, until the page may use the
//! microphone, so the selector offers "Allow microphone access": the library script
//! asks once, stops the stream it was given, and lists the devices again. The list
//! also follows `devicechange` (a headset plugged in). Errors are said in a polite
//! status line. Where there is no `navigator.mediaDevices` (an old browser, or a page
//! that is not served securely), it shows `unsupported_text` instead of a control.
//!
//! The chosen device's ID is the select's value: a page passes it to
//! `getUserMedia({ audio: { deviceId: { exact: id } } })`, and the select's own
//! `change` event says when it changes. A `bitop:mic-permission` event goes out from
//! the selector with `detail.granted` after asking.
//!
//! ```ignore
//! view! { mic_selector(id: "mic", name: "microphone", label: "Microphone") }
//! ```
//!
//! Differences from upstream: the select is the base set's native `<select>` rather
//! than a Base UI select. `useAudioInputs` and `formatLabel` are not ported; the
//! script names unnamed devices with `unnamed_label`. `onValueChange` is the select's
//! `change` event, and a controlled `value` is the first one.

use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    icon::{Icon, icon},
    select::select,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::css::mic_selector as css;

/// The size of a [`mic_selector`]'s control. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MicSelectorSize {
    Sm,
    #[default]
    Md,
}

impl MicSelectorSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A microphone selector.
///
/// - `id` goes on the `<select>`; `name` is its form field; `value` the device ID
///   chosen at first, once the devices are listed.
/// - `label` names it; `hide_label` keeps the label for assistive technology only.
/// - `placeholder` is the empty choice; `unnamed_label` (with `{n}`) names a device
///   without a name.
/// - `permission_label`, `unsupported_text`, `empty_text`, `blocked_text`,
///   `not_found_text` and `error_text` are the other strings.
/// - `size` and `disabled` are the control's.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn mic_selector(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] name: &str,
    #[default] value: &str,
    #[default] hide_label: bool,
    #[default("Select a microphone")] placeholder: &str,
    #[default("Microphone {n}")] unnamed_label: &str,
    #[default("Allow microphone access")] permission_label: &str,
    #[default("Microphone selection isn't available in this browser.")] unsupported_text: &str,
    #[default("No microphones found.")] empty_text: &str,
    #[default(
        "Microphone access is blocked. Allow it in your browser's site settings, then try again."
    )]
    blocked_text: &str,
    #[default("No microphone was found.")] not_found_text: &str,
    #[default("Couldn't access the microphone.")] error_text: &str,
    #[default] size: MicSelectorSize,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let label_class = class!(css::LABEL, hide_label.then_some("sr-only"));
    let plain_label_class = class!(css::LABEL, hide_label.then_some("sr-only"));
    Ok(view! {
        <div
            data-mic-selector=""
            data-status="loading"
            data-value=((!value.is_empty()).then_some(value))
            data-size=(size.as_str())
            data-disabled=(disabled.then_some(""))
            data-unnamed=(unnamed_label)
            data-empty-text=(empty_text)
            data-blocked-text=(blocked_text)
            data-not-found-text=(not_found_text)
            data-error-text=(error_text)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <div class=(css::FIELD) data-mic-field="">
                <label for=(id) class=(label_class)>(label)</label>
                select(
                    attrs: attributes! {
                        id=(id)
                        name=((!name.is_empty()).then_some(name))
                        disabled=(true)
                    },
                    <option value="">(placeholder)</option>
                )
            </div>
            <span hidden=(true) class=(plain_label_class) data-mic-label="">
                (label)
            </span>
            <p hidden=(true) class=(css::MESSAGE) data-mic-unsupported="">
                icon(name: Icon::Mic, attrs: attributes! { class=(css::MESSAGE_ICON) })
                (unsupported_text)
            </p>
            button(
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                attrs: attributes! {
                    hidden=(true)
                    disabled=(disabled)
                    data-mic-permission=""
                    class=(css::PERMISSION)
                },
                icon(name: Icon::Mic)
                (permission_label)
            )
            <div role="status" class=(css::STATUS) data-mic-status=""></div>
            // What the script puts in the status line: copies carry the scoped classes.
            <template data-mic-message-template="">
                <p class=(css::MESSAGE)><span data-mic-text=""></span></p>
            </template>
            <template data-mic-error-template="">
                <p class=(css::ERROR)>
                    icon(
                        name: Icon::CircleAlert,
                        attrs: attributes! { class=(css::MESSAGE_ICON) }
                    )
                    <span data-mic-text=""></span>
                </p>
            </template>
        </div>
    })
}
