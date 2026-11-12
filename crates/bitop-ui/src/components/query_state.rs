//! The switch every data view repeats: loading, error, empty, or the content.
//!
//! ```ignore
//! let state = QueryStatus::of(false, error.is_some(), projects.is_empty());
//! view! {
//!     query_state(state: state, error_status: error, empty_title: "No projects yet",
//!         project_table(rows: &projects)
//!     )
//! }
//! ```
//!
//! The order is loading, then error, then empty, then the content, which is `child`.
//! An error with `keep_content` shows the alert above the content, for a refresh that
//! failed while the last good data is still worth showing.
//!
//! Differences from upstream: upstream reads the state from a query object in the
//! browser and switches as it changes. Here the server knows the state when it
//! renders, so the state is an argument; a page that loads data later renders this
//! part again in what it sends. Upstream's `onRetry` handler is `retry_href`, a link
//! that requests the page again. The `loading` and `empty` slots replace the default
//! spinner and empty state when they are given.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    alert::error_alert,
    empty_state::empty_state,
    icon::Icon,
    spinner::{SpinnerSize, spinner},
};
use crate::css::{query_state as css, spinner as spinner_css};

/// What a [`query_state`] shows. Rendered as `data-query-state`, except for
/// [`QueryStatus::Ready`], which renders the content alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QueryStatus {
    /// The data is not here yet.
    Loading,
    /// The request failed.
    Error,
    /// The request worked and there is nothing to show.
    Empty,
    /// There is content.
    #[default]
    Ready,
}

impl QueryStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Error => "error",
            Self::Empty => "empty",
            Self::Ready => "ready",
        }
    }

    /// The state for three facts about a query, in upstream's order: loading wins,
    /// then an error, then being empty.
    #[must_use]
    pub fn of(loading: bool, error: bool, empty: bool) -> Self {
        if loading {
            Self::Loading
        } else if error {
            Self::Error
        } else if empty {
            Self::Empty
        } else {
            Self::Ready
        }
    }
}

/// Shows a loading indicator, an error alert, an empty state, or `child`.
///
/// Loading:
/// - `loading_label` is shown beside the spinner and announced as a status.
/// - `loading` is a slot that replaces the spinner, with skeleton rows for example;
///   the label is still announced.
///
/// Error, shown with [`error_alert`]:
/// - `error_status` is the HTTP status and `error_message` the error's own message.
/// - `retry_href` adds a retry link with the text `retry_label`.
/// - `keep_content` renders `child` under the alert.
///
/// Empty, shown with [`empty_state`]:
/// - `empty_title`, `empty_description` and `empty_icon` are its content, and
///   `empty_action` is a slot for its button.
/// - `empty` is a slot that replaces the whole empty state.
///
/// `attrs` go to the wrapper of the loading, error or empty state; the content is
/// rendered as it is.
#[component]
pub async fn query_state(
    cx: &Cx,
    state: QueryStatus,
    #[default]
    #[into]
    error_status: Option<u16>,
    #[default] error_message: &str,
    #[default] retry_href: &str,
    #[default("Try again")] retry_label: &str,
    #[default] keep_content: bool,
    #[default("Loading…")] loading_label: &str,
    #[default]
    #[into]
    loading: Child<'_>,
    #[default("Nothing here yet")] empty_title: &str,
    #[default] empty_description: &str,
    #[default]
    #[into]
    empty_icon: Option<Icon>,
    #[default]
    #[into]
    empty_action: Child<'_>,
    #[default]
    #[into]
    empty: Child<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = attrs.remove("class");
    Ok(view! {
        match state {
            QueryStatus::Loading => {
                <div
                    aria-busy="true"
                    data-query-state="loading"
                    class=(class!(class))
                    (attrs)
                >
                    <span role="status" class="sr-only">(loading_label)</span>
                    <div class=(css::CUSTOM)>(loading)</div>
                    <div
                        aria-hidden="true"
                        data-block=""
                        class=(class!(spinner_css::LOADING, css::STANDARD))
                    >
                        spinner(size: SpinnerSize::Sm)
                        <span>(loading_label)</span>
                    </div>
                </div>
            }
            QueryStatus::Error => {
                <div data-query-state="error" class=(class!(css::ERROR, class)) (attrs)>
                    error_alert(
                        status: error_status,
                        message: error_message,
                        retry_href: retry_href,
                        retry_label: retry_label
                    )
                </div>
                if keep_content {
                    (child)
                }
            }
            QueryStatus::Empty => {
                <div data-query-state="empty" class=(class!(class)) (attrs)>
                    <div class=(css::CUSTOM)>(empty)</div>
                    <div class=(css::STANDARD)>
                        empty_state(
                            title: empty_title,
                            description: empty_description,
                            icon: empty_icon,
                            (empty_action)
                        )
                    </div>
                </div>
            }
            QueryStatus::Ready => {
                (child)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::QueryStatus;

    #[test]
    fn loading_wins_then_error_then_empty() {
        assert_eq!(QueryStatus::of(true, true, true), QueryStatus::Loading);
        assert_eq!(QueryStatus::of(false, true, true), QueryStatus::Error);
        assert_eq!(QueryStatus::of(false, false, true), QueryStatus::Empty);
        assert_eq!(QueryStatus::of(false, false, false), QueryStatus::Ready);
        assert_eq!(QueryStatus::of(true, false, false), QueryStatus::Loading);
    }
}
