use bitop_ui::components::icon::Icon;
use bitop_ui_ai::components::package_info::{
    PackageChangeType, package_info, package_info_change_type, package_info_content,
    package_info_dependencies, package_info_dependency, package_info_description,
    package_info_header, package_info_name, package_info_version,
};
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
        page_header(slug: "package-info")
        example(
            title: "A major update, with its dependencies",
            <div data-example="major" style="inline-size: 100%; max-inline-size: 28rem">
                package_info(
                    name: "react",
                    current_version: "18.3.1",
                    new_version: "19.0.0",
                    change_type: PackageChangeType::Major,
                    package_info_description(
                        "React 19 removes legacy APIs such as string refs and propTypes."
                    )
                    package_info_content(
                        package_info_dependencies(
                            package_info_dependency(
                                name: "scheduler",
                                version: "^0.25.0"
                            )
                            package_info_dependency(
                                name: "loose-envify",
                                version: "^1.1.0"
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Every kind of change",
            <div
                data-example="kinds"
                style="display: grid; gap: var(--space-2); inline-size: 100%; max-inline-size: 28rem"
            >
                package_info(
                    name: "@base-ui/react",
                    current_version: "1.0.0",
                    new_version: "1.2.0",
                    change_type: PackageChangeType::Minor
                )
                package_info(
                    name: "lucide-react",
                    current_version: "0.468.0",
                    new_version: "0.468.2",
                    change_type: PackageChangeType::Patch
                )
                package_info(
                    name: "zod",
                    new_version: "3.24.1",
                    change_type: PackageChangeType::Added
                )
                package_info(
                    name: "moment",
                    current_version: "2.30.1",
                    change_type: PackageChangeType::Removed
                )
            </div>
        )
        example(
            title: "A header of its own",
            <div
                data-example="custom"
                style="inline-size: 100%; max-inline-size: 28rem"
            >
                package_info(
                    name: "serde",
                    hide_header: true,
                    package_info_header(
                        package_info_name(name: "serde", icon_name: Icon::FileBraces)
                        package_info_change_type(
                            change_type: PackageChangeType::Minor,
                            label: "Feature"
                        )
                    )
                    package_info_version(
                        current_version: "1.0.210",
                        new_version: "1.0.217"
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 28rem"
            >
                package_info(
                    name: "vite",
                    current_version: "5.4.0",
                    new_version: "6.0.0",
                    change_type: PackageChangeType::Major,
                    major_label: "Hauptversion",
                    from_label: "von ",
                    to_label: " auf ",
                    attrs: attributes! { id="german-package" },
                    package_info_content(
                        package_info_dependencies(
                            label: "Abhängigkeiten",
                            package_info_dependency(name: "esbuild", version: "^0.24.0")
                        )
                    )
                )
            </div>
        )
    })
}
