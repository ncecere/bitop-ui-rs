use bitop_ui::components::{
    button::{ButtonVariant, button},
    field::{field, field_description, field_error, field_label, fieldset, form, form_actions},
    icon::Icon,
    input::{InputSize, input, textarea},
    layout::{FlexAlign, inline, stack},
    select::native_select,
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
        page_header(slug: "input")
        example(
            title: "Description and error",
            stack(
                gap: 5,
                attrs: attributes! { style="inline-size: 100%" },
                field(
                    field_label(target: "project-name", "Project name")
                    input(
                        attrs: attributes! {
                            id="project-name"
                            name="project"
                            value="Marketing site"
                            aria-describedby="project-name-description"
                        }
                    )
                    field_description(
                        id: "project-name-description",
                        "Shown to everyone in the workspace."
                    )
                )
                field(
                    field_label(target: "webhook-url", "Webhook URL")
                    input(
                        attrs: attributes! {
                            id="webhook-url"
                            name="webhook"
                            value="http://example"
                            aria-invalid="true"
                            aria-describedby="webhook-url-description webhook-url-error"
                        }
                    )
                    field_description(id: "webhook-url-description", "Must use HTTPS.")
                    field_error(
                        id: "webhook-url-error",
                        "Enter a URL that starts with https://."
                    )
                )
            )
        )
        example(
            title: "Form with fieldset and actions",
            form(
                attrs: attributes! {
                    id="webhook-form"
                    action="/c/input"
                    style="inline-size: 100%"
                },
                fieldset(
                    legend: "Webhook",
                    description: "We POST a JSON payload for every deployment.",
                    field(
                        field_label(target: "webhook-name", "Name")
                        input(
                            attrs: attributes! {
                                id="webhook-name"
                                name="name"
                                required=(true)
                                placeholder="Production alerts"
                            }
                        )
                    )
                    field(
                        field_label(target: "webhook-endpoint", "Endpoint")
                        input(
                            attrs: attributes! {
                                id="webhook-endpoint"
                                name="endpoint"
                                type="url"
                                required=(true)
                                placeholder="https://hooks.example.com/deploys"
                            }
                        )
                    )
                )
                form_actions(
                    button(
                        variant: ButtonVariant::Secondary,
                        attrs: attributes! { type="reset" },
                        "Reset"
                    )
                    button(attrs: attributes! { type="submit" }, "Save webhook")
                )
            )
        )
        example(
            title: "Inputs, textarea and native select",
            stack(
                gap: 5,
                attrs: attributes! { style="inline-size: 100%" },
                field(
                    field_label(target: "search-projects", hide: true, "Search")
                    input(
                        start_icon: Icon::Search,
                        attrs: attributes! {
                            id="search-projects"
                            type="search"
                            placeholder="Search projects…"
                        }
                    )
                )
                field(
                    field_label(target: "review-notes", hint: "Optional", "Notes")
                    textarea(
                        attrs: attributes! {
                            id="review-notes"
                            name="notes"
                            placeholder="Anything reviewers should know?"
                        }
                    )
                )
                field(
                    field_label(target: "region", "Region")
                    native_select(
                        attrs: attributes! {
                            id="region"
                            name="region"
                            aria-describedby="region-description"
                        },
                        <option value="iad">"Washington, D.C. (iad1)"</option>
                        <option value="fra" selected=(true)>"Frankfurt (fra1)"</option>
                        <option value="hnd">"Tokyo (hnd1)"</option>
                    )
                    field_description(
                        id: "region-description",
                        "A native select: the best choice for forms."
                    )
                )
                inline(
                    gap: 3,
                    nowrap: true,
                    align: FlexAlign::Start,
                    field(
                        attrs: attributes! { style="flex: 1" },
                        field_label(target: "small-input", "Small")
                        input(
                            size: InputSize::Sm,
                            attrs: attributes! { id="small-input" placeholder="32px" }
                        )
                    )
                    field(
                        attrs: attributes! { style="flex: 1" },
                        field_label(target: "disabled-input", "Disabled")
                        input(
                            attrs: attributes! {
                                id="disabled-input"
                                value="Read-only value"
                                readonly=(true)
                                disabled=(true)
                            }
                        )
                    )
                )
            )
        )
    })
}
