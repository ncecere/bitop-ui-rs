//! An `<audio>` element with controls of the library's own: play and pause, optional
//! skip back and forward, a seek slider with the elapsed and total time, mute, volume
//! and playback speed.
//!
//! Every control is a native button, a range input (arrow keys, Page Up and Page Down,
//! Home and End) or a `<select>`, so the player works from the keyboard. The seek
//! slider is read as "1:05 of 3:20" and the volume as a percentage. The library script
//! drives the element and follows its events, so the controls also keep up with a
//! script that plays, pauses or seeks the element itself, whose ID is `{id}-audio`.
//!
//! ```ignore
//! view! {
//!     audio_player(id: "speech", label: "Generated speech", base64: &speech.base64,
//!         media_type: "audio/mpeg")
//!     audio_player(id: "episode", label: "Episode 12", src: "/audio/episode-12.mp3",
//!         skip_seconds: 15)
//! }
//! ```
//!
//! Differences from upstream: `onPlay`, `onPause`, `onEnded` and `onTimeUpdate` are
//! the `<audio>` element's own events, and `audioRef` its ID. Bytes become a `data:`
//! URL on the server, as in [`image`](super::image), rather than an object URL. Extra
//! `<track>` elements go in `child`.

use bitop_ui::{
    components::{
        icon::{Icon, icon},
        select::select,
        slider::{SliderSize, slider},
    },
    css::button as button_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::image::image_url;
use crate::css::audio_player as css;

/// Seconds as `m:ss`, or `h:mm:ss` from an hour; `0:00` for a time that is not known.
///
/// ```
/// use bitop_ui_ai::components::audio_player::format_time;
///
/// assert_eq!(format_time(200.0), "3:20");
/// assert_eq!(format_time(f64::NAN), "0:00");
/// ```
#[must_use]
pub fn format_time(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "0:00".to_owned();
    }
    super::transcription::format_timestamp(seconds)
}

/// The speeds offered by default.
pub const DEFAULT_RATES: &[f64] = &[0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

/// An audio player.
///
/// - `id` is the base of the parts' IDs; the `<audio>` is `{id}-audio`.
/// - `label` names the player, a group.
/// - The source is `src`, `base64` text (or a `data:` URL) or `bytes`, in that order;
///   `media_type` is the type of the last two.
/// - `rates` are the speeds offered (none hides the menu); `rate` is the first.
/// - `skip_seconds` adds buttons that jump back and forward that many seconds.
/// - `show_volume` shows the volume slider beside the mute button.
/// - `autoplay`, `looped` and `preload` are the element's.
/// - `play_label`, `pause_label`, `back_label` and `forward_label` (with `{n}`),
///   `seek_label`, `seek_text` (with `{current}` and `{total}`), `unknown_duration`,
///   `unknown_time`, `mute_label`, `volume_label`, `rate_label` and `rate_format`
///   (with `{rate}`) are the strings.
/// - `child` goes inside the `<audio>`, for `<track>` elements.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn audio_player(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] src: &str,
    #[default] base64: &str,
    #[default] bytes: &[u8],
    #[default("audio/mpeg")] media_type: &str,
    #[default(DEFAULT_RATES)] rates: &[f64],
    #[default(1.0)] rate: f64,
    #[default] skip_seconds: u32,
    #[default(true)] show_volume: bool,
    #[default] autoplay: bool,
    #[default] looped: bool,
    #[default("metadata")] preload: &str,
    #[default("Play")] play_label: &str,
    #[default("Pause")] pause_label: &str,
    #[default("Back {n} seconds")] back_label: &str,
    #[default("Forward {n} seconds")] forward_label: &str,
    #[default("Seek")] seek_label: &str,
    #[default("{current} of {total}")] seek_text: &str,
    #[default("unknown duration")] unknown_duration: &str,
    #[default("--:--")] unknown_time: &str,
    #[default("Mute")] mute_label: &str,
    #[default("Volume")] volume_label: &str,
    #[default("Playback speed")] rate_label: &str,
    #[default("{rate}×")] rate_format: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let url = image_url(src, base64, bytes, media_type);
    let audio_id = format!("{id}-audio");
    let seek_id = format!("{id}-seek");
    let volume_id = format!("{id}-volume");
    let n = skip_seconds.to_string();
    let back = back_label.replace("{n}", &n);
    let forward = forward_label.replace("{n}", &n);
    let options: Vec<(String, String, bool)> = rates
        .iter()
        .map(|r| {
            let value = r.to_string();
            let text = rate_format.replace("{rate}", &value);
            (value, text, (*r - rate).abs() < f64::EPSILON)
        })
        .collect();
    let seek_value_text = seek_text
        .replace("{current}", "0:00")
        .replace("{total}", unknown_duration);
    let button_class = format!("{} {}", button_css::ROOT, css::BUTTON);
    let no_source = url.is_empty();
    Ok(Child::new(view! {
        <div
            role="group"
            aria-label=(label)
            data-audio-player=""
            data-play-label=(play_label)
            data-pause-label=(pause_label)
            data-seek-text=(seek_text)
            data-unknown-duration=(unknown_duration)
            data-unknown-time=(unknown_time)
            data-skip=(skip_seconds)
            data-rate=(rate)
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <audio
                id=(&audio_id)
                src=((!no_source).then_some(url.as_str()))
                preload=(preload)
                autoplay=(autoplay)
                loop=(looped)
                class=(css::AUDIO)
            >
                (child)
            </audio>
            <button
                type="button"
                aria-label=(play_label)
                disabled=(no_source)
                data-player-play=""
                data-variant="outline"
                data-size="icon"
                class=(class!(button_css::ROOT, css::BUTTON, css::PLAY))
            >
                icon(name: Icon::Play)
                icon(name: Icon::Pause)
            </button>
            if skip_seconds > 0 {
                <button
                    type="button"
                    aria-label=(&back)
                    disabled=(true)
                    data-player-back=""
                    data-variant="ghost"
                    data-size="icon"
                    class=(button_class.as_str())
                >
                    icon(name: Icon::RotateCcw)
                </button>
                <button
                    type="button"
                    aria-label=(&forward)
                    disabled=(true)
                    data-player-forward=""
                    data-variant="ghost"
                    data-size="icon"
                    class=(button_class.as_str())
                >
                    icon(name: Icon::RotateCw)
                </button>
            }
            <span class=(css::TIME) data-player-current="">"0:00"</span>
            <div class=(css::SEEK)>
                slider(
                    id: &seek_id,
                    max: 1.0,
                    step: 1.0,
                    size: SliderSize::Sm,
                    disabled: true,
                    attrs: attributes! {
                        aria-label=(seek_label)
                        aria-valuetext=(&seek_value_text)
                        data-player-seek=""
                    }
                )
            </div>
            <span class=(css::TIME) data-player-duration="">(unknown_time)</span>
            <button
                type="button"
                aria-label=(mute_label)
                aria-pressed="false"
                data-player-mute=""
                data-variant="ghost"
                data-size="icon"
                class=(button_class.as_str())
            >
                icon(name: Icon::Volume2)
            </button>
            if show_volume {
                <div class=(css::VOLUME)>
                    slider(
                        id: &volume_id,
                        value: 1.0,
                        max: 1.0,
                        step: 0.05,
                        size: SliderSize::Sm,
                        attrs: attributes! {
                            aria-label=(volume_label)
                            aria-valuetext="100%"
                            data-player-volume=""
                        }
                    )
                </div>
            }
            if !options.is_empty() {
                <span class=(css::RATE)>
                    select(
                        attrs: attributes! { aria-label=(rate_label) data-player-rate="" },
                        for (value, text, chosen) in options {
                            <option value=(value) selected=(chosen)>(text)</option>
                        }
                    )
                </span>
            }
        </div>
    }))
}

#[cfg(test)]
mod tests {
    use super::format_time;

    #[test]
    fn times() {
        assert_eq!(format_time(0.0), "0:00");
        assert_eq!(format_time(-1.0), "0:00");
        assert_eq!(format_time(f64::INFINITY), "0:00");
        assert_eq!(format_time(3600.0), "1:00:00");
    }
}
