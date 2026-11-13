//! Tabs that switch between panels on one page, and nav tabs that link between pages,
//! ported from bitop-ui's `Tabs` and `NavTabs`.
//!
//! The server renders every panel and marks one as selected. The library script adds
//! the ARIA tabs keyboard pattern with manual activation, as Base UI's tabs have by
//! default: the arrow keys (wrapping at the ends), Home and End move focus between
//! tabs, and Enter, Space or a click selects the focused tab and shows its panel.
//! Only one tab is in the tab order: the selected one, then the last one focused.
//! Disabled tabs are skipped.
//!
//! The list is underlined ([`TabsVariant::Underline`]) or a segmented control of
//! pills ([`TabsVariant::Pills`]); the script slides an indicator under the selected
//! tab. Too many tabs for the width wrap onto more lines (pills' default) or scroll
//! sideways with a fade at the edge where tabs are hidden (underline's default).
//!
//! A tab and its panel are paired by the panel's ID: the tab gets the ID
//! `{panel}-tab`. Use [`nav_tabs`] when each tab is a page: it is a `<nav>` of links
//! that marks the current one with `aria-current="page"`.
//!
//! ```ignore
//! view! {
//!     tabs(
//!         tab_list(label: "Project views",
//!             tab(panel: "deployments", selected: true, count: 42, "Deployments")
//!             tab(panel: "domains", icon: Icon::Globe, "Domains")
//!         )
//!         tab_panel(id: "deployments", selected: true, "...")
//!         tab_panel(id: "domains", "...")
//!     )
//!     nav_tabs(label: "Project sections",
//!         nav_tab(href: "/overview", current: true, "Overview")
//!         nav_tab(href: "/domains", count: 3, "Domains")
//!     )
//! }
//! ```
//!
//! Differences from upstream: tabs are paired with panels by ID, not by `value`, and
//! `onValueChange` is the `bitop:change` event on the list, whose `detail` is the
//! selected panel's ID. Counts are grouped the English way ("1,234").

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

// The `icon` part under another name: the tabs' `icon` arguments hide it.
use super::icon::{Icon, icon as icon_part};
use crate::css::tabs as css;

/// The look of a [`tab_list`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    /// Tabs on a line, the selected one underlined.
    #[default]
    Underline,
    /// A segmented control, such as "24h / 7d / 30d".
    Pills,
}

impl TabsVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Underline => "underline",
            Self::Pills => "pills",
        }
    }
}

/// What a [`tab_list`] does with more tabs than fit. Rendered as
/// `data-overflow-mode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsOverflow {
    /// Every tab stays visible, on more lines.
    Wrap,
    /// The list scrolls sideways, and fades at the edge where tabs are hidden.
    Scroll,
}

impl TabsOverflow {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wrap => "wrap",
            Self::Scroll => "scroll",
        }
    }
}

/// Writes a count with a comma between thousands, as `toLocaleString` does in English.
fn group_thousands(count: u64) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// The wrapper around a [`tab_list`] and its [`tab_panel`]s.
#[component]
pub async fn tabs(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ROOT, attrs.remove("class"))) data-tabs="" (attrs)>
            (child)
        </div>
    })
}

/// The row of tabs. `label` names the set for assistive technology.
///
/// `overflow` is what happens to tabs that do not fit; by default pills wrap and
/// underlined tabs scroll.
#[component]
pub async fn tab_list(
    cx: &Cx,
    label: &str,
    #[default] variant: TabsVariant,
    #[default]
    #[into]
    overflow: Option<TabsOverflow>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let overflow = overflow.unwrap_or(match variant {
        TabsVariant::Pills => TabsOverflow::Wrap,
        TabsVariant::Underline => TabsOverflow::Scroll,
    });
    Ok(view! {
        <div
            role="tablist"
            aria-label=(label)
            data-variant=(variant.as_str())
            data-overflow-mode=(overflow.as_str())
            class=(class!(css::LIST, attrs.remove("class")))
            (attrs)
        >
            (child)
            <span class=(css::INDICATOR) data-variant=(variant.as_str())></span>
        </div>
    })
}

/// A tab that shows the [`tab_panel`] whose ID is `panel`; `child` is its label.
///
/// - `icon` is a decorative icon before the label.
/// - `count` is a small number after the label, read as part of the name.
/// - `disabled` keeps the tab in view but out of reach.
#[component]
pub async fn tab(
    cx: &Cx,
    panel: &str,
    #[default] selected: bool,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default]
    #[into]
    count: Option<u64>,
    #[default] disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let count = count.map(group_thousands);
    // Only the selected tab is in the tab order; arrow keys reach the others.
    Ok(view! {
        <button
            type="button"
            role="tab"
            id=(format!("{panel}-tab"))
            aria-controls=(panel)
            aria-selected=(if selected { "true" } else { "false" })
            aria-disabled=(disabled.then_some("true"))
            tabindex=(if selected { "0" } else { "-1" })
            class=(class!(css::TAB, attrs.remove("class")))
            (attrs)
        >
            if let Some(name) = icon {
                icon_part(name: name)
            }
            (child)
            if let Some(count) = count {
                " "
                <span class=(css::COUNT)>(count)</span>
            }
        </button>
    })
}

/// The content for one tab. Hidden unless `selected`.
#[component]
pub async fn tab_panel(
    cx: &Cx,
    id: &str,
    #[default] selected: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="tabpanel"
            id=(id)
            aria-labelledby=(format!("{id}-tab"))
            tabindex="0"
            if !selected {
                hidden=(true)
            }
            class=(class!(css::PANEL, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A bar of links between pages that looks like underlined tabs: a `<nav>` named
/// `label`, holding [`nav_tab`]s.
#[component]
pub async fn nav_tabs(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav aria-label=(label) class=(class!(css::NAV, attrs.remove("class"))) (attrs)>
            <ul class=(css::NAV_LIST)>(child)</ul>
        </nav>
    })
}

/// A link in [`nav_tabs`]; `child` is its label.
///
/// - `current` marks the page that is shown, with `aria-current="page"`.
/// - `icon` and `count` are as on a [`tab`].
/// - `attrs` are forwarded to the `<a>`.
#[component]
pub async fn nav_tab(
    cx: &Cx,
    href: &str,
    #[default] current: bool,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default]
    #[into]
    count: Option<u64>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let count = count.map(group_thousands);
    Ok(view! {
        <li class=(css::NAV_ITEM)>
            <a
                href=(href)
                aria-current=(current.then_some("page"))
                class=(class!(css::NAV_TAB, attrs.remove("class")))
                (attrs)
            >
                if let Some(name) = icon {
                    icon_part(name: name)
                }
                (child)
                if let Some(count) = count {
                    " "
                    <span class=(css::COUNT)>(count)</span>
                }
            </a>
        </li>
    })
}

#[cfg(test)]
mod tests {
    use super::group_thousands;

    #[test]
    fn groups_thousands() {
        assert_eq!(group_thousands(0), "0");
        assert_eq!(group_thousands(42), "42");
        assert_eq!(group_thousands(999), "999");
        assert_eq!(group_thousands(1000), "1,000");
        assert_eq!(group_thousands(1_234_567), "1,234,567");
    }
}
