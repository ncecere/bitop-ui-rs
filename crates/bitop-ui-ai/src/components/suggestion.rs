//! Prompt starters, as pill buttons in a labelled group.
//!
//! Each suggestion is a real button named by its text. What pressing one does depends
//! on its arguments:
//!
//! - With `name`, it is a submit button: the form around it is sent with
//!   `name=suggestion`, as a normal HTML form, with no script.
//! - With `target`, the ID of a text box such as the prompt input's textarea, the
//!   library script puts the suggestion in that box and focuses it; with `send` it
//!   then submits the box's form.
//! - Either way the script sends a `bitop:suggestion` event from the button, with the
//!   text in `detail.suggestion`, for pages that handle it themselves.
//!
//! ```ignore
//! view! {
//!     suggestions(
//!         suggestion(suggestion: "What is our leave policy?", target: &textarea_id, send: true)
//!         suggestion(suggestion: "Summarise this page", name: "message")
//!     )
//! }
//! ```
//!
//! Differences from upstream: `onSelect` is the `bitop:suggestion` event, or one of the
//! two built-in actions above. `layout` scroll fades the end of the row in the reading
//! direction, so it also works right to left.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::button as button_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::suggestion as css;

/// How a [`suggestions`] group lays out its chips. Rendered as `data-layout`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SuggestionsLayout {
    /// Several lines; a long suggestion wraps inside its chip.
    #[default]
    Wrap,
    /// One row that scrolls sideways, for a narrow composer.
    Scroll,
}

impl SuggestionsLayout {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wrap => "wrap",
            Self::Scroll => "scroll",
        }
    }
}

/// Where wrapped rows of a [`suggestions`] group line up. Rendered as `data-align`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SuggestionsAlign {
    /// Centred, under a centred greeting.
    #[default]
    Center,
    /// From the start, under text that starts there.
    Start,
}

impl SuggestionsAlign {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Center => "center",
            Self::Start => "start",
        }
    }
}

/// A group of [`suggestion`]s, named by `label`. `attrs` are forwarded to the group.
#[component]
pub async fn suggestions(
    cx: &Cx,
    #[default] layout: SuggestionsLayout,
    #[default] align: SuggestionsAlign,
    #[default("Suggestions")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-layout=(layout.as_str())
            data-align=(align.as_str())
            class=(class!(css::SUGGESTIONS, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One suggestion.
///
/// - `suggestion` is the prompt text: what is sent or put in the text box, and the
///   button's text unless `text` gives a shorter one.
/// - `name` makes it a submit button that sends `name=suggestion` with its form.
/// - `target` is the ID of a text box that the script fills with the suggestion;
///   `send` also submits that box's form.
/// - `icon_name` is a decorative icon before the text.
/// - `attrs` are forwarded to the `<button>`.
#[component]
pub async fn suggestion(
    cx: &Cx,
    suggestion: &str,
    #[default] name: &str,
    #[default] target: &str,
    #[default] send: bool,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] text: &str,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let submits = !name.is_empty();
    let shown = if text.is_empty() { suggestion } else { text };
    Ok(view! {
        <button
            type=(if submits { "submit" } else { "button" })
            name=(submits.then_some(name))
            value=(submits.then_some(suggestion))
            disabled=(disabled)
            data-suggestion=(suggestion)
            data-suggestion-target=((!target.is_empty()).then_some(target))
            data-suggestion-send=(send.then_some(""))
            data-variant="secondary"
            data-size="sm"
            class=(class!(button_css::ROOT, css::SUGGESTION, attrs.remove("class")))
            (attrs)
        >
            if let Some(name) = icon_name {
                icon(name: name)
            }
            <span class=(css::TEXT)>(shown)</span>
        </button>
    })
}
