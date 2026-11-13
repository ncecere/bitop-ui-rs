//! One turn of a chat: a bubble for the person, plain prose for the assistant, with
//! actions, feedback and alternative versions of a response.
//!
//! Each message is an `<article>` named "You said" or "Assistant said", so that
//! screen reader users can move between turns.
//!
//! - [`message_actions`] is a group of icon buttons under a message. With
//!   `visibility: Hover` it shows on hover and on focus, and always on touch screens.
//! - [`message_action`] is one of them. With `pressed` it is a toggle (the base
//!   set's), for thumbs up and down; `exclusive` actions in one group release each
//!   other, and with a `name` a pressed one submits `name=value` with its form.
//! - [`message_copy_action`] copies the message's text with the base set's copy button.
//! - [`message_branch`], [`message_branch_content`] and [`message_branch_selector`]
//!   show one of several server-rendered versions of a response ("2 of 3"), and the
//!   library script switches between them. The index is kept inside the versions there
//!   are, also when the server renders fewer later; a change, and a clamp, is reported
//!   with a `bitop:branch-change` event from the branch, with `detail.index`.
//!
//! ```ignore
//! view! {
//!     message(from: MessageRole::User, message_content("Hi!"))
//!     message(from: MessageRole::Assistant,
//!         message_content(response(source: &text))
//!         message_actions(
//!             message_copy_action(value: &text)
//!             message_action(label: "Good response", pressed: false, exclusive: true,
//!                 icon(name: Icon::ThumbsUp))
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: a message part does not learn who sent it from its
//! message; the content's default variant comes from the message's `data-from` in CSS.
//! Branches are switched by the script among versions the server rendered, and the
//! selector takes the index and the count. A tooltip needs an ID, as the base set's
//! tooltips do; `onClick` handlers are the toggle, the copy button or the caller's own
//! script.

use bitop_ui::{
    components::{
        avatar::{AvatarShape, AvatarSize, avatar},
        button::ButtonVariant,
        copy_button::copy_button,
        icon::{Icon, icon},
    },
    css::{button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::message as css;

/// Who sent a [`message`]. Rendered as `data-from`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageRole {
    User,
    #[default]
    Assistant,
}

impl MessageRole {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

/// How a [`message_content`] is drawn. Rendered as `data-variant`; without it, a
/// user's message is a bubble and an assistant's is plain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageContentVariant {
    Bubble,
    Plain,
}

impl MessageContentVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bubble => "bubble",
            Self::Plain => "plain",
        }
    }
}

/// When [`message_actions`] show. Rendered as `data-visibility`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageActionsVisibility {
    #[default]
    Always,
    /// On hover or focus inside the message; always where there is no hover.
    Hover,
}

impl MessageActionsVisibility {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Hover => "hover",
        }
    }
}

/// A message: an `<article>`. `label` names it; by default "You said" or
/// "Assistant said". `attrs` are forwarded to the `<article>`.
#[component]
pub async fn message(
    cx: &Cx,
    from: MessageRole,
    #[default] label: &str,
    #[default("You said")] user_label: &str,
    #[default("Assistant said")] assistant_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let name = if !label.is_empty() {
        label
    } else if from == MessageRole::User {
        user_label
    } else {
        assistant_label
    };
    Ok(view! {
        <article
            aria-label=(name)
            data-from=(from.as_str())
            data-message=""
            class=(class!(css::MESSAGE, attrs.remove("class")))
            (attrs)
        >
            (child)
        </article>
    })
}

/// The message's text or other content. `variant` overrides the default drawing.
#[component]
pub async fn message_content(
    cx: &Cx,
    #[default]
    #[into]
    variant: Option<MessageContentVariant>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-variant=(variant.map(MessageContentVariant::as_str))
            class=(class!(css::CONTENT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A picture or initials beside a message. It is decorative: the message's name says
/// who is speaking.
///
/// `color` is a CSS colour for the tile behind the initials, such as an agent's accent;
/// pick one with 4.5:1 contrast against `--color-primary-contrast`. `child` replaces
/// the avatar, for a logo. Without `color` the avatar's tint is chosen from the name.
/// `shape: AvatarShape::Square` suits an assistant or a bot.
#[component]
pub async fn message_avatar(
    cx: &Cx,
    name: &str,
    #[default]
    #[into]
    src: Option<&str>,
    #[default] color: &str,
    #[default(AvatarSize::Sm)] size: AvatarSize,
    #[default] shape: AvatarShape,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let style = bitop_ui::merge_style(
        &mut attrs,
        &if color.is_empty() {
            String::new()
        } else {
            format!("--message-avatar-color: {color}")
        },
    );
    Ok(view! {
        <span
            aria-hidden="true"
            data-color=((!color.is_empty()).then_some(""))
            class=(class!(css::AVATAR, attrs.remove("class")))
            style=(style)
            (attrs)
        >
            (child)
            avatar(name: name, src: src, size: size, shape: shape, decorative: true)
        </span>
    })
}

/// A group of actions under a message, named by `label`.
#[component]
pub async fn message_actions(
    cx: &Cx,
    #[default("Message actions")] label: &str,
    #[default] visibility: MessageActionsVisibility,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-visibility=(visibility.as_str())
            class=(class!(css::ACTIONS, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One action: an icon button named by `label`. `child` is the icon.
///
/// - `tooltip` is an ID for a tooltip with `tooltip_text`, or the label.
/// - `pressed` makes it a toggle with that initial state; `exclusive` releases the other
///   exclusive actions of its group when it is pressed; `name` and `value` make a
///   pressed action submit with its form.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn message_action(
    cx: &Cx,
    label: &str,
    #[default] tooltip: &str,
    #[default] tooltip_text: &str,
    #[default]
    #[into]
    pressed: Option<bool>,
    #[default] exclusive: bool,
    #[default] name: &str,
    #[default("on")] value: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
    let tip = if tooltip_text.is_empty() {
        label
    } else {
        tooltip_text
    };
    let toggle = pressed.is_some();
    let is_pressed = pressed.unwrap_or(false);
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            aria-pressed=(pressed.map(|on| if on { "true" } else { "false" }))
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            data-toggle=(toggle.then_some(""))
            data-message-exclusive=(exclusive.then_some(""))
            disabled=(disabled)
            data-variant="ghost"
            data-size="icon"
            class=(class!(button_css::ROOT, css::ACTION, attrs.remove("class")))
            (attrs)
        >
            (child)
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
                (tip)
            </span>
        }
    })
}

/// An action that copies `value`, usually the message's Markdown, with the base set's
/// copy button: a check mark for a moment and a polite announcement.
///
/// `label` names what is copied ("Copy message"); the other strings are the copy
/// button's, in which `{label}` is replaced.
#[component]
pub async fn message_copy_action(
    cx: &Cx,
    value: &str,
    #[default("message")] label: &str,
    #[default] tooltip: &str,
    #[default("Copy {label}")] copy_label: &str,
    #[default("Copied {label}")] copied_label: &str,
    #[default("Copy")] copy_text: &str,
    #[default("Copied")] copied_text: &str,
    #[default("Copied {label} to clipboard")] copied_status: &str,
    #[default("Couldn't copy. Select the text and copy it manually.")] failed_status: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = class!(css::ACTION, attrs.remove("class"));
    attrs.insert(cx, "class", class);
    Ok(view! {
        copy_button(
            value: value,
            label: label,
            tooltip: tooltip,
            variant: ButtonVariant::Ghost,
            copy_label: copy_label,
            copied_label: copied_label,
            copy_text: copy_text,
            copied_text: copied_text,
            copied_status: copied_status,
            failed_status: failed_status,
            attrs: attrs
        )
    })
}

/// Alternative versions of a response, such as after "Regenerate". Put a
/// [`message_branch_content`] holding the versions and a [`message_branch_selector`]
/// inside. `index` is the version shown first, counted from 0.
#[component]
pub async fn message_branch(
    cx: &Cx,
    #[default] index: usize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-message-branch=""
            data-index=(index)
            class=(class!(css::BRANCHES, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The versions: each child element is one. The script shows the current one and
/// hides the rest; before it runs, CSS does the same for the first ten.
#[component]
pub async fn message_branch_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-message-branch-content=""
            class=(class!(css::BRANCH_CONTENT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// "‹ 2 / 3 ›": buttons for the previous and next version, and a polite status. It is
/// hidden when there is one version.
///
/// - `index` and `count` are the version shown first and how many there are; `index`
///   is kept below `count`.
/// - `label` names the group; `previous_label` and `next_label` the buttons;
///   `status_label` is announced, with `{index}` and `{count}`.
#[component]
pub async fn message_branch_selector(
    cx: &Cx,
    index: usize,
    count: usize,
    #[default("Response versions")] label: &str,
    #[default("Previous response")] previous_label: &str,
    #[default("Next response")] next_label: &str,
    #[default("Response {index} of {count}")] status_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let shown = index.min(count.saturating_sub(1));
    let status = status_label
        .replace("{index}", &(shown + 1).to_string())
        .replace("{count}", &count.to_string());
    let page = format!("{} / {count}", shown + 1);
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            hidden=(count < 2)
            data-message-branch-selector=""
            data-status-label=(status_label)
            class=(class!(css::BRANCH, attrs.remove("class")))
            (attrs)
        >
            <button
                type="button"
                aria-label=(previous_label)
                disabled=(shown == 0)
                data-branch-step="-1"
                data-variant="ghost"
                data-size="icon"
                class=(class!(button_css::ROOT, css::ACTION))
            >
                icon(
                    name: Icon::ChevronLeft,
                    attrs: attributes! { class=(css::STEP_ICON) }
                )
            </button>
            <span class=(css::BRANCH_PAGE)>
                <span aria-hidden="true" data-branch-page="">(page)</span>
                <span class="sr-only" aria-live="polite" data-branch-status="">
                    (status)
                </span>
            </span>
            <button
                type="button"
                aria-label=(next_label)
                disabled=(shown + 1 >= count)
                data-branch-step="1"
                data-variant="ghost"
                data-size="icon"
                class=(class!(button_css::ROOT, css::ACTION))
            >
                icon(
                    name: Icon::ChevronRight,
                    attrs: attributes! { class=(css::STEP_ICON) }
                )
            </button>
        </div>
    })
}
