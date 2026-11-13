//! A data grid on top of [`table`](super::table): sortable headers, row selection
//! with bulk actions, a columns menu, a footer for paging, and loading and empty
//! states.
//!
//! The server sorts, filters and pages; this part renders the result and the
//! controls that ask for another. Rows are the caller's, built from the table parts:
//!
//! ```ignore
//! let columns = [
//!     DataTableColumn::new("number", "Invoice").sortable(),
//!     DataTableColumn::new("amount", "Amount").sortable().numeric(),
//! ];
//! view! {
//!     <form method="post" action="/invoices/bulk">
//!         data_table(
//!             id: "invoices", caption: "Invoices", columns: &columns,
//!             sort: sort, sort_href: "/invoices?sort={column}&dir={direction}",
//!             selectable: true, columns_menu: true,
//!             bulk_actions: view! { button(attrs: attributes! { type="submit" }, "Delete") },
//!             footer: view! { paginator(page: 2, page_count: 5, href: "/invoices?page={page}") },
//!             for invoice in invoices {
//!                 tr(
//!                     data_table_select(value: &invoice.id, label: &invoice.number)
//!                     th((invoice.number))
//!                     td(numeric: true, (invoice.amount))
//!                 )
//!             }
//!         )
//!     </form>
//! }
//! ```
//!
//! - **Sorting.** A sortable header is a link, and the sorted column has `aria-sort`.
//!   `sort_href` is the URL of a sort, with `{column}` and `{direction}`; the links
//!   cycle ascending, descending, then `sort_clear_href`. [`next_sort`] is the cycle.
//! - **Selection.** [`data_table_select`] is a cell with a checkbox named after its
//!   row; the checkboxes have a `name`, so the selection submits with the form around
//!   the table. The header checkbox selects every row and shows a mixed state. While
//!   rows are selected a bar shows the count, the `bulk_actions` slot and "Clear
//!   selection". The bar appears without a script; the script keeps the count, the
//!   header checkbox and the rows' tint in step.
//! - **Columns.** `columns_menu` adds a menu of checkbox items that show and hide
//!   columns in the browser. `columns_storage_key` remembers the choice in
//!   `localStorage`. A column with `hidden()` starts hidden.
//! - **States.** `loading` marks the table busy and dims the rows it has.
//!   [`data_table_skeleton`] is rows of placeholders, [`data_table_message`] a line
//!   such as "No rows.", and [`table_empty`](super::table::table_empty) holds an
//!   empty state. The `error` slot sits above the table, for an
//!   [`error_alert`](super::alert::error_alert).
//! - **Rows that open.** Put a [`data_table_row_link`] in the row header and
//!   `data-clickable` on the row: a click anywhere on the row that is not on a
//!   control follows the link.
//!
//! Differences from upstream: upstream sorts, filters and pages an array in the
//! browser and takes cell renderers; here the server does the work and the caller
//! writes the rows. Sorting, paging, retrying and loading more are links, not click
//! handlers. The text filter and the facets are slots (`search`, `filters`) for the
//! caller's own form or a [`filter_bar`](super::filter_bar). Selection is not kept
//! across pages, since a page shows only its own rows. Columns cannot start hidden
//! on narrow screens only. When a focused control goes away, focus is not moved for
//! it, except that "Clear selection" moves focus to the header checkbox.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant},
    icon::{Icon, icon},
    menu::{menu, menu_checkbox_item, menu_group, menu_trigger},
    popover::PopupAlign,
    skeleton::{SkeletonShape, skeleton},
    table::{TableDensity, table},
};
use crate::css::{
    button as button_css, checkbox as checkbox_css, data_table as css, table as table_css,
};

/// The direction of a sort.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    /// The value of `aria-sort`: `ascending` or `descending`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }

    /// The short form for a URL: `asc` or `desc`.
    #[must_use]
    pub fn as_param(self) -> &'static str {
        match self {
            Self::Ascending => "asc",
            Self::Descending => "desc",
        }
    }

    /// Reads `asc`, `ascending`, `desc` or `descending`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "asc" | "ascending" => Some(Self::Ascending),
            "desc" | "descending" => Some(Self::Descending),
            _ => None,
        }
    }
}

/// The sort of a [`data_table`]: a column and a direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTableSort<'a> {
    /// The ID of the sorted column.
    pub column: &'a str,
    /// Which way it is sorted.
    pub direction: SortDirection,
}

/// The sort that a click on the header of `column` asks for: ascending, then
/// descending, then none.
#[must_use]
pub fn next_sort<'a>(
    current: Option<DataTableSort<'_>>,
    column: &'a str,
) -> Option<DataTableSort<'a>> {
    match current {
        Some(sort) if sort.column == column => match sort.direction {
            SortDirection::Ascending => Some(DataTableSort {
                column,
                direction: SortDirection::Descending,
            }),
            SortDirection::Descending => None,
        },
        _ => Some(DataTableSort {
            column,
            direction: SortDirection::Ascending,
        }),
    }
}

/// The URL of a sort: `pattern` with `{column}` and `{direction}` (`asc` or `desc`)
/// filled in. No sort gives `clear_href`, or the ascending URL of `column` when that
/// is empty, so that the header always leads somewhere.
#[must_use]
pub fn sort_url(
    pattern: &str,
    clear_href: &str,
    column: &str,
    sort: Option<DataTableSort<'_>>,
) -> String {
    let fill = |column: &str, direction: SortDirection| {
        pattern
            .replace("{column}", column)
            .replace("{direction}", direction.as_param())
    };
    match sort {
        Some(sort) => fill(sort.column, sort.direction),
        None if clear_href.is_empty() => fill(column, SortDirection::Ascending),
        None => clear_href.to_owned(),
    }
}

/// The first and the last row on `page` (counted from 1) of `total` rows, for a
/// summary such as "Rows 11–20 of 42". No rows gives `(0, 0)`.
#[must_use]
pub fn page_rows(page: u32, page_size: u32, total: u64) -> (u64, u64) {
    if total == 0 || page_size == 0 {
        return (0, 0);
    }
    let size = u64::from(page_size);
    let pages = total.div_ceil(size);
    let page = u64::from(page).clamp(1, pages);
    ((page - 1) * size + 1, (page * size).min(total))
}

/// One column of a [`data_table`]: `DataTableColumn::new("amount", "Amount")`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTableColumn<'a> {
    /// The column's ID, used in sort URLs and by the columns menu.
    pub id: &'a str,
    /// The header text. It also names the column in the columns menu.
    pub label: &'a str,
    /// Whether the header is a sort link.
    pub sortable: bool,
    /// Aligns the header to the end, over numbers.
    pub numeric: bool,
    /// Hides the header text visually; it is still announced.
    pub hide_label: bool,
    /// A CSS width for the column.
    pub width: &'a str,
    /// Whether the columns menu offers the column.
    pub hideable: bool,
    /// Whether the column starts hidden.
    pub hidden: bool,
}

impl<'a> DataTableColumn<'a> {
    /// A column that is not sortable and can be hidden.
    #[must_use]
    pub fn new(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            sortable: false,
            numeric: false,
            hide_label: false,
            width: "",
            hideable: true,
            hidden: false,
        }
    }

    /// Makes the header a sort link.
    #[must_use]
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// Aligns the header to the end.
    #[must_use]
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    /// Hides the header text visually.
    #[must_use]
    pub fn hide_label(mut self) -> Self {
        self.hide_label = true;
        self
    }

    /// Sets a CSS width.
    #[must_use]
    pub fn width(mut self, width: &'a str) -> Self {
        self.width = width;
        self
    }

    /// Keeps the column out of the columns menu: the name column, usually.
    #[must_use]
    pub fn fixed(mut self) -> Self {
        self.hideable = false;
        self
    }

    /// Starts the column hidden.
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }
}

/// What [`data_table`] renders for one header cell.
struct Head {
    id: String,
    label: String,
    href: Option<String>,
    active: Option<SortDirection>,
    aria_sort: Option<&'static str>,
    numeric: bool,
    hide_label: bool,
    style: Option<String>,
    hidden: bool,
}

/// What [`data_table`] renders for one item of the columns menu.
struct Toggle {
    id: String,
    label: String,
    checked: bool,
    only: bool,
}

/// The data table. `child` is its rows; `attrs` go to the root element.
///
/// - `id` is the root's ID; the columns menu's ID is made from it.
/// - `caption` and `columns` describe the table. `show_caption`, `sticky_header`,
///   `max_height`, `density`, `striped`, `framed` and `stack` are passed to
///   [`table`].
/// - `sort` is the current sort, and `sort_href` and `sort_clear_href` the URLs; see
///   the module documentation.
/// - `selectable` adds the header checkbox and the bulk bar; put a
///   [`data_table_select`] first in each row. `row_count` is the number of rows in
///   all pages, for "2 of 42 selected".
/// - `row_actions` adds a last column, pinned to the end, with a hidden header; put
///   a `td(sticky_end: true)` last in each row.
/// - `columns_menu` adds the columns menu, and `columns_storage_key` remembers it.
/// - `search`, `toolbar` and `filters` are slots above the table: a search form, extra
///   buttons, and a filter bar.
/// - `bulk_actions` is a slot in the bulk bar, for submit buttons.
/// - `error` is a slot above the table; `more` one below it, for a
///   [`data_table_more`]; `footer` one beside the summary, for pagination.
/// - `summary` is text such as "Rows 11–20 of 42"; see [`page_rows`].
/// - `loading` marks the table busy; `stale` also dims the rows.
/// - `status` is a message announced politely, such as "12 rows match the filter".
/// - The other arguments are the strings. `selected_label` and `selection_label`
///   are templates with `{count}` and `{total}`.
#[component]
pub async fn data_table(
    cx: &Cx,
    id: &str,
    caption: &str,
    columns: &[DataTableColumn<'_>],
    #[default]
    #[into]
    sort: Option<DataTableSort<'_>>,
    #[default] sort_href: &str,
    #[default] sort_clear_href: &str,
    #[default] selectable: bool,
    #[default] selected_count: usize,
    #[default] row_count: usize,
    #[default] row_actions: bool,
    #[default] columns_menu: bool,
    #[default] columns_storage_key: &str,
    #[default]
    #[into]
    search: Child<'_>,
    #[default]
    #[into]
    toolbar: Child<'_>,
    #[default]
    #[into]
    filters: Child<'_>,
    #[default]
    #[into]
    bulk_actions: Child<'_>,
    #[default]
    #[into]
    error: Child<'_>,
    #[default]
    #[into]
    more: Child<'_>,
    #[default]
    #[into]
    footer: Child<'_>,
    #[default] summary: &str,
    #[default] loading: bool,
    #[default] stale: bool,
    #[default] status: &str,
    #[default] show_caption: bool,
    #[default] sticky_header: bool,
    #[default] max_height: &str,
    #[default] density: TableDensity,
    #[default] striped: bool,
    #[default(true)] framed: bool,
    #[default] stack: bool,
    #[default("Select all rows")] select_all_label: &str,
    #[default("{count} selected")] selected_label: &str,
    #[default("{count} of {total} selected")] selection_label: &str,
    #[default("Clear selection")] clear_selection_label: &str,
    #[default("Bulk actions")] bulk_label: &str,
    #[default("Actions")] row_actions_label: &str,
    #[default("Columns")] columns_menu_label: &str,
    #[default("Show columns")] columns_group_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let heads: Vec<Head> = columns
        .iter()
        .map(|column| {
            let active = sort
                .filter(|sort| sort.column == column.id)
                .map(|sort| sort.direction);
            Head {
                id: column.id.to_owned(),
                label: column.label.to_owned(),
                href: column.sortable.then(|| {
                    sort_url(
                        sort_href,
                        sort_clear_href,
                        column.id,
                        next_sort(sort, column.id),
                    )
                }),
                active,
                aria_sort: column
                    .sortable
                    .then(|| active.map_or("none", SortDirection::as_str)),
                numeric: column.numeric,
                hide_label: column.hide_label,
                style: (!column.width.is_empty()).then(|| format!("width: {}", column.width)),
                hidden: column.hidden,
            }
        })
        .collect();
    let shown = columns.iter().filter(|column| !column.hidden).count();
    let toggles: Vec<Toggle> = columns
        .iter()
        .filter(|column| column.hideable)
        .map(|column| Toggle {
            id: column.id.to_owned(),
            label: column.label.to_owned(),
            checked: !column.hidden,
            only: !column.hidden && shown == 1,
        })
        .collect();
    let menu_id = format!("{id}-columns");
    let fill = |template: &str| {
        template
            .replace("{count}", &selected_count.to_string())
            .replace("{total}", &row_count.to_string())
    };
    let selected_text = fill(selected_label);
    let selection_text = fill(selection_label);
    let storage_key = (!columns_storage_key.is_empty()).then_some(columns_storage_key);
    let total = row_count.to_string();

    Ok(view! {
        <div
            id=(id)
            class=(class!(css::ROOT, attrs.remove("class")))
            data-data-table=""
            data-loading=(loading.then_some(""))
            data-stale=((loading && stale).then_some(""))
            data-columns-key=(storage_key)
            data-row-count=(total)
            (attrs)
        >
            <div class=(css::TOOLBAR)>
                <div class=(css::FILTER)>(search)</div>
                <div class=(css::ACTIONS)>
                    (toolbar)
                    if columns_menu {
                        menu_trigger(
                            target: &menu_id,
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Outline,
                            icon(
                                name: Icon::Columns3,
                                attrs: attributes! { class=(css::COLUMNS_ICON) }
                            )
                            (columns_menu_label)
                        )
                        menu(
                            id: &menu_id,
                            align: PopupAlign::End,
                            menu_group(
                                label: columns_group_label,
                                for toggle in toggles {
                                    menu_checkbox_item(
                                        checked: toggle.checked,
                                        disabled: toggle.only,
                                        attrs: attributes! { data-column-toggle=(toggle.id) },
                                        (toggle.label)
                                    )
                                }
                            )
                        )
                    }
                </div>
            </div>
            <div class=(css::FILTERS)>(filters)</div>
            if selectable {
                <div
                    class=(css::BULK)
                    role="group"
                    aria-label=(bulk_label)
                    data-bulk=""
                >
                    <span
                        class=(css::BULK_COUNT)
                        data-selected-count=""
                        data-template=(selected_label)
                    >
                        (selected_text)
                    </span>
                    <div class=(css::BULK_ACTIONS)>(bulk_actions)</div>
                    <button
                        type="button"
                        class=(class!(button_css::ROOT, css::BULK_CLEAR))
                        data-variant="ghost"
                        data-size="sm"
                        data-clear-selection=""
                    >
                        icon(name: Icon::X)
                        (clear_selection_label)
                    </button>
                </div>
            }
            <div class=(css::ERROR)>(error)</div>
            table(
                caption: caption,
                show_caption: show_caption,
                sticky_header: sticky_header,
                max_height: max_height,
                density: density,
                striped: striped,
                framed: framed,
                stack: stack,
                sticky_end: row_actions,
                attrs: attributes! { class=(css::TABLE) aria-busy=(loading.then_some("true")) },
                columns: view! {
                    if selectable {
                        <th scope="col" class=(class!(table_css::TH, css::SELECT_HEAD))>
                            <span class=(css::CHECK)>
                                <input
                                    type="checkbox"
                                    class=(checkbox_css::CHECKBOX)
                                    aria-label=(select_all_label)
                                    data-select-all=""
                                >
                            </span>
                        </th>
                    }
                    for head in heads {
                        <th
                            scope="col"
                            aria-sort=(head.aria_sort)
                            data-column=(head.id)
                            data-numeric=(head.numeric.then_some(""))
                            hidden=(head.hidden.then_some(""))
                            style=(head.style)
                            class=(table_css::TH)
                        >
                            if let Some(href) = head.href {
                                <a
                                    href=(href)
                                    class=(css::SORT)
                                    data-active=(head.active.map(|_| ""))
                                    data-sort=""
                                >
                                    <span class=(head.hide_label.then_some("sr-only"))>
                                        (head.label)
                                    </span>
                                    icon(
                                        name: match head.active {
                                            Some(SortDirection::Ascending) => Icon::ArrowUp,
                                            Some(SortDirection::Descending) => Icon::ArrowDown,
                                            None => Icon::ChevronsUpDown,
                                        },
                                        attrs: attributes! { class=(css::SORT_ICON) }
                                    )
                                </a>
                            } else if head.hide_label {
                                <span class="sr-only">(head.label)</span>
                            } else {
                                (head.label)
                            }
                        </th>
                    }
                    if row_actions {
                        <th
                            scope="col"
                            data-sticky-end=""
                            style="width: 1%"
                            class=(table_css::TH)
                        >
                            <span class="sr-only">(row_actions_label)</span>
                        </th>
                    }
                },
                (child)
            )
            <div class=(css::MORE)>(more)</div>
            <div class=(css::FOOTER)>
                <p class=(css::SUMMARY)>
                    if selectable {
                        <span data-selection-summary="" data-template=(selection_label)>
                            (selection_text)
                        </span>
                    }
                    if selectable && !summary.is_empty() {
                        " · "
                    }
                    (summary)
                </p>
                (footer)
            </div>
            <p role="status" class="sr-only">(status)</p>
        </div>
    })
}

/// The first cell of a selectable row: a checkbox named after the row.
///
/// - `value` is what the checkbox submits, usually the row's ID, under `name`.
/// - `label` names the row: the checkbox is announced as `select_label` with
///   `{label}` filled in.
/// - `checked` selects the row from the start; mark its `tr` `selected` too.
/// - `form` is the ID of a form elsewhere on the page that the checkbox belongs to.
#[component]
pub async fn data_table_select(
    cx: &Cx,
    value: &str,
    label: &str,
    #[default("selected")] name: &str,
    #[default] checked: bool,
    #[default] disabled: bool,
    #[default] form: &str,
    #[default("Select {label}")] select_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let named = select_label.replace("{label}", label);
    Ok(view! {
        <td
            class=(class!(table_css::TD, css::SELECT_CELL, attrs.remove("class")))
            (attrs)
        >
            <span class=(css::CHECK)>
                <input
                    type="checkbox"
                    class=(checkbox_css::CHECKBOX)
                    name=(name)
                    value=(value)
                    form=((!form.is_empty()).then_some(form))
                    aria-label=(named)
                    checked=(checked.then_some(""))
                    disabled=(disabled.then_some(""))
                    data-row-select=""
                >
            </span>
        </td>
    })
}

/// A link that opens a row's record, for the row header cell. With `data-clickable`
/// on the row, a click anywhere on the row that is not on a control follows it.
#[component]
pub async fn data_table_row_link(
    cx: &Cx,
    href: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            class=(class!(css::ROW_OPEN, attrs.remove("class")))
            data-row-open=""
            (attrs)
        >
            (child)
        </a>
    })
}

/// How a [`cell_text`] breaks long words. Rendered as `data-wrap`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CellWrap {
    /// Wraps at spaces, and breaks a word only when it cannot fit.
    #[default]
    Word,
    /// Also breaks long unbroken values, such as IDs and URLs.
    Anywhere,
}

impl CellWrap {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Word => "word",
            Self::Anywhere => "anywhere",
        }
    }
}

/// The common cell of two lines: a name over a muted subtitle. A space separates
/// the lines for assistive technology and for copying.
#[component]
pub async fn cell_text(
    cx: &Cx,
    primary: &str,
    #[default] secondary: &str,
    #[default] wrap: CellWrap,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            class=(class!(css::CELL_TEXT, attrs.remove("class")))
            data-wrap=(wrap.as_str())
            (attrs)
        >
            <span class=(css::CELL_PRIMARY)>(primary)</span>
            if !secondary.is_empty() {
                " "
                <span class=(css::CELL_SECONDARY)>(secondary)</span>
            }
        </span>
    })
}

/// Rows of placeholders for a table whose rows are loading. They are hidden from
/// assistive technology; say "Loading rows…" in the table's `status`.
///
/// `columns` is the number of cells in a row, counting the checkbox and the actions.
#[component]
pub async fn data_table_skeleton(
    cx: &Cx,
    columns: usize,
    #[default(5)] rows: usize,
) -> Result<impl View> {
    const WIDTHS: [&str; 5] = ["70%", "45%", "60%", "35%", "80%"];
    Ok(view! {
        for row in 0..rows.max(1) {
            <tr
                aria-hidden="true"
                class=(class!(table_css::TR, css::SKELETON_ROW))
                data-skeleton=""
            >
                for column in 0..columns.max(1) {
                    <td class=(table_css::TD)>
                        skeleton(
                            shape: SkeletonShape::Text,
                            width: WIDTHS[(row + column * 2) % WIDTHS.len()]
                        )
                    </td>
                }
            </tr>
        }
    })
}

/// A line of text in place of the rows, such as "No rows." or "No results for
/// “acme”.", in a [`table_empty`](super::table::table_empty).
#[component]
pub async fn data_table_message(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::MESSAGE, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// A "Load more" link after the table, for the `more` slot.
#[component]
pub async fn data_table_more(
    cx: &Cx,
    href: &str,
    #[default("Load more")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            class=(class!(button_css::ROOT, attrs.remove("class")))
            data-variant="outline"
            data-size="sm"
            (attrs)
        >
            (label)
        </a>
    })
}

#[cfg(test)]
mod tests {
    use super::{DataTableSort, SortDirection::*, next_sort, page_rows, sort_url};

    fn sort(column: &str, direction: super::SortDirection) -> Option<DataTableSort<'_>> {
        Some(DataTableSort { column, direction })
    }

    #[test]
    fn a_header_cycles_ascending_descending_none() {
        assert_eq!(next_sort(None, "name"), sort("name", Ascending));
        assert_eq!(
            next_sort(sort("name", Ascending), "name"),
            sort("name", Descending)
        );
        assert_eq!(next_sort(sort("name", Descending), "name"), None);
        // Another column starts again at ascending.
        assert_eq!(
            next_sort(sort("name", Descending), "size"),
            sort("size", Ascending)
        );
    }

    #[test]
    fn sort_urls() {
        let pattern = "/files?sort={column}&dir={direction}";
        assert_eq!(
            sort_url(pattern, "/files", "name", sort("name", Ascending)),
            "/files?sort=name&dir=asc"
        );
        assert_eq!(
            sort_url(pattern, "/files", "name", sort("name", Descending)),
            "/files?sort=name&dir=desc"
        );
        assert_eq!(sort_url(pattern, "/files", "name", None), "/files");
        // With nowhere to clear to, the header goes round to ascending.
        assert_eq!(
            sort_url(pattern, "", "name", None),
            "/files?sort=name&dir=asc"
        );
    }

    #[test]
    fn directions_parse_both_forms() {
        use super::SortDirection;
        assert_eq!(SortDirection::parse("asc"), Some(Ascending));
        assert_eq!(SortDirection::parse("descending"), Some(Descending));
        assert_eq!(SortDirection::parse("up"), None);
        assert_eq!(Ascending.as_param(), "asc");
        assert_eq!(Descending.as_str(), "descending");
    }

    #[test]
    fn rows_of_a_page() {
        assert_eq!(page_rows(1, 10, 42), (1, 10));
        assert_eq!(page_rows(2, 10, 42), (11, 20));
        assert_eq!(page_rows(5, 10, 42), (41, 42));
        assert_eq!(page_rows(9, 10, 42), (41, 42));
        assert_eq!(page_rows(0, 10, 42), (1, 10));
        assert_eq!(page_rows(1, 10, 0), (0, 0));
        assert_eq!(page_rows(1, 0, 5), (0, 0));
    }
}
