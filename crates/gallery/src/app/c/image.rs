use bitop_ui_ai::components::image::image;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{PICTURE, example, page_header};

/// A 4 by 3 teal PNG, as bytes a model API might return.
const PNG: [u8; 73] = [
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 4, 0, 0, 0, 3, 8, 2, 0,
    0, 0, 59, 150, 57, 145, 0, 0, 0, 16, 73, 68, 65, 84, 120, 156, 99, 224, 47, 203, 131, 35, 6,
    156, 28, 0, 217, 209, 11, 101, 122, 58, 68, 97, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "image")
        example(
            title: "From a URL, with an aspect ratio",
            <div data-example="url" style="inline-size: 16rem">
                image(
                    src: PICTURE,
                    alt: "A lake below a mountain at dusk",
                    aspect_ratio: "4 / 3"
                )
            </div>
        )
        example(
            title: "From bytes and from base64",
            <div data-example="bytes" style="display: flex; gap: var(--space-4)">
                <span style="inline-size: 8rem">
                    image(bytes: &PNG, alt: "A teal tile")
                </span>
                <span style="inline-size: 8rem">
                    image(
                        base64: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
                        alt: "One pixel",
                        aspect_ratio: "1",
                        framed: false
                    )
                </span>
            </div>
        )
        example(
            title: "Failed to load",
            <div data-example="error" style="display: flex; gap: var(--space-4)">
                <span style="inline-size: 14rem">
                    image(
                        src: "data:image/png;base64,AAAA",
                        alt: "A chart of monthly sign-ups"
                    )
                </span>
                <span style="inline-size: 14rem">
                    image(alt: "", frame_attrs: attributes! { data-missing="" })
                </span>
                <span style="inline-size: 14rem" lang="de">
                    image(
                        src: "data:image/png;base64,AAAA",
                        alt: "Ein Diagramm",
                        fallback: "Bild nicht verfügbar"
                    )
                </span>
            </div>
        )
    })
}
