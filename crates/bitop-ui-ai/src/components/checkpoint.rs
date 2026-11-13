//! A divider in a conversation that marks a saved state, with a button that rolls the
//! chat (or the agent's files) back to it.
//!
//! It is a group named "Checkpoint 10:42". The restore button's name says which
//! checkpoint it restores, so that several of them can be told apart.
//!
//! The restore button has two forms. With `restore_name`, it is a submit button that
//! sends `restore_name=restore_value` with the form around it. Without it, the library
//! script sends a `bitop:checkpoint-restore` event from the checkpoint, with its
//! `id` in `detail.id`, for a page that restores in script.
//!
//! ```ignore
//! view! {
//!     <form method="post" action="/chat/restore">
//!         checkpoint(time: "10:42", restore: true, restore_name: "checkpoint",
//!             restore_value: "c-41")
//!     </form>
//! }
//! ```
//!
//! Differences from upstream: `onRestore` is the submit button or the event above, and
//! `time` is text, with `datetime` to make it a `<time>`.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::button as button_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::css::checkpoint as css;

/// A checkpoint.
///
/// - `label` is its text; `time` when it was taken, shown after it. `datetime`, a
///   machine-readable date, makes the time a `<time>`.
/// - `name` replaces "{label} {time}" as the group's name and in the restore button's.
/// - `restore` shows the restore button, with `restore_label` as its text; its name is
///   `restore_aria_label`, in which `{restore}` is the text and `{name}` the
///   checkpoint's name in lower case.
/// - `restore_name` and `restore_value` make the button submit; `id` goes in the
///   event otherwise, as `data-checkpoint-id`.
/// - `disabled` disables the button.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn checkpoint(
    cx: &Cx,
    #[default("Checkpoint")] label: &str,
    #[default] time: &str,
    #[default] datetime: &str,
    #[default] name: &str,
    #[default] restore: bool,
    #[default("Restore")] restore_label: &str,
    #[default("{restore} {name}")] restore_aria_label: &str,
    #[default] restore_name: &str,
    #[default] restore_value: &str,
    #[default] id: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let described = if !name.is_empty() {
        name.to_owned()
    } else if time.is_empty() {
        label.to_owned()
    } else {
        format!("{label} {time}")
    };
    let restore_aria = restore_aria_label
        .replace("{restore}", restore_label)
        .replace("{name}", &described.to_lowercase());
    let submits = !restore_name.is_empty();
    Ok(view! {
        <div
            role="group"
            aria-label=(&described)
            data-checkpoint=""
            data-checkpoint-id=((!id.is_empty()).then_some(id))
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::LINE)></span>
            <span class=(css::LABEL)>
                icon(name: Icon::Bookmark, attrs: attributes! { class=(css::ICON) })
                (label)
                if !time.is_empty() {
                    if datetime.is_empty() {
                        <span class=(css::TIME)>(time)</span>
                    } else {
                        <time datetime=(datetime) class=(css::TIME)>(time)</time>
                    }
                }
            </span>
            if restore {
                <button
                    type=(if submits { "submit" } else { "button" })
                    name=(submits.then_some(restore_name))
                    value=(submits.then_some(restore_value))
                    aria-label=(&restore_aria)
                    disabled=(disabled)
                    data-checkpoint-restore=(if submits { "submit" } else { "" })
                    data-variant="ghost"
                    data-size="sm"
                    class=(class!(button_css::ROOT, css::RESTORE))
                >
                    icon(name: Icon::RotateCcw)
                    <span aria-hidden="true">(restore_label)</span>
                </button>
            }
            <span aria-hidden="true" class=(css::LINE)></span>
        </div>
    })
}
