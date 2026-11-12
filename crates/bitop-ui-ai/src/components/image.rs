//! A generated image from a URL, base64 text or bytes, with a shimmering placeholder
//! while it loads and a text fallback if it fails.
//!
//! `alt` is required: describe the image, or pass `""` only when it is decorative.
//! The library script marks the frame loaded or failed (`data-state`), which removes
//! the placeholder or shows the fallback; it also handles images that finished before
//! it ran.
//!
//! ```ignore
//! view! {
//!     image(base64: &result.base64, media_type: "image/png",
//!         alt: "A lighthouse at dusk", aspect_ratio: "1 / 1")
//!     image(bytes: &png, alt: "A chart of monthly sign-ups")
//!     image(src: "/renders/42.png", alt: "The floor plan")
//! }
//! ```
//!
//! Differences from upstream: bytes become a `data:` URL on the server, not an object
//! URL, so nothing needs revoking. `fallback` is a text label, followed by the alt text.
//! `className` and `frameClassName` are `attrs` (on the `<img>`) and `frame_attrs`.

use bitop_ui::components::icon::{Icon, icon};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::css::image as css;

const BASE64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encodes bytes as standard base64 with padding.
///
/// ```
/// use bitop_ui_ai::components::image::base64_encode;
///
/// assert_eq!(base64_encode(b"Man"), "TWFu");
/// assert_eq!(base64_encode(b"Ma"), "TWE=");
/// ```
#[must_use]
pub fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for (i, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            if i <= chunk.len() {
                out.push(char::from(BASE64[((n >> shift) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// The URL for an image given as a URL, base64 text or bytes, in that order of
/// preference. Base64 text that already starts with `data:` is used as it is.
///
/// ```
/// use bitop_ui_ai::components::image::image_url;
///
/// assert_eq!(image_url("", "iVBO", &[], "image/png"), "data:image/png;base64,iVBO");
/// assert_eq!(image_url("/a.png", "iVBO", &[], "image/png"), "/a.png");
/// ```
#[must_use]
pub fn image_url(src: &str, base64: &str, bytes: &[u8], media_type: &str) -> String {
    let media_type = if media_type.is_empty() {
        "image/png"
    } else {
        media_type
    };
    if !src.is_empty() {
        src.to_owned()
    } else if base64.starts_with("data:") {
        base64.to_owned()
    } else if !base64.is_empty() {
        format!("data:{media_type};base64,{base64}")
    } else if !bytes.is_empty() {
        format!("data:{media_type};base64,{}", base64_encode(bytes))
    } else {
        String::new()
    }
}

/// An image in a frame.
///
/// - `alt` is the text alternative.
/// - `src`, `base64` or `bytes` is the image, with `media_type` for the last two.
/// - `aspect_ratio` reserves space before the image loads, such as `16 / 9`.
/// - `framed` draws a rounded frame and a ring.
/// - `fallback` is shown with the alt text when the image fails to load.
/// - `attrs` are forwarded to the `<img>`, and `frame_attrs` to the frame.
#[component]
pub async fn image(
    cx: &Cx,
    alt: &str,
    #[default] src: &str,
    #[default] base64: &str,
    #[default] bytes: &[u8],
    #[default("image/png")] media_type: &str,
    #[default] aspect_ratio: &str,
    #[default(true)] framed: bool,
    #[default("Image unavailable")] fallback: &str,
    #[default] mut frame_attrs: Attributes,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let url = image_url(src, base64, bytes, media_type);
    let state = if url.is_empty() { "error" } else { "loading" };
    let style = bitop_ui::merge_style(
        &mut frame_attrs,
        &if aspect_ratio.is_empty() {
            String::new()
        } else {
            format!("--image-aspect: {aspect_ratio}")
        },
    );
    let has_url = !url.is_empty();
    Ok(view! {
        <span
            class=(class!(css::FRAME, frame_attrs.remove("class")))
            data-image-frame=""
            data-state=(state)
            data-framed=(framed.then_some(""))
            data-aspect=((!aspect_ratio.is_empty()).then_some(""))
            aria-busy=(has_url.then_some("true"))
            style=(style)
            (frame_attrs)
        >
            if has_url {
                <img
                    decoding="async"
                    src=(&url)
                    alt=(alt)
                    data-image=""
                    class=(class!(css::IMG, attrs.remove("class")))
                    (attrs)
                >
            }
            <span class=(css::FALLBACK)>
                icon(
                    name: Icon::ImageOff,
                    attrs: attributes! { class=(css::FALLBACK_ICON) }
                )
                <span>
                    (fallback)
                    if !alt.is_empty() {
                        <span class=(css::FALLBACK_ALT)>(alt)</span>
                    }
                </span>
            </span>
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::{base64_encode, image_url};

    #[test]
    fn encodes() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        assert_eq!(base64_encode(&[0xff, 0xfe, 0xfd, 0x00]), "//79AA==");
    }

    #[test]
    fn urls() {
        assert_eq!(
            image_url("", "", b"foo", "image/webp"),
            "data:image/webp;base64,Zm9v"
        );
        assert_eq!(
            image_url("", "data:image/gif;base64,R0lG", &[], "image/png"),
            "data:image/gif;base64,R0lG"
        );
        assert_eq!(image_url("", "", &[], "image/png"), "");
        assert_eq!(image_url("", "AAAA", &[], ""), "data:image/png;base64,AAAA");
    }
}
