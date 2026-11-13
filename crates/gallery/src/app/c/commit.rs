use bitop_ui::components::input::textarea;
use bitop_ui_ai::components::commit::{
    CommitFileStatus, LineCountLabels, commit, commit_actions, commit_author, commit_author_avatar,
    commit_content, commit_copy_button, commit_file, commit_files, commit_hash, commit_header,
    commit_info, commit_message, commit_metadata, commit_separator, commit_timestamp,
    commit_trigger,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const SHA: &str = "8f3c2a1d9e7b6f5a4c3b2a1908f7e6d5c4b3a291";
const GERMAN: LineCountLabels<'static> = LineCountLabels {
    added_one: ", {n} Zeile hinzugefügt",
    added_other: ", {n} Zeilen hinzugefügt",
    removed_one: ", {n} Zeile entfernt",
    removed_other: ", {n} Zeilen entfernt",
};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let now = bitop_ui::format::now();
    Ok(view! {
        page_header(slug: "commit")
        example(
            title: "Closed, with its changed files",
            <div
                data-example="closed"
                style="inline-size: 100%; max-inline-size: 36rem"
            >
                commit(
                    attrs: attributes! { id="token-commit" },
                    commit_header(
                        commit_author_avatar(name: "Ada Lovelace")
                        commit_info(
                            commit_message("Fix the token refresh race")
                            commit_metadata(
                                commit_author("Ada Lovelace")
                                commit_separator()
                                commit_timestamp(timestamp: now - 3 * 3600, now: now)
                            )
                        )
                        commit_actions(
                            commit_hash(hash: SHA)
                            commit_copy_button(hash: SHA)
                        )
                    )
                    commit_trigger(
                        panel: "token-files",
                        file_count: 4,
                        additions: 14,
                        deletions: 3
                    )
                    commit_content(
                        id: "token-files",
                        commit_files(
                            commit_file(
                                path: "src/auth.rs",
                                status: CommitFileStatus::Modified,
                                additions: 12,
                                deletions: 3
                            )
                            commit_file(
                                path: "src/auth_test.rs",
                                status: CommitFileStatus::Added,
                                additions: 2
                            )
                            commit_file(
                                path: "src/legacy_token.rs",
                                status: CommitFileStatus::Deleted
                            )
                            commit_file(
                                path: "src/session/refresh.rs",
                                previous_path: "src/refresh.rs",
                                status: CommitFileStatus::Renamed,
                                additions: 1,
                                deletions: 1
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Open, one file",
            <div data-example="open" style="inline-size: 100%; max-inline-size: 36rem">
                commit(
                    commit_header(
                        commit_author_avatar(name: "Grace Hopper")
                        commit_info(
                            commit_message(
                                "Document the deployment checklist and the rollback steps for the database migration"
                            )
                            commit_metadata(
                                commit_author("Grace Hopper")
                                commit_separator()
                                commit_timestamp(timestamp: now - 2 * 86400, now: now)
                            )
                        )
                        commit_actions(
                            commit_hash(hash: "c0ffee1234567890", length: 10)
                            commit_copy_button(hash: "c0ffee1234567890")
                        )
                    )
                    commit_trigger(
                        panel: "docs-files",
                        open: true,
                        file_count: 1,
                        additions: 1
                    )
                    commit_content(
                        id: "docs-files",
                        open: true,
                        commit_files(
                            commit_file(
                                path: "docs/deploy.md",
                                status: CommitFileStatus::Modified,
                                additions: 1
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 36rem"
            >
                commit(
                    commit_header(
                        commit_info(
                            commit_message("Übersetzungen ergänzen")
                            commit_metadata(commit_author("Lena"))
                        )
                        commit_actions(
                            commit_hash(hash: SHA, label: "Commit ")
                            commit_copy_button(
                                hash: SHA,
                                label: "Commit-Hash",
                                copy_label: "{label} kopieren",
                                copied_label: "{label} kopiert"
                            )
                        )
                    )
                    commit_trigger(
                        panel: "german-files",
                        file_count: 2,
                        additions: 5,
                        deletions: 1,
                        files_one: "{n} Datei geändert",
                        files_other: "{n} Dateien geändert",
                        line_labels: GERMAN
                    )
                    commit_content(
                        id: "german-files",
                        commit_files(
                            label: "Geänderte Dateien",
                            commit_file(
                                path: "locales/de.json",
                                status: CommitFileStatus::Added,
                                additions: 5,
                                added_label: "Hinzugefügt: ",
                                line_labels: GERMAN
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Paste here to try copying",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                textarea(attrs: attributes! { aria-label="Paste target" })
            </div>
        )
    })
}
