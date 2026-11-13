//! Pagination: a navigation landmark with previous, next and page links.
//!
//! Every control is a link, so paging works without a script: the server renders the
//! list for the current page. [`paginator`] does it all from the current page, the
//! page count and a URL pattern; it shows the first and last pages, the pages around
//! the current one, and an ellipsis where pages are skipped. [`pagination_range`] is
//! the arithmetic behind it.
//!
//! The parts compose by hand too, for example previous and next alone when the data
//! has cursors and no page numbers:
//!
//! ```ignore
//! view! {
//!     pagination(
//!         pagination_content(
//!             pagination_item(pagination_previous(href: "?before=abc"))
//!             pagination_item(pagination_next(disabled: true))
//!         )
//!     )
//! }
//! ```
//!
//! The current page has `aria-current="page"`. A link cannot be disabled, so at either
//! end previous and next are disabled buttons.
//!
//! Differences from upstream: upstream's `Paginator` can also render buttons that call
//! `onPageChange`, for tables paged in the browser; here it renders links only, from
//! `href`, a pattern in which `{page}` stands for the page number. The current page is
//! an outline button, since this library's button has no secondary variant.
//!
//! ```ignore
//! view! { paginator(page: 5, page_count: 20, href: "/items?page={page}") }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    button::ButtonSize,
    icon::{Icon, icon},
};
use crate::css::{button as button_css, pagination as css};

/// One entry of the list of pages: a page, or a run of skipped pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaginationRangeItem {
    /// A page number, starting at 1.
    Page(u32),
    /// Pages skipped between the first pages and the current one.
    StartEllipsis,
    /// Pages skipped between the current page and the last ones.
    EndEllipsis,
}

/// The pages to show for `page` of `page_count`, both counted from 1, with a marker
/// where pages are skipped.
///
/// `siblings` is how many pages show on each side of the current one, and `boundaries`
/// how many at the start and at the end; upstream's default for both is 1. The list
/// has the same length wherever the current page is, so the controls do not move as
/// the reader pages. A marker never stands for a single page: that page is shown.
///
/// ```
/// use bitop_ui::components::pagination::{PaginationRangeItem::*, pagination_range};
///
/// assert_eq!(
///     pagination_range(5, 10, 1, 1),
///     [Page(1), StartEllipsis, Page(4), Page(5), Page(6), EndEllipsis, Page(10)]
/// );
/// ```
#[must_use]
pub fn pagination_range(
    page: u32,
    page_count: u32,
    siblings: u32,
    boundaries: u32,
) -> Vec<PaginationRangeItem> {
    use PaginationRangeItem::{EndEllipsis, Page, StartEllipsis};

    if page_count == 0 {
        return Vec::new();
    }
    let count = i64::from(page_count);
    let current = i64::from(page).clamp(1, count);
    let siblings = i64::from(siblings);
    let boundaries = i64::from(boundaries);
    let pages = |start: i64, end: i64| {
        (start..=end).map(|n| Page(u32::try_from(n).expect("a page number is in 1..=page_count")))
    };

    let end_pages_start = (count - boundaries + 1).max(boundaries + 1);
    let siblings_start = (current - siblings)
        .min(count - boundaries - siblings * 2 - 1)
        .max(boundaries + 2);
    let siblings_end =
        (current + siblings)
            .max(boundaries + siblings * 2 + 2)
            .min(if end_pages_start <= count {
                end_pages_start - 2
            } else {
                count - 1
            });

    let mut range = Vec::new();
    range.extend(pages(1, boundaries.min(count)));
    if siblings_start > boundaries + 2 {
        range.push(StartEllipsis);
    } else if boundaries + 1 < count - boundaries {
        range.extend(pages(boundaries + 1, boundaries + 1));
    }
    range.extend(pages(siblings_start, siblings_end));
    if siblings_end < count - boundaries - 1 {
        range.push(EndEllipsis);
    } else if count - boundaries > boundaries {
        range.extend(pages(count - boundaries, count - boundaries));
    }
    range.extend(pages(end_pages_start, count));
    range
}

/// The landmark. `label` names it; make it unique when a page has two.
///
/// `compact` hides the text of the previous and next controls on a narrow screen,
/// leaving their arrows; the text is still announced.
#[component]
pub async fn pagination(
    cx: &Cx,
    #[default("Pagination")] label: &str,
    #[default] compact: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            aria-label=(label)
            class=(class!(css::NAV, attrs.remove("class")))
            data-pagination=""
            if compact {
                data-compact=""
            }
            (attrs)
        >
            (child)
        </nav>
    })
}

/// The list of controls inside a [`pagination`]. Pass [`pagination_item`]s.
#[component]
pub async fn pagination_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul class=(class!(css::LIST, attrs.remove("class"))) (attrs)>(child)</ul>
    })
}

/// One entry of a [`pagination_content`], around a link or an ellipsis.
#[component]
pub async fn pagination_item(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!(css::ITEM, attrs.remove("class"))) (attrs)>(child)</li>
    })
}

/// A link to a page. `child` is what it shows, usually the page number.
///
/// - `active` marks the current page: `aria-current="page"` and a raised style.
/// - `disabled` renders a disabled button, which cannot be focused, not a link.
/// - `label` is the accessible name, such as "Page 3".
#[component]
pub async fn pagination_link(
    cx: &Cx,
    #[default] href: &str,
    #[default] active: bool,
    #[default] disabled: bool,
    #[default] size: ButtonSize,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = class!(button_css::ROOT, css::PAGE, attrs.remove("class"));
    let variant = if active { "outline" } else { "ghost" };
    let label = (!label.is_empty()).then_some(label);
    Ok(view! {
        if disabled {
            <button
                type="button"
                disabled=(true)
                aria-label=(label)
                class=(class)
                data-variant=(variant)
                data-size=(size.as_str())
                (attrs)
            >
                (child)
            </button>
        } else {
            <a
                href=(href)
                aria-label=(label)
                aria-current=(active.then_some("page"))
                class=(class)
                data-variant=(variant)
                data-size=(size.as_str())
                data-active=(active.then_some(""))
                (attrs)
            >
                (child)
            </a>
        }
    })
}

/// The link to the previous page. `disabled` on the first page.
///
/// `text` is what shows beside the arrow and `label` the accessible name. `compact`
/// hides the text on a narrow screen.
#[component]
pub async fn pagination_previous(
    cx: &Cx,
    #[default] href: &str,
    #[default] disabled: bool,
    #[default] size: ButtonSize,
    #[default] compact: bool,
    #[default("Previous")] text: &str,
    #[default("Go to previous page")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = class!(
        button_css::ROOT,
        css::PAGE,
        css::STEP,
        attrs.remove("class")
    );
    let compact = compact.then_some("");
    Ok(view! {
        if disabled {
            <button
                type="button"
                disabled=(true)
                aria-label=(label)
                class=(class)
                data-variant="ghost"
                data-size=(size.as_str())
                data-step="previous"
                data-compact=(compact)
                (attrs)
            >
                icon(name: Icon::ChevronLeft, attrs: attributes! { class=(css::FLIP) })
                <span class=(css::STEP_TEXT)>(text)</span>
            </button>
        } else {
            <a
                href=(href)
                rel="prev"
                aria-label=(label)
                class=(class)
                data-variant="ghost"
                data-size=(size.as_str())
                data-step="previous"
                data-compact=(compact)
                (attrs)
            >
                icon(name: Icon::ChevronLeft, attrs: attributes! { class=(css::FLIP) })
                <span class=(css::STEP_TEXT)>(text)</span>
            </a>
        }
    })
}

/// The link to the next page. `disabled` on the last page.
///
/// `text` is what shows beside the arrow and `label` the accessible name. `compact`
/// hides the text on a narrow screen.
#[component]
pub async fn pagination_next(
    cx: &Cx,
    #[default] href: &str,
    #[default] disabled: bool,
    #[default] size: ButtonSize,
    #[default] compact: bool,
    #[default("Next")] text: &str,
    #[default("Go to next page")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = class!(
        button_css::ROOT,
        css::PAGE,
        css::STEP,
        attrs.remove("class")
    );
    let compact = compact.then_some("");
    Ok(view! {
        if disabled {
            <button
                type="button"
                disabled=(true)
                aria-label=(label)
                class=(class)
                data-variant="ghost"
                data-size=(size.as_str())
                data-step="next"
                data-compact=(compact)
                (attrs)
            >
                <span class=(css::STEP_TEXT)>(text)</span>
                icon(name: Icon::ChevronRight, attrs: attributes! { class=(css::FLIP) })
            </button>
        } else {
            <a
                href=(href)
                rel="next"
                aria-label=(label)
                class=(class)
                data-variant="ghost"
                data-size=(size.as_str())
                data-step="next"
                data-compact=(compact)
                (attrs)
            >
                <span class=(css::STEP_TEXT)>(text)</span>
                icon(name: Icon::ChevronRight, attrs: attributes! { class=(css::FLIP) })
            </a>
        }
    })
}

/// Marks skipped pages. It cannot be focused; `label` is what is announced.
#[component]
pub async fn pagination_ellipsis(
    cx: &Cx,
    #[default("More pages")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::ELLIPSIS, attrs.remove("class"))) (attrs)>
            icon(name: Icon::Ellipsis)
            <span class="sr-only">(label)</span>
        </span>
    })
}

/// What [`paginator`] renders for one entry of the range.
enum Entry {
    Page {
        number: u32,
        href: String,
        label: String,
        active: bool,
    },
    Ellipsis,
}

/// The whole control for `page` of `page_count`, both counted from 1.
///
/// - `href` is the URL of a page, with `{page}` where the number goes:
///   `"/items?page={page}"`.
/// - `siblings` and `boundaries` are as in [`pagination_range`].
/// - `label` names the landmark.
/// - `page_label` is a page link's accessible name, with `{page}` for the number.
/// - `previous_text`, `next_text`, `previous_label`, `next_label` and
///   `ellipsis_label` are the other strings.
/// - `compact` hides the previous and next text on a narrow screen.
///
/// A `page` outside the range is clamped into it. With no pages at all, previous and
/// next are both disabled.
#[component]
pub async fn paginator(
    cx: &Cx,
    page: u32,
    page_count: u32,
    href: &str,
    #[default(1)] siblings: u32,
    #[default(1)] boundaries: u32,
    #[default("Pagination")] label: &str,
    #[default] size: ButtonSize,
    #[default] compact: bool,
    #[default("Page {page}")] page_label: &str,
    #[default("Previous")] previous_text: &str,
    #[default("Next")] next_text: &str,
    #[default("Go to previous page")] previous_label: &str,
    #[default("Go to next page")] next_label: &str,
    #[default("More pages")] ellipsis_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let current = page.clamp(1, page_count.max(1));
    let url = |page: u32| href.replace("{page}", &page.to_string());
    let entries: Vec<Entry> = pagination_range(current, page_count, siblings, boundaries)
        .into_iter()
        .map(|item| match item {
            PaginationRangeItem::Page(number) => Entry::Page {
                number,
                href: url(number),
                label: page_label.replace("{page}", &number.to_string()),
                active: number == current,
            },
            PaginationRangeItem::StartEllipsis | PaginationRangeItem::EndEllipsis => {
                Entry::Ellipsis
            }
        })
        .collect();
    let at_start = current <= 1;
    let at_end = current >= page_count;
    let previous_href = url(current.saturating_sub(1).max(1));
    let next_href = url(current.saturating_add(1));
    let size = size.as_str();
    let step_class = class!(button_css::ROOT, css::PAGE, css::STEP);

    Ok(view! {
        <nav
            aria-label=(label)
            class=(class!(css::NAV, attrs.remove("class")))
            data-pagination=""
            if compact {
                data-compact=""
            }
            (attrs)
        >
            <ul class=(css::LIST)>
                <li class=(css::ITEM)>
                    if at_start {
                        <button
                            type="button"
                            disabled=(true)
                            aria-label=(previous_label)
                            class=(step_class)
                            data-variant="ghost"
                            data-size=(size)
                            data-step="previous"
                        >
                            icon(
                                name: Icon::ChevronLeft,
                                attrs: attributes! { class=(css::FLIP) }
                            )
                            <span class=(css::STEP_TEXT)>(previous_text)</span>
                        </button>
                    } else {
                        <a
                            href=(previous_href)
                            rel="prev"
                            aria-label=(previous_label)
                            class=(step_class)
                            data-variant="ghost"
                            data-size=(size)
                            data-step="previous"
                        >
                            icon(
                                name: Icon::ChevronLeft,
                                attrs: attributes! { class=(css::FLIP) }
                            )
                            <span class=(css::STEP_TEXT)>(previous_text)</span>
                        </a>
                    }
                </li>
                for entry in entries {
                    <li class=(css::ITEM)>
                        match entry {
                            Entry::Page { number, href, label, active } => {
                                <a
                                    href=(href)
                                    aria-label=(label)
                                    aria-current=(active.then_some("page"))
                                    class=(class!(button_css::ROOT, css::PAGE))
                                    data-variant=(if active { "outline" } else { "ghost" })
                                    data-size=(size)
                                    data-active=(active.then_some(""))
                                >
                                    (number)
                                </a>
                            }
                            Entry::Ellipsis => {
                                <span class=(css::ELLIPSIS)>
                                    icon(name: Icon::Ellipsis)
                                    <span class="sr-only">(ellipsis_label)</span>
                                </span>
                            }
                        }
                    </li>
                }
                <li class=(css::ITEM)>
                    if at_end {
                        <button
                            type="button"
                            disabled=(true)
                            aria-label=(next_label)
                            class=(step_class)
                            data-variant="ghost"
                            data-size=(size)
                            data-step="next"
                        >
                            <span class=(css::STEP_TEXT)>(next_text)</span>
                            icon(
                                name: Icon::ChevronRight,
                                attrs: attributes! { class=(css::FLIP) }
                            )
                        </button>
                    } else {
                        <a
                            href=(next_href)
                            rel="next"
                            aria-label=(next_label)
                            class=(step_class)
                            data-variant="ghost"
                            data-size=(size)
                            data-step="next"
                        >
                            <span class=(css::STEP_TEXT)>(next_text)</span>
                            icon(
                                name: Icon::ChevronRight,
                                attrs: attributes! { class=(css::FLIP) }
                            )
                        </a>
                    }
                </li>
            </ul>
        </nav>
    })
}

#[cfg(test)]
mod tests {
    use super::{
        PaginationRangeItem::{self, EndEllipsis, Page, StartEllipsis},
        pagination_range,
    };

    fn pages(numbers: impl IntoIterator<Item = u32>) -> Vec<PaginationRangeItem> {
        numbers.into_iter().map(Page).collect()
    }

    #[test]
    fn no_pages() {
        assert_eq!(pagination_range(1, 0, 1, 1), []);
    }

    #[test]
    fn few_pages_show_in_full() {
        assert_eq!(pagination_range(1, 1, 1, 1), pages(1..=1));
        assert_eq!(pagination_range(2, 3, 1, 1), pages(1..=3));
        // Seven entries is the length of a list with both markers, so seven pages fit.
        for page in 1..=7 {
            assert_eq!(pagination_range(page, 7, 1, 1), pages(1..=7), "page {page}");
        }
    }

    #[test]
    fn middle_has_both_markers() {
        assert_eq!(
            pagination_range(5, 10, 1, 1),
            [
                Page(1),
                StartEllipsis,
                Page(4),
                Page(5),
                Page(6),
                EndEllipsis,
                Page(10)
            ]
        );
    }

    #[test]
    fn near_the_start_and_the_end() {
        let start = [
            Page(1),
            Page(2),
            Page(3),
            Page(4),
            Page(5),
            EndEllipsis,
            Page(10),
        ];
        for page in 1..=3 {
            assert_eq!(pagination_range(page, 10, 1, 1), start, "page {page}");
        }
        assert_eq!(
            pagination_range(4, 10, 1, 1),
            [
                Page(1),
                Page(2),
                Page(3),
                Page(4),
                Page(5),
                EndEllipsis,
                Page(10)
            ]
        );
        let end = [
            Page(1),
            StartEllipsis,
            Page(6),
            Page(7),
            Page(8),
            Page(9),
            Page(10),
        ];
        for page in 7..=10 {
            assert_eq!(pagination_range(page, 10, 1, 1), end, "page {page}");
        }
    }

    #[test]
    fn length_is_constant_and_pages_ascend() {
        for (count, siblings, boundaries) in [(10, 1, 1), (50, 2, 1), (50, 1, 2), (30, 0, 1)] {
            let length = pagination_range(1, count, siblings, boundaries).len();
            for page in 1..=count {
                let range = pagination_range(page, count, siblings, boundaries);
                assert_eq!(range.len(), length, "page {page} of {count}");
                assert!(range.contains(&Page(page)), "page {page} of {count}");
                assert_eq!(range.first(), Some(&Page(1)));
                assert_eq!(range.last(), Some(&Page(count)));
                let numbers: Vec<u32> = range
                    .iter()
                    .filter_map(|item| match item {
                        Page(number) => Some(*number),
                        _ => None,
                    })
                    .collect();
                assert!(
                    numbers.windows(2).all(|pair| pair[0] < pair[1]),
                    "{range:?}"
                );
            }
        }
    }

    #[test]
    fn a_marker_never_hides_one_page() {
        for count in 1..=30 {
            for page in 1..=count {
                let range = pagination_range(page, count, 1, 1);
                for (index, item) in range.iter().enumerate() {
                    if !matches!(item, StartEllipsis | EndEllipsis) {
                        continue;
                    }
                    let (Page(before), Page(after)) = (range[index - 1], range[index + 1]) else {
                        panic!("a marker is between two pages: {range:?}");
                    };
                    assert!(after - before > 2, "page {page} of {count}: {range:?}");
                }
            }
        }
    }

    #[test]
    fn more_siblings_and_boundaries() {
        assert_eq!(
            pagination_range(10, 20, 2, 1),
            [
                Page(1),
                StartEllipsis,
                Page(8),
                Page(9),
                Page(10),
                Page(11),
                Page(12),
                EndEllipsis,
                Page(20)
            ]
        );
        assert_eq!(
            pagination_range(10, 20, 1, 2),
            [
                Page(1),
                Page(2),
                StartEllipsis,
                Page(9),
                Page(10),
                Page(11),
                EndEllipsis,
                Page(19),
                Page(20)
            ]
        );
        assert_eq!(
            pagination_range(5, 10, 0, 1),
            [Page(1), StartEllipsis, Page(5), EndEllipsis, Page(10)]
        );
    }

    #[test]
    fn no_boundaries_and_more_boundaries_than_pages() {
        assert_eq!(pagination_range(1, 2, 0, 0), pages(1..=2));
        assert_eq!(
            pagination_range(3, 9, 0, 0),
            [StartEllipsis, Page(3), EndEllipsis]
        );
        assert_eq!(pagination_range(1, 3, 1, 2), pages(1..=3));
        assert_eq!(pagination_range(1, 1, 1, 3), pages(1..=1));
        assert_eq!(
            pagination_range(5, 8, 1, 1),
            [
                Page(1),
                StartEllipsis,
                Page(4),
                Page(5),
                Page(6),
                Page(7),
                Page(8)
            ]
        );
    }

    #[test]
    fn out_of_range_pages_are_clamped() {
        assert_eq!(pagination_range(0, 10, 1, 1), pagination_range(1, 10, 1, 1));
        assert_eq!(
            pagination_range(99, 10, 1, 1),
            pagination_range(10, 10, 1, 1)
        );
    }
}
