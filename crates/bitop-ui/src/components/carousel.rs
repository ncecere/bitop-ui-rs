//! A row, or a column, of slides on a CSS scroll-snap track, with previous and next
//! buttons and a dot for each slide.
//!
//! The browser does the scrolling, the swiping and the snapping, so the track works
//! before the script loads. The library script adds the buttons, the dots, the arrow
//! keys and the announcements.
//!
//! Accessibility (the WAI-ARIA carousel pattern): the root is a region with
//! `aria-roledescription="carousel"`, named by `label`; every slide is a group with
//! `aria-roledescription="slide"` and a label such as "2 of 5". Moving with the
//! buttons, the dots or the arrow keys (while focus is inside the carousel) announces
//! "Slide 2 of 5" in a polite live region. At either end a button is `aria-disabled`
//! and stays focusable, so that focus is not lost. The track can be focused, so that
//! the keyboard reaches slides that hold nothing focusable. There is no autoplay.
//! With reduced motion the track jumps to a slide instead of gliding.
//!
//! Differences from upstream: [`carousel_indicators`] is an addition; upstream leaves
//! dots to the page. The parts cannot read the carousel's state, so the buttons and
//! the dots take the track's ID, the dots take the number of slides, and the script
//! writes the slides' labels and the current state. Upstream's `onIndexChange` is the
//! `bitop:change` event on the root, with `detail.index`; `setApi` is
//! `bitopUi.carousel.scrollTo(root, index)` and `bitopUi.carousel.index(root)`.
//!
//! ```ignore
//! let track = element_id(cx);
//! view! {
//!     carousel(label: "Featured templates",
//!         carousel_content(id: &track,
//!             carousel_item("One")
//!             carousel_item("Two")
//!             carousel_item("Three")
//!         )
//!         carousel_previous(target: &track)
//!         carousel_next(target: &track)
//!         carousel_indicators(target: &track, count: 3)
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    button::ButtonVariant,
    icon::{Icon, icon},
};
use crate::{
    css::{button as button_css, carousel as css},
    merge_style,
};

/// The direction a [`carousel`] scrolls in. Rendered as `data-orientation`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    /// Slides stacked; give the track a height.
    Vertical,
}

impl CarouselOrientation {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// The carousel. `label` names its region, such as "Featured templates".
///
/// - `default_index` is the slide shown first, counted from 0.
/// - `slide_label` is each slide's label and `announcement` what is announced after a
///   move; in both, `{index}` stands for the slide's number and `{count}` for the
///   number of slides.
#[component]
pub async fn carousel(
    cx: &Cx,
    label: &str,
    #[default] orientation: CarouselOrientation,
    #[default] default_index: usize,
    #[default("{index} of {count}")] slide_label: &str,
    #[default("Slide {index} of {count}")] announcement: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section
            aria-roledescription="carousel"
            aria-label=(label)
            class=(class!(css::ROOT, attrs.remove("class")))
            data-carousel=""
            data-orientation=(orientation.as_str())
            data-index=(default_index)
            data-slide-label=(slide_label)
            data-announcement=(announcement)
            (attrs)
        >
            (child)
            <div role="status" aria-live="polite" aria-atomic="true" class="sr-only"></div>
        </section>
    })
}

/// The scroll-snap track. Its children must be [`carousel_item`]s. `id` is what the
/// buttons and the dots point at.
///
/// `slides_per_view` is how many slides show at once; they size themselves to fit,
/// gaps included.
#[component]
pub async fn carousel_content(
    cx: &Cx,
    id: &str,
    #[default(1)] slides_per_view: u8,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let own = if slides_per_view > 1 {
        format!("--carousel-per-view: {slides_per_view}")
    } else {
        String::new()
    };
    let style = merge_style(&mut attrs, &own);
    Ok(view! {
        <div
            id=(id)
            tabindex="0"
            class=(class!(css::TRACK, attrs.remove("class")))
            style=(style)
            data-carousel-track=""
            (attrs)
        >
            (child)
        </div>
    })
}

/// One slide.
///
/// `basis` is its size along the track, such as `18rem`, in place of the size that
/// `slides_per_view` gives. `label` replaces the label the script writes ("2 of 5").
#[component]
pub async fn carousel_item(
    cx: &Cx,
    #[default] basis: &str,
    #[default] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let own = if basis.is_empty() {
        String::new()
    } else {
        format!("--carousel-basis: {basis}")
    };
    let style = merge_style(&mut attrs, &own);
    Ok(view! {
        <div
            role="group"
            aria-roledescription="slide"
            aria-label=((!label.is_empty()).then_some(label))
            class=(class!(css::SLIDE, attrs.remove("class")))
            style=(style)
            data-carousel-slide=""
            data-basis=((!basis.is_empty()).then_some(""))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The button that goes to the previous slide of the track whose ID is `target`.
#[component]
pub async fn carousel_previous(
    cx: &Cx,
    target: &str,
    #[default("Previous slide")] label: &str,
    #[default(ButtonVariant::Secondary)] variant: ButtonVariant,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            aria-controls=(target)
            class=(class!(button_css::ROOT, css::BUTTON, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size="icon"
            data-carousel-step="prev"
            (attrs)
        >
            icon(name: Icon::ChevronLeft)
        </button>
    })
}

/// The button that goes to the next slide of the track whose ID is `target`.
#[component]
pub async fn carousel_next(
    cx: &Cx,
    target: &str,
    #[default("Next slide")] label: &str,
    #[default(ButtonVariant::Secondary)] variant: ButtonVariant,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            aria-label=(label)
            aria-controls=(target)
            class=(class!(button_css::ROOT, css::BUTTON, attrs.remove("class")))
            data-variant=(variant.as_str())
            data-size="icon"
            data-carousel-step="next"
            (attrs)
        >
            icon(name: Icon::ChevronRight)
        </button>
    })
}

/// A dot for each slide of the track whose ID is `target`, under the track. A dot goes
/// to its slide, and the current slide's dot has `aria-current="true"`.
///
/// - `count` is the number of slides.
/// - `label` names the group of dots.
/// - `item_label` is a dot's accessible name, with `{index}` for the slide's number.
#[component]
pub async fn carousel_indicators(
    cx: &Cx,
    target: &str,
    count: usize,
    #[default("Choose a slide")] label: &str,
    #[default("Go to slide {index}")] item_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let labels: Vec<String> = (1..=count)
        .map(|index| item_label.replace("{index}", &index.to_string()))
        .collect();
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            class=(class!(css::INDICATORS, attrs.remove("class")))
            data-carousel-indicators=""
            (attrs)
        >
            for (index, label) in labels.into_iter().enumerate() {
                <button
                    type="button"
                    aria-label=(label)
                    aria-controls=(target)
                    class=(css::INDICATOR)
                    data-carousel-indicator=(index)
                ></button>
            }
        </div>
    })
}
