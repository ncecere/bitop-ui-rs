use bitop_ui::components::diff_viewer::JsonValue;
use bitop_ui_ai::components::schema_display::{
    HttpMethod, ParameterLocation, SchemaLabels, SchemaParameter, SchemaProperty, schema_display,
    schema_display_example, schema_display_properties, schema_display_section,
    schema_to_properties_from_str,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

const INVOICE: &str = r#"{
  "type": "object",
  "required": ["id", "total"],
  "properties": {
    "id": { "type": "string", "format": "uuid", "description": "The invoice's ID." },
    "status": { "enum": ["draft", "open", "paid"], "description": "Where the invoice is." },
    "total": { "type": "number" },
    "lines": {
      "type": "array",
      "description": "One line per product.",
      "items": {
        "type": "object",
        "required": ["sku"],
        "properties": {
          "sku": { "type": "string" },
          "quantity": { "type": "integer" }
        }
      }
    },
    "paid_at": { "type": ["string", "null"], "format": "date-time" }
  }
}"#;

const EXAMPLE: &str = "{\n  \"id\": \"6c1f…\",\n  \"status\": \"paid\",\n  \"total\": 120.5\n}";

const GERMAN: SchemaLabels<'static> = SchemaLabels {
    required: "Pflicht",
    one_of: "Eines von:",
};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let parameters = vec![
        SchemaParameter::new("id", "string")
            .location(ParameterLocation::Path)
            .required()
            .description("The user's ID."),
        SchemaParameter::new("dry_run", "boolean").location(ParameterLocation::Query),
    ];
    let request = vec![
        SchemaProperty::new("label", "string")
            .required()
            .description("A name to tell the key apart."),
        SchemaProperty::new("scopes", "array")
            .items(SchemaProperty::new("scope", "string"))
            .enum_values(vec![
                JsonValue::String("read".to_owned()),
                JsonValue::String("write".to_owned()),
            ]),
        SchemaProperty::new("limits", "object")
            .description("Rate limits for the key.")
            .properties(vec![
                SchemaProperty::new("per_minute", "integer"),
                SchemaProperty::new("burst", "object").properties(vec![
                    SchemaProperty::new("size", "integer").required(),
                    SchemaProperty::new("window", "object")
                        .properties(vec![SchemaProperty::new("seconds", "integer")]),
                ]),
            ]),
    ];
    let response = vec![
        SchemaProperty::new("key", "string")
            .required()
            .format("password"),
        SchemaProperty::new("created_at", "string").format("date-time"),
    ];
    let invoice = schema_to_properties_from_str(INVOICE).unwrap_or_default();
    let german_parameters = vec![
        SchemaParameter::new("seite", "integer")
            .location(ParameterLocation::Query)
            .required(),
    ];
    Ok(view! {
        page_header(slug: "schema-display")
        example(
            title: "An endpoint",
            <div
                data-example="endpoint"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                schema_display(
                    id: "create-key",
                    method: HttpMethod::Post,
                    path: "/users/{id}/keys",
                    description: "Create an API key for a user.",
                    parameters: &parameters,
                    request_body: &request,
                    response_body: &response
                )
            </div>
        )
        example(
            title: "A JSON Schema, with an example",
            <div
                data-example="schema"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                schema_display(
                    id: "invoice",
                    title: "Invoice",
                    response_label: "Properties",
                    response_body: &invoice,
                    schema_display_example(code: EXAMPLE, filename: "invoice.json")
                )
            </div>
        )
        example(
            title: "Every method",
            <div
                data-example="methods"
                style="display: grid; gap: var(--space-2); inline-size: 100%; max-inline-size: 40rem"
            >
                schema_display(id: "m-get", method: HttpMethod::Get, path: "/users")
                schema_display(
                    id: "m-put",
                    method: HttpMethod::Put,
                    path: "/users/{id}"
                )
                schema_display(
                    id: "m-patch",
                    method: HttpMethod::Patch,
                    path: "/users/{id}"
                )
                schema_display(
                    id: "m-delete",
                    method: HttpMethod::Delete,
                    path: "/users/{id}"
                )
                schema_display(id: "m-head", method: HttpMethod::Head, path: "/health")
                schema_display(
                    id: "m-options",
                    method: HttpMethod::Options,
                    path: "/users"
                )
            </div>
        )
        example(
            title: "A section composed from the parts, closed",
            <div
                data-example="custom"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                schema_display(
                    id: "webhook",
                    title: "Webhook payload",
                    schema_display_section(
                        title: "Event",
                        open: false,
                        schema_display_properties(
                            id: "webhook-event",
                            properties: &response
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
                schema_display(
                    id: "german",
                    method: HttpMethod::Get,
                    path: "/rechnungen",
                    parameters: &german_parameters,
                    parameters_label: "Parameter",
                    labels: GERMAN
                )
            </div>
        )
    })
}
