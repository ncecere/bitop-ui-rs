//! The reading direction of a part of the page.
//!
//! In HTML the direction is the `dir` attribute, and everything in this library reads
//! it from there: the styles use logical properties, and the scripts ask the browser
//! for the computed direction of the component they act on, so arrow keys, popups,
//! sliders and carousels mirror by themselves. Set `dir` on `<html>` for a whole page,
//! or wrap a section in [`direction`] to give it another direction than the page's.
//!
//! [`Direction::of_language`] picks the direction for a language tag, such as the one
//! in an `Accept-Language` header or a user's settings.
//!
//! Differences from upstream: upstream's `DirectionProvider` tells Base UI's components
//! the direction and does not set `dir`; the page has to set it as well. Here the `dir`
//! attribute is the only source, so there is no provider to keep in step with it and no
//! `useDirection` hook. A popup is a child of the element it is written in, not moved
//! to the end of `<body>`, so it has its section's direction without being told.
//!
//! ```ignore
//! view! {
//!     direction(dir: Direction::of_language("ar"),
//!         <p>"مرحبا"</p>
//!     )
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, component, view},
};

/// A reading direction. Rendered as the `dir` attribute.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// Left to right.
    #[default]
    Ltr,
    /// Right to left.
    Rtl,
    /// Decided by the browser from the first strong character of the text, for text
    /// that users wrote.
    Auto,
}

impl Direction {
    /// The value of the `dir` attribute.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
            Self::Auto => "auto",
        }
    }

    /// The direction a language is written in, from a BCP 47 tag such as `ar`, `he-IL`
    /// or `ku-Arab`. A script subtag decides when there is one (`az-Arab` is right to
    /// left, `ur-Latn` is not); otherwise the language does. Unknown tags are left to
    /// right.
    #[must_use]
    pub fn of_language(tag: &str) -> Self {
        // Scripts written right to left, and the languages written in one by default.
        const RTL_SCRIPTS: &[&str] = &[
            "arab", "hebr", "thaa", "syrc", "nkoo", "adlm", "rohg", "mand", "samr",
        ];
        const RTL_LANGUAGES: &[&str] = &[
            "ar", "arc", "ckb", "dv", "fa", "he", "iw", "ks", "nqo", "ps", "sd", "syr", "ug", "ur",
            "yi",
        ];
        let mut subtags = tag.split(['-', '_']).map(str::to_ascii_lowercase);
        let language = subtags.next().unwrap_or_default();
        let script = subtags
            .find(|subtag| subtag.len() == 4 && subtag.chars().all(|c| c.is_ascii_alphabetic()));
        let rtl = match script {
            Some(script) => RTL_SCRIPTS.contains(&script.as_str()),
            None => RTL_LANGUAGES.contains(&language.as_str()),
        };
        if rtl { Self::Rtl } else { Self::Ltr }
    }
}

/// A section with its own reading direction: a `<div>` with the `dir` attribute.
///
/// Components inside it mirror: their layout, their arrow keys and the side their
/// popups open on. `lang` sets the language of the section too, when it differs from
/// the page's.
#[component]
pub async fn direction(
    cx: &Cx,
    #[default] dir: Direction,
    #[default] lang: &str,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            dir=(dir.as_str())
            if !lang.is_empty() {
                lang=(lang)
            }
            data-direction=""
            (attrs)
        >
            (child)
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn language_decides_without_a_script() {
        for tag in ["ar", "he", "fa-IR", "ur_PK", "AR-eg", "yi", "ckb"] {
            assert_eq!(Direction::of_language(tag), Direction::Rtl, "{tag}");
        }
        for tag in ["en", "en-US", "fr", "ja", "zh-Hant-TW", "", "x"] {
            assert_eq!(Direction::of_language(tag), Direction::Ltr, "{tag}");
        }
    }

    #[test]
    fn script_overrides_the_language() {
        assert_eq!(Direction::of_language("az-Arab"), Direction::Rtl);
        assert_eq!(Direction::of_language("ku-Arab-IQ"), Direction::Rtl);
        assert_eq!(Direction::of_language("ur-Latn"), Direction::Ltr);
        assert_eq!(Direction::of_language("ar-Latn-EG"), Direction::Ltr);
    }

    #[test]
    fn attribute_values() {
        assert_eq!(Direction::Ltr.as_str(), "ltr");
        assert_eq!(Direction::Rtl.as_str(), "rtl");
        assert_eq!(Direction::Auto.as_str(), "auto");
    }
}
