use bitop_ui::components::icon::Icon;
use bitop_ui_ai::components::file_tree::{file_tree, file_tree_file, file_tree_folder};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "file-tree")
        example(
            title: "A project, one file selected",
            <div
                data-example="project"
                style="inline-size: 100%; max-inline-size: 22rem"
            >
                file_tree(
                    label: "Project files",
                    attrs: attributes! { id="project-tree" data-gallery-tree="" },
                    file_tree_folder(
                        path: "src",
                        name: "src",
                        open: true,
                        file_tree_folder(
                            path: "src/components",
                            name: "components",
                            file_tree_file(
                                path: "src/components/button.rs",
                                name: "button.rs"
                            )
                            file_tree_file(
                                path: "src/components/badge.rs",
                                name: "badge.rs"
                            )
                        )
                        file_tree_file(
                            path: "src/main.rs",
                            name: "main.rs",
                            selected: true,
                            meta: "M"
                        )
                        file_tree_file(path: "src/lib.rs", name: "lib.rs")
                        file_tree_file(
                            path: "src/logo.svg",
                            name: "logo.svg",
                            meta: "A"
                        )
                    )
                    file_tree_folder(
                        path: "tests",
                        name: "tests",
                        file_tree_file(path: "tests/smoke.rs", name: "smoke.rs")
                    )
                    file_tree_folder(
                        path: "docs",
                        name: "docs",
                        file_tree_file(path: "docs/guide.md", name: "guide.md")
                    )
                    file_tree_file(path: "Cargo.toml", name: "Cargo.toml")
                    file_tree_file(path: "README.md", name: "README.md")
                    file_tree_file(path: "LICENSE", name: "LICENSE")
                )
                <p data-tree-event="" style="margin-block-start: var(--space-2)"></p>
            </div>
        )
        example(
            title: "Levels and icons given, nothing selected",
            <div data-example="given" style="inline-size: 100%; max-inline-size: 22rem">
                file_tree(
                    label: "Uploads",
                    attrs: attributes! { id="uploads-tree" },
                    file_tree_folder(
                        path: "images",
                        name: "images",
                        level: 1,
                        icon_name: Icon::Image,
                        meta: "2 files",
                        file_tree_file(path: "images/a.png", name: "a.png", level: 2)
                        file_tree_file(path: "images/b.png", name: "b.png", level: 2)
                    )
                    file_tree_file(
                        path: "notes.txt",
                        name: "notes.txt",
                        level: 1,
                        icon_name: Icon::BookOpenText
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 22rem"
            >
                file_tree(
                    label: "Projektdateien",
                    file_tree_folder(
                        path: "quelle",
                        name: "quelle",
                        open: true,
                        meta: "geändert",
                        file_tree_file(path: "quelle/start.rs", name: "start.rs")
                    )
                )
            </div>
        )
    })
}
