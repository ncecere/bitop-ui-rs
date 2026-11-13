use bitop_ui::components::button::ButtonVariant;
use bitop_ui_ai::components::confirmation::{
    ConfirmationState, confirmation, confirmation_accepted, confirmation_action,
    confirmation_actions, confirmation_rejected, confirmation_request, confirmation_title,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header, query_param};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    // The form example's answer comes back in the query, as a server would record it.
    let state = match query_param(cx, "decision").as_deref() {
        Some("approve") => ConfirmationState::Accepted,
        Some("reject") => ConfirmationState::Rejected,
        _ => ConfirmationState::Requested,
    };
    Ok(view! {
        page_header(slug: "confirmation")
        example(
            title: "Answered by the server",
            <div data-example="form" style="inline-size: 100%">
                confirmation(
                    label: "Send the email?",
                    state: state,
                    attrs: attributes! { id="form-confirmation" },
                    confirmation_title("Send the email to 42 recipients?")
                    confirmation_request(
                        "The agent wants to call "
                        <code>"send_email"</code>
                        "."
                    )
                    confirmation_accepted("Approved. Sending…")
                    confirmation_rejected("Rejected. Nothing was sent.")
                    confirmation_actions(
                        action: "/c/confirmation",
                        method: "get",
                        <input type="hidden" name="call" value="call-7">
                        confirmation_action(
                            value: "reject",
                            variant: ButtonVariant::Secondary,
                            "Reject"
                        )
                        confirmation_action(value: "approve", "Approve")
                    )
                )
            </div>
        )
        example(
            title: "Answered in script",
            <div data-example="script" style="inline-size: 100%">
                confirmation(
                    label: "Delete the branch?",
                    attrs: attributes! { id="script-confirmation" data-gallery-confirm="" },
                    confirmation_title("Delete the branch feature/old-billing?")
                    confirmation_request("This can't be undone.")
                    confirmation_accepted("Approved. The branch was deleted.")
                    confirmation_rejected("Rejected. The branch was kept.")
                    confirmation_actions(
                        confirmation_action(
                            value: "reject",
                            variant: ButtonVariant::Secondary,
                            "Keep it"
                        )
                        confirmation_action(
                            value: "approve",
                            variant: ButtonVariant::Danger,
                            "Delete"
                        )
                    )
                )
            </div>
        )
        example(
            title: "Accepted and rejected",
            <div
                data-example="answered"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                confirmation(
                    label: "Run the migration?",
                    state: ConfirmationState::Accepted,
                    confirmation_title("Run the migration?")
                    confirmation_request("The agent wants to call run_migration.")
                    confirmation_accepted("Approved.")
                    confirmation_rejected("Rejected.")
                )
                confirmation(
                    label: "Post to the channel?",
                    state: ConfirmationState::Rejected,
                    confirmation_title("Post to the channel?")
                    confirmation_accepted("Approved.")
                    confirmation_rejected("Rejected. Nothing was posted.")
                )
            </div>
        )
    })
}
