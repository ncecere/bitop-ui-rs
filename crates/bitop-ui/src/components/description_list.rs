//! Key and value facts in two shapes.
//!
//! [`description_list`] is a `<dl>` of label and value pairs for detail pages and
//! sheets. The grid layout puts labels in a column beside their values, stacking in a
//! narrow container; the stacked layout puts each label above its value, optionally
//! in several columns.
//!
//! [`facts_line`] is one wrapping line of short facts for the header of a detail
//! page: "Website · Nomic 768 · 58 documents". It is a list, so the count is
//! announced; the "·" separators are CSS and are not read out, and a separator that
//! would start a wrapped line is clipped. Give a fact a `label` when the value alone
//! does not say what it is: it is read before the value and hidden visually unless
//! `show_labels` is set.
//!
//! ```ignore
//! view! {
//!     description_list(
//!         description_item(label: "Start URL", text_link(href: url, (url)))
//!         description_item(label: "Schedule", "Weekly")
//!         description_item(label: "Owner", blank: true)
//!     )
//!     facts_line(
//!         facts_line_item("Website")
//!         facts_line_item(label: "Embedding profile", "Nomic 768")
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream also takes the pairs as an `items` array;
//! here they are always parts. Upstream shows the placeholder when a value is empty;
//! a slot cannot be tested for being empty, so the caller says so with `blank`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::description_list as css;

/// How a [`description_list`] arranges its pairs. Rendered as `data-layout`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptionLayout {
    /// A column of labels beside their values.
    #[default]
    Grid,
    /// Each label above its value.
    Stacked,
}

impl DescriptionLayout {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Grid => "grid",
            Self::Stacked => "stacked",
        }
    }
}

/// The text size of a [`description_list`] or a [`facts_line`]. Rendered as
/// `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptionSize {
    Sm,
    #[default]
    Md,
}

impl DescriptionSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// A `<dl>` of [`description_item`]s.
///
/// - `columns` is how many columns of pairs the stacked layout has, 1 to 4; they wrap
///   to fewer in a narrow container.
/// - `dividers` draws hairlines between the rows of a grid.
#[component]
pub async fn description_list(
    cx: &Cx,
    #[default] layout: DescriptionLayout,
    #[default(1)] columns: u8,
    #[default] size: DescriptionSize,
    #[default] dividers: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let columns = (layout == DescriptionLayout::Stacked).then(|| columns.clamp(1, 4).to_string());
    Ok(view! {
        <dl
            data-layout=(layout.as_str())
            data-columns=(columns)
            data-size=(size.as_str())
            data-dividers=(dividers.then_some(""))
            class=(class!(css::LIST, attrs.remove("class")))
            (attrs)
        >
            (child)
        </dl>
    })
}

/// One pair: a `<div>` holding a `<dt>` and a `<dd>`. `child` is the value.
///
/// `blank` says there is no value: the `<dd>` then shows `empty_text` ("—"), which is
/// announced as `empty_label` ("Not set").
#[component]
pub async fn description_item(
    cx: &Cx,
    label: &str,
    #[default] blank: bool,
    #[default("—")] empty_text: &str,
    #[default("Not set")] empty_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ITEM, attrs.remove("class"))) (attrs)>
            <dt class=(css::TERM)>(label)</dt>
            <dd class=(css::DETAIL) data-empty=(blank.then_some(""))>
                if blank {
                    <span aria-hidden="true">(empty_text)</span>
                    <span class="sr-only">(empty_label)</span>
                } else {
                    (child)
                }
            </dd>
        </div>
    })
}

/// One wrapping line of [`facts_line_item`]s. `attrs` go to the list.
///
/// `show_labels` shows each fact's label ("Profile: Nomic 768"); otherwise labels
/// are only announced.
#[component]
pub async fn facts_line(
    cx: &Cx,
    #[default] show_labels: bool,
    #[default] size: DescriptionSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::FACTS_CLIP, attrs.remove("class")))
            data-size=(size.as_str())
        >
            <ul class=(css::FACTS) data-show-labels=(show_labels.then_some("")) (attrs)>
                (child)
            </ul>
        </div>
    })
}

/// One fact in a [`facts_line`]. `child` is the value.
///
/// - `label` says what the fact is, such as "Embedding profile". It is read before
///   the value, followed by `label_separator`.
/// - `icon` is a decorative icon before the fact.
#[component]
pub async fn facts_line_item(
    cx: &Cx,
    #[default] label: &str,
    #[default(": ")] label_separator: &str,
    #[default]
    #[into]
    icon: Option<Icon>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let name = icon;
    Ok(view! {
        <li class=(class!(css::FACT, attrs.remove("class"))) (attrs)>
            if let Some(name) = name {
                <span aria-hidden="true" class=(css::FACT_ICON)>icon(name: name)</span>
            }
            if !label.is_empty() {
                <span class=(css::FACT_LABEL)>
                    (label)
                    (label_separator)
                </span>
            }
            <span class=(css::FACT_VALUE)>(child)</span>
        </li>
    })
}
