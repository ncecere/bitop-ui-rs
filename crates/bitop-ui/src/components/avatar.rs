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

/// Up to two initials from a name: the first letters of its first and last words.
#[must_use]
pub fn initials(name: &str) -> String {
    let mut words = name.split_whitespace();
    let first = words.next().and_then(|word| word.chars().next());
    let last = words.next_back().and_then(|word| word.chars().next());
    first
        .into_iter()
        .chain(last)
        .flat_map(char::to_uppercase)
        .collect()
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
    fn initials_use_first_and_last_words() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("ada king lovelace"), "AL");
        assert_eq!(initials("Plato"), "P");
        assert_eq!(initials("  "), "");
        assert_eq!(initials("émile zola"), "ÉZ");
    }
}
