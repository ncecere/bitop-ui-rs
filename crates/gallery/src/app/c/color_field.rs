use bitop_ui::components::{
    color_field::{ColorPreset, color_field},
    field::{field, field_label},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

const PRESETS: &[ColorPreset<'static>] = &[
    ColorPreset {
        value: "#1d4ed8",
        label: "Blue",
    },
    ColorPreset {
        value: "#166534",
        label: "Green",
    },
    ColorPreset {
        value: "#b91c1c",
        label: "Red",
    },
    ColorPreset {
        value: "#fde047",
        label: "Yellow",
    },
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let accent_id = element_id(cx);
    let badge_id = element_id(cx);
    let plain_id = element_id(cx);
    let bad_id = element_id(cx);
    let locked_id = element_id(cx);
    Ok(view! {
        page_header(slug: "color-field")
        example(
            title: "With presets and a contrast check",
            <form id="accent-form">
                field(
                    field_label(target: &accent_id, "Accent colour")
                    color_field(
                        id: &accent_id,
                        name: "accent",
                        default_color: "#1d4ed8",
                        contrast_with: "#ffffff",
                        contrast_label: "white text",
                        presets: PRESETS
                    )
                )
            </form>
        )
        example(
            title: "Dark text",
            field(
                field_label(target: &badge_id, "Badge colour")
                color_field(
                    id: &badge_id,
                    name: "badge",
                    default_color: "#fde047",
                    contrast_with: "#111827",
                    contrast_label: "dark text"
                )
            )
        )
        example(
            title: "No contrast check",
            field(
                field_label(target: &plain_id, "Border colour")
                color_field(
                    id: &plain_id,
                    name: "border",
                    value: "#94a3b8",
                    default_color: "#94a3b8"
                )
            )
        )
        example(
            title: "Not a colour",
            field(
                field_label(target: &bad_id, "Link colour")
                color_field(
                    id: &bad_id,
                    name: "link",
                    value: "#12",
                    default_color: "#1d4ed8",
                    contrast_with: "#ffffff"
                )
            )
        )
        example(
            title: "Disabled",
            field(
                field_label(target: &locked_id, "Locked colour")
                color_field(
                    id: &locked_id,
                    name: "locked",
                    value: "#166534",
                    default_color: "#1d4ed8",
                    contrast_with: "#ffffff",
                    presets: PRESETS,
                    disabled: true
                )
            )
        )
    })
}
