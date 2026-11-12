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
//! Differences from upstream: there is no `onValueComplete`; listen for `change` on
//! the hidden input and compare the length of its value.
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
/// - `alphanumeric` accepts letters as well as digits.
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
    #[default] alphanumeric: bool,
    #[default] disabled: bool,
    #[default] invalid: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let accepts = |c: &char| {
        if alphanumeric {
            c.is_alphanumeric()
        } else {
            c.is_ascii_digit()
        }
    };
    let chars: Vec<char> = value.chars().filter(accepts).take(length).collect();
    let code: String = chars.iter().collect();
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
            data-alphanumeric=(alphanumeric)
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
                            type="text"
                            id=(slot.id)
                            inputmode=(if alphanumeric { "text" } else { "numeric" })
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
