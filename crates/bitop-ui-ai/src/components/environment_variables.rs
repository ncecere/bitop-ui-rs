//! A list of environment variables whose values are masked until they are revealed.
//!
//! A switch in the header shows or hides every value. Each row has its own toggle
//! button (`aria-pressed`) and a copy button. Flipping the switch resets the rows'
//! own choices, so "Show values" off really hides everything.
//!
//! A masked value is dots for sighted readers and "hidden" for assistive technology;
//! the copy button always copies the real value, in the row's [`CopyFormat`]. Names
//! and values stay left to right in a right-to-left page.
//!
//! The library script reveals and hides values, and keeps the switch, the toggles and
//! their tooltips in step. The value is in the page from the start (hidden), as the
//! copy button needs it; mask only what the reader may see.
//!
//! ```ignore
//! view! {
//!     environment_variables(label: "Production",
//!         environment_variables_header(
//!             environment_variables_title(title: "Production")
//!             environment_variables_toggle()
//!         )
//!         environment_variables_content(
//!             environment_variable(name: "DATABASE_URL", value: &url, required: true)
//!             environment_variable(name: "LOG_LEVEL", value: "info")
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: there is no context. The group is named by `label` with
//! `aria-label` (not by the title, which Playwright's Firefox crashes on), and each row
//! takes its initial `shown` state itself, which should match the group's
//! `show_values`. `onShowValuesChange` is the switch's native `change` event. A row's
//! custom content is its child after the default cells are left out with `custom`.

use bitop_ui::{
    components::{
        checkbox::{SwitchLabelPosition, switch},
        copy_button::copy_button,
        icon::{Icon, icon},
    },
    css::{badge as badge_css, button as button_css, tooltip as tooltip_css},
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::environment_variables as css;

/// What a row's copy button copies. Rendered as `data-copy-format` on the row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopyFormat {
    /// The value alone.
    #[default]
    Value,
    /// The name alone.
    Name,
    /// `export NAME="value"`, for a shell.
    Export,
    /// `NAME=value`, for a `.env` file; quoted as JSON when the value needs it.
    Dotenv,
}

impl CopyFormat {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Name => "name",
            Self::Export => "export",
            Self::Dotenv => "dotenv",
        }
    }
}

/// A variable as text for copying.
///
/// ```
/// use bitop_ui_ai::components::environment_variables::{CopyFormat, format_environment_variable};
///
/// assert_eq!(format_environment_variable("KEY", "a b", CopyFormat::Dotenv), r#"KEY="a b""#);
/// assert_eq!(format_environment_variable("KEY", "$x", CopyFormat::Export), r#"export KEY="\$x""#);
/// ```
#[must_use]
pub fn format_environment_variable(name: &str, value: &str, format: CopyFormat) -> String {
    match format {
        CopyFormat::Value => value.to_owned(),
        CopyFormat::Name => name.to_owned(),
        CopyFormat::Export => {
            let mut escaped = String::with_capacity(value.len());
            for c in value.chars() {
                if matches!(c, '"' | '\\' | '$' | '`') {
                    escaped.push('\\');
                }
                escaped.push(c);
            }
            format!("export {name}=\"{escaped}\"")
        }
        CopyFormat::Dotenv => {
            let needs_quotes = value
                .chars()
                .any(|c| c.is_whitespace() || matches!(c, '#' | '"' | '\''));
            if needs_quotes {
                let quoted =
                    bitop_ui::components::diff_viewer::JsonValue::String(value.to_owned()).format();
                format!("{name}={quoted}")
            } else {
                format!("{name}={value}")
            }
        }
    }
}

/// The list: a group named by `label`.
///
/// - `show_values` says whether the values start revealed; give the switch and the
///   rows the same.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn environment_variables(
    cx: &Cx,
    #[default("Environment variables")] label: &str,
    #[default] show_values: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-env=""
            data-show-values=(show_values.then_some(""))
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The header row, for the title and the switch.
#[component]
pub async fn environment_variables_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The title. `level` makes it a heading (`2` to `6`); by default it is a `<p>`.
#[component]
pub async fn environment_variables_title(
    cx: &Cx,
    #[default("Environment variables")] title: &str,
    #[default] level: u8,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let class = class!(css::TITLE, attrs.remove("class"));
    Ok(view! {
        match level {
            2 => <h2 class=(class) (attrs)>(title)</h2>,
            3 => <h3 class=(class) (attrs)>(title)</h3>,
            4 => <h4 class=(class) (attrs)>(title)</h4>,
            5 => <h5 class=(class) (attrs)>(title)</h5>,
            6 => <h6 class=(class) (attrs)>(title)</h6>,
            _ => <p class=(class) (attrs)>(title)</p>,
        }
    })
}

/// The switch that shows or hides every value, labelled `label` before it.
/// `checked` should match the group's `show_values`. `attrs` are forwarded to the
/// switch's `<input>`.
#[component]
pub async fn environment_variables_toggle(
    cx: &Cx,
    #[default("Show values")] label: &str,
    #[default] checked: bool,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        switch(
            checked: checked,
            label_position: SwitchLabelPosition::Start,
            attrs: attributes! { data-env-toggle="" (attrs) },
            (label)
        )
    })
}

/// The rows: a `<ul>` of [`environment_variable`]s.
#[component]
pub async fn environment_variables_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul class=(class!(css::LIST, attrs.remove("class"))) (attrs)>(child)</ul>
    })
}

/// One variable.
///
/// - `name` and `value` are the variable.
/// - `shown` reveals the value at first.
/// - `required` adds a "Required" badge (`required_label`).
/// - `description` is a short note under the name, such as "Used by the worker".
/// - `copy_format` is what the copy button copies.
/// - `custom` leaves the default cells out; `child` is then the row's content, built
///   from the parts, which take the same `name`, `value` and `shown`.
/// - `tooltip` is an ID for the toggle's tooltip, which says `show_tip` or `hide_tip`.
/// - The other strings are those of [`environment_variable_value`],
///   [`environment_variable_visibility_toggle`] and [`environment_variable_copy_button`].
/// - `attrs` are forwarded to the `<li>`.
#[component]
pub async fn environment_variable(
    cx: &Cx,
    name: &str,
    value: &str,
    #[default] shown: bool,
    #[default] required: bool,
    #[default] description: &str,
    #[default] copy_format: CopyFormat,
    #[default] custom: bool,
    #[default] tooltip: &str,
    #[default("Required")] required_label: &str,
    #[default("hidden")] hidden_label: &str,
    #[default("(empty)")] empty_label: &str,
    #[default("Show {name}")] toggle_label: &str,
    #[default("Show value")] show_tip: &str,
    #[default("Hide value")] hide_tip: &str,
    #[default("Copy {label}")] copy_label: &str,
    #[default("Copied {label}")] copied_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-env-var=""
            data-shown=(shown.then_some(""))
            data-copy-format=(copy_format.as_str())
            class=(class!(css::ROW, attrs.remove("class")))
            (attrs)
        >
            if !custom {
                <div class=(css::NAME_CELL)>
                    <span class=(css::NAME_LINE)>
                        environment_variable_name(name: name)
                        if required {
                            " "
                            environment_variable_required(label: required_label)
                        }
                    </span>
                    if !description.is_empty() {
                        <span class=(css::DESCRIPTION)>(description)</span>
                    }
                </div>
                <div class=(css::VALUE_CELL)>
                    environment_variable_value(
                        value: value,
                        shown: shown,
                        hidden_label: hidden_label,
                        empty_label: empty_label
                    )
                    environment_variable_visibility_toggle(
                        name: name,
                        shown: shown,
                        label: toggle_label,
                        tooltip: tooltip,
                        show_tip: show_tip,
                        hide_tip: hide_tip
                    )
                    environment_variable_copy_button(
                        name: name,
                        value: value,
                        format: copy_format,
                        copy_label: copy_label,
                        copied_label: copied_label
                    )
                </div>
            }
            (child)
        </li>
    })
}

/// The name, in monospace.
#[component]
pub async fn environment_variable_name(
    cx: &Cx,
    name: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <code dir="ltr" class=(class!(css::NAME, attrs.remove("class"))) (attrs)>
            (name)
        </code>
    })
}

/// The value, or `mask_length` dots read as `hidden_label` while it is masked. An empty
/// value shows `empty_label`.
#[component]
pub async fn environment_variable_value(
    cx: &Cx,
    value: &str,
    #[default] shown: bool,
    #[default(8)] mask_length: usize,
    #[default("hidden")] hidden_label: &str,
    #[default("(empty)")] empty_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let dots = "•".repeat(mask_length);
    Ok(view! {
        <code
            dir="ltr"
            data-env-value=""
            data-masked=((!shown).then_some(""))
            class=(class!(css::VALUE, attrs.remove("class")))
            (attrs)
        >
            <span data-env-mask="" hidden=(shown)>
                <span aria-hidden="true">(dots)</span>
                <span class="sr-only">(hidden_label)</span>
            </span>
            <span data-env-plain="" hidden=(!shown)>
                if value.is_empty() {
                    <span class=(css::EMPTY)>(empty_label)</span>
                } else {
                    (value)
                }
            </span>
        </code>
    })
}

/// The toggle that shows or hides the row's value: a button named `label` ("Show
/// {name}") whose `aria-pressed` says whether the value is shown. `tooltip` is an ID for
/// a tooltip that says `show_tip` or `hide_tip`.
#[component]
pub async fn environment_variable_visibility_toggle(
    cx: &Cx,
    name: &str,
    #[default] shown: bool,
    #[default("Show {name}")] label: &str,
    #[default] tooltip: &str,
    #[default("Show value")] show_tip: &str,
    #[default("Hide value")] hide_tip: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let has_tooltip = !tooltip.is_empty();
    Ok(view! {
        <button
            type="button"
            aria-label=(label.replace("{name}", name))
            aria-pressed=(if shown { "true" } else { "false" })
            aria-describedby=(has_tooltip.then_some(tooltip))
            data-tooltip=(has_tooltip.then_some(tooltip))
            data-env-reveal=""
            data-variant="ghost"
            data-size="icon"
            class=(class!(button_css::ROOT, css::ACTION, attrs.remove("class")))
            (attrs)
        >
            icon(name: if shown { Icon::EyeOff } else { Icon::Eye })
        </button>
        if has_tooltip {
            <span
                id=(tooltip)
                role="tooltip"
                popover="manual"
                data-show=(show_tip)
                data-hide=(hide_tip)
                class=(tooltip_css::POPUP)
            >
                (if shown { hide_tip } else { show_tip })
            </span>
        }
    })
}

/// The copy button: copies the variable in `format`, and is named "Copy {name}".
/// `label` replaces the name in its strings.
#[component]
pub async fn environment_variable_copy_button(
    cx: &Cx,
    name: &str,
    value: &str,
    #[default] format: CopyFormat,
    #[default] label: &str,
    #[default("Copy {label}")] copy_label: &str,
    #[default("Copied {label}")] copied_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let text = format_environment_variable(name, value, format);
    let what = if label.is_empty() { name } else { label };
    Ok(view! {
        copy_button(
            value: &text,
            label: what,
            copy_label: copy_label,
            copied_label: copied_label,
            attrs: attributes! {
                class=(class!(css::ACTION, attrs.remove("class")))
                (attrs)
            }
        )
    })
}

/// The "Required" badge.
#[component]
pub async fn environment_variable_required(
    cx: &Cx,
    #[default("Required")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-variant="warning"
            data-env-required=""
            class=(class!(badge_css::ROOT, css::REQUIRED, attrs.remove("class")))
            (attrs)
        >
            (label)
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::{CopyFormat, format_environment_variable as format};

    #[test]
    fn formats() {
        assert_eq!(format("A", "x", CopyFormat::Value), "x");
        assert_eq!(format("A", "x", CopyFormat::Name), "A");
        assert_eq!(format("A", "x", CopyFormat::Export), "export A=\"x\"");
        assert_eq!(
            format("A", "say \"hi\" `now` $HOME \\", CopyFormat::Export),
            "export A=\"say \\\"hi\\\" \\`now\\` \\$HOME \\\\\""
        );
        assert_eq!(format("A", "plain", CopyFormat::Dotenv), "A=plain");
        assert_eq!(
            format("A", "has#hash", CopyFormat::Dotenv),
            "A=\"has#hash\""
        );
        assert_eq!(format("A", "it's", CopyFormat::Dotenv), "A=\"it's\"");
        assert_eq!(
            format("A", "a\"b\nc", CopyFormat::Dotenv),
            "A=\"a\\\"b\\nc\""
        );
        assert_eq!(format("A", "", CopyFormat::Dotenv), "A=");
    }
}
