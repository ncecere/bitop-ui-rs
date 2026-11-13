use bitop_ui::components::button::{ButtonSize, ButtonVariant, button};
use bitop_ui_ai::components::plan::{
    PlanStepStatus, plan, plan_content, plan_footer, plan_header, plan_step,
};
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
        page_header(slug: "plan")
        example(
            title: "Every step state, with actions",
            <div data-example="states" style="inline-size: 100%">
                plan(
                    id: "migration",
                    label: "Migrate the billing service",
                    plan_header(
                        id: "migration",
                        title: "Migrate the billing service",
                        description: "4 steps · about 20 minutes",
                        actions: view! {
                            button(
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                "Edit"
                            )
                        }
                    )
                    plan_content(
                        id: "migration",
                        plan_step(
                            status: PlanStepStatus::Complete,
                            "Snapshot the database"
                        )
                        plan_step(status: PlanStepStatus::Active, "Run the migration")
                        plan_step("Switch traffic to the new service")
                        plan_step("Remove the old tables")
                    )
                    plan_footer(
                        button(
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            "Revise"
                        )
                        button(size: ButtonSize::Sm, "Approve plan")
                    )
                )
            </div>
        )
        example(
            title: "Streaming",
            <div
                data-example="streaming"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                plan(
                    id: "live-plan",
                    label: "Draft the quarterly report",
                    streaming: true,
                    plan_header(
                        id: "live-plan",
                        streaming: true,
                        title: "Draft the quarterly report",
                        description: "Planning…"
                    )
                    plan_content(
                        id: "live-plan",
                        plan_step(status: PlanStepStatus::Active, "Collect the numbers")
                    )
                )
                <div style="display: flex; gap: var(--space-2)">
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        attrs: attributes! {
                            data-gallery-attr="data-streaming"
                            data-gallery-target="live-plan"
                        },
                        "Finish streaming"
                    )
                </div>
            </div>
        )
        example(
            title: "Collapsed",
            <div data-example="collapsed" style="inline-size: 100%">
                plan(
                    id: "collapsed-plan",
                    label: "Clean up old branches",
                    open: false,
                    plan_header(
                        id: "collapsed-plan",
                        open: false,
                        title: "Clean up old branches"
                    )
                    plan_content(
                        id: "collapsed-plan",
                        open: false,
                        plan_step(
                            status: PlanStepStatus::Complete,
                            "List merged branches"
                        )
                        plan_step(status: PlanStepStatus::Complete, "Delete them")
                    )
                )
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                plan(
                    id: "german-plan",
                    label: "Bericht entwerfen",
                    plan_header(
                        id: "german-plan",
                        title: "Bericht entwerfen",
                        toggle_label: "Schritte zeigen"
                    )
                    plan_content(
                        id: "german-plan",
                        label: "Schritte",
                        plan_step(
                            status: PlanStepStatus::Complete,
                            complete_label: "erledigt",
                            "Zahlen sammeln"
                        )
                    )
                )
            </div>
        )
    })
}
