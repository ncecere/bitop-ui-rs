//! A person's picture, or their initials when there is no picture.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::avatar as css;

/// The size of an [`avatar`]. Rendered as `data-size`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// Lowercase name particles and joining words, passed over for the second initial
/// ("Ludwig van Beethoven" gives "LB"). Capitalised, they count ("Van Morrison").
const PARTICLES: &[&str] = &[
    "a", "al", "and", "bin", "bint", "da", "das", "de", "del", "della", "der", "des", "di", "do",
    "dos", "du", "el", "for", "ibn", "la", "le", "of", "the", "ten", "ter", "van", "von", "y",
    "zu",
];

/// Whether `c` is a combining mark, which belongs to the letter before it.
fn is_mark(c: char) -> bool {
    matches!(
        c,
        '\u{0300}'..='\u{036F}'
            | '\u{1AB0}'..='\u{1AFF}'
            | '\u{1DC0}'..='\u{1DFF}'
            | '\u{20D0}'..='\u{20FF}'
            | '\u{FE20}'..='\u{FE2F}'
    )
}

/// A word's first letter or digit with any marks after it, and whether the word is a
/// lowercase particle. `None` for a word with no letters or digits, such as "&".
fn word_start(word: &str) -> Option<(String, bool)> {
    let at = word.find(char::is_alphanumeric)?;
    let mut chars = word[at..].chars();
    let mut start: String = chars.next().into_iter().collect();
    start.extend(chars.take_while(|&c| is_mark(c)));
    let letters: String = word.chars().filter(|c| c.is_alphabetic()).collect();
    Some((start, PARTICLES.contains(&letters.as_str())))
}

/// Up to two initials, as bitop-ui makes them: the first letter or digit of the first
/// two words ("IT Help Desk" gives "IH"). Punctuation is skipped ("Go docs
/// (signed-in)" gives "GD"), the second initial passes over lowercase particles
/// ("Charles de Gaulle" gives "CG") unless only particles follow, and a name with no
/// letters or digits gives "?".
#[must_use]
pub fn initials(name: &str) -> String {
    let words: Vec<_> = name.split_whitespace().filter_map(word_start).collect();
    let Some((first, rest)) = words.split_first() else {
        return "?".to_owned();
    };
    let second = rest.iter().find(|(_, particle)| !particle).or(rest.first());
    let mut out = first.0.clone();
    if let Some((start, _)) = second {
        out.push_str(start);
    }
    out.to_uppercase()
}

/// An avatar for the person called `name`.
///
/// The initials are always rendered and the image, if `src` is given, covers them. The
/// library script removes an image that fails to load, which leaves the initials.
#[component]
pub async fn avatar(
    cx: &Cx,
    name: &str,
    #[default]
    #[into]
    src: Option<&str>,
    #[default] size: AvatarSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            role="img"
            aria-label=(name)
            class=(class!(css::ROOT, attrs.remove("class")))
            data-size=(size.as_str())
            (attrs)
        >
            <span aria-hidden="true">(initials(name))</span>
            if let Some(src) = src {
                <img class=(css::IMAGE) src=(src) alt="" data-avatar-image="">
            }
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::initials;

    #[test]
    fn initials_match_bitop_ui() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("IT Help Desk"), "IH");
        assert_eq!(initials("Go docs (signed-in)"), "GD");
        assert_eq!(initials("Ludwig van Beethoven"), "LB");
        assert_eq!(initials("Office of the Registrar"), "OR");
        assert_eq!(initials("Charles de Gaulle"), "CG");
        assert_eq!(initials("Van Morrison"), "VM");
        assert_eq!(initials("Maria de"), "MD");
        assert_eq!(initials("Research & Development"), "RD");
        assert_eq!(initials("Plato"), "P");
        assert_eq!(initials("  "), "?");
        assert_eq!(initials("— ("), "?");
        assert_eq!(initials("émile zola"), "ÉZ");
        assert_eq!(initials("e\u{301}mile zola"), "E\u{301}Z");
    }
}
