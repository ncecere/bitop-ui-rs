//! Parts shared by the gallery pages.

use topcoat::{
    Result,
    context::Cx,
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
