use bitop_ui::components::avatar::{AvatarSize, avatar};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

/// A 1x1 PNG, so the avatar with a picture needs no network request.
const PIXEL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "avatar")
        example(
            title: "Sizes",
            avatar(name: "Ada Lovelace", size: AvatarSize::Sm)
            avatar(name: "Grace Hopper")
            avatar(name: "Alan Turing", size: AvatarSize::Lg)
        )
        example(
            title: "With a picture, and with one that fails to load",
            avatar(name: "With Picture", src: PIXEL)
            avatar(name: "Broken Picture", src: "data:image/png;base64,AAAA")
        )
    })
}
