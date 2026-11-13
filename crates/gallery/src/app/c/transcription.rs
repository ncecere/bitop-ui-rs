use bitop_ui_ai::components::{
    audio_player::audio_player,
    transcription::{TranscriptSegment, TranscriptionLayout, transcription},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, silent_wav_data_url};

const SEGMENTS: &[TranscriptSegment<'static>] = &[
    TranscriptSegment::new(0.0, 2.5, "Welcome back to the show.").speaker("Ana"),
    TranscriptSegment::new(2.5, 5.0, "Today: what changed in the leave policy.").speaker("Ana"),
    TranscriptSegment::new(5.0, 6.0, "  "),
    TranscriptSegment::new(6.0, 8.5, "Parental leave is now sixteen weeks.").speaker("Ben"),
    TranscriptSegment::new(8.5, 12.0, "And it starts on the first day of the job.").speaker("Ben"),
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let audio = silent_wav_data_url(12);
    Ok(view! {
        page_header(slug: "transcription")
        example(
            title: "Following an audio player",
            <div
                data-example="media"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                audio_player(
                    id: "episode",
                    label: "Episode 12",
                    src: &audio,
                    preload: "auto"
                )
                transcription(
                    segments: SEGMENTS,
                    media: "episode-audio",
                    label: "Episode 12 transcript"
                )
            </div>
        )
        example(
            title: "Inline, following data-current-time",
            <div data-example="inline" style="inline-size: 100%">
                transcription(
                    segments: SEGMENTS,
                    layout: TranscriptionLayout::Inline,
                    seekable: true,
                    current_time: 3.0,
                    label: "Inline transcript",
                    attrs: attributes! { id="inline-transcript" }
                )
            </div>
        )
        example(
            title: "Plain text, no timestamps, scrolling",
            <div
                data-example="plain"
                style="inline-size: 100%; max-block-size: 5rem; overflow: auto"
                tabindex="0"
                role="group"
                aria-label="Scrolling transcript"
            >
                transcription(
                    segments: SEGMENTS,
                    show_timestamps: false,
                    auto_scroll: true,
                    current_time: 0.5,
                    label: "Plain transcript",
                    attrs: attributes! { id="plain-transcript" }
                )
            </div>
        )
    })
}
