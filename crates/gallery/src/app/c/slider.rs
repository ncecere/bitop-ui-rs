use bitop_ui::components::{
    layout::{inline, stack},
    slider::{SliderSize, slider},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "slider")
        example(
            title: "Labels, value readout and sizes",
            stack(
                gap: 6,
                attrs: attributes! { style="inline-size: 100%" },
                slider(
                    id: "temperature",
                    name: "temperature",
                    label: "Temperature",
                    value: 0.7,
                    max: 2.0,
                    step: 0.1,
                    show_value: true
                )
                slider(
                    id: "opacity",
                    name: "opacity",
                    label: "Opacity",
                    value: 80.0,
                    step: 5.0,
                    show_value: true,
                    format: "{value}%",
                    value_text: "{value} percent",
                    size: SliderSize::Sm
                )
                slider(
                    id: "volume",
                    value: 40.0,
                    disabled: true,
                    attrs: attributes! { aria-label="Volume" }
                )
            )
        )
        example(
            title: "Range with per-thumb names",
            <form id="price-form" style="inline-size: 100%">
                slider(
                    id: "price",
                    name: "price",
                    label: "Price",
                    range: (40.0, 160.0),
                    max: 250.0,
                    step: 5.0,
                    min_steps_between: 2.0,
                    thumb_labels: &["Minimum price", "Maximum price"],
                    show_value: true,
                    format: "${value}"
                )
            </form>
        )
        example(
            title: "Vertical",
            inline(
                gap: 8,
                nowrap: true,
                slider(
                    id: "bass",
                    name: "bass",
                    value: 60.0,
                    vertical: true,
                    attrs: attributes! { aria-label="Bass" }
                )
                slider(
                    id: "mid",
                    name: "mid",
                    value: 45.0,
                    vertical: true,
                    attrs: attributes! { aria-label="Mid" }
                )
                slider(
                    id: "treble",
                    name: "treble",
                    value: 70.0,
                    vertical: true,
                    attrs: attributes! { aria-label="Treble" }
                )
            )
        )
        example(
            title: "A form with a single value",
            <form id="level-form" style="inline-size: 100%; max-inline-size: 20rem">
                slider(
                    id: "level",
                    name: "level",
                    label: "Level",
                    value: 40.0,
                    show_value: true
                )
            </form>
        )
    })
}
