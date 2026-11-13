//! A menu of links that open a prompt in other chat products, each in a new tab.
//!
//! It is the base set's [`menu`]: the trigger opens it, the arrow keys move through
//! the items, and each item is a link built by [`ChatProvider::url`] from the prompt.
//!
//! ```ignore
//! view! {
//!     open_in_chat(id: "open-in", query: "Explain this stack trace: …",
//!         providers: &[ChatProvider::CHATGPT, ChatProvider::CLAUDE,
//!             ChatProvider::custom("campus", "Open in Campus AI",
//!                 "https://ai.example.edu/new?q={query}")])
//! }
//! ```
//!
//! Differences from upstream: a provider's URL is a template or a built-in format, not
//! a function. There is no `onOpen` callback; a script can listen for clicks on the
//! links. The trigger is always the default button, and the built-in icon is the only
//! one, as upstream ships no brand marks either.
//!
//! [`menu`]: bitop_ui::components::menu::menu

use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
    menu::{menu, menu_group, menu_item, menu_trigger},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::open_in_chat as css;

/// How a [`ChatProvider`] puts the prompt in its URL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChatUrl<'a> {
    /// A base URL and query parameters, the prompt going in the parameter whose value
    /// is `{query}`; encoded as a browser encodes a form (`URLSearchParams`).
    Params(&'a str, &'a [(&'a str, &'a str)]),
    /// A template in which `{query}` is replaced by the prompt, encoded as a URL
    /// component (`encodeURIComponent`).
    Template(&'a str),
}

/// A chat product that a prompt can be opened in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChatProvider<'a> {
    /// Identifies it.
    pub id: &'a str,
    /// The item's text, such as "Open in Claude".
    pub title: &'a str,
    /// How the URL is built.
    pub url: ChatUrl<'a>,
}

impl<'a> ChatProvider<'a> {
    /// ChatGPT, with web search hinted.
    pub const CHATGPT: ChatProvider<'static> = ChatProvider {
        id: "chatgpt",
        title: "Open in ChatGPT",
        url: ChatUrl::Params(
            "https://chatgpt.com/",
            &[("hints", "search"), ("prompt", "{query}")],
        ),
    };
    /// Claude.
    pub const CLAUDE: ChatProvider<'static> = ChatProvider {
        id: "claude",
        title: "Open in Claude",
        url: ChatUrl::Params("https://claude.ai/new", &[("q", "{query}")]),
    };
    /// v0.
    pub const V0: ChatProvider<'static> = ChatProvider {
        id: "v0",
        title: "Open in v0",
        url: ChatUrl::Params("https://v0.app/", &[("q", "{query}")]),
    };
    /// Cursor.
    pub const CURSOR: ChatProvider<'static> = ChatProvider {
        id: "cursor",
        title: "Open in Cursor",
        url: ChatUrl::Params("https://cursor.com/link/prompt", &[("text", "{query}")]),
    };
    /// T3 Chat.
    pub const T3: ChatProvider<'static> = ChatProvider {
        id: "t3",
        title: "Open in T3 Chat",
        url: ChatUrl::Params("https://t3.chat/new", &[("q", "{query}")]),
    };
    /// Scira.
    pub const SCIRA: ChatProvider<'static> = ChatProvider {
        id: "scira",
        title: "Open in Scira",
        url: ChatUrl::Params("https://scira.ai/", &[("q", "{query}")]),
    };
    /// The providers listed when none are given: ChatGPT, Claude and v0.
    pub const DEFAULTS: [ChatProvider<'static>; 3] = [Self::CHATGPT, Self::CLAUDE, Self::V0];

    /// A provider of one's own, with a URL template in which `{query}` is replaced.
    #[must_use]
    pub const fn custom(id: &'a str, title: &'a str, template: &'a str) -> Self {
        Self {
            id,
            title,
            url: ChatUrl::Template(template),
        }
    }

    /// The same provider with another item text, for another language.
    #[must_use]
    pub const fn titled(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// The URL that opens `query` in this provider.
    ///
    /// ```
    /// use bitop_ui_ai::components::open_in_chat::ChatProvider;
    ///
    /// assert_eq!(ChatProvider::CLAUDE.url("Hi & bye"), "https://claude.ai/new?q=Hi+%26+bye");
    /// assert_eq!(
    ///     ChatProvider::custom("x", "X", "https://x.example/?q={query}").url("a b"),
    ///     "https://x.example/?q=a%20b",
    /// );
    /// ```
    #[must_use]
    pub fn url(&self, query: &str) -> String {
        match self.url {
            ChatUrl::Params(base, params) => {
                let pairs: Vec<String> = params
                    .iter()
                    .map(|(key, value)| {
                        let value = if *value == "{query}" { query } else { value };
                        format!("{}={}", form_encode(key), form_encode(value))
                    })
                    .collect();
                format!("{base}?{}", pairs.join("&"))
            }
            ChatUrl::Template(template) => {
                template.replace("{query}", &encode_uri_component(query))
            }
        }
    }
}

/// Encodes text as `application/x-www-form-urlencoded`, as `URLSearchParams` does:
/// letters, digits and `*-._` stay, a space is `+`, and every other byte is `%XX`.
#[must_use]
pub fn form_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_' => {
                out.push(char::from(byte));
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Encodes text as a URL component, as `encodeURIComponent` does: letters, digits and
/// `-_.!~*'()` stay, and every other byte is `%XX`.
#[must_use]
pub fn encode_uri_component(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => out.push(char::from(byte)),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// The menu.
///
/// - `id` goes on the menu; the trigger points at it.
/// - `query` is the prompt to open.
/// - `providers` are listed in order; [`ChatProvider::DEFAULTS`] when none are given.
///   Pass `no_providers` to list only `child`.
/// - `label` is the trigger's text; `group_label` an optional heading in the menu.
/// - `new_tab_label` is read after each item.
/// - `child` is extra items, such as [`open_in_chat_item`]s.
#[component]
pub async fn open_in_chat(
    cx: &Cx,
    id: &str,
    query: &str,
    #[default] providers: &[ChatProvider<'_>],
    #[default] no_providers: bool,
    #[default("Open in chat")] label: &str,
    #[default] group_label: &str,
    #[default("(opens in a new tab)")] new_tab_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let list: &[ChatProvider<'_>] = if no_providers {
        &[]
    } else if providers.is_empty() {
        &ChatProvider::DEFAULTS
    } else {
        providers
    };
    let items: Vec<(String, String)> = list
        .iter()
        .map(|provider| (provider.url(query), provider.title.to_owned()))
        .collect();
    let grouped = !group_label.is_empty();
    let class = class!(css::POPUP, attrs.remove("class"));
    attrs.insert(cx, "class", class);
    Ok(Child::new(view! {
        menu_trigger(
            target: id,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Sm,
            attrs: attributes! { data-open-in-chat="" },
            (label)
            icon(name: Icon::ChevronDown)
        )
        menu(
            id: id,
            attrs: attrs,
            if grouped {
                menu_group(
                    label: group_label,
                    for (href, title) in &items {
                        open_in_chat_link(
                            href: href,
                            title: title,
                            new_tab_label: new_tab_label
                        )
                    }
                    (child)
                )
            } else {
                for (href, title) in &items {
                    open_in_chat_link(
                        href: href,
                        title: title,
                        new_tab_label: new_tab_label
                    )
                }
                (child)
            }
        )
    }))
}

/// One provider's link, for extra items in an [`open_in_chat`] menu. `query` is the
/// prompt.
#[component]
pub async fn open_in_chat_item(
    cx: &Cx,
    provider: ChatProvider<'_>,
    query: &str,
    #[default("(opens in a new tab)")] new_tab_label: &str,
) -> Result<impl View> {
    let href = provider.url(query);
    Ok(view! {
        open_in_chat_link(
            href: &href,
            title: provider.title,
            new_tab_label: new_tab_label
        )
    })
}

#[component]
async fn open_in_chat_link(
    cx: &Cx,
    href: &str,
    title: &str,
    new_tab_label: &str,
) -> Result<impl View> {
    Ok(view! {
        menu_item(
            href: href,
            attrs: attributes! {
                target="_blank"
                rel="noopener noreferrer"
                class=(css::ITEM)
                data-open-in-chat-item=""
            },
            <span class=(css::ICON)>icon(name: Icon::MessageSquare)</span>
            <span class=(css::TITLE)>(title)</span>
            <span class="sr-only">
                " "
                (new_tab_label)
            </span>
            icon(name: Icon::ExternalLink, attrs: attributes! { class=(css::EXTERNAL) })
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{ChatProvider, encode_uri_component, form_encode};

    #[test]
    fn built_in_urls() {
        let query = "Explain: a+b = c? 100% & ünïcode/✓";
        assert_eq!(
            ChatProvider::CHATGPT.url(query),
            "https://chatgpt.com/?hints=search&prompt=Explain%3A+a%2Bb+%3D+c%3F+100%25+%26+%C3%BCn%C3%AFcode%2F%E2%9C%93"
        );
        assert_eq!(
            ChatProvider::CLAUDE.url("hello world"),
            "https://claude.ai/new?q=hello+world"
        );
        assert_eq!(ChatProvider::V0.url("x"), "https://v0.app/?q=x");
        assert_eq!(
            ChatProvider::CURSOR.url("a\nb"),
            "https://cursor.com/link/prompt?text=a%0Ab"
        );
        assert_eq!(ChatProvider::T3.url(""), "https://t3.chat/new?q=");
        assert_eq!(ChatProvider::SCIRA.url("*-._"), "https://scira.ai/?q=*-._");
    }

    #[test]
    fn custom_urls_and_encoding() {
        let campus = ChatProvider::custom(
            "campus",
            "Open in Campus AI",
            "https://ai.example.edu/new?q={query}&src=docs",
        );
        assert_eq!(
            campus.url("a b&c/d"),
            "https://ai.example.edu/new?q=a%20b%26c%2Fd&src=docs"
        );
        assert_eq!(form_encode("~!'()"), "%7E%21%27%28%29");
        assert_eq!(encode_uri_component("~!'() é"), "~!'()%20%C3%A9");
    }
}
