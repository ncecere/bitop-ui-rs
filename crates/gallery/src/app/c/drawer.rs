use bitop_ui::components::{
    button::{ButtonVariant, button},
    drawer::{
        DrawerSide, drawer, drawer_body, drawer_close, drawer_description, drawer_footer,
        drawer_header, drawer_title, drawer_trigger,
    },
    field::{field, field_label},
    input::input,
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
    let goal_id = element_id(cx);
    let goal_title_id = element_id(cx);
    let goal_description_id = element_id(cx);
    let steps_id = element_id(cx);
    let right_id = element_id(cx);
    let right_title_id = element_id(cx);
    let left_id = element_id(cx);
    let left_title_id = element_id(cx);
    let top_id = element_id(cx);
    let top_title_id = element_id(cx);

    Ok(view! {
        page_header(slug: "drawer")
        example(
            title: "From the bottom, with a handle",
            drawer_trigger(target: &goal_id, "Set goal")
            drawer(
                id: &goal_id,
                labelled_by: &goal_title_id,
                described_by: &goal_description_id,
                drawer_header(
                    drawer_title(id: &goal_title_id, "Move goal")
                    drawer_description(
                        attrs: attributes! { id=(&goal_description_id) },
                        "Set your daily activity goal."
                    )
                )
                drawer_body(
                    field(
                        field_label(target: &steps_id, "Steps a day")
                        input(
                            attrs: attributes! { id=(&steps_id) name="steps" type="number" value="8000" }
                        )
                    )
                )
                drawer_footer(
                    button(
                        attrs: attributes! { commandfor=(&goal_id) command="close" },
                        "Submit"
                    )
                    drawer_close(target: &goal_id, "Cancel")
                )
            )
        )
        example(
            title: "Other sides",
            drawer_trigger(target: &right_id, "Open right")
            drawer(
                id: &right_id,
                labelled_by: &right_title_id,
                side: DrawerSide::Right,
                show_close: true,
                drawer_header(
                    drawer_title(id: &right_title_id, "Details")
                    drawer_description(
                        "A drawer on the right edge, with a close button."
                    )
                )
                drawer_body(<p>"Swipe it to the right to dismiss it."</p>)
            )
            drawer_trigger(target: &left_id, "Open left")
            drawer(
                id: &left_id,
                labelled_by: &left_title_id,
                side: DrawerSide::Left,
                show_handle: Some(true),
                drawer_header(
                    drawer_title(id: &left_title_id, "Navigation")
                    drawer_description("A drawer on the left edge, with a handle.")
                )
                drawer_footer(
                    drawer_close(
                        target: &left_id,
                        variant: ButtonVariant::Primary,
                        "Done"
                    )
                )
            )
            drawer_trigger(target: &top_id, "Open top")
            drawer(
                id: &top_id,
                labelled_by: &top_title_id,
                side: DrawerSide::Top,
                drawer_header(
                    drawer_title(id: &top_title_id, "Notifications")
                    drawer_description("A drawer on the top edge.")
                )
                drawer_footer(drawer_close(target: &top_id, "Close"))
            )
        )
    })
}
