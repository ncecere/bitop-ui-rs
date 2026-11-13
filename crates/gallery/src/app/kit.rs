//! Parts shared by the gallery pages.

use topcoat::{
    Result,
    context::Cx,
    router::request::uri,
    view::{Child, View, component, view},
};

use super::GROUPS;

/// The heading of a component page, taken from its entry in [`GROUPS`].
#[component]
pub async fn page_header(cx: &Cx, slug: &str) -> Result<impl View> {
    let entry = GROUPS
        .iter()
        .flat_map(|group| group.entries)
        .find(|entry| entry.slug == slug)
        .unwrap_or_else(|| panic!("no gallery entry for `{slug}`"));
    Ok(view! {
        <h1>(entry.title)</h1>
        <p
            style="margin-block: var(--space-2) var(--space-6); color: var(--color-text-muted)"
        >
            (entry.summary)
        </p>
    })
}

/// One titled example on a component page.
#[component]
pub async fn example(cx: &Cx, title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section style="margin-block: var(--space-6)">
            <h2
                style="margin-block-end: var(--space-3); color: var(--color-text-muted); font-size: var(--font-size-sm); font-weight: var(--font-weight-medium)"
            >
                (title)
            </h2>
            <div
                style="display: flex; flex-wrap: wrap; gap: var(--space-2) var(--space-4); align-items: center; padding: var(--space-5); border: 1px solid var(--color-border); border-radius: var(--radius-card); background: var(--color-surface)"
            >
                (child)
            </div>
        </section>
    })
}

/// The raw query string of the request, without the `?`.
pub fn query_string(cx: &Cx) -> String {
    uri(cx).query().unwrap_or_default().to_owned()
}

/// The first value of a query parameter that needs no decoding, such as a page
/// number or a column name.
pub fn query_param(cx: &Cx, name: &str) -> Option<String> {
    query_string(cx).split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then(|| value.to_owned())
    })
}

/// A small landscape picture as a data URL, for examples that show an image without
/// a request to another host.
pub const PICTURE: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 40 30'%3E%3Crect width='40' height='30' fill='%234f6bed'/%3E%3Ccircle cx='11' cy='9' r='5' fill='%23fde68a'/%3E%3Cpath d='M0 30 15 15 24 23 30 18 40 26V30z' fill='%23155e75'/%3E%3C/svg%3E";
