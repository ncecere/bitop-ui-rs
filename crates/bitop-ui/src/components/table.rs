//! A data table with real `<th scope="col">` headers and a caption.
//!
//! ```ignore
//! view! {
//!     table(caption: "Documents",
//!         columns: view! {
//!             table_column("Name")
//!             table_column(numeric: true, "Size")
//!             table_column()
//!         },
//!         tr(
//!             th("handbook.pdf")
//!             td(numeric: true, "1.2 MB")
//!             td(table_actions(button(size: ButtonSize::Sm, "Open")))
//!         )
//!     )
//! }
//! ```
//!
//! A [`table_column`] with no content becomes an "Actions" header for assistive
//! technology only.
//!
//! Whenever the table overflows its wrapper (too wide for a narrow screen, or taller
//! than `max_height`), the library script makes the wrapper a focusable region named
//! by the caption, so that it can be scrolled from the keyboard. A table that fits is
//! not a tab stop. While columns are hidden to one side, that edge shows a shadow.
//!
//! - `sticky_end` on the table, on a column and on that column's cells pins the
//!   column to the end edge while the rest scrolls under it, for row actions.
//! - [`table_empty`] is a full-width row for an empty state, centred on the visible
//!   width of the scroll box.
//! - `stack`: below 600 pixels each row becomes a block, the first cell on its own
//!   line and the others under it, each with its column's name. The script copies
//!   the names from the header into `data-label`; a cell that already has a
//!   `data-label` keeps it.
//!
//! Differences from upstream: the columns are parts in a slot, not an array, so the
//! empty row takes its column count as an argument. `striped` is an addition.
//! Without a script the wrapper still scrolls, but it is not made focusable, and a
//! stacked table has no column names unless the cells carry `data-label`.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::table as css;

/// The row height of a [`table`]. Rendered as `data-density`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableDensity {
    #[default]
    Comfortable,
    Compact,
}

impl TableDensity {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Compact => "compact",
        }
    }
}

/// How a column is sorted. Rendered as `aria-sort`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableSort {
    Ascending,
    Descending,
    /// Sortable, and not the sorted column.
    None,
    Other,
}

impl TableSort {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
            Self::None => "none",
            Self::Other => "other",
        }
    }
}

/// The table. `child` is its rows ([`tr`]); `attrs` go to the `<table>`.
///
/// - `caption` describes the table for assistive technology, and names the scroll
///   region. `show_caption` also shows it.
/// - `columns` is a slot for the header cells: [`table_column`]s.
/// - `sticky_header` keeps the header in view while the body scrolls; set
///   `max_height` too, a CSS length such as `"24rem"`.
/// - `density` is the row height.
/// - `striped` tints every other row.
/// - `framed` wraps the table in a card-like surface.
/// - `stack` shows each row as a block on a narrow screen.
/// - `sticky_end` says a column is pinned to the end edge.
#[component]
pub async fn table(
    cx: &Cx,
    caption: &str,
    #[default] show_caption: bool,
    #[into] columns: Child<'_>,
    #[default] sticky_header: bool,
    #[default] max_height: &str,
    #[default] density: TableDensity,
    #[default] striped: bool,
    #[default] framed: bool,
    #[default] stack: bool,
    #[default] sticky_end: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let wrap_style = (!max_height.is_empty()).then(|| format!("max-height: {max_height}"));
    Ok(view! {
        <div
            class=(css::FRAME)
            data-table-frame=""
            data-framed=(framed.then_some(""))
            data-sticky-end=(sticky_end.then_some(""))
        >
            <div
                class=(css::WRAP)
                data-table-wrap=""
                data-label=(caption)
                data-scroll=((!max_height.is_empty()).then_some(""))
                style=(wrap_style)
            >
                <table
                    data-density=(density.as_str())
                    data-sticky=(sticky_header.then_some(""))
                    data-striped=(striped.then_some(""))
                    data-stack=(stack.then_some(""))
                    class=(class!(css::TABLE, attrs.remove("class")))
                    (attrs)
                >
                    <caption
                        class=(if show_caption { css::CAPTION } else { "sr-only" })
                    >
                        (caption)
                    </caption>
                    <thead><tr>(columns)</tr></thead>
                    <tbody>(child)</tbody>
                </table>
            </div>
            <span aria-hidden="true" class=(css::EDGE) data-side="start"></span>
            <span aria-hidden="true" class=(css::EDGE) data-side="end"></span>
        </div>
    })
}

/// A column header: `<th scope="col">`. `child` is its label.
///
/// - `blank` says there is no label: the header is then `actions_label`, for
///   assistive technology only. Use it for a column of row actions.
/// - `hide_label` hides the label visually; it is still announced.
/// - `numeric` aligns the header to the end, over a column of numbers.
/// - `width` is a CSS width for the column.
/// - `sort` sets `aria-sort`; use it on the sorted column.
/// - `sticky_end` pins the column to the end edge; mark its cells too.
#[component]
pub async fn table_column(
    cx: &Cx,
    #[default] blank: bool,
    #[default("Actions")] actions_label: &str,
    #[default] hide_label: bool,
    #[default] numeric: bool,
    #[default] width: &str,
    #[default]
    #[into]
    sort: Option<TableSort>,
    #[default] sticky_end: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let style = if width.is_empty() {
        crate::merge_style(&mut attrs, "")
    } else {
        crate::merge_style(&mut attrs, &format!("width: {width}"))
    };
    Ok(view! {
        <th
            scope="col"
            aria-sort=(sort.map(TableSort::as_str))
            data-numeric=(numeric.then_some(""))
            data-sticky-end=(sticky_end.then_some(""))
            style=(style)
            class=(class!(css::TH, attrs.remove("class")))
            (attrs)
        >
            if blank {
                <span class="sr-only">(actions_label)</span>
            } else if hide_label {
                <span class="sr-only">(child)</span>
            } else {
                (child)
            }
        </th>
    })
}

/// A row. `selected` tints it.
#[component]
pub async fn tr(
    cx: &Cx,
    #[default] selected: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <tr
            data-selected=(selected.then_some(""))
            class=(class!(css::TR, attrs.remove("class")))
            (attrs)
        >
            (child)
        </tr>
    })
}

/// A cell.
///
/// - `numeric` aligns to the end with tabular figures.
/// - `muted` is for secondary text.
/// - `nowrap` keeps the content on one line.
/// - `sticky_end` marks a cell of a pinned column.
#[component]
pub async fn td(
    cx: &Cx,
    #[default] numeric: bool,
    #[default] muted: bool,
    #[default] nowrap: bool,
    #[default] sticky_end: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <td
            data-numeric=(numeric.then_some(""))
            data-muted=(muted.then_some(""))
            data-nowrap=(nowrap.then_some(""))
            data-sticky-end=(sticky_end.then_some(""))
            class=(class!(css::TD, attrs.remove("class")))
            (attrs)
        >
            (child)
        </td>
    })
}

/// A row header cell, `<th scope="row">`: the name column, for example.
#[component]
pub async fn th(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <th scope="row" class=(class!(css::ROW_HEADER, attrs.remove("class"))) (attrs)>
            (child)
        </th>
    })
}

/// A container for a row's action buttons, aligned to the end.
#[component]
pub async fn table_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A full-width row, in place of the rows of a table with nothing in it. `child` is
/// usually an [`empty_state`](super::empty_state::empty_state). `columns` is the
/// number of columns of the table.
#[component]
pub async fn table_empty(
    cx: &Cx,
    columns: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <tr class=(class!(css::EMPTY_ROW, attrs.remove("class"))) (attrs)>
            <td colspan=(columns.max(1).to_string()) class=(css::EMPTY_CELL)>
                <div class=(css::EMPTY_CONTENT)>(child)</div>
            </td>
        </tr>
    })
}
