use bitop_ui::components::{
    alert::{AlertTone, alert},
    button::{ButtonSize, ButtonVariant, button},
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
        page_header(slug: "alert")
        example(
            title: "Tones",
            <div style="display: grid; gap: var(--space-3); inline-size: 100%">
                alert(
                    title: "Maintenance tonight",
                    "The service restarts at 02:00 UTC."
                )
                alert(
                    tone: AlertTone::Success,
                    title: "Saved",
                    "Your changes are live."
                )
                alert(
                    tone: AlertTone::Warning,
                    title: "Quota almost used",
                    "You have used 90% of this month's tokens."
                )
                alert(
                    tone: AlertTone::Danger,
                    title: "Could not save",
                    "The server did not respond."
                )
            </div>
        )
        example(
            title: "With an action, and dismissible",
            <div style="display: grid; gap: var(--space-3); inline-size: 100%">
                alert(
                    tone: AlertTone::Danger,
                    actions: view! {
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Outline,
                            "Try again"
                        )
                    },
                    "Could not load the conversation."
                )
                alert(
                    dismissible: true,
                    hide_icon: true,
                    "This alert can be dismissed."
                )
            </div>
        )
    })
}
