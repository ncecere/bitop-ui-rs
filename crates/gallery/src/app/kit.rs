//! Parts shared by the gallery pages.

use topcoat::{
    Result,
    context::Cx,
    router::request::uri,
    view::{Child, View, component, view},
};

use super::GROUPS;

/// The heading of a component page, taken from its entry in [`GROUPS`].
#[component]
pub async fn page_header(cx: &Cx, slug: &str) -> Result<impl View> {
    let entry = GROUPS
        .iter()
        .flat_map(|group| group.entries)
        .find(|entry| entry.slug == slug)
        .unwrap_or_else(|| panic!("no gallery entry for `{slug}`"));
    Ok(view! {
        <h1>(entry.title)</h1>
        <p
            style="margin-block: var(--space-2) var(--space-6); color: var(--color-text-muted)"
        >
            (entry.summary)
        </p>
    })
}

/// One titled example on a component page.
#[component]
pub async fn example(cx: &Cx, title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section style="margin-block: var(--space-6)">
            <h2
                style="margin-block-end: var(--space-3); color: var(--color-text-muted); font-size: var(--font-size-sm); font-weight: var(--font-weight-medium)"
            >
                (title)
            </h2>
            <div
                style="display: flex; flex-wrap: wrap; gap: var(--space-2) var(--space-4); align-items: center; padding: var(--space-5); border: 1px solid var(--color-border); border-radius: var(--radius-card); background: var(--color-surface)"
            >
                (child)
            </div>
        </section>
    })
}

/// The raw query string of the request, without the `?`.
pub fn query_string(cx: &Cx) -> String {
    uri(cx).query().unwrap_or_default().to_owned()
}

/// The first value of a query parameter that needs no decoding, such as a page
/// number or a column name.
pub fn query_param(cx: &Cx, name: &str) -> Option<String> {
    query_string(cx).split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then(|| value.to_owned())
    })
}

/// A small landscape picture as a data URL, for examples that show an image without
/// a request to another host.
pub const PICTURE: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 40 30'%3E%3Crect width='40' height='30' fill='%234f6bed'/%3E%3Ccircle cx='11' cy='9' r='5' fill='%23fde68a'/%3E%3Cpath d='M0 30 15 15 24 23 30 18 40 26V30z' fill='%23155e75'/%3E%3C/svg%3E";

/// The sample rate of [`silent_wav`]: low, to keep the data URL small.
const WAV_RATE: u32 = 8000;

/// A mono, 8-bit WAV file of `seconds` of silence, so that a media element has real
/// audio to load, play and seek without a request.
pub fn silent_wav(seconds: u32) -> Vec<u8> {
    let samples = WAV_RATE * seconds;
    let mut wav = Vec::with_capacity(44 + samples as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + samples).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&WAV_RATE.to_le_bytes());
    wav.extend_from_slice(&WAV_RATE.to_le_bytes()); // bytes per second
    wav.extend_from_slice(&1u16.to_le_bytes()); // bytes per frame
    wav.extend_from_slice(&8u16.to_le_bytes()); // bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&samples.to_le_bytes());
    // Unsigned 8-bit samples: 128 is silence.
    wav.resize(44 + samples as usize, 128);
    wav
}

/// [`silent_wav`] as a `data:` URL.
pub fn silent_wav_data_url(seconds: u32) -> String {
    format!(
        "data:audio/wav;base64,{}",
        bitop_ui_ai::components::image::base64_encode(&silent_wav(seconds))
    )
}

#[cfg(test)]
mod tests {
    use super::{silent_wav, silent_wav_data_url};

    #[test]
    fn wav_header() {
        let wav = silent_wav(2);
        assert_eq!(wav.len(), 44 + 16_000);
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(
            u32::from_le_bytes(wav[4..8].try_into().unwrap()),
            36 + 16_000
        );
        assert_eq!(&wav[8..16], b"WAVEfmt ");
        assert_eq!(u32::from_le_bytes(wav[24..28].try_into().unwrap()), 8000);
        assert_eq!(&wav[36..40], b"data");
        assert_eq!(u32::from_le_bytes(wav[40..44].try_into().unwrap()), 16_000);
        assert!(wav[44..].iter().all(|&sample| sample == 128));
        assert!(silent_wav_data_url(1).starts_with("data:audio/wav;base64,UklGR"));
    }
}
