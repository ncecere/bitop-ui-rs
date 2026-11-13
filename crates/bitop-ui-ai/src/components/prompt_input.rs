//! The chat composer: a `<form>` with a growing text box, attachments, tool buttons and
//! a submit button that becomes a stop button while an answer arrives.
//!
//! It is a real form and is sent as one: the text under the textarea's `name`
//! (`message`), the files under the file input's (`files`, with
//! `enctype="multipart/form-data"`), and a pressed tool button's `name=value`. The
//! library script adds what a chat needs:
//!
//! - Enter sends, Shift+Enter starts a new line, and the Enter that confirms an input
//!   method's composition (Japanese, Chinese, Korean…) never sends. An empty message,
//!   and a send while an answer is on its way, are ignored.
//! - The text box grows with its content up to a maximum height, then scrolls. With
//!   `max_length` a counter shows near the limit, and reaching it is announced.
//! - Before the form is sent, a `bitop:prompt-submit` event goes out from it with
//!   `detail.text` and `detail.files`. A page that sends the message itself cancels
//!   the event: the form is then not sent, and the text box and the files are cleared.
//! - The submit button follows the form's `data-status` ([`ChatStatus`]): it sends
//!   while ready, shows a spinner once submitted, and stops while streaming. Stop
//!   submits the form with `stop_name=stop_value`, after a cancelable
//!   `bitop:prompt-stop` event. An app that streams into the page changes `data-status`
//!   on the form, and the button follows.
//! - With `attachments`, files come from the attach button, a paste or a drop on the
//!   form. They are checked against `accept`, `max_file_size` and `max_files` (a drop
//!   ignores the picker's filter), shown as [`attachment`] chips with image previews,
//!   and removed with their button. Preview URLs are revoked when a file is removed,
//!   replaced or sent, and when the page goes away. A disabled composer takes no
//!   files and removes none.
//!
//! ```ignore
//! view! {
//!     prompt_input(action: "/chat", status: ChatStatus::Ready, attachments: true,
//!         accept: "image/*,.pdf",
//!         prompt_input_attachments()
//!         prompt_input_textarea(placeholder: "Ask anything…")
//!         prompt_input_toolbar(
//!             prompt_input_tools(prompt_input_attach_button())
//!             prompt_input_submit(status: ChatStatus::Ready)
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: `onSubmit` and `onStop` are the form's submission and the
//! two events above, and `onFileError` is a polite status line. Parts take the status
//! and the disabled state as arguments rather than from context. A controlled `value`
//! is the textarea's initial text.
//!
//! [`ChatStatus`]: crate::ChatStatus
//! [`attachment`]: super::attachments::attachment

use bitop_ui::{
    components::{
        icon::{Icon, icon},
        spinner::{SpinnerSize, spinner},
    },
    css::{button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::attachments::{attachment, attachments};
use crate::{
    ChatStatus,
    css::{attachments as attachments_css, prompt_input as css},
};

/// When a [`prompt_input_textarea`] shows its character count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PromptCount {
    /// From 80% of `max_length`.
    #[default]
    Auto,
    Always,
    Never,
}

impl PromptCount {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        }
    }
}

/// The composer: a `<form>`.
///
/// - `action` and `method` are the form's; `post` by default.
/// - `status` is the chat's, as `data-status`.
/// - `disabled` blocks typing, sending, tools and files.
/// - `attachments` takes files into a file input named `files_name`; `accept`,
///   `multiple`, `max_files` and `max_file_size` (bytes, 0 for no limit) limit them.
/// - `type_error`, `size_error` (with `{name}`) and `count_error` (with `{max}`) are
///   said when a file is left out.
/// - `attrs` are forwarded to the `<form>`.
#[component]
pub async fn prompt_input(
    cx: &Cx,
    #[default] action: &str,
    #[default("post")] method: &str,
    #[default] status: ChatStatus,
    #[default] disabled: bool,
    #[default] attachments: bool,
    #[default("files")] files_name: &str,
    #[default] accept: &str,
    #[default(true)] multiple: bool,
    #[default] max_files: usize,
    #[default] max_file_size: u64,
    #[default("{name} isn't a supported file type.")] type_error: &str,
    #[default("{name} is too large.")] size_error: &str,
    #[default("You can attach up to {max} files.")] count_error: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let count_error = count_error.replace("{max}", &max_files.to_string());
    Ok(view! {
        <form
            action=((!action.is_empty()).then_some(action))
            method=(method)
            enctype=(attachments.then_some("multipart/form-data"))
            novalidate=(true)
            data-prompt-input=""
            data-status=(status.as_str())
            data-disabled=(disabled.then_some(""))
            data-max-files=((max_files > 0).then(|| max_files.to_string()))
            data-max-size=((max_file_size > 0).then(|| max_file_size.to_string()))
            data-type-error=(type_error)
            data-size-error=(size_error)
            data-count-error=(&count_error)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            if attachments {
                <input
                    type="file"
                    name=(files_name)
                    hidden=(true)
                    tabindex="-1"
                    multiple=(multiple)
                    accept=((!accept.is_empty()).then_some(accept))
                    disabled=(disabled)
                    data-prompt-files=""
                >
            }
            (child)
            // Always rendered: a live region must exist before its text changes.
            <p role="status" class="sr-only" data-prompt-file-error=""></p>
        </form>
    })
}

/// The text box.
///
/// - `name` is its form field; `value` its initial text.
/// - `label` names it, unless `attrs` has `aria-labelledby`.
/// - `max_length` limits it; `show_count` says when the counter shows; `count_label`
///   is read after the count and `limit_label` (with `{max}`) announced at the limit.
/// - `submit_on_enter` makes Enter send; Ctrl or Cmd+Enter always sends.
/// - `disabled` disables it; pass the composer's.
/// - `attrs` are forwarded to the `<textarea>`.
#[component]
pub async fn prompt_input_textarea(
    cx: &Cx,
    #[default("message")] name: &str,
    #[default] value: &str,
    #[default("Ask anything…")] placeholder: &str,
    #[default("Message")] label: &str,
    #[default]
    #[into]
    max_length: Option<usize>,
    #[default] show_count: PromptCount,
    #[default("characters")] count_label: &str,
    #[default("Character limit of {max} reached")] limit_label: &str,
    #[default(true)] submit_on_enter: bool,
    #[default] disabled: bool,
    #[default(1)] rows: u32,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let labelled = attrs.contains_key("aria-labelledby");
    let length = value.chars().count();
    let near = max_length.is_some_and(|max| length * 10 >= max * 8);
    let counted = max_length.is_some()
        && match show_count {
            PromptCount::Always => true,
            PromptCount::Auto => near,
            PromptCount::Never => false,
        };
    let at_limit = max_length.is_some_and(|max| length >= max);
    let count_text = max_length.map(|max| format!("{length} / {max}"));
    let limit = max_length.map(|max| limit_label.replace("{max}", &max.to_string()));
    Ok(view! {
        <div class=(css::TEXTAREA_WRAP)>
            <textarea
                name=(name)
                rows=(rows)
                maxlength=(max_length)
                placeholder=(placeholder)
                aria-label=((!labelled).then_some(label))
                disabled=(disabled)
                data-prompt-textarea=""
                data-submit-on-enter=(if submit_on_enter { "true" } else { "false" })
                data-show-count=(show_count.as_str())
                class=(class!(css::TEXTAREA, attrs.remove("class")))
                (attrs)
            >
                (value)
            </textarea>
            if let Some(count_text) = count_text {
                <span
                    class=(css::COUNTER)
                    data-prompt-count=""
                    data-limit=(at_limit.then_some(""))
                    hidden=(!counted)
                >
                    <span data-prompt-count-text="">(count_text)</span>
                    " "
                    <span class="sr-only">(count_label)</span>
                </span>
            }
            <span role="status" class="sr-only" data-prompt-limit=(limit)>
                if at_limit {
                    (limit_label.replace(
                        "{max}",
                        &max_length.unwrap_or_default().to_string(),
                    ))
                }
            </span>
        </div>
    })
}

/// The row under the text box: tools at the start, the submit button at the end.
#[component]
pub async fn prompt_input_toolbar(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::TOOLBAR, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The tool buttons at the start of the toolbar.
#[component]
pub async fn prompt_input_tools(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::TOOLS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A tool button. `child` is the icon.
///
/// - `label` names it; with `show_label` it is shown beside the icon, otherwise
///   `tooltip` is an ID for a tooltip with it.
/// - `pressed` makes it a toggle ("Search the web"); with `name` a pressed one submits
///   `name=value`.
/// - `disabled` disables it; pass the composer's.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn prompt_input_button(
    cx: &Cx,
    label: &str,
    #[default] show_label: bool,
    #[default] tooltip: &str,
    #[default]
    #[into]
    pressed: Option<bool>,
    #[default] name: &str,
    #[default("on")] value: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty() && !show_label;
    let toggle = pressed.is_some();
    let is_pressed = pressed.unwrap_or(false);
    Ok(view! {
        <button
            type="button"
            aria-label=((!show_label).then_some(label))
            aria-pressed=(pressed.map(|on| if on { "true" } else { "false" }))
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            data-toggle=(toggle.then_some(""))
            disabled=(disabled)
            data-variant="ghost"
            data-size=(if show_label { "sm" } else { "icon" })
            data-prompt-tool=""
            class=(class!(button_css::ROOT, css::TOOL_BUTTON, attrs.remove("class")))
            (attrs)
        >
            (child)
            if show_label {
                <span>(label)</span>
            }
            if toggle && !name.is_empty() {
                <input type="hidden" name=(name) value=(value) disabled=(!is_pressed)>
            }
        </button>
        if has_tooltip {
            <span
                id=(tooltip)
                role="tooltip"
                popover="manual"
                class=(tooltip_css::POPUP)
            >
                (label)
            </span>
        }
    })
}

/// Opens the file picker of a composer with `attachments`.
#[component]
pub async fn prompt_input_attach_button(
    cx: &Cx,
    #[default("Attach files")] label: &str,
    #[default] tooltip: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            disabled=(disabled)
            data-variant="ghost"
            data-size="icon"
            data-prompt-attach=""
            data-prompt-tool=""
            class=(class!(button_css::ROOT, css::TOOL_BUTTON, attrs.remove("class")))
            (attrs)
        >
            icon(name: Icon::Paperclip)
        </button>
        if has_tooltip {
            <span
                id=(tooltip)
                role="tooltip"
                popover="manual"
                class=(tooltip_css::POPUP)
            >
                (label)
            </span>
        }
    })
}

/// The submit button, which follows the chat's status.
///
/// - `status` is the composer's; the script follows the form's `data-status` after.
/// - `stop_while_submitted` offers Stop already while waiting for the first token.
/// - `send_label`, `stop_label` and `sending_label` name it in each state;
///   `show_label` shows `send_text`, `stop_text` or `sending_text` beside the icon.
/// - `stop_name` and `stop_value` are sent when Stop submits the form.
/// - `disabled` disables it; pass the composer's.
#[component]
pub async fn prompt_input_submit(
    cx: &Cx,
    #[default] status: ChatStatus,
    #[default] stop_while_submitted: bool,
    #[default("Send message")] send_label: &str,
    #[default("Stop generating")] stop_label: &str,
    #[default("Sending message")] sending_label: &str,
    #[default] show_label: bool,
    #[default("Send")] send_text: &str,
    #[default("Stop")] stop_text: &str,
    #[default("Sending")] sending_text: &str,
    #[default("action")] stop_name: &str,
    #[default("stop")] stop_value: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let mode = submit_mode(status, stop_while_submitted);
    let (label, text) = match mode {
        "stop" => (stop_label, stop_text),
        "sending" => (sending_label, sending_text),
        _ => (send_label, send_text),
    };
    let stop = mode == "stop";
    Ok(view! {
        <button
            type="submit"
            aria-label=(label)
            name=(stop.then_some(stop_name))
            value=(stop.then_some(stop_value))
            formnovalidate=(true)
            disabled=(disabled || mode == "sending")
            data-prompt-submit=""
            data-mode=(mode)
            data-status=(status.as_str())
            data-stop-early=(stop_while_submitted.then_some(""))
            data-stop-name=(stop_name)
            data-stop-value=(stop_value)
            data-label-send=(send_label)
            data-label-stop=(stop_label)
            data-label-sending=(sending_label)
            data-labelled=(show_label.then_some(""))
            data-variant="primary"
            data-size=(if show_label { "sm" } else { "icon" })
            class=(class!(button_css::ROOT, css::SUBMIT, attrs.remove("class")))
            (attrs)
        >
            icon(name: Icon::ArrowUp, attrs: attributes! { class=(css::SEND_ICON) })
            icon(name: Icon::Square, attrs: attributes! { class=(css::STOP_ICON) })
            spinner(size: SpinnerSize::Sm, attrs: attributes! { class=(css::SPINNER) })
            if show_label {
                <span
                    aria-hidden="true"
                    data-prompt-submit-text=""
                    data-send=(send_text)
                    data-stop=(stop_text)
                    data-sending=(sending_text)
                >
                    (text)
                </span>
            }
        </button>
    })
}

/// What the submit button does for a status: `send`, `sending` or `stop`.
fn submit_mode(status: ChatStatus, stop_while_submitted: bool) -> &'static str {
    match status {
        ChatStatus::Streaming => "stop",
        ChatStatus::Submitted if stop_while_submitted => "stop",
        ChatStatus::Submitted => "sending",
        ChatStatus::Ready | ChatStatus::Error => "send",
    }
}

/// Chips for the files chosen in the composer, with their remove buttons. The script
/// fills it; `remove_label` (with `{name}`) names a remove button, and
/// `uploading_label` and `error_label` are the chip's. `disabled` disables the remove
/// buttons; pass the composer's.
#[component]
pub async fn prompt_input_attachments(
    cx: &Cx,
    #[default("Attachments")] label: &str,
    #[default("Remove {name}")] remove_label: &str,
    #[default("Remove")] remove_tooltip: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = class!(css::ATTACHMENTS, attrs.remove("class"));
    attrs.insert(cx, "class", class);
    attrs.insert(cx, "data-prompt-attachments", "");
    Ok(view! {
        attachments(label: label, attrs: attrs)
        // The chip the script copies for each file: the scoped class names come with it.
        <template
            data-prompt-attachment-template=""
            data-remove-label=(remove_label)
            data-image-class=(attachments_css::IMAGE)
        >
            attachment(
                name: "",
                removable: true,
                disabled: disabled,
                remove_label: "",
                remove_tooltip: remove_tooltip
            )
        </template>
    })
}

#[cfg(test)]
mod tests {
    use super::submit_mode;
    use crate::ChatStatus;

    #[test]
    fn submit_follows_the_status() {
        assert_eq!(submit_mode(ChatStatus::Ready, false), "send");
        assert_eq!(submit_mode(ChatStatus::Error, true), "send");
        assert_eq!(submit_mode(ChatStatus::Submitted, false), "sending");
        assert_eq!(submit_mode(ChatStatus::Submitted, true), "stop");
        assert_eq!(submit_mode(ChatStatus::Streaming, false), "stop");
    }
}
