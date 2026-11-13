//! A flexible row with media, a title, a description and actions, for settings
//! lists, file lists, people pickers and link lists.
//!
//! ```ignore
//! view! {
//!     item_group(label: "Integrations",
//!         item(in_group: true, variant: ItemVariant::Outline,
//!             item_media(variant: ItemMediaVariant::Icon, icon(name: Icon::Inbox))
//!             item_content(
//!                 item_title("GitHub")
//!                 item_description("Deploy on every push.")
//!             )
//!             item_actions(button(size: ButtonSize::Sm, "Connect"))
//!         )
//!     )
//! }
//! ```
//!
//! A whole row can be a link: give [`item`] an `href`. Do not put other interactive
//! controls inside a link item.
//!
//! Differences from upstream: upstream's `Item` learns from React context that it is
//! inside an `ItemGroup` and wraps itself in a list item. Here the caller says so with
//! `in_group`, on [`item`] and on [`item_separator`]. Upstream's `render` prop, which
//! swaps the element, is `href` here: the row is a `<div>` or an `<a>`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::item as css;

/// The surface of an [`item`]. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
            Self::Muted => "muted",
        }
    }
}

/// The size of an [`item`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemSize {
    #[default]
    Md,
    Sm,
    Xs,
}

impl ItemSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Md => "md",
            Self::Sm => "sm",
            Self::Xs => "xs",
        }
    }
}

/// What an [`item_media`] holds. Rendered as `data-variant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    /// An icon in a tinted square.
    Icon,
    /// A cropped `<img>`: an avatar or a thumbnail.
    Image,
}

impl ItemMediaVariant {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
            Self::Image => "image",
        }
    }
}

/// A list of items (`role="list"`). `label` names it when the context does not.
/// Give every [`item`] and [`item_separator`] inside it `in_group: true`.
#[component]
pub async fn item_group(
    cx: &Cx,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="list"
            aria-label=((!label.is_empty()).then_some(label))
            class=(class!(css::GROUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One row.
///
/// - `href` makes the whole row a link.
/// - `in_group` wraps the row in a list item, for a row inside an [`item_group`].
///   `attrs` still go to the row itself.
#[component]
pub async fn item(
    cx: &Cx,
    #[default] variant: ItemVariant,
    #[default] size: ItemSize,
    #[default] href: &str,
    #[default] in_group: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::ITEM, attrs.remove("class"));
    let row = view! {
        if href.is_empty() {
            <div
                class=(class)
                data-variant=(variant.as_str())
                data-size=(size.as_str())
                (attrs)
            >
                (child)
            </div>
        } else {
            <a
                href=(href)
                class=(class)
                data-variant=(variant.as_str())
                data-size=(size.as_str())
                (attrs)
            >
                (child)
            </a>
        }
    };
    Ok(view! {
        if in_group {
            <div role="listitem" class=(css::LIST_ITEM)>(row)</div>
        } else {
            (row)
        }
    })
}

/// The leading media of an [`item`]: an icon, an avatar or a thumbnail.
#[component]
pub async fn item_media(
    cx: &Cx,
    #[default] variant: ItemMediaVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-variant=(variant.as_str())
            class=(class!(css::MEDIA, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The text column of an [`item`]: an [`item_title`] and an [`item_description`].
#[component]
pub async fn item_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The title of an [`item`].
#[component]
pub async fn item_title(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::TITLE, attrs.remove("class")))
            data-item-title=""
            (attrs)
        >
            (child)
        </div>
    })
}

/// The description under an [`item_title`], clamped to two lines.
#[component]
pub async fn item_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            class=(class!(css::DESCRIPTION, attrs.remove("class")))
            data-item-description=""
            (attrs)
        >
            (child)
        </p>
    })
}

/// Buttons or other controls at the end of an [`item`].
#[component]
pub async fn item_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A full-width row above the item's main line, such as a cover image or a label.
#[component]
pub async fn item_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A full-width row below the item's main line, such as metadata.
#[component]
pub async fn item_footer(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::FOOTER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A decorative hairline between items, hidden from assistive technology.
/// `in_group` wraps it in a presentational list item, as a list may hold only those.
#[component]
pub async fn item_separator(
    cx: &Cx,
    #[default] in_group: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = class!(css::SEPARATOR, attrs.remove("class"));
    Ok(view! {
        if in_group {
            <div role="listitem" aria-hidden="true" class=(css::SEPARATOR_ITEM)>
                <div aria-hidden="true" class=(class) (attrs)></div>
            </div>
        } else {
            <div aria-hidden="true" class=(class) (attrs)></div>
        }
    })
}
