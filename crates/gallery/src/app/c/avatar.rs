use bitop_ui::components::avatar::{AvatarShape, AvatarSize, avatar};
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
            title: "Sizes and shapes",
            avatar(name: "Ada Lovelace", size: AvatarSize::Xs)
            avatar(name: "Grace Hopper", size: AvatarSize::Sm)
            avatar(name: "Alan Turing")
            avatar(name: "Acme Inc", shape: AvatarShape::Square, size: AvatarSize::Lg)
            avatar(name: "Katherine Johnson", size: AvatarSize::Xl)
        )
        example(
            title: "Initials",
            avatar(name: "IT Help Desk", shape: AvatarShape::Square)
            avatar(name: "Go docs (signed-in)", shape: AvatarShape::Square)
            avatar(name: "Ludwig van Beethoven")
            avatar(name: "Cher")
        )
        example(
            title: "With a picture, and with one that fails to load",
            avatar(name: "With Picture", src: PIXEL)
            avatar(name: "Broken Picture", src: "data:image/png;base64,AAAA")
        )
        example(
            title: "Decorative, beside the name",
            <span
                style="display: inline-flex; align-items: center; gap: var(--space-2)"
            >
                avatar(name: "Ada Lovelace", size: AvatarSize::Sm, decorative: true)
                "Ada Lovelace"
            </span>
        )
    })
}
