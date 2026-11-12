//! Tabs that switch between panels on one page.
//!
//! The server renders every panel and marks one as selected. The library script
//! switches panels on click and adds the ARIA tabs keyboard pattern: arrow keys, Home
//! and End move between tabs and select them.
//!
//! A tab and its panel are paired by the panel's ID: the tab gets the ID
//! `{panel}-tab`.
//!
//! ```ignore
//! view! {
//!     tabs(
//!         tab_list(label: "Settings",
//!             tab(panel: "general", selected: true, "General")
//!             tab(panel: "models", "Models")
//!         )
//!         tab_panel(id: "general", selected: true, "...")
//!         tab_panel(id: "models", "...")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::tabs as css;

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
#[component]
pub async fn tab_list(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="tablist"
            aria-label=(label)
            class=(class!(css::LIST, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A tab that shows the [`tab_panel`] whose ID is `panel`.
#[component]
pub async fn tab(
    cx: &Cx,
    panel: &str,
    #[default] selected: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // Only the selected tab is in the tab order; arrow keys reach the others.
    Ok(view! {
        <button
            type="button"
            role="tab"
            id=(format!("{panel}-tab"))
            aria-controls=(panel)
            aria-selected=(if selected { "true" } else { "false" })
            tabindex=(if selected { "0" } else { "-1" })
            class=(class!(css::TAB, attrs.remove("class")))
            (attrs)
        >
            (child)
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
