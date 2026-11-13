use bitop_ui::components::{
    alert::{AlertTone, alert, error_alert},
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
                            variant: ButtonVariant::Secondary,
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
        example(
            title: "Errors, from an HTTP status",
            <div
                data-example="errors"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                error_alert(status: 404, retry_href: "/c/alert?retried=1")
                error_alert(status: 429)
                error_alert(status: 503, message: "The index is rebuilding.")
                error_alert(message: "The upload was interrupted.")
                error_alert(
                    status: 401,
                    title: "Sitzung abgelaufen",
                    message: "Bitte melden Sie sich erneut an.",
                    retry_href: "/c/alert",
                    retry_label: "Erneut versuchen",
                    actions: view! {
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Ghost,
                            "Details"
                        )
                    }
                )
                error_alert(status: 403, tone: AlertTone::Info, hide_icon: true)
            </div>
        )
    })
}
