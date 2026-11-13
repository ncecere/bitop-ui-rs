//! An API endpoint (method, path, parameters, request and response bodies) or a plain
//! JSON Schema, with nested object and array properties that open and close.
//!
//! The caller builds the tree with [`SchemaProperty`] and [`SchemaParameter`], or turns
//! a JSON Schema into properties with [`schema_to_properties`]. Each section is a
//! native `<details>`; each nested property is a button with `aria-expanded` that shows
//! and hides its children, with its description always shown under it. Nested
//! properties start open down to the second level. "required" is written out and types
//! are text, so nothing is shown by colour alone.
//!
//! ```ignore
//! let body = vec![
//!     SchemaProperty::new("label", "string").required(),
//!     SchemaProperty::new("scopes", "array").items(SchemaProperty::new("item", "string")),
//! ];
//! view! {
//!     schema_display(id: "create-key", method: HttpMethod::Post, path: "/users/{id}/keys",
//!         description: "Create an API key.", request_body: &body)
//!     schema_display(id: "invoice", title: "Invoice", response_label: "Properties",
//!         response_body: &schema_to_properties_from_str(INVOICE_SCHEMA).unwrap_or_default())
//! }
//! ```
//!
//! Differences from upstream: descriptions are text. JSON Schema comes in as text or as
//! the base set's [`JsonValue`] (no JSON dependency is added). The parts take what they
//! show as arguments rather than from a context; `id` names the nested panels. A
//! section's `onOpenChange` is the native `toggle` event; a nested property's trigger
//! sends `bitop:collapsible-toggle`.

use bitop_ui::{
    components::{
        diff_viewer::JsonValue,
        icon::{Icon, icon},
    },
    css::badge as badge_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::code_block::code_block;
use crate::css::schema_display as css;

/// An HTTP method. Shown as its name in a badge, coloured by kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    /// The method's name: `GET`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
        }
    }

    /// The base badge's colour; POST has its own, the info colours.
    fn badge(self) -> &'static str {
        match self {
            Self::Get => "success",
            Self::Put | Self::Patch => "warning",
            Self::Delete => "danger",
            Self::Post | Self::Head | Self::Options => "neutral",
        }
    }
}

/// Where a parameter goes. Shown as a small badge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

impl ParameterLocation {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Query => "query",
            Self::Header => "header",
            Self::Cookie => "cookie",
        }
    }
}

/// A parameter of an endpoint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SchemaParameter {
    pub name: String,
    /// The type as text: `string`, `integer`.
    pub type_name: String,
    pub required: bool,
    pub description: String,
    pub location: Option<ParameterLocation>,
}

impl SchemaParameter {
    /// A parameter of `name` and type `type_name`.
    #[must_use]
    pub fn new(name: &str, type_name: &str) -> Self {
        Self {
            name: name.to_owned(),
            type_name: type_name.to_owned(),
            ..Self::default()
        }
    }

    /// Marks it required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets where it goes.
    #[must_use]
    pub fn location(mut self, location: ParameterLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets its description.
    #[must_use]
    pub fn description(mut self, description: &str) -> Self {
        description.clone_into(&mut self.description);
        self
    }
}

/// A property of a body or a schema, with its children for an object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SchemaProperty {
    pub name: String,
    /// The type as text: `string`, `object`, `string | null`.
    pub type_name: String,
    pub required: bool,
    pub description: String,
    /// The properties of an object.
    pub properties: Vec<SchemaProperty>,
    /// The element of an array; an object element shows its properties.
    pub items: Option<Box<SchemaProperty>>,
    /// The values allowed, shown after the description as JSON.
    pub enum_values: Vec<JsonValue>,
    /// A format such as `date-time` or `uuid`, shown after the type.
    pub format: String,
}

impl SchemaProperty {
    /// A property of `name` and type `type_name`.
    #[must_use]
    pub fn new(name: &str, type_name: &str) -> Self {
        Self {
            name: name.to_owned(),
            type_name: type_name.to_owned(),
            ..Self::default()
        }
    }

    /// Marks it required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets its description.
    #[must_use]
    pub fn description(mut self, description: &str) -> Self {
        description.clone_into(&mut self.description);
        self
    }

    /// Sets its format.
    #[must_use]
    pub fn format(mut self, format: &str) -> Self {
        format.clone_into(&mut self.format);
        self
    }

    /// Sets the properties of an object.
    #[must_use]
    pub fn properties(mut self, properties: Vec<SchemaProperty>) -> Self {
        self.properties = properties;
        self
    }

    /// Sets the element of an array.
    #[must_use]
    pub fn items(mut self, items: SchemaProperty) -> Self {
        self.items = Some(Box::new(items));
        self
    }

    /// Sets the values allowed.
    #[must_use]
    pub fn enum_values(mut self, values: Vec<JsonValue>) -> Self {
        self.enum_values = values;
        self
    }

    /// The children shown under it: its own properties, or those of its element.
    #[must_use]
    pub fn children(&self) -> &[SchemaProperty] {
        if self.properties.is_empty() {
            self.items
                .as_ref()
                .map_or(&[], |items| items.properties.as_slice())
        } else {
            &self.properties
        }
    }

    /// The type shown: an `array` of a known element is `string[]`.
    #[must_use]
    pub fn type_label(&self) -> String {
        match &self.items {
            Some(items) if self.type_name == "array" => format!("{}[]", items.type_name),
            _ => self.type_name.clone(),
        }
    }
}

fn field<'a>(value: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    match value {
        JsonValue::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
        _ => None,
    }
}

fn text_field(value: &JsonValue, key: &str) -> String {
    match field(value, key) {
        Some(JsonValue::String(text)) => text.clone(),
        _ => String::new(),
    }
}

/// The type of a JSON Schema as text: unions joined with `|`, arrays as `item[]`.
fn json_type(schema: &JsonValue) -> String {
    if let Some(JsonValue::Array(union)) = field(schema, "anyOf").or_else(|| field(schema, "oneOf"))
    {
        return union.iter().map(json_type).collect::<Vec<_>>().join(" | ");
    }
    match field(schema, "type") {
        Some(JsonValue::Array(types)) => types
            .iter()
            .filter_map(|t| match t {
                JsonValue::String(t) => Some(t.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" | "),
        Some(JsonValue::String(kind)) if kind == "array" => match field(schema, "items") {
            Some(items) => {
                let inner = json_type(items);
                if inner.contains('|') {
                    format!("({inner})[]")
                } else {
                    format!("{inner}[]")
                }
            }
            None => kind.clone(),
        },
        Some(JsonValue::String(kind)) => kind.clone(),
        _ if field(schema, "properties").is_some() => "object".to_owned(),
        _ if field(schema, "enum").is_some() => "enum".to_owned(),
        _ => "any".to_owned(),
    }
}

/// The `properties` of a JSON Schema object as [`SchemaProperty`] rows, recursively;
/// an array of objects shows its element's properties. It reads `type` (a name or a
/// list), `format`, `description`, `properties`, `required`, `items`, `enum`, `anyOf`
/// and `oneOf`.
///
/// ```
/// use bitop_ui::components::diff_viewer::JsonValue;
/// use bitop_ui_ai::components::schema_display::schema_to_properties;
///
/// let schema = JsonValue::parse(r#"{"properties": {"id": {"type": "string"}}, "required": ["id"]}"#).unwrap();
/// let rows = schema_to_properties(&schema);
/// assert_eq!((rows[0].name.as_str(), rows[0].type_name.as_str(), rows[0].required), ("id", "string", true));
/// ```
#[must_use]
pub fn schema_to_properties(schema: &JsonValue) -> Vec<SchemaProperty> {
    let required: Vec<&str> = match field(schema, "required") {
        Some(JsonValue::Array(names)) => names
            .iter()
            .filter_map(|n| match n {
                JsonValue::String(n) => Some(n.as_str()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    };
    let Some(JsonValue::Object(entries)) = field(schema, "properties") else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|(name, property)| {
            let items = field(property, "items");
            let own = field(property, "properties").map(|_| schema_to_properties(property));
            let item_rows = items
                .filter(|items| field(items, "properties").is_some())
                .map(schema_to_properties);
            SchemaProperty {
                name: name.clone(),
                type_name: json_type(property),
                required: required.contains(&name.as_str()),
                description: text_field(property, "description"),
                format: text_field(property, "format"),
                enum_values: match field(property, "enum") {
                    Some(JsonValue::Array(values)) => values.clone(),
                    _ => Vec::new(),
                },
                properties: own.or(item_rows).unwrap_or_default(),
                items: None,
            }
        })
        .collect()
}

/// [`schema_to_properties`] of a JSON Schema written as text; `None` when it is not
/// JSON.
#[must_use]
pub fn schema_to_properties_from_str(schema: &str) -> Option<Vec<SchemaProperty>> {
    JsonValue::parse(schema).map(|schema| schema_to_properties(&schema))
}

/// The strings of a schema display.
#[derive(Clone, Copy, Debug)]
pub struct SchemaLabels<'a> {
    /// The badge of a required parameter or property.
    pub required: &'a str,
    /// Before the values an enum allows.
    pub one_of: &'a str,
}

impl SchemaLabels<'_> {
    /// The English strings.
    pub const ENGLISH: SchemaLabels<'static> = SchemaLabels {
        required: "required",
        one_of: "One of:",
    };
}

impl Default for SchemaLabels<'_> {
    fn default() -> Self {
        SchemaLabels::ENGLISH
    }
}

/// The display, with the default composition: a header (method, path, title), the
/// description, then the parameters, request body and response sections that have
/// rows.
///
/// - `id` names the nested panels, which the parts number from it.
/// - `parameters_label`, `request_label` and `response_label` are the sections'
///   titles; `labels` the other strings.
/// - `child` follows, for more sections or a [`schema_display_example`].
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn schema_display(
    cx: &Cx,
    id: &str,
    #[default]
    #[into]
    method: Option<HttpMethod>,
    #[default] path: &str,
    #[default] title: &str,
    #[default] description: &str,
    #[default] parameters: &[SchemaParameter],
    #[default] request_body: &[SchemaProperty],
    #[default] response_body: &[SchemaProperty],
    #[default("Parameters")] parameters_label: &str,
    #[default("Request body")] request_label: &str,
    #[default("Response")] response_label: &str,
    #[default] labels: SchemaLabels<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let header = method.is_some() || !path.is_empty() || !title.is_empty();
    let request_id = format!("{id}-request");
    let response_id = format!("{id}-response");
    Ok(Child::new(view! {
        <div
            id=(id)
            data-schema-display=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            if header {
                schema_display_header(
                    if let Some(method) = method {
                        schema_display_method(method: method)
                    }
                    if !path.is_empty() {
                        schema_display_path(path: path)
                    }
                    if !title.is_empty() {
                        schema_display_title((title))
                    }
                )
            }
            if !description.is_empty() {
                schema_display_description((description))
            }
            if !parameters.is_empty() {
                schema_display_parameters(
                    parameters: parameters,
                    title: parameters_label,
                    labels: labels
                )
            }
            if !request_body.is_empty() {
                schema_display_section(
                    title: request_label,
                    schema_display_properties(
                        id: &request_id,
                        properties: request_body,
                        labels: labels
                    )
                )
            }
            if !response_body.is_empty() {
                schema_display_section(
                    title: response_label,
                    schema_display_properties(
                        id: &response_id,
                        properties: response_body,
                        labels: labels
                    )
                )
            }
            (child)
        </div>
    }))
}

/// The header row: method, path and title.
#[component]
pub async fn schema_display_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The method as a badge ("GET"); the text carries the meaning.
#[component]
pub async fn schema_display_method(
    cx: &Cx,
    method: HttpMethod,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-variant=(method.badge())
            data-method=(method.as_str())
            class=(class!(badge_css::ROOT, css::METHOD, attrs.remove("class")))
            (attrs)
        >
            (method.as_str())
        </span>
    })
}

/// Splits a path into text and `{parameters}`.
fn path_parts(path: &str) -> Vec<(bool, &str)> {
    let mut parts = Vec::new();
    let mut rest = path;
    while let Some(open) = rest.find('{') {
        let Some(close) = rest[open..].find('}') else {
            break;
        };
        if open > 0 {
            parts.push((false, &rest[..open]));
        }
        parts.push((true, &rest[open..=open + close]));
        rest = &rest[open + close + 1..];
    }
    if !rest.is_empty() {
        parts.push((false, rest));
    }
    parts
}

/// The path in monospace, with its `{parameters}` set off.
#[component]
pub async fn schema_display_path(
    cx: &Cx,
    path: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let parts: Vec<(bool, String)> = path_parts(path)
        .into_iter()
        .map(|(param, text)| (param, text.to_owned()))
        .collect();
    Ok(view! {
        <code dir="ltr" class=(class!(css::PATH, attrs.remove("class"))) (attrs)>
            for (param, text) in parts {
                if param {
                    <span class=(css::PATH_PARAM)>(text)</span>
                } else {
                    (text)
                }
            }
        </code>
    })
}

/// A title, for a schema that is not an endpoint.
#[component]
pub async fn schema_display_title(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <p class=(class!(css::TITLE, attrs.remove("class"))) (attrs)>(child)</p> })
}

/// The description under the header.
#[component]
pub async fn schema_display_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::DESCRIPTION, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// A section that opens and closes: a `<details>`, open by default, whose `<summary>`
/// shows `title` and `count`, if given.
#[component]
pub async fn schema_display_section(
    cx: &Cx,
    title: &str,
    #[default]
    #[into]
    count: Option<usize>,
    #[default(true)] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-schema-section=""
            class=(class!(css::SECTION, attrs.remove("class")))
            (attrs)
        >
            <summary class=(css::SECTION_TRIGGER)>
                icon(
                    name: Icon::ChevronRight,
                    attrs: attributes! { class=(css::CHEVRON) }
                )
                <span class=(css::SECTION_TITLE)>(title)</span>
                if let Some(count) = count {
                    " "
                    <span
                        data-variant="neutral"
                        class=(class!(badge_css::ROOT, css::COUNT))
                    >
                        (count)
                    </span>
                }
            </summary>
            <div class=(css::PANEL)>(child)</div>
        </details>
    })
}

/// The "required" badge.
#[component]
async fn required_mark(cx: &Cx, label: &str) -> Result<impl View> {
    Ok(view! {
        <span
            data-variant="danger"
            data-required=""
            class=(class!(badge_css::ROOT, css::REQUIRED))
        >
            (label)
        </span>
    })
}

/// The parameters section: a [`schema_display_section`] of [`schema_display_parameter`]
/// rows, with their count.
#[component]
pub async fn schema_display_parameters(
    cx: &Cx,
    parameters: &[SchemaParameter],
    #[default("Parameters")] title: &str,
    #[default(true)] open: bool,
    #[default] labels: SchemaLabels<'_>,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        schema_display_section(
            title: title,
            count: parameters.len(),
            open: open,
            attrs: attrs,
            <ul class=(css::ROWS)>
                for parameter in parameters {
                    schema_display_parameter(parameter: parameter, labels: labels)
                }
            </ul>
        )
    })
}

/// One parameter: its name, type, location and whether it is required, then its
/// description.
#[component]
pub async fn schema_display_parameter(
    cx: &Cx,
    parameter: &SchemaParameter,
    #[default] labels: SchemaLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!(css::ROW, attrs.remove("class"))) data-parameter="" (attrs)>
            <span class=(css::LINE)>
                <span aria-hidden="true" class=(css::SPACER)></span>
                <code dir="ltr" class=(css::NAME)>(&parameter.name)</code>
                " "
                <span dir="ltr" class=(css::TYPE)>(&parameter.type_name)</span>
                if let Some(location) = parameter.location {
                    " "
                    <span
                        data-variant="neutral"
                        data-location=""
                        class=(class!(badge_css::ROOT, css::LOCATION))
                    >
                        (location.as_str())
                    </span>
                }
                if parameter.required {
                    " "
                    required_mark(label: labels.required)
                }
            </span>
            if !parameter.description.is_empty() {
                <span class=(css::PROP_DESCRIPTION)>(&parameter.description)</span>
            }
        </li>
    })
}

/// A list of properties, at `depth` (0 at the top). `id` names their panels.
#[component]
pub async fn schema_display_properties(
    cx: &Cx,
    id: &str,
    properties: &[SchemaProperty],
    #[default] depth: usize,
    #[default] labels: SchemaLabels<'_>,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(property_list(
        cx,
        id.to_owned(),
        properties,
        depth,
        labels,
        attrs,
    ))
}

/// The list, as a boxed view: lists and properties nest each other, and a boxed view
/// ends the recursion of their types.
fn property_list<'a>(
    cx: &'a Cx,
    id: String,
    properties: &'a [SchemaProperty],
    depth: usize,
    labels: SchemaLabels<'a>,
    mut attrs: Attributes,
) -> Child<'a> {
    let extra = attrs.remove("class");
    Child::new(view! {
        <ul class=(class!(css::ROWS, extra)) data-depth=(depth) (attrs)>
            for (index, property) in properties.iter().enumerate() {
                (property_item(
                    cx,
                    format!("{id}-{index}"),
                    property,
                    depth,
                    None,
                    labels,
                    Attributes::default(),
                ))
            }
        </ul>
    })
}

/// One property, nested or not.
fn property_item<'a>(
    cx: &'a Cx,
    id: String,
    property: &'a SchemaProperty,
    depth: usize,
    open: Option<bool>,
    labels: SchemaLabels<'a>,
    mut attrs: Attributes,
) -> Child<'a> {
    let children = property.children();
    let nested = !children.is_empty();
    let open = open.unwrap_or(depth < 2);
    let type_label = property.type_label();
    let values: Vec<String> = property.enum_values.iter().map(JsonValue::format).collect();
    let has_details = !property.description.is_empty() || !values.is_empty();
    let extra = attrs.remove("class");
    let panel = format!("{id}-panel");
    let line = view! {
        <code dir="ltr" class=(css::NAME)>(&property.name)</code>
        " "
        <span dir="ltr" class=(css::TYPE)>
            (type_label)
            if !property.format.is_empty() {
                <span class=(css::FORMAT)>(format!("<{}>", property.format))</span>
            }
        </span>
        if property.required {
            " "
            required_mark(label: labels.required)
        }
    };
    let details = view! {
        if has_details {
            <span class=(css::PROP_DESCRIPTION)>
                (&property.description)
                if !values.is_empty() {
                    <span class=(css::ENUM)>
                        if !property.description.is_empty() {
                            " "
                        }
                        (labels.one_of)
                        " "
                        for (index, value) in values.into_iter().enumerate() {
                            if index > 0 {
                                ", "
                            }
                            <code dir="ltr">(value)</code>
                        }
                    </span>
                }
            </span>
        }
    };
    Child::new(view! {
        <li
            class=(class!(css::ROW, nested.then_some(css::NESTED), extra))
            data-property=(&property.name)
            (attrs)
        >
            if nested {
                <button
                    type="button"
                    aria-expanded=(if open { "true" } else { "false" })
                    aria-controls=(&panel)
                    data-collapsible=""
                    data-open=(open.then_some(""))
                    class=(css::PROPERTY_TRIGGER)
                >
                    icon(
                        name: Icon::ChevronRight,
                        attrs: attributes! { class=(css::CHEVRON) }
                    )
                    (line)
                </button>
                (details)
                <div id=(&panel) hidden=(!open) class=(css::NESTED_PANEL)>
                    (property_list(
                        cx,
                        id.clone(),
                        children,
                        depth + 1,
                        labels,
                        Attributes::default(),
                    ))
                </div>
            } else {
                <span class=(css::LINE)>
                    <span aria-hidden="true" class=(css::SPACER)></span>
                    (line)
                </span>
                (details)
            }
        </li>
    })
}

/// One property, for a list composed by hand: its name, type, format, whether it is
/// required, its description and allowed values, and its children, which open and
/// close. `id` names its panel; `depth` is its level (0 at the top); `open` replaces
/// the default (open above the second level).
#[component]
pub async fn schema_display_property(
    cx: &Cx,
    id: &str,
    property: &SchemaProperty,
    #[default] depth: usize,
    #[default]
    #[into]
    open: Option<bool>,
    #[default] labels: SchemaLabels<'_>,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(property_item(
        cx,
        id.to_owned(),
        property,
        depth,
        open,
        labels,
        attrs,
    ))
}

/// An example payload: a [`code_block`] of JSON (or `language`).
#[component]
pub async fn schema_display_example(
    cx: &Cx,
    code: &str,
    #[default("json")] language: &str,
    #[default] filename: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        code_block(
            code: code,
            language: language,
            filename: filename,
            attrs: attributes! {
                class=(class!(css::EXAMPLE, attrs.remove("class")))
                (attrs)
            }
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        assert_eq!(
            path_parts("/users/{id}/keys/{key}"),
            vec![
                (false, "/users/"),
                (true, "{id}"),
                (false, "/keys/"),
                (true, "{key}")
            ]
        );
        assert_eq!(path_parts("/{broken"), vec![(false, "/{broken")]);
        assert_eq!(path_parts(""), Vec::<(bool, &str)>::new());
    }

    #[test]
    fn json_schema() {
        let rows = schema_to_properties_from_str(
            r#"{
                "type": "object",
                "required": ["id", "lines"],
                "properties": {
                    "id": { "type": "string", "format": "uuid", "description": "The invoice." },
                    "status": { "enum": ["draft", "paid", null] },
                    "total": { "type": ["number", "null"] },
                    "lines": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "required": ["sku"],
                            "properties": { "sku": { "type": "string" }, "qty": { "type": "integer" } }
                        }
                    },
                    "tags": { "type": "array", "items": { "anyOf": [{ "type": "string" }, { "type": "integer" }] } },
                    "customer": { "properties": { "name": { "type": "string" } } },
                    "meta": {}
                }
            }"#,
        )
        .unwrap();
        let summary: Vec<(&str, &str, bool)> = rows
            .iter()
            .map(|r| (r.name.as_str(), r.type_name.as_str(), r.required))
            .collect();
        assert_eq!(
            summary,
            vec![
                ("id", "string", true),
                ("status", "enum", false),
                ("total", "number | null", false),
                ("lines", "object[]", true),
                ("tags", "(string | integer)[]", false),
                ("customer", "object", false),
                ("meta", "any", false),
            ]
        );
        assert_eq!(
            (rows[0].format.as_str(), rows[0].description.as_str()),
            ("uuid", "The invoice.")
        );
        let values: Vec<String> = rows[1].enum_values.iter().map(JsonValue::format).collect();
        assert_eq!(values, ["\"draft\"", "\"paid\"", "null"]);
        // An array of objects shows its element's properties.
        assert_eq!(rows[3].properties.len(), 2);
        assert!(rows[3].properties[0].required);
        assert_eq!(rows[5].children()[0].name, "name");
        assert!(schema_to_properties_from_str("not json").is_none());
    }

    #[test]
    fn type_labels() {
        let array = SchemaProperty::new("ids", "array").items(SchemaProperty::new("id", "string"));
        assert_eq!(array.type_label(), "string[]");
        let object_items = SchemaProperty::new("rows", "array").items(
            SchemaProperty::new("row", "object")
                .properties(vec![SchemaProperty::new("a", "string")]),
        );
        assert_eq!(object_items.children().len(), 1);
        assert_eq!(
            SchemaProperty::new("x", "string[]").type_label(),
            "string[]"
        );
    }
}
