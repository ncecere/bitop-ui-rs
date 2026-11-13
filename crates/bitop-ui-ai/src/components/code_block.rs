//! A block of code with a header (the language or a file name, actions and a copy
//! button), optional line numbers and wrapping.
//!
//! The `<pre>` scrolls sideways, so it is in the tab order and the arrow keys scroll
//! it. The copy button is the base set's
//! [`copy_button`] and always copies
//! the raw `code`.
//!
//! ```ignore
//! view! {
//!     code_block(code: "cargo add bitop-ui-ai", language: "bash")
//!     code_block(code: source, filename: "src/main.rs", language: "rust",
//!         show_line_numbers: true, max_height: "20rem")
//! }
//! ```
//!
//! Differences from upstream: there is no `highlight` function, as upstream bundles no
//! highlighter either; the code is plain text in a `<code class="language-…">`, which
//! a highlighter in the browser can pick up. `filename` is text, not any node.

use bitop_ui::components::copy_button::copy_button;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::code_block as css;

/// A readable name for a common language id: `rs` is "Rust", `tsx` is "TSX". An id
/// that is not known is returned as it is, and no id gives `fallback`.
///
/// ```
/// use bitop_ui_ai::components::code_block::language_label;
///
/// assert_eq!(language_label("py", "Code"), "Python");
/// assert_eq!(language_label("zig", "Code"), "zig");
/// assert_eq!(language_label("", "Code"), "Code");
/// ```
#[must_use]
pub fn language_label(language: &str, fallback: &str) -> String {
    if language.is_empty() {
        return fallback.to_owned();
    }
    let known = match language.to_ascii_lowercase().as_str() {
        "js" => "JavaScript",
        "jsx" => "JSX",
        "ts" => "TypeScript",
        "tsx" => "TSX",
        "py" | "python" => "Python",
        "sh" | "zsh" | "shell" => "Shell",
        "bash" => "Bash",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "md" | "markdown" => "Markdown",
        "html" => "HTML",
        "css" => "CSS",
        "sql" => "SQL",
        "go" => "Go",
        "rust" | "rs" => "Rust",
        "text" | "plaintext" => "Text",
        _ => return language.to_owned(),
    };
    known.to_owned()
}

/// A code block.
///
/// - `code` is the text; a final line break is not shown.
/// - `language` is an id such as `rust` or `bash`. It is shown in the header through
///   [`language_label`], set as `data-language`, and given to the `<code>` as a
///   `language-…` class.
/// - `filename` is shown in place of the language.
/// - `show_line_numbers` adds a gutter of line numbers; not with `wrap`.
/// - `wrap` wraps long lines instead of scrolling sideways.
/// - `max_height` is a CSS length; the block scrolls beyond it.
/// - `hide_header` leaves the header out; the copy button then floats in the corner.
/// - `copyable` shows the copy button.
/// - `actions` is a slot for more header buttons, before the copy button.
/// - `fallback_label` is the header's text when there is no language.
/// - `copy_label` names what is copied, as "Copy {label}": `{language}` is replaced
///   with the language's name; `copy_label_plain` is used without a language.
/// - `copy_button_label`, `copied_button_label`, `copied_status` and `failed_status` are
///   the copy button's strings, in which `{label}` is replaced; see
///   [`copy_button`].
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn code_block(
    cx: &Cx,
    code: &str,
    #[default] language: &str,
    #[default] filename: &str,
    #[default] show_line_numbers: bool,
    #[default] wrap: bool,
    #[default] max_height: &str,
    #[default] hide_header: bool,
    #[default(true)] copyable: bool,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default("Code")] fallback_label: &str,
    #[default("{language} code")] copy_label: &str,
    #[default("code")] copy_label_plain: &str,
    #[default("Copy {label}")] copy_button_label: &str,
    #[default("Copied {label}")] copied_button_label: &str,
    #[default("Copied {label} to clipboard")] copied_status: &str,
    #[default("Couldn't copy. Select the text and copy it manually.")] failed_status: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let shown = code.strip_suffix('\n').unwrap_or(code);
    let numbered = show_line_numbers && !wrap;
    let gutter: String = (1..=shown.split('\n').count())
        .map(|line| format!("{line}\n"))
        .collect();
    let label = language_label(language, fallback_label);
    let heading = if filename.is_empty() {
        label.clone()
    } else {
        filename.to_owned()
    };
    let what = if language.is_empty() {
        copy_label_plain.to_owned()
    } else {
        copy_label.replace("{language}", &label)
    };
    let code_class = (!language.is_empty()).then(|| format!("language-{language}"));
    let style = bitop_ui::merge_style(
        &mut attrs,
        &if max_height.is_empty() {
            String::new()
        } else {
            format!("--code-max-height: {max_height}")
        },
    );
    let floating_what = what.clone();
    Ok(view! {
        <div
            class=(class!(css::ROOT, attrs.remove("class")))
            data-code-block=""
            data-language=((!language.is_empty()).then_some(language))
            data-wrap=(wrap.then_some(""))
            style=(style)
            (attrs)
        >
            if !hide_header {
                <div class=(css::HEADER)>
                    <span class=(css::LANGUAGE)>(heading)</span>
                    <span class=(css::ACTIONS)>
                        (actions)
                        if copyable {
                            copy_button(
                                value: code,
                                label: &what,
                                copy_label: copy_button_label,
                                copied_label: copied_button_label,
                                copied_status: copied_status,
                                failed_status: failed_status,
                                attrs: attributes! { class=(css::COPY) }
                            )
                        }
                    </span>
                </div>
            }
            <div class=(css::BODY)>
                if numbered {
                    <span aria-hidden="true" class=(css::GUTTER)>(gutter)</span>
                }
                <pre tabindex="0" class=(css::PRE)>
                    <code class=(code_class)>(shown)</code>
                </pre>
                if hide_header && copyable {
                    <span class=(css::FLOATING)>
                        copy_button(
                            value: code,
                            label: &floating_what,
                            copy_label: copy_button_label,
                            copied_label: copied_button_label,
                            copied_status: copied_status,
                            failed_status: failed_status,
                            attrs: attributes! { class=(css::COPY) }
                        )
                    </span>
                }
            </div>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::language_label;

    #[test]
    fn labels() {
        assert_eq!(language_label("RS", "Code"), "Rust");
        assert_eq!(language_label("yml", "Code"), "YAML");
        assert_eq!(language_label("bash", "Code"), "Bash");
        assert_eq!(language_label("", "Kode"), "Kode");
        assert_eq!(language_label("elixir", "Code"), "elixir");
    }
}
