//! A small numbered chip after a claim ("… in 2019 \[1\]") that opens a card describing
//! its source or sources.
//!
//! The chip is a button that toggles the card, a popover: click, Enter and Space open
//! it and move focus into it, so that its link and its previous and next buttons are
//! reached with Tab; Escape closes it and returns focus to the chip. The library script
//! also opens it on hover, after a short delay, without moving focus.
//!
//! The chip's accessible name includes the source's title ("Source 1: Annual report
//! 2024"), so a screen reader says what is cited without opening the card.
//!
//! With `activate_target`, the ID of a [`source`](super::sources::source) card listed
//! under the answer, a press opens that list and focuses the card instead of opening
//! the popup; hover still previews it. With `source_action`, the card ends with a
//! link-style button that closes it and goes to the current source's `target` in the
//! same way ("Show source 2 below").
//!
//! With `verification` (the claim was checked against its source), the chip carries a
//! check or a warning, and the explanation is part of its name and heads the card.
//!
//! The chip and the card are `<span>`s, so that they can sit inside a paragraph.
//!
//! ```ignore
//! let sources = [CitationSource::new("Annual report 2024").href("https://example.com/r")];
//! view! {
//!     <p>"Revenue grew 12% "
//!         inline_citation(id: "cite-1", index: &[1], sources: &sources)
//!     </p>
//! }
//! ```
//!
//! Differences from upstream: `onActivate` and the source action go to an element by
//! its ID instead of calling a function. A source's `description`, `quote` and icon
//! are text and an icon of the set. The card's ID is an argument.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::{button as button_css, popover as popover_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::sources::hostname;
use crate::css::inline_citation as css;

/// One source of an [`inline_citation`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CitationSource<'a> {
    /// Its title, shown as the card's heading.
    pub title: &'a str,
    /// A link to it, opened in a new tab.
    pub href: &'a str,
    /// The publisher or site; defaults to the host of `href`.
    pub site_name: &'a str,
    /// A short summary or snippet.
    pub description: &'a str,
    /// The passage that supports the claim.
    pub quote: &'a str,
    /// The ID of the element that shows this source in full, such as a
    /// [`source`](super::sources::source) card, for the card's source action.
    pub target: &'a str,
}

impl<'a> CitationSource<'a> {
    /// A source with only a title.
    #[must_use]
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            ..Self::default()
        }
    }

    /// Adds a link.
    #[must_use]
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = href;
        self
    }

    /// Adds the site's name.
    #[must_use]
    pub fn site_name(mut self, site_name: &'a str) -> Self {
        self.site_name = site_name;
        self
    }

    /// Adds a summary.
    #[must_use]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Adds the supporting passage.
    #[must_use]
    pub fn quote(mut self, quote: &'a str) -> Self {
        self.quote = quote;
        self
    }

    /// Adds the ID of the element that shows the source in full.
    #[must_use]
    pub fn target(mut self, target: &'a str) -> Self {
        self.target = target;
        self
    }

    /// The site's name, or the host of the link.
    #[must_use]
    pub fn site(&self) -> Option<String> {
        if self.site_name.is_empty() {
            hostname(self.href)
        } else {
            Some(self.site_name.to_owned())
        }
    }
}

/// The outcome of checking a claim against its source. Rendered as
/// `data-verification`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CitationVerification {
    /// The source supports the claim.
    Verified,
    /// The source does not support it.
    Unsupported,
    /// The source says otherwise.
    Contradicted,
}

impl CitationVerification {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Unsupported => "unsupported",
            Self::Contradicted => "contradicted",
        }
    }

    /// The default explanation, such as "Not supported by this source".
    #[must_use]
    pub fn text(self) -> &'static str {
        match self {
            Self::Verified => "Verified: the source supports this",
            Self::Unsupported => "Not supported by this source",
            Self::Contradicted => "Contradicted by this source",
        }
    }

    fn icon(self) -> Icon {
        match self {
            Self::Verified => Icon::Check,
            Self::Unsupported | Self::Contradicted => Icon::TriangleAlert,
        }
    }
}

/// The side of the chip the card opens on. Rendered as `data-side`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CitationSide {
    #[default]
    Top,
    Bottom,
}

impl CitationSide {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// The chip's text: the numbers, or without them the first source's site and how
/// many more there are ("example.com +2").
#[must_use]
pub fn citation_chip_text(sources: &[CitationSource<'_>], index: &[usize]) -> String {
    if !index.is_empty() {
        return index
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
    }
    let site = sources.first().map_or_else(
        || "source".to_owned(),
        |first| first.site().unwrap_or_else(|| first.title.to_owned()),
    );
    if sources.len() > 1 {
        format!("{site} +{}", sources.len() - 1)
    } else {
        site
    }
}

/// The chip's accessible name: "Source 1: Annual report", "Sources 1, 3: Annual
/// report and 1 more". The words are the arguments of the same names on
/// [`inline_citation`].
#[must_use]
pub fn citation_name(
    sources: &[CitationSource<'_>],
    index: &[usize],
    source_label: &str,
    sources_label: &str,
    more_label: &str,
) -> String {
    let first = sources.first().map_or("", |source| source.title);
    let more = if sources.len() > 1 {
        more_label.replace("{count}", &(sources.len() - 1).to_string())
    } else {
        String::new()
    };
    let lead = if index.is_empty() {
        citation_chip_text(sources, index)
    } else {
        let noun = if index.len() > 1 {
            sources_label
        } else {
            source_label
        };
        format!("{noun} {}", citation_chip_text(sources, index))
    };
    format!("{lead}: {first}{more}")
}

/// A citation chip and its card.
///
/// - `id` goes on the card; the chip points at it. Each citation on a page needs its
///   own.
/// - `sources` are the cited sources; with several, the card pages through them.
/// - `index` are the citation numbers, shown in the chip. Without them the chip shows
///   the first source's site.
/// - `label` replaces the chip's text; keep it part of the name.
/// - `side` is where the card opens.
/// - `activate_target` is the ID of an element that a press focuses instead of opening
///   the card.
/// - `verification` marks a checked claim; `verification_label` replaces its
///   explanation.
/// - `source_action` adds a button at the end of the card that goes to the current
///   source's `target`; `{index}` in it is replaced with the source's number.
/// - `source_label`, `sources_label` and `more_label` (with `{count}`) make the chip's
///   name; `previous_label`, `next_label` and `page_label` (with `{page}` and
///   `{count}`) are the pager's; `new_tab_label` is read after a link.
#[component]
pub async fn inline_citation(
    cx: &Cx,
    id: &str,
    sources: &[CitationSource<'_>],
    #[default] index: &[usize],
    #[default] label: &str,
    #[default] side: CitationSide,
    #[default] activate_target: &str,
    #[default]
    #[into]
    verification: Option<CitationVerification>,
    #[default] verification_label: &str,
    #[default] source_action: &str,
    #[default("Source")] source_label: &str,
    #[default("Sources")] sources_label: &str,
    #[default(" and {count} more")] more_label: &str,
    #[default("Previous source")] previous_label: &str,
    #[default("Next source")] next_label: &str,
    #[default("{page} of {count}")] page_label: &str,
    #[default("(opens in a new tab)")] new_tab_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let text = if label.is_empty() {
        citation_chip_text(sources, index)
    } else {
        label.to_owned()
    };
    let note = verification.map(|verification| {
        if verification_label.is_empty() {
            verification.text().to_owned()
        } else {
            verification_label.to_owned()
        }
    });
    let mut name = citation_name(sources, index, source_label, sources_label, more_label);
    if let Some(note) = &note {
        name = format!("{name}. {note}");
    }
    let count = sources.len();
    let multiple = count > 1;
    let pages: Vec<CitationPage> = sources
        .iter()
        .enumerate()
        .map(|(position, source)| CitationPage {
            title_id: format!("{id}-title-{position}"),
            title: source.title.to_owned(),
            href: source.href.to_owned(),
            site: source.site().unwrap_or_default(),
            description: source.description.to_owned(),
            quote: source.quote.to_owned(),
            target: source.target.to_owned(),
            action: source_action.replace(
                "{index}",
                &index
                    .get(position)
                    .copied()
                    .unwrap_or(position + 1)
                    .to_string(),
            ),
        })
        .collect();
    let first_title = format!("{id}-title-0");
    let page_text = page_label
        .replace("{page}", "1")
        .replace("{count}", &count.to_string());
    let note_icon = verification.map(CitationVerification::icon);
    let verification_attr = verification.map(CitationVerification::as_str);
    let activates = !activate_target.is_empty();
    let has_action = !source_action.is_empty();
    // Boxed: the nested type of this view is deep enough that a page holding several
    // overflows the compiler's recursion limit when it checks that the page is `Send`.
    Ok(Child::new(view! {
        if count > 0 {
            <span class=(css::ROOT) data-citation="">
                <button
                    type="button"
                    popovertarget=((!activates).then_some(id))
                    aria-label=(&name)
                    data-citation-trigger=(id)
                    data-citation-activate=(activates.then_some(activate_target))
                    data-verification=(verification_attr)
                    class=(class!(css::CHIP, attrs.remove("class")))
                    (attrs)
                >
                    (text)
                    if let Some(mark) = note_icon {
                        icon(name: mark, attrs: attributes! { class=(css::MARK) })
                    }
                </button>
                <span
                    id=(id)
                    popover="auto"
                    role="dialog"
                    aria-labelledby=(&first_title)
                    tabindex="-1"
                    data-citation-card=""
                    data-side=(side.as_str())
                    data-page="0"
                    data-page-label=(page_label)
                    class=(class!(popover_css::POPUP, css::CARD))
                >
                    if multiple {
                        <span class=(css::PAGER)>
                            <button
                                type="button"
                                aria-label=(previous_label)
                                disabled=(true)
                                data-citation-step="-1"
                                data-variant="ghost"
                                data-size="icon"
                                class=(class!(button_css::ROOT, css::STEP))
                            >
                                icon(
                                    name: Icon::ChevronLeft,
                                    attrs: attributes! { class=(css::STEP_ICON) }
                                )
                            </button>
                            <span
                                class=(css::PAGE_INDEX)
                                aria-live="polite"
                                data-citation-index=""
                            >
                                (page_text)
                            </span>
                            <button
                                type="button"
                                aria-label=(next_label)
                                data-citation-step="1"
                                data-variant="ghost"
                                data-size="icon"
                                class=(class!(button_css::ROOT, css::STEP))
                            >
                                icon(
                                    name: Icon::ChevronRight,
                                    attrs: attributes! { class=(css::STEP_ICON) }
                                )
                            </button>
                        </span>
                    }
                    for (position, page) in pages.iter().enumerate() {
                        <span
                            class=(css::SOURCE)
                            data-citation-page=(position)
                            hidden=(position > 0)
                        >
                            if let Some(note) = &note {
                                <span
                                    class=(css::VERIFICATION)
                                    data-verification=(verification_attr)
                                >
                                    if let Some(mark) = note_icon {
                                        icon(
                                            name: mark,
                                            attrs: attributes! { class=(css::VERIFICATION_ICON) }
                                        )
                                    }
                                    (note)
                                </span>
                            }
                            if !page.site.is_empty() {
                                <span class=(css::SITE)>(&page.site)</span>
                            }
                            <span id=(&page.title_id) class=(css::TITLE)>
                                if page.href.is_empty() {
                                    (&page.title)
                                } else {
                                    <a
                                        href=(&page.href)
                                        target="_blank"
                                        rel="noreferrer noopener"
                                        class=(css::LINK)
                                    >
                                        (&page.title)
                                        icon(
                                            name: Icon::ArrowUpRight,
                                            attrs: attributes! { class=(css::LINK_ICON) }
                                        )
                                        " "
                                        <span class="sr-only">(new_tab_label)</span>
                                    </a>
                                }
                            </span>
                            if !page.description.is_empty() {
                                <span class=(css::DESCRIPTION)>(&page.description)</span>
                            }
                            if !page.quote.is_empty() {
                                <q class=(css::QUOTE)>(&page.quote)</q>
                            }
                            if has_action && !page.target.is_empty() {
                                <button
                                    type="button"
                                    data-citation-go=(&page.target)
                                    class=(css::SOURCE_ACTION)
                                >
                                    (&page.action)
                                </button>
                            }
                        </span>
                    }
                </span>
            </span>
        }
    }))
}

struct CitationPage {
    title_id: String,
    title: String,
    href: String,
    site: String,
    description: String,
    quote: String,
    target: String,
    action: String,
}

#[cfg(test)]
mod tests {
    use super::{CitationSource, citation_chip_text, citation_name};

    #[test]
    fn chip_text_and_name() {
        let one = [CitationSource::new("Annual report").href("https://www.example.com/r")];
        let two = [
            CitationSource::new("Annual report").href("https://www.example.com/r"),
            CitationSource::new("Press release").site_name("Newswire"),
        ];
        assert_eq!(citation_chip_text(&one, &[1]), "1");
        assert_eq!(citation_chip_text(&two, &[1, 3]), "1, 3");
        assert_eq!(citation_chip_text(&one, &[]), "example.com");
        assert_eq!(citation_chip_text(&two, &[]), "example.com +1");
        assert_eq!(
            citation_chip_text(&[CitationSource::new("Notes")], &[]),
            "Notes"
        );
        let name = |sources: &[CitationSource<'_>], index: &[usize]| {
            citation_name(sources, index, "Source", "Sources", " and {count} more")
        };
        assert_eq!(name(&one, &[1]), "Source 1: Annual report");
        assert_eq!(
            name(&two, &[1, 3]),
            "Sources 1, 3: Annual report and 1 more"
        );
        assert_eq!(name(&two, &[]), "example.com +1: Annual report and 1 more");
    }
}
