//! A text input for amounts, with thousands separators and step buttons.
//!
//! While it is not being edited the input shows grouping ("50,000"); on focus it
//! shows the plain number so that it is easy to change. The form always receives the
//! plain value: digits with an optional "." and a leading "-", or "" for empty. Text
//! that is not a number is left as typed, for the server to reject.
//!
//! It is an `<input type="text" inputmode="decimal">` with `role="spinbutton"`, not
//! `type="number"`: number inputs cannot show grouping and change on scroll. The
//! library script formats the text and adds the spin button keyboard pattern: the up
//! and down arrow keys step by `step`, Page Up and Page Down by ten steps, and Home
//! and End go to `min` and `max` when they are set.
//!
//! Differences from upstream: the step buttons are new, and the value is formatted
//! by the script when the page loads, so it is plain until then.
//!
//! ```ignore
//! view! {
//!     number_input(name: "limit", value: "50000", min: "0", step: "1000", unit: "tokens",
//!         attrs: attributes! { id=(&id) }
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use super::icon::{Icon, icon};
use crate::css::{input as input_css, number_input as css};

/// A number input.
///
/// - `value` is the plain value, such as `"50000"` or `"1.5"`, or `""` for empty.
/// - `min`, `max` and `step` are plain numbers too; `step` defaults to 1.
/// - `locale` sets the grouping and the decimal mark, as a BCP 47 tag such as
///   `"de-DE"`; the default is the browser's.
/// - `fraction_digits` is the most fraction digits shown while not editing. Zero
///   also gives touch keyboards without a decimal mark.
/// - `unit` is shown after the input, such as "GiB". It is decorative: put the unit
///   in the label or the description too.
/// - `hide_steppers` leaves out the step buttons.
/// - `attrs` are forwarded to the `<input>`; pass `id`, `required` and so on there.
#[component]
pub async fn number_input(
    cx: &Cx,
    name: &str,
    #[default] value: &str,
    #[default] min: &str,
    #[default] max: &str,
    #[default("1")] step: &str,
    #[default] locale: &str,
    #[default(3)] fraction_digits: u8,
    #[default] unit: &str,
    #[default] disabled: bool,
    #[default] hide_steppers: bool,
    #[default("Decrease")] decrease_label: &str,
    #[default("Increase")] increase_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let some = |text: &str| (!text.is_empty()).then(|| text.to_owned());
    let now = value.parse::<f64>().is_ok().then(|| value.to_owned());
    Ok(view! {
        <span
            class=(css::ROOT)
            data-number-input=""
            data-locale=(some(locale))
            data-step=(step)
            data-fraction-digits=(fraction_digits)
        >
            <span class=(css::FIELD)>
                <input
                    type="text"
                    role="spinbutton"
                    inputmode=(if fraction_digits > 0 { "decimal" } else { "numeric" })
                    autocomplete="off"
                    name=(name)
                    value=(value)
                    aria-valuenow=(now)
                    aria-valuemin=(some(min))
                    aria-valuemax=(some(max))
                    disabled=(disabled)
                    class=(class!(input_css::ROOT, css::INPUT, attrs.remove("class")))
                    (attrs)
                >
                if !hide_steppers {
                    // Not tab stops: the arrow keys do the same from the input.
                    <button
                        type="button"
                        tabindex="-1"
                        class=(css::STEP)
                        data-number-step="-1"
                        aria-label=(decrease_label)
                        disabled=(disabled)
                    >
                        icon(name: Icon::Minus)
                    </button>
                    <button
                        type="button"
                        tabindex="-1"
                        class=(css::STEP)
                        data-number-step="1"
                        aria-label=(increase_label)
                        disabled=(disabled)
                    >
                        icon(name: Icon::Plus)
                    </button>
                }
            </span>
            if !unit.is_empty() {
                <span aria-hidden="true" class=(css::UNIT)>(unit)</span>
            }
        </span>
    })
}
