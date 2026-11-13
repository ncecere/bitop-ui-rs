//! A commit: its message, author, time and hash with a copy button, and a list of the
//! files it changed that opens and closes.
//!
//! [`commit_trigger`] is a button with `aria-expanded` that shows and hides
//! [`commit_content`] (the WAI-ARIA disclosure pattern); the library script does the
//! showing and hiding. A file's status is a letter (A, M, D, R) and a hidden word
//! ("Modified"), and line counts carry + and − signs, so nothing depends on colour.
//!
//! ```ignore
//! view! {
//!     commit(
//!         commit_header(
//!             commit_author_avatar(name: "Ada Lovelace")
//!             commit_info(
//!                 commit_message("Fix token refresh race")
//!                 commit_metadata(
//!                     commit_author("Ada Lovelace")
//!                     commit_separator()
//!                     commit_timestamp(iso: "2026-11-10T09:30:00Z")
//!                 )
//!             )
//!             commit_actions(commit_hash(hash: SHA) commit_copy_button(hash: SHA))
//!         )
//!         commit_trigger(panel: "files", file_count: 2, additions: 14, deletions: 3)
//!         commit_content(id: "files",
//!             commit_files(
//!                 commit_file(path: "src/auth.rs", status: CommitFileStatus::Modified,
//!                     additions: 12, deletions: 3)
//!             )
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: upstream's Base UI collapsible is a trigger button and a
//! panel, paired here by the panel's ID (`panel` and `id`); there is no
//! `onOpenChange`, but the trigger sends `bitop:collapsible-toggle` with
//! `detail.open`. The timestamp is the base set's [`time`] in its relative format,
//! which the script writes in the visitor's language; it takes a Unix time or an ISO
//! string, not a `Date`. The file icon is an [`Icon`].

use bitop_ui::components::{
    avatar::{AvatarSize, avatar},
    copy_button::copy_button,
    icon::{Icon, icon},
    time::{TimeFormat, time},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::commit as css;

/// What happened to a file. Rendered as `data-status`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitFileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

impl CommitFileStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Modified => "modified",
            Self::Deleted => "deleted",
            Self::Renamed => "renamed",
        }
    }

    /// The letter shown for the status, as `git status --short` writes it.
    #[must_use]
    pub fn letter(self) -> &'static str {
        match self {
            Self::Added => "A",
            Self::Modified => "M",
            Self::Deleted => "D",
            Self::Renamed => "R",
        }
    }
}

/// The words for line counts: `added_one` and `added_other`, `removed_one` and
/// `removed_other`, with `{n}` replaced. They are read after the visible "+14" and
/// "−3", so they start with a comma.
#[derive(Clone, Copy, Debug)]
pub struct LineCountLabels<'a> {
    pub added_one: &'a str,
    pub added_other: &'a str,
    pub removed_one: &'a str,
    pub removed_other: &'a str,
}

impl LineCountLabels<'_> {
    /// The English words.
    pub const ENGLISH: LineCountLabels<'static> = LineCountLabels {
        added_one: ", {n} line added",
        added_other: ", {n} lines added",
        removed_one: ", {n} line removed",
        removed_other: ", {n} lines removed",
    };
}

impl Default for LineCountLabels<'_> {
    fn default() -> Self {
        LineCountLabels::ENGLISH
    }
}

fn count_text(n: u32, one: &str, other: &str) -> String {
    (if n == 1 { one } else { other }).replace("{n}", &n.to_string())
}

/// The root. Put a [`commit_header`] in it, then a [`commit_trigger`] and its
/// [`commit_content`].
#[component]
pub async fn commit(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-commit="" class=(class!(css::ROOT, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// The top row: an avatar, the [`commit_info`] and the [`commit_actions`].
#[component]
pub async fn commit_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The author's avatar: the base [`avatar`], small by default. `src` is an image URL.
#[component]
pub async fn commit_author_avatar(
    cx: &Cx,
    name: &str,
    #[default]
    #[into]
    src: Option<&str>,
    #[default(AvatarSize::Sm)] size: AvatarSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        avatar(
            name: name,
            src: src,
            size: size,
            attrs: attributes! {
                class=(class!(css::AVATAR, attrs.remove("class")))
                (attrs)
            }
        )
    })
}

/// The message and the metadata, in a column.
#[component]
pub async fn commit_info(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::INFO, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The subject line, cut with an ellipsis when long.
#[component]
pub async fn commit_message(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::MESSAGE, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// The line under the message: author, separators and time.
#[component]
pub async fn commit_metadata(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::METADATA, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The author's name.
#[component]
pub async fn commit_author(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::AUTHOR, attrs.remove("class"))) (attrs)>(child)</span>
    })
}

/// A decorative "•" (`text`) between metadata items.
#[component]
pub async fn commit_separator(
    cx: &Cx,
    #[default("•")] text: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            aria-hidden="true"
            class=(class!(css::SEPARATOR, attrs.remove("class")))
            (attrs)
        >
            (text)
        </span>
    })
}

/// When the commit was made: a [`time`] that says "3 hours ago", with the full date as
/// its title. `timestamp` is a Unix time in seconds, or `iso` an ISO 8601 string;
/// `now` is the reference time (the server's clock by default), and `locale` the
/// language the script writes it in.
#[component]
pub async fn commit_timestamp(
    cx: &Cx,
    #[default]
    #[into]
    timestamp: Option<i64>,
    #[default] iso: &str,
    #[default]
    #[into]
    now: Option<i64>,
    #[default] locale: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        time(
            timestamp: timestamp,
            iso: iso,
            now: now,
            locale: locale,
            format: TimeFormat::Relative,
            attrs: attributes! {
                class=(class!(css::TIMESTAMP, attrs.remove("class")))
                (attrs)
            }
        )
    })
}

/// The row of the hash and buttons at the end of the header.
#[component]
pub async fn commit_actions(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ACTIONS, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The first `length` characters of the hash in monospace, read as "Commit 1a2b3c4"
/// (`label`), with the full hash as the title.
#[component]
pub async fn commit_hash(
    cx: &Cx,
    hash: &str,
    #[default(7)] length: usize,
    #[default("Commit ")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let short: String = hash.chars().take(length).collect();
    Ok(view! {
        <code
            dir="ltr"
            title=(hash)
            data-commit-hash=""
            class=(class!(css::HASH, attrs.remove("class")))
            (attrs)
        >
            icon(
                name: Icon::GitCommitHorizontal,
                attrs: attributes! { class=(css::HASH_ICON) }
            )
            <span class="sr-only">(label)</span>
            (short)
        </code>
    })
}

/// A button that copies the full hash, named "Copy commit hash". `label` is what is
/// copied; `copy_label` and `copied_label` are the copy button's templates.
#[component]
pub async fn commit_copy_button(
    cx: &Cx,
    hash: &str,
    #[default("commit hash")] label: &str,
    #[default("Copy {label}")] copy_label: &str,
    #[default("Copied {label}")] copied_label: &str,
    #[default("Copied {label} to clipboard")] copied_status: &str,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        copy_button(
            value: hash,
            label: label,
            copy_label: copy_label,
            copied_label: copied_label,
            copied_status: copied_status,
            attrs: attrs
        )
    })
}

/// "+14" and "−3", with the counts in words for assistive technology.
#[component]
async fn line_counts(
    cx: &Cx,
    additions: u32,
    deletions: u32,
    labels: LineCountLabels<'_>,
) -> Result<impl View> {
    Ok(view! {
        if additions > 0 || deletions > 0 {
            <span class=(css::CHANGES)>
                if additions > 0 {
                    <span class=(css::ADDITIONS) data-additions="">
                        <span aria-hidden="true">(format!("+{additions}"))</span>
                        <span class="sr-only">
                            (count_text(additions, labels.added_one, labels.added_other))
                        </span>
                    </span>
                }
                if deletions > 0 {
                    <span class=(css::DELETIONS) data-deletions="">
                        <span aria-hidden="true">(format!("\u{2212}{deletions}"))</span>
                        <span class="sr-only">
                            (count_text(
                                deletions,
                                labels.removed_one,
                                labels.removed_other,
                            ))
                        </span>
                    </span>
                }
            </span>
        }
    })
}

/// The button that shows and hides the files: a chevron, "3 files changed" and the line
/// counts.
///
/// - `panel` is the ID of the [`commit_content`] it controls; `open` its first state.
/// - `file_count` gives the label `files_one` or `files_other` (with `{n}`); without
///   it the label is `files_label`. `label` replaces it.
/// - `additions` and `deletions` are the total lines, read with `line_labels`.
#[component]
pub async fn commit_trigger(
    cx: &Cx,
    panel: &str,
    #[default] open: bool,
    #[default]
    #[into]
    file_count: Option<u32>,
    #[default] additions: u32,
    #[default] deletions: u32,
    #[default] label: &str,
    #[default("Changed files")] files_label: &str,
    #[default("{n} file changed")] files_one: &str,
    #[default("{n} files changed")] files_other: &str,
    #[default] line_labels: LineCountLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let text = if !label.is_empty() {
        label.to_owned()
    } else if let Some(n) = file_count {
        count_text(n, files_one, files_other)
    } else {
        files_label.to_owned()
    };
    Ok(view! {
        <button
            type="button"
            aria-expanded=(if open { "true" } else { "false" })
            aria-controls=(panel)
            data-collapsible=""
            data-open=(open.then_some(""))
            class=(class!(css::TRIGGER, attrs.remove("class")))
            (attrs)
        >
            icon(name: Icon::ChevronDown, attrs: attributes! { class=(css::CHEVRON) })
            <span class=(css::TRIGGER_LABEL)>(text)</span>
            line_counts(additions: additions, deletions: deletions, labels: line_labels)
        </button>
    })
}

/// The panel the trigger shows and hides. `id` pairs it with the trigger; `open` is
/// its first state, the same as the trigger's.
#[component]
pub async fn commit_content(
    cx: &Cx,
    id: &str,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            hidden=(!open)
            class=(class!(css::PANEL, attrs.remove("class")))
            (attrs)
        >
            <div class=(css::CONTENT)>(child)</div>
        </div>
    })
}

/// The list of [`commit_file`]s, named `label`.
#[component]
pub async fn commit_files(
    cx: &Cx,
    #[default("Changed files")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            aria-label=(label)
            class=(class!(css::FILES, attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One changed file.
///
/// - `status` is shown as a letter and read as `added_label`, `modified_label`,
///   `deleted_label` or `renamed_label` (each ending in ": ").
/// - `previous_path` is the old path of a renamed file: "old → new", read with
///   `to_label` in place of the arrow.
/// - `additions` and `deletions` are its line counts, read with `line_labels`.
/// - `icon_name` replaces the file icon.
#[component]
pub async fn commit_file(
    cx: &Cx,
    path: &str,
    status: CommitFileStatus,
    #[default] additions: u32,
    #[default] deletions: u32,
    #[default] previous_path: &str,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default("Added: ")] added_label: &str,
    #[default("Modified: ")] modified_label: &str,
    #[default("Deleted: ")] deleted_label: &str,
    #[default("Renamed: ")] renamed_label: &str,
    #[default(" to ")] to_label: &str,
    #[default] line_labels: LineCountLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let said = match status {
        CommitFileStatus::Added => added_label,
        CommitFileStatus::Modified => modified_label,
        CommitFileStatus::Deleted => deleted_label,
        CommitFileStatus::Renamed => renamed_label,
    };
    Ok(view! {
        <li
            data-status=(status.as_str())
            class=(class!(css::FILE, attrs.remove("class")))
            (attrs)
        >
            <span class=(css::STATUS)>
                <span aria-hidden="true">(status.letter())</span>
                <span class="sr-only">(said)</span>
            </span>
            <span aria-hidden="true" class=(css::FILE_ICON)>
                icon(name: icon_name.unwrap_or(Icon::FileText))
            </span>
            <span dir="ltr" class=(css::PATH)>
                if !previous_path.is_empty() {
                    (previous_path)
                    <span aria-hidden="true">" \u{2192} "</span>
                    <span class="sr-only">(to_label)</span>
                }
                (path)
            </span>
            line_counts(additions: additions, deletions: deletions, labels: line_labels)
        </li>
    })
}

#[cfg(test)]
mod tests {
    use super::{CommitFileStatus, count_text};

    #[test]
    fn counts() {
        assert_eq!(count_text(1, "{n} file", "{n} files"), "1 file");
        assert_eq!(count_text(3, "{n} file", "{n} files"), "3 files");
        assert_eq!(CommitFileStatus::Renamed.letter(), "R");
    }
}
