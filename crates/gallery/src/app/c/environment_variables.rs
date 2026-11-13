use bitop_ui::components::input::textarea;
use bitop_ui_ai::components::environment_variables::{
    CopyFormat, environment_variable, environment_variable_copy_button, environment_variable_name,
    environment_variable_value, environment_variables, environment_variables_content,
    environment_variables_header, environment_variables_title, environment_variables_toggle,
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
        page_header(slug: "environment-variables")
        example(
            title: "Masked, with a switch for every value",
            <div
                data-example="masked"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                environment_variables(
                    label: "Production",
                    attrs: attributes! { id="production" },
                    environment_variables_header(
                        environment_variables_title(title: "Production", level: 3)
                        environment_variables_toggle()
                    )
                    environment_variables_content(
                        environment_variable(
                            name: "DATABASE_URL",
                            value: "postgres://app:s3cret@db.internal:5432/app",
                            required: true,
                            description: "Used by the web server and the worker",
                            tooltip: "database-url-tip"
                        )
                        environment_variable(
                            name: "API_KEY",
                            value: "sk-live-$abc\"123",
                            copy_format: CopyFormat::Export
                        )
                        environment_variable(
                            name: "GREETING",
                            value: "hello world",
                            copy_format: CopyFormat::Dotenv
                        )
                        environment_variable(name: "FEATURE_FLAGS", value: "")
                    )
                )
            </div>
        )
        example(
            title: "Shown from the start",
            <div data-example="shown" style="inline-size: 100%; max-inline-size: 40rem">
                environment_variables(
                    label: "Development",
                    show_values: true,
                    environment_variables_header(
                        environment_variables_title(title: "Development", level: 3)
                        environment_variables_toggle(checked: true)
                    )
                    environment_variables_content(
                        environment_variable(
                            name: "LOG_LEVEL",
                            value: "debug",
                            shown: true
                        )
                        environment_variable(
                            name: "PORT",
                            value: "3000",
                            shown: true,
                            copy_format: CopyFormat::Name
                        )
                    )
                )
            </div>
        )
        example(
            title: "A row of its own",
            <div
                data-example="custom"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                environment_variables(
                    environment_variables_content(
                        environment_variable(
                            name: "REGION",
                            value: "eu-west-1",
                            custom: true,
                            environment_variable_name(name: "REGION")
                            environment_variable_value(
                                value: "eu-west-1",
                                mask_length: 4
                            )
                            environment_variable_copy_button(
                                name: "REGION",
                                value: "eu-west-1",
                                format: CopyFormat::Dotenv,
                                label: "region"
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
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                environment_variables(
                    label: "Umgebungsvariablen",
                    environment_variables_header(
                        environment_variables_title(
                            title: "Umgebungsvariablen",
                            level: 3
                        )
                        environment_variables_toggle(label: "Werte zeigen")
                    )
                    environment_variables_content(
                        environment_variable(
                            name: "SECRET",
                            value: "geheim",
                            required: true,
                            required_label: "Pflicht",
                            hidden_label: "verborgen",
                            toggle_label: "{name} zeigen",
                            copy_label: "{label} kopieren",
                            copied_label: "{label} kopiert"
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
