//! The top of a page: its title, with an optional description, breadcrumbs, status
//! badges, key facts and actions.
//!
//! The title is the page's `<h1>` unless `level` says otherwise. The actions sit at
//! the end of the title's row and stay level with the title however long the
//! description is; on a narrow screen they wrap under it.
//!
//! Differences from upstream: the title and the description are text, not nodes. The
//! other four are named slots, always rendered and hidden while empty.
//!
//! ```ignore
//! view! {
//!     page_header(
//!         title: "Deployments",
//!         description: "Every release of this project, newest first.",
//!         breadcrumbs: view! {
//!             breadcrumbs(
//!                 breadcrumb(href: "/", label: "Acme")
//!                 breadcrumb(label: "Deployments")
//!             )
//!         },
//!         meta: view! { badge("Live") },
//!         actions: view! { button("Deploy") },
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::page_header as css;

/// The heading level of a [`page_header`]'s title.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageHeaderLevel {
    #[default]
    H1,
    /// For a header inside another page, such as a panel with its own title.
    H2,
    H3,
}

/// A page header.
///
/// - `title` is the heading; `level` is its heading level and `title_id` its ID, for
///   an `aria-labelledby` elsewhere.
/// - `description` is one or two sentences under the title.
/// - `breadcrumbs` is a slot above the title, for
///   [`breadcrumbs`](super::breadcrumbs::breadcrumbs).
/// - `meta` is a slot beside the title, for status badges.
/// - `facts` is a slot under the description, for a line of key facts.
/// - `actions` is a slot at the end of the title's row, for buttons.
#[component]
pub async fn page_header(
    cx: &Cx,
    title: &str,
    #[default] title_id: &str,
    #[default] level: PageHeaderLevel,
    #[default] description: &str,
    #[default]
    #[into]
    breadcrumbs: Child<'_>,
    #[default]
    #[into]
    meta: Child<'_>,
    #[default]
    #[into]
    facts: Child<'_>,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let title_id = (!title_id.is_empty()).then_some(title_id);
    Ok(view! {
        <div
            class=(class!(css::HEADER, attrs.remove("class")))
            data-page-header=""
            (attrs)
        >
            <div class=(css::BREADCRUMBS)>(breadcrumbs)</div>
            <div class=(css::ROW)>
                <div class=(css::HEADING)>
                    <div class=(css::TITLE_ROW)>
                        match level {
                            PageHeaderLevel::H1 => {
                                <h1 id=(title_id) class=(css::TITLE)>(title)</h1>
                            }
                            PageHeaderLevel::H2 => {
                                <h2 id=(title_id) class=(css::TITLE)>(title)</h2>
                            }
                            PageHeaderLevel::H3 => {
                                <h3 id=(title_id) class=(css::TITLE)>(title)</h3>
                            }
                        }
                        <div class=(css::META)>(meta)</div>
                    </div>
                    if !description.is_empty() {
                        <p class=(css::DESCRIPTION)>(description)</p>
                    }
                    <div class=(css::FACTS)>(facts)</div>
                </div>
                <div class=(css::ACTIONS)>(actions)</div>
            </div>
        </div>
    })
}
