use bitop_ui::components::button::{ButtonVariant, button};
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
        page_header(slug: "toast")
        example(
            title: "Tones and actions",
            button(
                variant: ButtonVariant::Secondary,
                attrs: attributes! {
                    data-gallery-toast="success"
                    data-gallery-title="Deployment ready"
                    data-gallery-description="marketing-site is live."
                },
                "Success"
            )
            button(
                variant: ButtonVariant::Secondary,
                attrs: attributes! {
                    data-gallery-toast="info"
                    data-gallery-title="Build queued"
                },
                "Info"
            )
            button(
                variant: ButtonVariant::Secondary,
                attrs: attributes! {
                    data-gallery-toast="warning"
                    data-gallery-title="Slow build"
                    data-gallery-description="Took 4m, 3× the usual time."
                },
                "Warning"
            )
            button(
                variant: ButtonVariant::Secondary,
                attrs: attributes! {
                    data-gallery-toast="danger"
                    data-gallery-title="Deployment failed"
                    data-gallery-description="Check the build logs."
                },
                "Error"
            )
            button(
                variant: ButtonVariant::Secondary,
                attrs: attributes! {
                    data-gallery-toast="neutral"
                    data-gallery-title="Token revoked"
                    data-gallery-action="Undo"
                    data-gallery-action-toast="Token restored"
                },
                "With action"
            )
        )
    })
}
