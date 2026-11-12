use bitop_ui::components::kbd::{KbdSize, kbd, kbd_shortcut};
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
        page_header(slug: "kbd")
        example(
            title: "Keys",
            kbd("K")
            kbd(size: KbdSize::Sm, "Esc")
        )
        example(
            title: "Shortcuts",
            kbd_shortcut(keys: &["mod", "K"])
            kbd_shortcut(keys: &["mod", "shift", "P"], size: KbdSize::Sm)
        )
    })
}
