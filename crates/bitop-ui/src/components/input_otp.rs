//! A field for a one-time code, with one input per character.
//!
//! The library script joins the slots into one field: typing advances to the next
//! slot, Backspace goes back, the arrow keys, Home and End move between slots, and
//! pasting a whole code fills every slot. The first slot has
//! `autocomplete="one-time-code"`, so that a code from a text message can be filled
//! in. The code is submitted with the form under `name`, through a hidden input; a
//! `change` event fires on that input when the code changes.
//!
//! `id` goes on the first slot, so a [`field_label`](super::field::field_label) can
//! point at it; without a visible label, pass `label`. The other slots are named
//! "Character 2 of 6" and so on.
//!
//! `validation` picks the characters that are accepted, and the keyboard a phone
//! shows; `normalize` changes their case as they are typed or pasted, such as upper
//! case for a recovery code. `mask` hides the characters as a password field does.
//! As in Base UI, only one slot is in the tab order: the first empty one, or the
//! one that had focus.
//!
//! Differences from upstream: there is no `onValueComplete`; listen for `change` on
//! the hidden input and compare the length of its value. `normalizeValue` is the
//! [`OtpNormalize`] choice of case, not a function.
//!
//! ```ignore
//! view! {
//!     field(
//!         field_label(target: &id, "Verification code")
//!         input_otp(id: &id, name: "code", length: 6, groups: &[3, 3])
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::input_otp as css;

/// The size of an [`input_otp`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputOtpSize {
    Sm,
    #[default]
    Md,
}

impl InputOtpSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

/// The characters an [`input_otp`] accepts. Rendered as `data-validation`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpValidation {
    /// Digits, with a numeric keyboard.
    #[default]
    Numeric,
    /// Latin letters and digits.
    Alphanumeric,
    /// Latin letters only.
    Alpha,
    /// Any character.
    None,
}

impl OtpValidation {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Alphanumeric => "alphanumeric",
            Self::Alpha => "alpha",
            Self::None => "none",
        }
    }

    fn accepts(self, c: char) -> bool {
        match self {
            Self::Numeric => c.is_ascii_digit(),
            Self::Alphanumeric => c.is_ascii_alphanumeric(),
            Self::Alpha => c.is_ascii_alphabetic(),
            Self::None => !c.is_whitespace(),
        }
    }
}

/// How an [`input_otp`] changes what is typed. Rendered as `data-normalize`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpNormalize {
    /// As typed.
    #[default]
    None,
    /// Upper case, such as for a recovery code.
    Upper,
    /// Lower case.
    Lower,
}

impl OtpNormalize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Upper => "upper",
            Self::Lower => "lower",
        }
    }

    fn apply(self, text: &str) -> String {
        match self {
            Self::None => text.to_owned(),
            Self::Upper => text.to_uppercase(),
            Self::Lower => text.to_lowercase(),
        }
    }
}

/// Splits `length` slots into groups of the given sizes. What the sizes leave over
/// becomes a last group; sizes past `length` are dropped.
fn split_groups(length: usize, groups: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut left = length;
    for &group in groups {
        if left == 0 {
            break;
        }
        let count = group.clamp(1, left);
        out.push(count);
        left -= count;
    }
    if left > 0 {
        out.push(left);
    }
    out
}

/// One slot, as it is rendered.
struct Slot {
    id: Option<String>,
    label: Option<String>,
    value: String,
    autocomplete: &'static str,
    tabindex: &'static str,
}

/// A one-time-code field.
///
/// - `id` goes on the first slot; `name` on the hidden input that holds the code.
/// - `length` is the number of characters. `groups` splits the slots into visual
///   groups with a separator between them, such as `&[3, 3]` for "123-456".
/// - `value` is the initial code.
/// - `label` names the field when it has no visible label.
/// - `slot_label` names the slots after the first; `{position}` and `{length}` are
///   replaced.
/// - `validation` is which characters are accepted (digits by default);
///   `normalize` changes their case.
/// - `mask` hides the characters, as a password field does.
/// - `invalid` marks every slot with `aria-invalid`.
/// - `attrs` are forwarded to the wrapper, which has `role="group"`.
#[component]
pub async fn input_otp(
    cx: &Cx,
    id: &str,
    name: &str,
    length: usize,
    #[default] groups: &[usize],
    #[default] value: &str,
    #[default] label: &str,
    #[default("Character {position} of {length}")] slot_label: &str,
    #[default] size: InputOtpSize,
    #[default] validation: OtpValidation,
    #[default] normalize: OtpNormalize,
    #[default] mask: bool,
    #[default] disabled: bool,
    #[default] invalid: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let chars: Vec<char> = normalize
        .apply(value)
        .chars()
        .filter(|c| validation.accepts(*c))
        .take(length)
        .collect();
    let code: String = chars.iter().collect();
    // The one slot in the tab order: the first empty one.
    let active = chars.len().min(length.saturating_sub(1));
    let mut slots = (0..length).map(|index| Slot {
        id: (index == 0).then(|| id.to_owned()),
        label: if index == 0 {
            (!label.is_empty()).then(|| label.to_owned())
        } else {
            Some(
                slot_label
                    .replace("{position}", &(index + 1).to_string())
                    .replace("{length}", &length.to_string()),
            )
        },
        value: chars.get(index).map(char::to_string).unwrap_or_default(),
        autocomplete: if index == 0 { "one-time-code" } else { "off" },
        tabindex: if index == active { "0" } else { "-1" },
    });
    let groups: Vec<Vec<Slot>> = split_groups(length, groups)
        .into_iter()
        .map(|count| slots.by_ref().take(count).collect())
        .collect();
    Ok(view! {
        <div
            role="group"
            aria-label=((!label.is_empty()).then_some(label))
            data-otp=""
            data-size=(size.as_str())
            data-validation=(validation.as_str())
            data-normalize=(normalize.as_str())
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            for (index, group) in groups.into_iter().enumerate() {
                if index > 0 {
                    <div class=(css::SEPARATOR) aria-hidden="true">
                        icon(name: Icon::Minus)
                    </div>
                }
                <div class=(css::GROUP)>
                    for slot in group {
                        <input
                            type=(if mask { "password" } else { "text" })
                            id=(slot.id)
                            tabindex=(slot.tabindex)
                            inputmode=(if validation == OtpValidation::Numeric {
                                "numeric"
                            } else {
                                "text"
                            })
                            autocomplete=(slot.autocomplete)
                            autocapitalize="off"
                            spellcheck="false"
                            aria-label=(slot.label)
                            aria-invalid=(invalid.then_some("true"))
                            value=(slot.value)
                            disabled=(disabled)
                            data-otp-slot=""
                            class=(css::SLOT)
                        >
                    }
                </div>
            }
            <input type="hidden" name=(name) value=(code) disabled=(disabled)>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::split_groups;

    #[test]
    fn splits_groups() {
        assert_eq!(split_groups(6, &[]), [6]);
        assert_eq!(split_groups(6, &[3, 3]), [3, 3]);
        assert_eq!(split_groups(6, &[2]), [2, 4]);
        assert_eq!(split_groups(6, &[4, 4, 4]), [4, 2]);
        assert_eq!(split_groups(4, &[0, 3]), [1, 3]);
    }
}
