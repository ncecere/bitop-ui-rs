//! A unit of agent work that opens to show what it touched: "Searched 3 folders", then
//! the files found and the pages read.
//!
//! [`task`] is a native `<details>` and [`task_trigger`] its `<summary>`, so the browser
//! opens and closes it on click, Enter and Space. A running task shows a loader, and
//! "(in progress)" is read after its title.
//!
//! ```ignore
//! view! {
//!     task(open: true,
//!         task_trigger(title: "Searched 3 folders")
//!         task_content(
//!             task_item("Read " task_item_file("refunds.md"))
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream is Base UI's collapsible; a `<summary>` is
//! announced the same way. There is no `onOpenChange`: listen for the native `toggle`
//! event. The title is text and the icon one of the icon set.

use bitop_ui::components::{
    icon::{Icon, icon},
    loader::{LoaderSize, loader},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::task as css;

/// The task: a `<details>`. Put a [`task_trigger`] first, then [`task_content`].
/// `open` opens it; `attrs` are forwarded to the `<details>`.
#[component]
pub async fn task(
    cx: &Cx,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-task=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The `<summary>`: an icon, the title, a loader while `running`, and a chevron.
///
/// - `icon_name` replaces the magnifier.
/// - `in_progress_label` is read after the title while running.
#[component]
pub async fn task_trigger(
    cx: &Cx,
    title: &str,
    #[default] running: bool,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default("(in progress)")] in_progress_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-running=(running.then_some(""))
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::ICON)>
                icon(name: icon_name.unwrap_or(Icon::Search))
            </span>
            <span class=(css::TITLE) title=(title)>
                (title)
                if running {
                    " "
                    <span class="sr-only">(in_progress_label)</span>
                }
            </span>
            if running {
                loader(size: LoaderSize::Sm)
            }
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
        </summary>
    })
}

/// The panel: a list of [`task_item`]s with a guide line. `attrs` are forwarded to the
/// panel.
#[component]
pub async fn task_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::PANEL, attrs.remove("class"))) (attrs)>
            <ul class=(css::ITEMS)>(child)</ul>
        </div>
    })
}

/// One thing the task did: an `<li>`.
#[component]
pub async fn task_item(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!(css::ITEM, attrs.remove("class"))) (attrs)>(child)</li>
    })
}

/// A file name as a chip inside a [`task_item`]. `icon_name` replaces the page icon.
#[component]
pub async fn task_item_file(
    cx: &Cx,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::FILE, attrs.remove("class"))) (attrs)>
            <span aria-hidden="true" class=(css::FILE_ICON)>
                icon(name: icon_name.unwrap_or(Icon::FileText))
            </span>
            (child)
        </span>
    })
}
