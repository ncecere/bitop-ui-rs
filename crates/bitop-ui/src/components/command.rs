//! A command palette, ported from bitop-ui's `CommandPalette`: a dialog with a search
//! box over grouped actions and links, and a search-field-looking trigger.
//!
//! It is a modal `<dialog>`, so the browser supplies the focus trap, Escape and focus
//! return; the library script supplies filtering, ranking and the keys. Every word of
//! the query must appear in an item's label or keywords, anywhere in a word, in the
//! singular or the plural ("members" finds "Add member", "policies" finds "Policy").
//! While typing, the best matches come first, and their group with them: a label that
//! is the query, then labels that start with it, then labels with a word starting
//! with each typed word, then a keyword that is the query, then any other match. A
//! group with `keep_order` keeps its items' order. The first item is always
//! highlighted, so Enter runs the command that was named. Choosing an item closes the
//! palette, then follows its link or clicks its button.
//!
//! Open it with [`command_palette_trigger`], any
//! [`dialog_trigger`](super::dialog::dialog_trigger), or a `shortcut` such as `"k"`,
//! which toggles it with Ctrl+K, or Command+K on a Mac.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     command_palette_trigger(target: &id, label: "Search or jump to…")
//!     command_palette(id: &id, shortcut: "k",
//!         command_group(label: "Jump to",
//!             command_item(href: "/projects", icon: Icon::ArrowRight, hint: "Page", "Projects")
//!         )
//!         command_group(label: "Actions",
//!             command_item(icon: Icon::Moon, shortcut: &["shift", "D"], "Toggle dark mode")
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: commands are server-rendered links and buttons rather
//! than `onSelect` callbacks (a button's click is the command); `open` is the dialog's
//! state and `onOpenChange` its `close` event; there is no controlled `query` or
//! `finalFocus`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    icon::{Icon, icon},
    kbd::{KbdSize, kbd_shortcut},
};
use crate::css::{command as css, dialog as dialog_css};

/// The palette.
///
/// - `label` names the dialog and its search box.
/// - `shortcut` is one key that toggles the palette when pressed with Ctrl or Command.
/// - `empty` is shown when nothing matches.
/// - `close_label` names the Esc button; `hint` describes the keys to screen readers;
///   `navigate_label`, `select_label` and `dismiss_label` are the footer's words.
#[component]
pub async fn command_palette(
    cx: &Cx,
    id: &str,
    #[default("Command palette")] label: &str,
    #[default("Search pages and actions…")] placeholder: &str,
    #[default("No results found.")] empty: &str,
    #[default] shortcut: &str,
    #[default("Esc — Close command palette")] close_label: &str,
    #[default(
        "Use the up and down arrow keys to move between results and Enter to run the highlighted one."
    )]
    hint: &str,
    #[default("to navigate")] navigate_label: &str,
    #[default("to select")] select_label: &str,
    #[default("to close")] dismiss_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let list = format!("{id}-list");
    let hint_id = format!("{id}-hint");
    Ok(Child::new(view! {
        <dialog
            id=(id)
            aria-label=(label)
            closedby="any"
            data-command-palette=""
            data-shortcut=((!shortcut.is_empty()).then_some(shortcut))
            class=(class!(dialog_css::POPUP, css::POPUP, attrs.remove("class")))
            (attrs)
        >
            <div class=(css::INPUT_ROW)>
                icon(
                    name: Icon::Search,
                    attrs: attributes! { class=(css::SEARCH_ICON) }
                )
                <input
                    type="text"
                    role="combobox"
                    aria-label=(label)
                    aria-controls=(&list)
                    aria-expanded="true"
                    aria-haspopup="listbox"
                    aria-autocomplete="list"
                    aria-describedby=(&hint_id)
                    autocomplete="off"
                    spellcheck="false"
                    placeholder=(placeholder)
                    class=(css::INPUT)
                    data-command-input=""
                >
                <button
                    type="button"
                    commandfor=(id)
                    command="close"
                    aria-label=(close_label)
                    class=(css::ESC)
                >
                    "Esc"
                </button>
            </div>
            <div class=(css::RESULTS)>
                <div
                    role="status"
                    aria-live="polite"
                    aria-atomic="true"
                    class=(css::EMPTY)
                    data-empty=(empty)
                    data-command-empty=""
                ></div>
                <div id=(&list) role="listbox" aria-label=(label) class=(css::LIST)>
                    (child)
                </div>
            </div>
            <div class=(css::FOOTER)>
                <span id=(&hint_id) class="sr-only">(hint)</span>
                <span aria-hidden="true" class=(css::FOOTER_HINT)>
                    kbd_shortcut(keys: &["up"], size: KbdSize::Sm)
                    kbd_shortcut(keys: &["down"], size: KbdSize::Sm)
                    (navigate_label)
                </span>
                <span aria-hidden="true" class=(css::FOOTER_HINT)>
                    kbd_shortcut(keys: &["enter"], size: KbdSize::Sm)
                    (select_label)
                </span>
                <span
                    aria-hidden="true"
                    class=(class!(css::FOOTER_HINT, css::FOOTER_END))
                >
                    kbd_shortcut(keys: &["esc"], size: KbdSize::Sm)
                    (dismiss_label)
                </span>
            </div>
        </dialog>
    }))
}

/// A labelled set of [`command_item`]s. It hides when none of its items match.
/// `keep_order` keeps its items in the given order while typing, for results a search
/// already ranked; the group still moves up when it holds the best match.
#[component]
pub async fn command_group(
    cx: &Cx,
    label: &str,
    #[default] keep_order: bool,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-keep-order=(keep_order.then_some(""))
            class=(css::GROUP)
        >
            <div class=(css::GROUP_LABEL) aria-hidden="true">(label)</div>
            (child)
        </div>
    })
}

/// One command: a link when `href` is given, otherwise a button whose click is the
/// command.
///
/// - `keywords` are extra words, separated by commas, that the search matches but
///   does not show, such as "invoices" for Billing.
/// - `icon` is a decorative icon, `hint` muted text at the end ("Page"), and
///   `shortcut` the keys of a shortcut hint, such as `&["G", "S"]`.
/// - `child` is the label.
#[component]
pub async fn command_item(
    cx: &Cx,
    #[default]
    #[into]
    href: Option<&str>,
    #[default] keywords: &str,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default] hint: &str,
    #[default] shortcut: &[&str],
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::ITEM, attrs.remove("class"));
    let leading = icon;
    let content = view! {
        if let Some(name) = leading {
            <span class=(css::ITEM_ICON)>icon(name: name)</span>
        }
        <span class=(css::ITEM_LABEL) data-command-label="">(child)</span>
        if !hint.is_empty() {
            <span class=(css::ITEM_HINT)>(hint)</span>
        }
        if !shortcut.is_empty() {
            <span aria-hidden="true" class=(css::ITEM_SHORTCUT)>
                kbd_shortcut(keys: shortcut, size: KbdSize::Sm)
            </span>
        }
    };
    // Focus stays in the search box, so items leave the tab order.
    Ok(view! {
        if let Some(href) = href {
            <a
                href=(href)
                role="option"
                aria-selected="false"
                tabindex="-1"
                data-keywords=(keywords)
                class=(class)
                (attrs)
            >
                (content)
            </a>
        } else {
            <button
                type="button"
                role="option"
                aria-selected="false"
                tabindex="-1"
                data-keywords=(keywords)
                class=(class)
                (attrs)
            >
                (content)
            </button>
        }
    })
}

/// A search-field-looking button for a top bar that opens the palette whose ID is
/// `target`: a search icon, `label`, and the shortcut hint `keys` (Ctrl K, or ⌘ K on a
/// Mac). On a narrow screen it is an icon button.
#[component]
pub async fn command_palette_trigger(
    cx: &Cx,
    target: &str,
    #[default("Search…")] label: &str,
    #[default(&["mod", "K"])] keys: &[&str],
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            commandfor=(target)
            command="show-modal"
            aria-haspopup="dialog"
            aria-keyshortcuts="Meta+K Control+K"
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            icon(name: Icon::Search, attrs: attributes! { class=(css::TRIGGER_ICON) })
            <span class=(css::TRIGGER_LABEL)>(label)</span>
            <span aria-hidden="true" class=(css::TRIGGER_KBD)>
                kbd_shortcut(keys: keys, size: KbdSize::Sm)
            </span>
        </button>
    })
}
