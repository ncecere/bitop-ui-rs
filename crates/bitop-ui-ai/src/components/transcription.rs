//! A transcript in timed segments. The segment playing is highlighted with
//! `aria-current` and a bar at its start edge, not by colour alone, and a segment that
//! is a button seeks to its start on click, Enter and Space.
//!
//! The library script follows a media element: give `media` the ID of an `<audio>` or
//! `<video>` (an [`audio_player`]'s, say) and the highlight follows its playback, and a
//! segment sets its `currentTime`. Without a media element, set `data-current-time` on
//! the transcript and the highlight follows that. Before seeking, a cancelable
//! `bitop:transcription-seek` event goes out from the transcript with `detail.time`,
//! for a page with a player of its own.
//!
//! ```ignore
//! let segments = [
//!     TranscriptSegment::new(0.0, 4.2, "Welcome back to the show.").speaker("Ana"),
//!     TranscriptSegment::new(4.2, 9.0, "Today: what changed in the leave policy."),
//! ];
//! view! {
//!     audio_player(id: "episode", label: "Episode 12", src: "/episode-12.mp3")
//!     transcription(segments: &segments, media: "episode-audio")
//! }
//! ```
//!
//! Differences from upstream: `onSeek` and a controlled `currentTime` are the media
//! element, the event and `data-current-time` above, and `current_time` is the time
//! rendered on the server. Segments are rendered by the component; there is no
//! render function or `render` prop.
//!
//! [`audio_player`]: super::audio_player::audio_player

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

use crate::css::transcription as css;

/// One segment of a transcript, with times in seconds.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TranscriptSegment<'a> {
    /// When it starts, in seconds.
    pub start: f64,
    /// When it ends, in seconds: the next segment's start, usually.
    pub end: f64,
    /// What is said. A segment of blank text is left out.
    pub text: &'a str,
    /// Shown before the text in the list layout.
    pub speaker: &'a str,
}

impl<'a> TranscriptSegment<'a> {
    /// A segment from `start` to `end` seconds.
    #[must_use]
    pub const fn new(start: f64, end: f64, text: &'a str) -> Self {
        Self {
            start,
            end,
            text,
            speaker: "",
        }
    }

    /// Adds who speaks.
    #[must_use]
    pub const fn speaker(mut self, speaker: &'a str) -> Self {
        self.speaker = speaker;
        self
    }
}

/// How a [`transcription`] is laid out. Rendered as `data-layout`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TranscriptionLayout {
    /// One row per segment, with its time.
    #[default]
    List,
    /// The text flows as a paragraph; the times are read but not shown.
    Inline,
}

impl TranscriptionLayout {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Inline => "inline",
        }
    }
}

/// Seconds as `m:ss`, or `h:mm:ss` from an hour.
///
/// ```
/// use bitop_ui_ai::components::transcription::format_timestamp;
///
/// assert_eq!(format_timestamp(65.4), "1:05");
/// assert_eq!(format_timestamp(3725.0), "1:02:05");
/// ```
#[must_use]
pub fn format_timestamp(seconds: f64) -> String {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let s = if seconds.is_finite() {
        seconds.max(0.0).floor() as u64
    } else {
        0
    };
    let (h, m, sec) = (s / 3600, (s % 3600) / 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{sec:02}")
    } else {
        format!("{m}:{sec:02}")
    }
}

struct Row<'a> {
    segment: TranscriptSegment<'a>,
    stamp: String,
    active: bool,
    past: bool,
}

/// A transcript: a region named by `label`, with an ordered list of segments.
/// Segments with no text are left out.
///
/// - `media` is the ID of the media element it follows and seeks.
/// - `seekable` makes each segment a button; it is on by default with `media`.
/// - `current_time` is the time highlighted when the page is rendered.
/// - `layout`, `show_timestamps` (list layout) and `auto_scroll`, which keeps the
///   playing segment in view inside a scrolling container.
/// - `attrs` are forwarded to the root `<div>`.
#[component]
pub async fn transcription(
    cx: &Cx,
    segments: &[TranscriptSegment<'_>],
    #[default("Transcript")] label: &str,
    #[default] media: &str,
    #[default]
    #[into]
    seekable: Option<bool>,
    #[default] current_time: f64,
    #[default] layout: TranscriptionLayout,
    #[default(true)] show_timestamps: bool,
    #[default] auto_scroll: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let seek = seekable.unwrap_or(!media.is_empty());
    let hide_time = layout == TranscriptionLayout::Inline || !show_timestamps;
    let list = layout == TranscriptionLayout::List;
    let rows: Vec<Row<'_>> = segments
        .iter()
        .filter(|segment| !segment.text.trim().is_empty())
        .map(|segment| Row {
            segment: *segment,
            stamp: if layout == TranscriptionLayout::Inline {
                format!("{} ", format_timestamp(segment.start))
            } else {
                format_timestamp(segment.start)
            },
            active: current_time >= segment.start && current_time < segment.end,
            past: current_time >= segment.end,
        })
        .collect();
    let time_class = if hide_time {
        format!("{} sr-only", css::TIME)
    } else {
        css::TIME.to_owned()
    };
    Ok(view! {
        <div
            role="region"
            aria-label=(label)
            data-transcription=""
            data-layout=(layout.as_str())
            data-media=((!media.is_empty()).then_some(media))
            data-auto-scroll=(auto_scroll.then_some(""))
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            <ol class=(css::LIST)>
                for row in rows {
                    <li class=(css::ITEM) data-active=(row.active.then_some(""))>
                        if seek {
                            <button
                                type="button"
                                aria-current=(row.active.then_some("true"))
                                data-active=(row.active.then_some(""))
                                data-past=(row.past.then_some(""))
                                data-interactive=""
                                data-transcription-segment=""
                                data-start=(row.segment.start)
                                data-end=(row.segment.end)
                                class=(css::SEGMENT)
                            >
                                <span class=(time_class.as_str())>(row.stamp)</span>
                                if list {
                                    " "
                                    if !row.segment.speaker.is_empty() {
                                        <span class=(css::SPEAKER)>(row.segment.speaker)</span>
                                        " "
                                    }
                                }
                                <span class=(css::TEXT)>(row.segment.text)</span>
                            </button>
                        } else {
                            <span
                                aria-current=(row.active.then_some("true"))
                                data-active=(row.active.then_some(""))
                                data-past=(row.past.then_some(""))
                                data-transcription-segment=""
                                data-start=(row.segment.start)
                                data-end=(row.segment.end)
                                class=(css::SEGMENT)
                            >
                                <span class=(time_class.as_str())>(row.stamp)</span>
                                if list {
                                    " "
                                    if !row.segment.speaker.is_empty() {
                                        <span class=(css::SPEAKER)>(row.segment.speaker)</span>
                                        " "
                                    }
                                }
                                <span class=(css::TEXT)>(row.segment.text)</span>
                            </span>
                        }
                    </li>
                }
            </ol>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::format_timestamp;

    #[test]
    fn timestamps() {
        assert_eq!(format_timestamp(0.0), "0:00");
        assert_eq!(format_timestamp(9.99), "0:09");
        assert_eq!(format_timestamp(600.0), "10:00");
        assert_eq!(format_timestamp(-3.0), "0:00");
        assert_eq!(format_timestamp(f64::NAN), "0:00");
    }
}
