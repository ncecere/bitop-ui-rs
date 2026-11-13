//! A collapsible "Used 4 sources" list under an answer.
//!
//! It is a native `<details>`: [`sources_trigger`] is its `<summary>`, which the
//! browser opens and closes on click, Enter and Space and announces as expanded or
//! collapsed. [`sources_content`] holds a numbered list of [`source`] cards.
//!
//! Give each source the same `index` as its `[n]` citation marker so that the two can
//! be matched up. To let a citation chip jump to its source, give the source an `id`
//! and `focusable`, and pass that ID to
//! [`inline_citation`](super::inline_citation::inline_citation)'s `activate_target`: the
//! script opens the list and focuses the card, which shows a focus ring.
//!
//! With `selectable`, the card is a button that opens the source somewhere in the app,
//! such as a side panel: pressing it sends a `bitop:source-select` event with the
//! source's `index` in `detail.index`. Its `href` then becomes a separate link beside
//! it. The button is named by what it shows, so its name contains its visible text;
//! `select_action` is read before that text without being shown ("Show source 2: Leave
//! policy …").
//!
//! ```ignore
//! view! {
//!     sources(
//!         sources_trigger(count: 2)
//!         sources_content(
//!             source(index: 1, title: "Leave policy 2025", href: "https://hr.example.com/leave")
//!             source(index: 2, title: "Benefits FAQ", meta: "p. 12")
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream is Base UI's collapsible, a `<button>` with
//! `aria-expanded`; a `<summary>` is announced the same way, and the panel opens at
//! once where the browser cannot animate a `<details>`. `onSelect` is the
//! `bitop:source-select` event. `title`, `description` and `meta` are text, and the
//! icon is one of the icon set.

use bitop_ui::components::icon::{Icon, icon};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::sources as css;

/// The host of an `http` or `https` URL without a leading `www.`, such as
/// `example.com`; none for anything else.
///
/// ```
/// use bitop_ui_ai::components::sources::hostname;
///
/// assert_eq!(hostname("https://www.Example.com:8080/a?b").as_deref(), Some("example.com"));
/// assert_eq!(hostname("/relative"), None);
/// ```
#[must_use]
pub fn hostname(href: &str) -> Option<String> {
    let lower = href.trim().to_ascii_lowercase();
    let rest = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"))?;
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let host = authority.rsplit('@').next().unwrap_or_default();
    let host = if host.starts_with('[') {
        host.split_inclusive(']').next().unwrap_or_default()
    } else {
        host.split(':').next().unwrap_or_default()
    };
    let host = host.strip_prefix("www.").unwrap_or(host);
    (!host.is_empty()).then(|| host.to_owned())
}

/// The collapsible list: a `<details>`. Put a [`sources_trigger`] first, then
/// [`sources_content`]. `open` is the initial state; `attrs` are forwarded to the
/// `<details>`.
#[component]
pub async fn sources(
    cx: &Cx,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-sources=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>` that opens and closes the list.
///
/// Its text is `label_one` for one source and `label_other` otherwise, with `{count}`
/// replaced; or `label`, when given.
#[component]
pub async fn sources_trigger(
    cx: &Cx,
    count: usize,
    #[default] label: &str,
    #[default("Used {count} source")] label_one: &str,
    #[default("Used {count} sources")] label_other: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let text = if !label.is_empty() {
        label.to_owned()
    } else if count == 1 {
        label_one.replace("{count}", "1")
    } else {
        label_other.replace("{count}", &count.to_string())
    };
    Ok(view! {
        <summary class=(class!(css::TRIGGER, attrs.remove("class"))) (attrs)>
            icon(name: Icon::BookOpen, attrs: attributes! { class=(css::TRIGGER_ICON) })
            <span>(text)</span>
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
        </summary>
    })
}

/// The panel: a numbered list named by `label`, holding [`source`]s.
#[component]
pub async fn sources_content(
    cx: &Cx,
    #[default("Sources")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::PANEL, attrs.remove("class"))) (attrs)>
            <ol aria-label=(label) class=(css::LIST)>(child)</ol>
        </div>
    })
}

/// One source.
///
/// - `title` names it; `href` links to it, in a new tab. Without `href` it is plain
///   text, such as an uploaded file.
/// - `description` is a snippet or summary, cut after two lines.
/// - `meta` is muted text such as `p. 12`; it defaults to the host of `href`.
/// - `index` is its citation number, matching the `[n]` markers.
/// - `icon_name` replaces the document icon.
/// - `id` and `focusable` make it a target that a citation chip can focus.
/// - `selectable` makes the card a button that sends `bitop:source-select`;
///   `select_action` is read before its text, `select_label` replaces its whole name,
///   and `link_label` names the separate link.
/// - `source_label` and `new_tab_label` are read with the number and after a link.
/// - `attrs` are forwarded to the `<li>`.
#[component]
pub async fn source(
    cx: &Cx,
    title: &str,
    #[default] href: &str,
    #[default] description: &str,
    #[default] meta: &str,
    #[default]
    #[into]
    index: Option<usize>,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] id: &str,
    #[default] focusable: bool,
    #[default] selectable: bool,
    #[default] select_action: &str,
    #[default] select_label: &str,
    #[default("Open the link")] link_label: &str,
    #[default("Source")] source_label: &str,
    #[default("(opens in a new tab)")] new_tab_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let meta_text = if meta.is_empty() {
        hostname(href).unwrap_or_default()
    } else {
        meta.to_owned()
    };
    let shown_icon = icon_name.unwrap_or(Icon::FileText);
    let base_id = if id.is_empty() {
        None
    } else {
        Some(id.to_owned())
    };
    let meta_id = base_id.as_ref().map(|id| format!("{id}-meta"));
    let description_id = base_id.as_ref().map(|id| format!("{id}-description"));
    let described_by = (!select_label.is_empty())
        .then(|| {
            [
                (!meta_text.is_empty()).then_some(meta_id.clone()).flatten(),
                (!description.is_empty())
                    .then_some(description_id.clone())
                    .flatten(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" ")
        })
        .filter(|ids| !ids.is_empty());
    let has_href = !href.is_empty();
    let link_in_title = has_href && !selectable;
    let number = index.map(|index| index.to_string());
    let index_value = number.clone();
    let body = view! {
        <span aria-hidden="true" class=(css::ICON)>icon(name: shown_icon)</span>
        <span class=(css::TEXT)>
            <span class=(css::TITLE)>
                if let Some(number) = &number {
                    <span class=(css::INDEX)>
                        <span aria-hidden="true">(number)</span>
                        <span class="sr-only">
                            (source_label)
                            " "
                            (number)
                            ":"
                        </span>
                    </span>
                    " "
                }
                (title)
                if link_in_title {
                    icon(
                        name: Icon::ArrowUpRight,
                        attrs: attributes! { class=(css::EXTERNAL) }
                    )
                    " "
                    <span class="sr-only">(new_tab_label)</span>
                }
            </span>
            if !meta_text.is_empty() {
                <span id=(meta_id) class=(css::META)>(meta_text)</span>
            }
            if !description.is_empty() {
                <span id=(description_id) class=(css::DESCRIPTION)>(description)</span>
            }
        </span>
    };
    Ok(view! {
        <li
            id=(base_id)
            tabindex=(focusable.then_some("-1"))
            data-source=""
            data-index=(index_value)
            data-selectable=(selectable.then_some(""))
            class=(class!(css::SOURCE, attrs.remove("class")))
            (attrs)
        >
            if selectable {
                <button
                    type="button"
                    class=(css::ROW)
                    data-source-select=""
                    aria-label=((!select_label.is_empty()).then_some(select_label))
                    aria-describedby=(described_by)
                >
                    if !select_action.is_empty() && select_label.is_empty() {
                        <span class="sr-only">(select_action)</span>
                        " "
                    }
                    (body)
                </button>
                if has_href {
                    <a
                        href=(href)
                        target="_blank"
                        rel="noreferrer noopener"
                        class=(css::LINK)
                    >
                        icon(name: Icon::ArrowUpRight)
                        <span class="sr-only">
                            (link_label)
                            " "
                            (new_tab_label)
                        </span>
                    </a>
                }
            } else if has_href {
                <a
                    href=(href)
                    target="_blank"
                    rel="noreferrer noopener"
                    class=(css::ROW)
                >
                    (body)
                </a>
            } else {
                <div class=(css::ROW)>(body)</div>
            }
        </li>
    })
}

#[cfg(test)]
mod tests {
    use super::hostname;

    #[test]
    fn hosts() {
        assert_eq!(
            hostname("https://www.example.com/a").as_deref(),
            Some("example.com")
        );
        assert_eq!(
            hostname("http://user@docs.example.org:8443?q").as_deref(),
            Some("docs.example.org")
        );
        assert_eq!(
            hostname("HTTPS://WWW.EXAMPLE.COM").as_deref(),
            Some("example.com")
        );
        assert_eq!(hostname("https://[::1]:80/").as_deref(), Some("[::1]"));
        assert_eq!(hostname("mailto:a@example.com"), None);
        assert_eq!(hostname("javascript:alert(1)"), None);
        assert_eq!(hostname("https:///nothing"), None);
        assert_eq!(hostname(""), None);
    }
}
