use bitop_ui::components::{
    button::{ButtonVariant, button, icon_button},
    checkbox::switch,
    drawer::{
        DrawerSide, drawer, drawer_body, drawer_close, drawer_description, drawer_footer,
        drawer_header, drawer_title, drawer_trigger,
    },
    field::{field, field_label},
    icon::{Icon, icon},
    input::input,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

/// Upstream's example wraps the drawer's content in a column at most 24rem wide, so
/// that the header, the body and the full-width footer buttons stay that wide.
const STACK: &str = "display: flex; flex: 1; flex-direction: column; min-block-size: 0; inline-size: 100%; max-inline-size: 24rem; margin-inline: auto";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let goal_id = element_id(cx);
    let goal_title_id = element_id(cx);
    let goal_description_id = element_id(cx);
    let goal_value_id = element_id(cx);
    let sides = [
        ("Top", DrawerSide::Top, "up", element_id(cx), element_id(cx)),
        (
            "Right",
            DrawerSide::Right,
            "right",
            element_id(cx),
            element_id(cx),
        ),
        (
            "Bottom",
            DrawerSide::Bottom,
            "down",
            element_id(cx),
            element_id(cx),
        ),
        (
            "Left",
            DrawerSide::Left,
            "left",
            element_id(cx),
            element_id(cx),
        ),
    ];
    let rename_id = element_id(cx);
    let rename_title_id = element_id(cx);
    let project_id = element_id(cx);

    Ok(view! {
        page_header(slug: "drawer")
        example(
            title: "Bottom sheet",
            drawer_trigger(target: &goal_id, "Set daily goal")
            drawer(
                id: &goal_id,
                labelled_by: &goal_title_id,
                described_by: &goal_description_id,
                <div style=(STACK)>
                    drawer_header(
                        drawer_title(id: &goal_title_id, "Move goal")
                        drawer_description(
                            attrs: attributes! { id=(&goal_description_id) },
                            "Set your daily activity goal."
                        )
                    )
                    drawer_body(
                        <div
                            style="display: flex; align-items: center; justify-content: center; gap: var(--space-4); padding: var(--space-4) 0"
                        >
                            icon_button(
                                label: "Decrease goal by 10",
                                variant: ButtonVariant::Secondary,
                                attrs: attributes! {
                                    data-gallery-step="-10"
                                    data-gallery-target=(&goal_value_id)
                                },
                                icon(name: Icon::Minus)
                            )
                            <div
                                aria-live="polite"
                                style="display: flex; flex-direction: column; align-items: center; min-inline-size: 6rem; font-size: var(--font-size-3xl); font-weight: var(--font-weight-bold); font-variant-numeric: tabular-nums; letter-spacing: var(--letter-spacing-tight)"
                            >
                                <span id=(&goal_value_id) data-min="200" data-max="500">
                                    "350"
                                </span>
                                <span
                                    style="color: var(--color-text-muted); font-size: var(--font-size-xs); font-weight: var(--font-weight-medium); letter-spacing: var(--letter-spacing-caps); text-transform: uppercase"
                                >
                                    "Calories / day"
                                </span>
                            </div>
                            icon_button(
                                label: "Increase goal by 10",
                                variant: ButtonVariant::Secondary,
                                attrs: attributes! {
                                    data-gallery-step="10"
                                    data-gallery-target=(&goal_value_id)
                                },
                                icon(name: Icon::Plus)
                            )
                        </div>
                    )
                    drawer_footer(
                        drawer_close(
                            target: &goal_id,
                            variant: ButtonVariant::Primary,
                            attrs: attributes! { data-gallery-goal=(&goal_value_id) },
                            "Save goal"
                        )
                        drawer_close(target: &goal_id, "Cancel")
                    )
                </div>
            )
        )
        example(
            title: "Sides",
            for (label, side, swipe, id, title_id) in &sides {
                drawer_trigger(target: id, (*label))
                drawer(
                    id: id,
                    labelled_by: title_id,
                    side: *side,
                    show_close: true,
                    drawer_header(
                        drawer_title(id: title_id, "Notification settings")
                        drawer_description(
                            "Swipe "
                            (*swipe)
                            " or press Escape to dismiss."
                        )
                    )
                    drawer_body(
                        switch(checked: true, "Deployment failures")
                        switch("Weekly usage report")
                        switch(checked: true, "New team members")
                    )
                    drawer_footer(drawer_close(target: id, "Done"))
                )
            }
        )
        example(
            title: "Controlled with a form",
            drawer_trigger(
                target: &rename_id,
                variant: ButtonVariant::Primary,
                "Rename project…"
            )
            drawer(
                id: &rename_id,
                labelled_by: &rename_title_id,
                side: DrawerSide::Right,
                show_close: true,
                <form
                    method="dialog"
                    style=(STACK)
                    data-gallery-submit-toast="Project renamed"
                >
                    drawer_header(
                        drawer_title(id: &rename_title_id, "Rename project")
                        drawer_description(
                            "The URL of existing deployments won't change."
                        )
                    )
                    drawer_body(
                        field(
                            field_label(target: &project_id, "Project name")
                            input(
                                attrs: attributes! { id=(&project_id) name="project" value="marketing-site" }
                            )
                        )
                    )
                    drawer_footer(
                        button(attrs: attributes! { type="submit" }, "Save")
                        drawer_close(target: &rename_id, "Cancel")
                    )
                </form>
            )
        )
    })
}
