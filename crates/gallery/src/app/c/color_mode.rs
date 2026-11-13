use bitop_ui::components::{
    button::ButtonVariant,
    color_mode::{color_mode_select, color_mode_toggle},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "color-mode")
        example(
            title: "Toggle between light and dark",
            color_mode_toggle()
            color_mode_toggle(label: "Dark theme", variant: ButtonVariant::Outline)
        )
        example(title: "Select, with the system setting", color_mode_select())
    })
}
