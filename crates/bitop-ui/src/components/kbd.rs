//! A key cap, and a key combination.
//!
//! ```ignore
//! view! { kbd_shortcut(keys: &["mod", "K"]) }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::kbd as css;

/// The size of a key cap. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdSize {
    Sm,
    #[default]
    Md,
}

impl KbdSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A single key cap.
#[component]
pub async fn kbd(
    cx: &Cx,
    #[default] size: KbdSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <kbd
            data-size=(size.as_str())
            class=(class!(css::KBD, attrs.remove("class")))
            (attrs)
        >
            (child)
        </kbd>
    })
}

/// What a named key shows, and what a screen reader says for it.
fn symbol(key: &str) -> Option<(&'static str, &'static str)> {
    Some(match key.to_ascii_lowercase().as_str() {
        "mod" => ("Ctrl", "Command or Control"),
        "shift" => ("Shift", "Shift"),
        "alt" => ("Alt", "Option or Alt"),
        "enter" => ("Enter", "Enter"),
        "esc" => ("Esc", "Escape"),
        "up" => ("↑", "Up arrow"),
        "down" => ("↓", "Down arrow"),
        _ => return None,
    })
}

/// A key combination, such as `["mod", "K"]`.
///
/// The names `mod`, `shift`, `alt`, `enter`, `esc`, `up` and `down` are spelled out for
/// screen readers. The server cannot know the platform, so it renders Ctrl, Shift, Alt
/// and Enter, and the library script swaps in ⌘, ⇧, ⌥ and ↵ on Apple devices.
#[component]
pub async fn kbd_shortcut(
    cx: &Cx,
    keys: &[&str],
    #[default] size: KbdSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::SHORTCUT, attrs.remove("class"))) (attrs)>
            for key in keys {
                <kbd data-size=(size.as_str()) class=(css::KBD)>
                    if let Some((shown, spoken)) = symbol(key) {
                        <span
                            aria-hidden="true"
                            data-kbd-key=(key.to_ascii_lowercase())
                        >
                            (shown)
                        </span>
                        <span class="sr-only">(spoken)</span>
                    } else {
                        (*key)
                    }
                </kbd>
            }
        </span>
    })
}
