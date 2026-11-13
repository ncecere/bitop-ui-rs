//! Flex layout primitives with gaps from the spacing scale.
//!
//! - [`stack`] lays its children out in a column.
//! - [`inline`] lays them out in a row that wraps.
//! - [`container`] is the centred page column, 1200 pixels wide at most.
//!
//! The gap is a step of the spacing scale, 0 to 12 (`gap: 4` is `--space-4`), rendered
//! as `data-gap`. Alignment is `data-align` and `data-justify`. Both follow the reading
//! direction: in a right-to-left page a row starts on the right.
//!
//! Differences from upstream: upstream's `render` prop turns a stack into any element;
//! here `list: true` renders a `<ul>`, for a stack or row of `<li>`, and anything else
//! is a `<div>`. The gap is a `data-gap` attribute, not an inline style.
//!
//! ```ignore
//! view! {
//!     container(
//!         stack(gap: 6,
//!             <h1>"Settings"</h1>
//!             inline(gap: 2, justify: FlexJustify::End,
//!                 button(variant: ButtonVariant::Ghost, "Cancel")
//!                 button("Save")
//!             )
//!         )
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::layout as css;

/// How the children of a [`stack`] or an [`inline`] line up across its direction.
/// Rendered as `data-align`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl FlexAlign {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Stretch => "stretch",
            Self::Baseline => "baseline",
        }
    }
}

/// How the children of a [`stack`] or an [`inline`] are spread along its direction.
/// Rendered as `data-justify`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    Between,
}

impl FlexJustify {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "between",
        }
    }
}

/// A column of children with a gap between them.
///
/// `gap` is a step of the spacing scale, 0 to 12; without it the gap is `--space-4`.
/// `list` renders a `<ul>` for `<li>` children.
#[component]
pub async fn stack(
    cx: &Cx,
    #[default]
    #[into]
    gap: Option<u8>,
    #[default]
    #[into]
    align: Option<FlexAlign>,
    #[default]
    #[into]
    justify: Option<FlexJustify>,
    #[default] list: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::STACK, attrs.remove("class"));
    let align = align.map(FlexAlign::as_str);
    let justify = justify.map(FlexJustify::as_str);
    Ok(view! {
        if list {
            <ul
                class=(class)
                data-gap=(gap)
                data-align=(align)
                data-justify=(justify)
                (attrs)
            >
                (child)
            </ul>
        } else {
            <div
                class=(class)
                data-gap=(gap)
                data-align=(align)
                data-justify=(justify)
                (attrs)
            >
                (child)
            </div>
        }
    })
}

/// A row of children that wraps onto more lines, centred across the row.
///
/// `gap` is a step of the spacing scale, 0 to 12; without it the gap is `--space-2`.
/// `nowrap` keeps the row on one line. `list` renders a `<ul>` for `<li>` children.
#[component]
pub async fn inline(
    cx: &Cx,
    #[default]
    #[into]
    gap: Option<u8>,
    #[default]
    #[into]
    align: Option<FlexAlign>,
    #[default]
    #[into]
    justify: Option<FlexJustify>,
    #[default] nowrap: bool,
    #[default] list: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(css::INLINE, attrs.remove("class"));
    let align = align.map(FlexAlign::as_str);
    let justify = justify.map(FlexJustify::as_str);
    let nowrap = nowrap.then_some("");
    Ok(view! {
        if list {
            <ul
                class=(class)
                data-gap=(gap)
                data-align=(align)
                data-justify=(justify)
                data-nowrap=(nowrap)
                (attrs)
            >
                (child)
            </ul>
        } else {
            <div
                class=(class)
                data-gap=(gap)
                data-align=(align)
                data-justify=(justify)
                data-nowrap=(nowrap)
                (attrs)
            >
                (child)
            </div>
        }
    })
}

/// The greatest width of a [`container`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContainerSize {
    /// 56rem, for a page of text or a form.
    Md,
    /// The content width, 1200 pixels.
    #[default]
    Lg,
    /// No limit.
    Full,
}

impl ContainerSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Full => "full",
        }
    }
}

/// The centred page column, with side padding that grows on a wide screen.
#[component]
pub async fn container(
    cx: &Cx,
    #[default] size: ContainerSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(css::CONTAINER, attrs.remove("class")))
            data-size=(size.as_str())
            (attrs)
        >
            (child)
        </div>
    })
}
