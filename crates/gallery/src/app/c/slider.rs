use bitop_ui::components::slider::{SliderSize, slider};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let volume_id = element_id(cx);
    let zoom_id = element_id(cx);
    let opacity_id = element_id(cx);
    let level_id = element_id(cx);
    let locked_id = element_id(cx);
    Ok(view! {
        page_header(slug: "slider")
        example(
            title: "With a label and a value",
            <form id="volume-form" style="inline-size: 100%; max-inline-size: 20rem">
                slider(
                    id: &volume_id,
                    name: "volume",
                    label: "Volume",
                    value: 40.0,
                    show_value: true
                )
            </form>
        )
        example(
            title: "Steps, a formatted value and value text",
            <div style="inline-size: 100%; max-inline-size: 20rem">
                slider(
                    id: &zoom_id,
                    name: "zoom",
                    label: "Zoom",
                    min: 25.0,
                    max: 400.0,
                    step: 25.0,
                    value: 100.0,
                    show_value: true,
                    format: "{value}%",
                    value_text: "{value} percent"
                )
            </div>
        )
        example(
            title: "Small, named by aria-label",
            <div style="inline-size: 100%; max-inline-size: 20rem">
                slider(
                    id: &opacity_id,
                    name: "opacity",
                    max: 1.0,
                    step: 0.1,
                    value: 0.5,
                    size: SliderSize::Sm,
                    attrs: attributes! { aria-label="Opacity" }
                )
            </div>
        )
        example(
            title: "Vertical",
            <div style="block-size: 10rem">
                slider(
                    id: &level_id,
                    name: "level",
                    label: "Level",
                    hide_label: true,
                    value: 30.0,
                    vertical: true
                )
            </div>
        )
        example(
            title: "Disabled",
            <div style="inline-size: 100%; max-inline-size: 20rem">
                slider(
                    id: &locked_id,
                    label: "Disabled",
                    value: 60.0,
                    show_value: true,
                    disabled: true
                )
            </div>
        )
    })
}
