//! The scrolling message log of a chat, which keeps the latest message in view.
//!
//! **Sticking to the bottom.** While the reader is at the bottom, content that grows
//! (a streamed answer, an image loading) keeps the view on the latest message.
//! Scrolling up by wheel, touch, keyboard or scroll bar releases it, and
//! [`conversation_scroll_button`] appears; reaching the bottom again sticks it once more.
//! The library script does this with a `MutationObserver` and a `ResizeObserver` on
//! the log, so it works however the content arrives: rendered by the server, streamed
//! into the page as HTML, or added by a script.
//!
//! **Screen readers.** The log has `role="log"` but `aria-live="off"`, because a polite
//! log would read every streamed token. Render one [`conversation_announcer`] with the
//! chat's status: when its `data-status` changes, it says "Message sent", "Assistant is
//! responding" or "Response complete", and the reader reads the answer at their own
//! pace. The log takes focus, so that the arrow keys scroll it.
//!
//! Scripts can call `bitopUi.conversation.scrollToBottom(log, "smooth")` and
//! `bitopUi.conversation.scrollToElement(log, element, { offset })`; the second stops
//! following, to show an answer that arrived whole from its start.
//!
//! ```ignore
//! view! {
//!     conversation(
//!         conversation_content(
//!             message(from: MessageRole::User, message_content("Hi!"))
//!         )
//!         conversation_scroll_button()
//!     )
//!     conversation_announcer(status: ChatStatus::Streaming)
//! }
//! ```
//!
//! Differences from upstream: `useConversation` and `useStickToBottom` are the script
//! functions above. The announcer follows its `data-status` attribute instead of a
//! prop, so that an app that streams HTML can change it in place. The scroll button
//! has a tooltip only when given an ID, as the base set's tooltips do.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::{button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::{ChatStatus, css::conversation as css};

/// What the log's `aria-live` is. Keep it off and use [`conversation_announcer`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConversationLive {
    #[default]
    Off,
    Polite,
}

impl ConversationLive {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Polite => "polite",
        }
    }
}

/// The element of an empty state's title.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyTitle {
    #[default]
    Paragraph,
    H1,
    H2,
    H3,
}

/// The conversation: a positioned box around the scrolling log.
///
/// - `label` names the log.
/// - `live` is the log's `aria-live`.
/// - `threshold` is how many pixels from the bottom still count as at the bottom.
/// - `stick_to_bottom` follows new content; pass `false` while an empty state shows,
///   so that a tall welcome starts at its top. Setting `data-stick` on the log back to
///   `true` sticks it to the bottom once.
/// - `attrs` are forwarded to the box; `viewport_attrs` to the log.
#[component]
pub async fn conversation(
    cx: &Cx,
    #[default("Conversation")] label: &str,
    #[default] live: ConversationLive,
    #[default(64)] threshold: u32,
    #[default(true)] stick_to_bottom: bool,
    #[default] mut viewport_attrs: Attributes,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::ROOT, attrs.remove("class")))
            data-conversation=""
            (attrs)
        >
            <div
                role="log"
                aria-live=(live.as_str())
                aria-label=(label)
                tabindex="0"
                data-conversation-viewport=""
                data-threshold=(threshold)
                data-stick=(if stick_to_bottom { "true" } else { "false" })
                class=(class!(css::VIEWPORT, viewport_attrs.remove("class")))
                (viewport_attrs)
            >
                (child)
            </div>
        </div>
    })
}

/// The growing column of messages.
#[component]
pub async fn conversation_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::CONTENT, attrs.remove("class")))
            data-conversation-content=""
            (attrs)
        >
            (child)
        </div>
    })
}

/// What to show before the first message: a greeting, and usually suggestions as
/// `child`.
///
/// - `title` is the greeting, in a `title_as` element.
/// - `description` is a sentence under it.
/// - `icon_name` is a decorative icon in a tinted tile; `media` replaces it, for an
///   assistant's avatar.
#[component]
pub async fn conversation_empty_state(
    cx: &Cx,
    #[default("How can I help?")] title: &str,
    #[default] description: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default]
    #[into]
    media: Child<'_>,
    #[default] title_as: EmptyTitle,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::EMPTY, attrs.remove("class")))
            data-conversation-empty=""
            (attrs)
        >
            // The media slot is always rendered; CSS hides it when empty, and the icon
            // when it is not.
            <span aria-hidden="true" class=(css::EMPTY_MEDIA)>(media)</span>
            if let Some(name) = icon_name {
                <span aria-hidden="true" class=(css::EMPTY_ICON)>icon(name: name)</span>
            }
            match title_as {
                EmptyTitle::Paragraph => {
                    <p class=(css::EMPTY_TITLE)>(title)</p>
                }
                EmptyTitle::H1 => {
                    <h1 class=(css::EMPTY_TITLE)>(title)</h1>
                }
                EmptyTitle::H2 => {
                    <h2 class=(css::EMPTY_TITLE)>(title)</h2>
                }
                EmptyTitle::H3 => {
                    <h3 class=(css::EMPTY_TITLE)>(title)</h3>
                }
            }
            if !description.is_empty() {
                <p class=(css::EMPTY_DESCRIPTION)>(description)</p>
            }
            <div class=(css::EMPTY_CHILDREN)>(child)</div>
        </div>
    })
}

/// A button that appears when the reader has scrolled away from the latest message
/// and brings it back. Put it inside [`conversation`]. `tooltip` is an ID for a
/// tooltip with the label.
#[component]
pub async fn conversation_scroll_button(
    cx: &Cx,
    #[default("Scroll to latest message")] label: &str,
    #[default] tooltip: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
    Ok(view! {
        <span
            class=(class!(css::SCROLL_BUTTON, attrs.remove("class")))
            data-conversation-scroll=""
            hidden=(true)
            (attrs)
        >
            <button
                type="button"
                aria-label=(label)
                aria-describedby=(has_tooltip.then_some(tooltip))
                data-tooltip=(has_tooltip.then_some(tooltip))
                data-variant="outline"
                data-size="icon"
                class=(class!(button_css::ROOT, css::SCROLL))
            >
                icon(
                    name: Icon::ArrowDown,
                    attrs: attributes! { class=(css::SCROLL_ICON) }
                )
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
        </span>
    })
}

/// A visually hidden polite status that announces the chat's transitions, so that the
/// log can stay silent while tokens stream in.
///
/// `status` is the chat's status when the page is rendered. Change the element's
/// `data-status` to announce: `submitted`, `streaming` and `error` say their message,
/// and `ready` after `submitted` or `streaming` says `complete`. An empty message
/// stays silent. The same message twice in a row is announced again.
#[component]
pub async fn conversation_announcer(
    cx: &Cx,
    #[default] status: ChatStatus,
    #[default("Message sent")] submitted: &str,
    #[default("Assistant is responding")] streaming: &str,
    #[default("Response complete")] complete: &str,
    #[default("The response failed")] error: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            role="status"
            data-conversation-announcer=""
            data-status=(status.as_str())
            data-say-submitted=(submitted)
            data-say-streaming=(streaming)
            data-say-complete=(complete)
            data-say-error=(error)
            class=(class!("sr-only", attrs.remove("class")))
            (attrs)
        ></span>
    })
}
