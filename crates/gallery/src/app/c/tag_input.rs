use bitop_ui::components::{
    field::{field, field_description, field_label},
    tag_input::{TagInputSize, tag_input},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let tags_id = element_id(cx);
    let tags_help_id = element_id(cx);
    let origins_id = element_id(cx);
    let invalid_id = element_id(cx);
    let locked_id = element_id(cx);

    Ok(view! {
        page_header(slug: "tag-input")
        example(
            title: "Tags, five at most",
            <form id="tag-form" style="inline-size: 100%; max-inline-size: 28rem">
                field(
                    field_label(target: &tags_id, "Tags")
                    tag_input(
                        id: &tags_id,
                        name: "tags",
                        tags: &["rust", "web"],
                        max_tags: 5,
                        attrs: attributes! { aria-describedby=(&tags_help_id) }
                    )
                    field_description(id: &tags_help_id, "Press Enter after each tag.")
                )
            </form>
        )
        example(
            title: "Small, another noun, and the case kept",
            <div style="inline-size: 100%; max-inline-size: 28rem">
                field(
                    field_label(target: &origins_id, "Allowed origins")
                    tag_input(
                        id: &origins_id,
                        name: "origins",
                        tags: &["https://Example.edu"],
                        noun: "origin",
                        noun_plural: "origins",
                        keep_case: true,
                        max_tag_length: 24,
                        size: TagInputSize::Sm
                    )
                )
            </div>
        )
        example(
            title: "Invalid",
            <div style="inline-size: 100%; max-inline-size: 28rem">
                field(
                    field_label(target: &invalid_id, "Reviewers")
                    tag_input(
                        id: &invalid_id,
                        name: "reviewers",
                        noun: "reviewer",
                        noun_plural: "reviewers",
                        invalid: true
                    )
                )
            </div>
        )
        example(
            title: "Disabled",
            <div style="inline-size: 100%; max-inline-size: 28rem">
                field(
                    field_label(target: &locked_id, "Labels")
                    tag_input(
                        id: &locked_id,
                        name: "labels",
                        tags: &["archived", "read-only"],
                        disabled: true
                    )
                )
            </div>
        )
    })
}
