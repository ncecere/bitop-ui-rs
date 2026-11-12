//! A link in running text.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::text_link as css;

/// An inline link.
///
/// - `muted` is for secondary links in dense layouts.
/// - `external` opens a new tab, adds an icon, and tells screen readers so.
#[component]
pub async fn text_link(
    cx: &Cx,
    href: &str,
    #[default] muted: bool,
    #[default] external: bool,
    #[default("(opens in a new tab)")] external_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            if external {
                target="_blank"
                rel="noreferrer noopener"
                data-external=""
            }
            data-tone=(if muted { "muted" } else { "default" })
            class=(class!(css::LINK, attrs.remove("class")))
            (attrs)
        >
            (child)
            if external {
                icon(name: Icon::ArrowUpRight, attrs: attributes! { class=(css::ICON) })
                <span class="sr-only">
                    " "
                    (external_label)
                </span>
            }
        </a>
    })
}
