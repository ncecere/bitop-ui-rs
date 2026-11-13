//! A list of short text tags with an inline input.
//!
//! The server renders the tags it is given. The library script then lets the person
//! change them: Enter, a comma, or Tab with text adds a tag; Backspace in the empty
//! input removes the last one; pasting "a, b, c" adds three. Text that is still in the
//! input is added when the input loses focus, so a tag typed without pressing Enter is
//! not lost when the form is submitted. Each tag has its own Remove button, and a
//! polite status message announces additions and removals.
//!
//! Each tag is submitted with the form as one `name=tag` pair, through a hidden input,
//! so the server reads a list. A `change` event fires on the wrapper when the tags
//! change.
//!
//! `id` goes on the text input, so a [`field_label`](super::field::field_label) can
//! point at it.
//!
//! Differences from upstream: `normalize` is a function there. Here a tag is always
//! trimmed, and lower-cased unless `keep_case` is set. `noun` is the two strings
//! `noun` and `noun_plural`.
//!
//! ```ignore
//! view! {
//!     field(
//!         field_label(target: &id, "Tags")
//!         tag_input(id: &id, name: "tags", tags: &["rust", "web"], max_tags: 20)
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::tag_input as css;

/// The size of a [`tag_input`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TagInputSize {
    Sm,
    #[default]
    Md,
}

impl TagInputSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// Upper-cases the first character, for the list's name.
fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// A tag input.
///
/// - `tags` are the initial tags.
/// - `max_tags` is the most tags allowed, or 0 for no limit; the input is disabled
///   when it is reached. `max_tag_length` is the longest tag, in characters.
/// - `keep_case` keeps tags as typed; by default they are lower-cased.
/// - `noun` and `noun_plural` say what the tags are, such as "origin" and "origins".
///   They replace `{noun}` and `{nouns}` in the messages below.
/// - `placeholder` is shown in the input; `limit_placeholder` when the limit is
///   reached (`{max}` is replaced).
/// - `remove_label` names a tag's Remove button, and `added_message`,
///   `added_many_message` and `removed_message` are announced; `{tag}` and `{count}`
///   are replaced.
/// - `invalid` marks the input with `aria-invalid`.
/// - `attrs` are forwarded to the text `<input>`.
#[component]
pub async fn tag_input(
    cx: &Cx,
    id: &str,
    name: &str,
    #[default] tags: &[&str],
    #[default] max_tags: usize,
    #[default(64)] max_tag_length: usize,
    #[default] keep_case: bool,
    #[default("tag")] noun: &str,
    #[default("tags")] noun_plural: &str,
    #[default("Add {nouns}…")] placeholder: &str,
    #[default("Limit of {max} {nouns} reached")] limit_placeholder: &str,
    #[default("Remove {noun} {tag}")] remove_label: &str,
    #[default("Added {noun} {tag}")] added_message: &str,
    #[default("Added {count} {nouns}")] added_many_message: &str,
    #[default("Removed {noun} {tag}")] removed_message: &str,
    #[default] size: TagInputSize,
    #[default] disabled: bool,
    #[default] invalid: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let template = |text: &str| {
        text.replace("{nouns}", noun_plural)
            .replace("{noun}", noun)
            .replace("{max}", &max_tags.to_string())
    };
    let placeholder = template(placeholder);
    let limit_placeholder = template(limit_placeholder);
    let remove_label = template(remove_label);
    let added_message = template(added_message);
    let added_many_message = template(added_many_message);
    let removed_message = template(removed_message);
    let full = max_tags > 0 && tags.len() >= max_tags;
    let tags: Vec<(&str, String)> = tags
        .iter()
        .map(|tag| (*tag, remove_label.replace("{tag}", tag)))
        .collect();
    Ok(view! {
        <div
            class=(css::ROOT)
            data-tag-input=""
            data-name=(name)
            data-size=(size.as_str())
            data-disabled=(disabled)
            data-max-tags=((max_tags > 0).then(|| max_tags.to_string()))
            data-max-tag-length=(max_tag_length.to_string())
            data-keep-case=(keep_case)
            data-placeholder=(&placeholder)
            data-limit-placeholder=(&limit_placeholder)
            data-remove-label=(&remove_label)
            data-added-message=(&added_message)
            data-added-many-message=(&added_many_message)
            data-removed-message=(&removed_message)
        >
            <ul class=(css::TAGS) aria-label=(capitalize(noun_plural))>
                for (tag, label) in tags {
                    <li class=(css::TAG) data-tag=(tag)>
                        <span class=(css::TAG_TEXT)>(tag)</span>
                        if !disabled {
                            <button
                                type="button"
                                class=(css::REMOVE)
                                aria-label=(label)
                            >
                                icon(name: Icon::X)
                            </button>
                        }
                        <input
                            type="hidden"
                            name=(name)
                            value=(tag)
                            disabled=(disabled)
                        >
                    </li>
                }
            </ul>
            <input
                type="text"
                id=(id)
                autocomplete="off"
                autocapitalize="off"
                spellcheck="false"
                maxlength=((max_tag_length * 4).to_string())
                placeholder=(if full { &limit_placeholder } else { &placeholder })
                aria-invalid=(invalid.then_some("true"))
                disabled=(disabled || full)
                class=(class!(css::INPUT, attrs.remove("class")))
                (attrs)
            >
            <span role="status" class="sr-only"></span>
            // What the script copies to make a new tag.
            <template>
                <li class=(css::TAG)>
                    <span class=(css::TAG_TEXT)></span>
                    <button type="button" class=(css::REMOVE)>
                        icon(name: Icon::X)
                    </button>
                    <input type="hidden" name=(name)>
                </li>
            </template>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::capitalize;

    #[test]
    fn capitalizes() {
        assert_eq!(capitalize("tags"), "Tags");
        assert_eq!(capitalize("élan"), "Élan");
        assert_eq!(capitalize(""), "");
    }
}
