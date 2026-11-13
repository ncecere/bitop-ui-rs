//! A card for a dependency change: the package's name, the current and the new version, a
//! badge for the kind of change (major, minor, patch, added, removed), a description and
//! the package's own dependencies. It only displays.
//!
//! The card is an `<article>` named by the package. The kind of change is spelled out in
//! the badge, never shown by colour alone, and the versions read "from 18.3.1 to 19.0.0".
//!
//! ```ignore
//! view! {
//!     package_info(name: "react", current_version: "18.3.1", new_version: "19.0.0",
//!         change_type: PackageChangeType::Major,
//!         package_info_description("React 19 removes legacy APIs.")
//!         package_info_content(
//!             package_info_dependencies(
//!                 package_info_dependency(name: "scheduler", version: "^0.25.0")
//!             )
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: there is no context, so the parts of a custom header
//! ([`package_info_name`], [`package_info_change_type`], [`package_info_version`]) take
//! the name, the change and the versions themselves. The icon of the name is an
//! [`Icon`], not any node.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::badge as badge_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::package_info as css;

/// The kind of change. Rendered as `data-change` on the badge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackageChangeType {
    Major,
    Minor,
    Patch,
    Added,
    Removed,
}

impl PackageChangeType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Patch => "patch",
            Self::Added => "added",
            Self::Removed => "removed",
        }
    }

    /// The base badge's colour; added has its own, the info colours.
    fn badge(self) -> &'static str {
        match self {
            Self::Major => "danger",
            Self::Minor => "warning",
            Self::Patch => "success",
            Self::Added | Self::Removed => "neutral",
        }
    }

    fn icon(self) -> Icon {
        match self {
            Self::Major | Self::Minor | Self::Patch => Icon::ArrowRight,
            Self::Added => Icon::Plus,
            Self::Removed => Icon::Minus,
        }
    }
}

/// The card: an `<article>` named by `name`.
///
/// - `current_version`, `new_version` and `change_type` fill the default header: the
///   name and the badge, then the versions.
/// - `hide_header` leaves the default header out, to compose one from the parts.
/// - `major_label`, `minor_label`, `patch_label`, `added_label` and `removed_label`
///   are the badge's words; `from_label` and `to_label` are read around the versions.
/// - `child` follows the header: a [`package_info_description`] and a
///   [`package_info_content`].
/// - `attrs` are forwarded to the `<article>`.
#[component]
pub async fn package_info(
    cx: &Cx,
    name: &str,
    #[default] current_version: &str,
    #[default] new_version: &str,
    #[default]
    #[into]
    change_type: Option<PackageChangeType>,
    #[default] hide_header: bool,
    #[default("Major")] major_label: &str,
    #[default("Minor")] minor_label: &str,
    #[default("Patch")] patch_label: &str,
    #[default("Added")] added_label: &str,
    #[default("Removed")] removed_label: &str,
    #[default("from ")] from_label: &str,
    #[default(" to ")] to_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <article
            aria-label=(name)
            data-package-info=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            if !hide_header {
                package_info_header(
                    package_info_name(name: name)
                    if let Some(change) = change_type {
                        package_info_change_type(
                            change_type: change,
                            major_label: major_label,
                            minor_label: minor_label,
                            patch_label: patch_label,
                            added_label: added_label,
                            removed_label: removed_label
                        )
                    }
                )
                if !current_version.is_empty() || !new_version.is_empty() {
                    package_info_version(
                        current_version: current_version,
                        new_version: new_version,
                        from_label: from_label,
                        to_label: to_label
                    )
                }
            }
            (child)
        </article>
    })
}

/// The row of the name and the badge.
#[component]
pub async fn package_info_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The package's name in monospace, after a decorative icon (`icon_name`, a package by
/// default).
#[component]
pub async fn package_info_name(
    cx: &Cx,
    name: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::NAME, attrs.remove("class"))) (attrs)>
            <span aria-hidden="true" class=(css::ICON)>
                icon(name: icon_name.unwrap_or(Icon::Package))
            </span>
            <code>(name)</code>
        </p>
    })
}

/// The badge for the kind of change, with its word ("Major") and an arrow, a plus or a
/// minus. `label` replaces the word; the other strings are the words for each kind.
#[component]
pub async fn package_info_change_type(
    cx: &Cx,
    change_type: PackageChangeType,
    #[default] label: &str,
    #[default("Major")] major_label: &str,
    #[default("Minor")] minor_label: &str,
    #[default("Patch")] patch_label: &str,
    #[default("Added")] added_label: &str,
    #[default("Removed")] removed_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let word = if label.is_empty() {
        match change_type {
            PackageChangeType::Major => major_label,
            PackageChangeType::Minor => minor_label,
            PackageChangeType::Patch => patch_label,
            PackageChangeType::Added => added_label,
            PackageChangeType::Removed => removed_label,
        }
    } else {
        label
    };
    Ok(view! {
        <span
            data-variant=(change_type.badge())
            data-change=(change_type.as_str())
            class=(class!(badge_css::ROOT, css::BADGE, attrs.remove("class")))
            (attrs)
        >
            icon(name: change_type.icon())
            (word)
        </span>
    })
}

/// "18.3.1 → 19.0.0", read as "from 18.3.1 to 19.0.0". Either version may be left out:
/// an added package has only a new one. `child` replaces the line.
#[component]
pub async fn package_info_version(
    cx: &Cx,
    #[default] current_version: &str,
    #[default] new_version: &str,
    #[default("from ")] from_label: &str,
    #[default(" to ")] to_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let both = !current_version.is_empty() && !new_version.is_empty();
    Ok(view! {
        <p dir="ltr" class=(class!(css::VERSION, attrs.remove("class"))) (attrs)>
            if !current_version.is_empty() {
                if both {
                    <span class="sr-only">(from_label)</span>
                }
                <span class=(css::CURRENT) data-version="current">
                    (current_version)
                </span>
            }
            if both {
                icon(name: Icon::ArrowRight, attrs: attributes! { class=(css::ARROW) })
                <span class="sr-only">(to_label)</span>
            }
            if !new_version.is_empty() {
                <span class=(css::NEXT) data-version="new">(new_version)</span>
            }
            (child)
        </p>
    })
}

/// A short description of the change.
#[component]
pub async fn package_info_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::DESCRIPTION, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// The lower part of the card, set off by a line, for [`package_info_dependencies`].
#[component]
pub async fn package_info_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A list of [`package_info_dependency`] rows under `label`, which names the list.
#[component]
pub async fn package_info_dependencies(
    cx: &Cx,
    #[default("Dependencies")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::DEPENDENCIES, attrs.remove("class"))) (attrs)>
            <p aria-hidden="true" class=(css::SECTION_LABEL)>(label)</p>
            <ul aria-label=(label) class=(css::LIST)>(child)</ul>
        </div>
    })
}

/// One dependency: its name and, if given, its version or range ("^0.25.0").
#[component]
pub async fn package_info_dependency(
    cx: &Cx,
    name: &str,
    #[default] version: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!(css::DEPENDENCY, attrs.remove("class"))) (attrs)>
            <code class=(css::DEP_NAME)>(name)</code>
            if !version.is_empty() {
                " "
                <code class=(css::DEP_VERSION)>(version)</code>
            }
        </li>
    })
}
