use bitop_ui::components::{
    badge::{BadgeSize, BadgeTone, BadgeVariant, badge, status_badge, status_dot},
    layout::{FlexAlign, inline, stack},
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
        page_header(slug: "badge")
        example(
            title: "Tones, variants and sizes",
            stack(
                gap: 3,
                align: FlexAlign::Center,
                inline(
                    gap: 2,
                    badge("Neutral")
                    badge(tone: BadgeTone::Info, "Info")
                    badge(tone: BadgeTone::Success, "Success")
                    badge(tone: BadgeTone::Warning, "Warning")
                    badge(tone: BadgeTone::Danger, "Danger")
                )
                inline(
                    gap: 2,
                    badge(variant: BadgeVariant::Outline, "Outline")
                    badge(variant: BadgeVariant::Outline, tone: BadgeTone::Info, "Beta")
                    badge(size: BadgeSize::Sm, tone: BadgeTone::Info, "sm")
                )
            )
        )
        example(
            title: "Status",
            stack(
                gap: 3,
                align: FlexAlign::Center,
                inline(
                    gap: 2,
                    status_badge(tone: BadgeTone::Success, "Live")
                    status_badge(tone: BadgeTone::Info, pulse: true, "Building")
                    status_badge(tone: BadgeTone::Neutral, "Queued")
                    status_badge(tone: BadgeTone::Warning, "Degraded")
                    status_badge(tone: BadgeTone::Danger, "Failed")
                )
                inline(
                    gap: 3,
                    inline(
                        gap: 2,
                        status_dot(tone: BadgeTone::Success)
                        " "
                        <span>"Healthy"</span>
                    )
                    status_dot(
                        tone: BadgeTone::Danger,
                        pulse: true,
                        label: "Gateway down"
                    )
                )
            )
        )
    })
}
