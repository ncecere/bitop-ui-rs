//! A tree of folders and files with the WAI-ARIA tree view pattern.
//!
//! The server renders the nesting: a `role="tree"` list of `role="treeitem"` items, a
//! folder's children in a `role="group"` list that is hidden while it is closed. The
//! library script adds the keyboard model and keeps the state attributes in step:
//!
//! - Down and Up move to the next and previous visible item.
//! - Right opens a closed folder, then moves into it; Left closes an open folder, then
//!   moves to its parent. In a right-to-left tree the two swap.
//! - Home and End move to the first and last visible item.
//! - Enter and Space select the item; a folder also opens or closes.
//! - `*` opens every folder beside the current item.
//! - Typing a letter or more moves to the next item whose name starts with them.
//!
//! One item is in the tab order at a time (a roving `tabindex`): the selected one, or
//! the first. A click selects an item and opens or closes a folder. Selecting sends
//! `bitop:file-tree-select` from the tree with `path` and `kind` in `detail`; opening or
//! closing sends `bitop:file-tree-toggle` with `path` and `open`. Names stay left to
//! right in a right-to-left page.
//!
//! ```ignore
//! view! {
//!     file_tree(label: "Project files",
//!         file_tree_folder(path: "src", name: "src", open: true,
//!             file_tree_file(path: "src/main.rs", name: "main.rs", selected: true)
//!         )
//!         file_tree_file(path: "Cargo.toml", name: "Cargo.toml")
//!     )
//! }
//! ```
//!
//! Differences from upstream: the state is server-rendered (`open` on a folder,
//! `selected` on an item) in place of `expanded` and `selectedPath`; `onSelect` and
//! `onExpandedChange` are the two events. Closed folders keep their children in the
//! page, hidden. An item is named by its row's text (the name, then the meta text), as
//! the nested group is not part of a tree item's name; upstream points
//! `aria-labelledby` at the name, which Playwright's Firefox crashes on (see the
//! contributing guide). `aria-level` is rendered when
//! `level` is given; otherwise the script sets it from the nesting. Icons are [`Icon`]s.

use bitop_ui::components::icon::{Icon, icon};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::file_tree as css;

/// The icon for a file, from its extension: code, data, text, an image, or a plain file.
///
/// ```
/// use bitop_ui::components::icon::Icon;
/// use bitop_ui_ai::components::file_tree::file_icon;
///
/// assert_eq!(file_icon("main.rs"), Icon::FileCode);
/// assert_eq!(file_icon("Cargo.toml"), Icon::FileBraces);
/// assert_eq!(file_icon("LICENSE"), Icon::File);
/// ```
#[must_use]
pub fn file_icon(name: &str) -> Icon {
    let extension = name
        .rsplit_once('.')
        .map(|(_, ext)| ext.to_ascii_lowercase())
        .unwrap_or_default();
    match extension.as_str() {
        "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs" | "py" | "go" | "rs" | "rb" | "java"
        | "css" | "html" | "sh" => Icon::FileCode,
        "json" | "yaml" | "yml" | "toml" => Icon::FileBraces,
        "md" | "mdx" | "txt" | "rst" => Icon::FileText,
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "avif" | "ico" => Icon::FileImage,
        _ => Icon::File,
    }
}

/// The tree: a `<ul role="tree">` named `label`. Put [`file_tree_folder`]s and
/// [`file_tree_file`]s in it.
#[component]
pub async fn file_tree(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            role="tree"
            aria-label=(label)
            data-file-tree=""
            class=(class!(css::TREE, attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// A folder: a tree item that opens to show `child`, its items.
///
/// - `path` is unique in the tree, and sent with the events; `name` is shown, names
///   the item and is what typing matches.
/// - `open` shows the children; `selected` selects the folder.
/// - `level` is the nesting, from 1 at the top; without it the script works it out.
/// - `icon_name` replaces the folder icon, which otherwise opens with the folder.
/// - `meta` is short text at the end of the row, such as a count or a git status.
#[component]
pub async fn file_tree_folder(
    cx: &Cx,
    path: &str,
    name: &str,
    #[default] open: bool,
    #[default] selected: bool,
    #[default]
    #[into]
    level: Option<u32>,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] meta: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let shown = icon_name.unwrap_or(if open { Icon::FolderOpen } else { Icon::Folder });
    Ok(view! {
        <li
            role="treeitem"
            aria-expanded=(if open { "true" } else { "false" })
            aria-selected=(if selected { "true" } else { "false" })
            aria-level=(level)
            tabindex=(if selected { "0" } else { "-1" })
            data-path=(path)
            data-name=(name)
            data-kind="folder"
            data-selected=(selected.then_some(""))
            class=(class!(css::ITEM, attrs.remove("class")))
            (attrs)
        >
            <div data-tree-row="" class=(css::ROW)>
                icon(
                    name: Icon::ChevronRight,
                    attrs: attributes! { class=(css::CHEVRON) }
                )
                <span
                    aria-hidden="true"
                    data-kind="folder"
                    data-folder-icon=(icon_name.is_none().then_some(""))
                    class=(css::ICON)
                >
                    icon(name: shown)
                </span>
                <span dir="ltr" class=(css::NAME)>(name)</span>
                if !meta.is_empty() {
                    <span class=(css::META)>(meta)</span>
                }
            </div>
            <ul role="group" hidden=(!open) class=(css::GROUP)>(child)</ul>
        </li>
    })
}

/// A file: a tree item. The arguments are a folder's, without `open`; the icon is
/// chosen from the extension by [`file_icon`] unless `icon_name` is given.
#[component]
pub async fn file_tree_file(
    cx: &Cx,
    path: &str,
    name: &str,
    #[default] selected: bool,
    #[default]
    #[into]
    level: Option<u32>,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default] meta: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <li
            role="treeitem"
            aria-selected=(if selected { "true" } else { "false" })
            aria-level=(level)
            tabindex=(if selected { "0" } else { "-1" })
            data-path=(path)
            data-name=(name)
            data-kind="file"
            data-selected=(selected.then_some(""))
            class=(class!(css::ITEM, attrs.remove("class")))
            (attrs)
        >
            <div data-tree-row="" class=(css::ROW)>
                <span aria-hidden="true" class=(css::SPACER)></span>
                <span aria-hidden="true" data-kind="file" class=(css::ICON)>
                    icon(name: icon_name.unwrap_or_else(|| file_icon(name)))
                </span>
                <span dir="ltr" class=(css::NAME)>(name)</span>
                if !meta.is_empty() {
                    <span class=(css::META)>(meta)</span>
                }
            </div>
        </li>
    })
}

#[cfg(test)]
mod tests {
    use super::file_icon;
    use bitop_ui::components::icon::Icon;

    #[test]
    fn icons() {
        assert_eq!(file_icon("App.TSX"), Icon::FileCode);
        assert_eq!(file_icon("README.md"), Icon::FileText);
        assert_eq!(file_icon("logo.svg"), Icon::FileImage);
        assert_eq!(file_icon("data.yml"), Icon::FileBraces);
        assert_eq!(file_icon("archive.tar.gz"), Icon::File);
        assert_eq!(file_icon(".gitignore"), Icon::File);
    }
}
