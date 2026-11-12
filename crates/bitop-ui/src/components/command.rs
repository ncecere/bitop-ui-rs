//! A command palette: a dialog with a search box over a list of actions and links.
//!
//! It is a [`dialog`](super::dialog) that holds a combobox, so the browser supplies the
//! focus trap and Escape, and the library script supplies filtering and arrow keys.
//! Choosing an item follows its link or runs its button, then closes the palette.
//!
//! Open it with a [`dialog_trigger`](super::dialog::dialog_trigger), or give it a
//! `shortcut` such as `"k"` to open it with Ctrl+K, or Command+K on a Mac.
//!
//! ```ignore
//! let id = element_id(cx);
//! view! {
//!     dialog_trigger(target: &id, "Search")
//!     command_palette(id: &id, label: "Command palette", shortcut: "k",
//!         command_group(label: "Go to",
//!             command_item(href: "/", "New chat")
//!             command_item(href: "/settings", "Settings")
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::{combobox as combobox_css, command as css, dialog as dialog_css};

/// The palette.
///
/// - `label` names the dialog and its list for assistive technology.
/// - `shortcut` is one key that opens the palette when pressed with Ctrl or Command.
/// - `empty` is shown when nothing matches.
#[component]
pub async fn command_palette(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default("Search…")] placeholder: &str,
    #[default("No results")] empty: &str,
    #[default] shortcut: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let list = format!("{id}-list");
    Ok(view! {
        <dialog
            id=(id)
            aria-label=(label)
            closedby="any"
            data-combobox=""
            data-command-palette=""
            if !shortcut.is_empty() {
                data-shortcut=(shortcut)
            }
            class=(class!(dialog_css::POPUP, css::POPUP, attrs.remove("class")))
            (attrs)
        >
            <div class=(css::SEARCH)>
                <input
                    type="text"
                    role="combobox"
                    aria-label=(label)
                    aria-controls=(&list)
                    aria-expanded="true"
                    aria-autocomplete="list"
                    autocomplete="off"
                    spellcheck="false"
                    placeholder=(placeholder)
                    class=(css::INPUT)
                >
            </div>
            <div id=(&list) role="listbox" aria-label=(label) class=(css::LIST)>
                (child)
            </div>
            <div class=(combobox_css::EMPTY) data-combobox-empty="" hidden=(true)>
                (empty)
            </div>
        </dialog>
    })
}

/// A labelled set of [`command_item`]s. It hides when none of its items match.
#[component]
pub async fn command_group(cx: &Cx, label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div role="group" aria-label=(label) class=(css::GROUP)>
            <div class=(css::GROUP_LABEL) aria-hidden="true">(label)</div>
            (child)
        </div>
    })
}

/// One entry in the palette: a link when `href` is given, otherwise a button.
///
/// `keywords` are extra words the search matches but does not show.
#[component]
pub async fn command_item(
    cx: &Cx,
    #[default]
    #[into]
    href: Option<&str>,
    #[default] keywords: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(combobox_css::OPTION, attrs.remove("class"));
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
                (child)
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
                (child)
            </button>
        }
    })
}
