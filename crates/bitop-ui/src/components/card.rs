//! A surface that groups related content, with an optional header and footer.
//!
//! ```ignore
//! let title = element_id(cx);
//! view! {
//!     card(labelled_by: &title,
//!         card_header(title_id: &title, title: "Members", description: "People with access.",
//!             button(size: ButtonSize::Sm, "Invite")
//!         )
//!         card_body("...")
//!         card_footer("3 members")
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use crate::css::card as css;

/// The card. `labelled_by` is the ID of its title, if it has a [`card_header`].
///
/// An `interactive` card lifts slightly on hover; use it when the whole card is a link.
#[component]
pub async fn card(
    cx: &Cx,
    #[default] labelled_by: &str,
    #[default] interactive: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section
            if !labelled_by.is_empty() {
                aria-labelledby=(labelled_by)
            }
            if interactive {
                data-interactive=""
            }
            class=(class!(css::CARD, attrs.remove("class")))
            (attrs)
        >
            (child)
        </section>
    })
}

/// The heading level of a card title.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardTitleLevel {
    #[default]
    H2,
    H3,
    H4,
}

/// The top of a card: a title, an optional description, and actions as `child`.
#[component]
pub async fn card_header(
    cx: &Cx,
    title: &str,
    #[default] title_id: &str,
    #[default] level: CardTitleLevel,
    #[default] description: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <header class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>
            <div class=(css::HEADING)>
                match level {
                    CardTitleLevel::H2 => {
                        <h2
                            if !title_id.is_empty() {
                                id=(title_id)
                            }
                            class=(css::TITLE)
                        >
                            (title)
                        </h2>
                    }
                    CardTitleLevel::H3 => {
                        <h3
                            if !title_id.is_empty() {
                                id=(title_id)
                            }
                            class=(css::TITLE)
                        >
                            (title)
                        </h3>
                    }
                    CardTitleLevel::H4 => {
                        <h4
                            if !title_id.is_empty() {
                                id=(title_id)
                            }
                            class=(css::TITLE)
                        >
                            (title)
                        </h4>
                    }
                }
                if !description.is_empty() {
                    <p class=(css::DESCRIPTION)>(description)</p>
                }
            </div>
            <div class=(css::ACTIONS)>(child)</div>
        </header>
    })
}

/// The main content of a card. `flush` removes the padding, for a full-width table.
#[component]
pub async fn card_body(
    cx: &Cx,
    #[default] flush: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            if flush {
                data-flush=""
            }
            class=(class!(css::BODY, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The bottom of a card.
#[component]
pub async fn card_footer(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::FOOTER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}
