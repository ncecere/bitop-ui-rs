use bitop_ui_ai::components::audio_player::audio_player;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header, silent_wav_data_url};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let clip = silent_wav_data_url(20);
    let short = silent_wav_data_url(3);
    Ok(view! {
        page_header(slug: "audio-player")
        example(
            title: "With skip buttons",
            <div data-example="full" style="inline-size: 100%">
                audio_player(
                    id: "episode",
                    label: "Episode 12",
                    src: &clip,
                    skip_seconds: 5,
                    preload: "auto"
                )
            </div>
        )
        example(
            title: "Speech, without volume or speeds",
            <div data-example="compact" style="inline-size: 100%">
                audio_player(
                    id: "speech",
                    label: "Generated speech",
                    base64: &short,
                    show_volume: false,
                    rates: &[]
                )
            </div>
        )
        example(
            title: "Nothing to play yet",
            <div data-example="empty" style="inline-size: 100%">
                audio_player(id: "pending", label: "Pending speech")
            </div>
        )
        example(
            title: "Other strings, starting fast",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                audio_player(
                    id: "german",
                    label: "Folge 3",
                    src: &short,
                    rate: 1.5,
                    rates: &[1.0, 1.5],
                    play_label: "Abspielen",
                    pause_label: "Pause",
                    seek_label: "Position",
                    seek_text: "{current} von {total}",
                    unknown_duration: "unbekannte Dauer",
                    mute_label: "Stumm",
                    volume_label: "Lautstärke",
                    rate_label: "Geschwindigkeit"
                )
            </div>
        )
    })
}
