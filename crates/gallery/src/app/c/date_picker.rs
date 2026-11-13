use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    calendar::Date,
    date_picker::{DatePickerMode, DatePickerSize, date_picker, date_range_presets},
    field::{field, field_description, field_label},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use super::calendar::TODAY;
use crate::app::kit::{example, page_header};

fn day(year: i32, month: u8, day: u8) -> Date {
    Date::new(year, month, day).expect("a real date")
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let today = day(TODAY.0, TODAY.1, TODAY.2);
    let presets = date_range_presets(today);
    let start_id = element_id(cx);
    let start_help_id = element_id(cx);
    let period_id = element_id(cx);
    let trip_id = element_id(cx);
    let small_id = element_id(cx);
    let wide_id = element_id(cx);
    let invalid_id = element_id(cx);
    let locked_id = element_id(cx);
    // Slices are borrowed by the view, so they are bound before it.
    let trip = [day(2026, 11, 24), day(2026, 12, 3)];
    let due = [day(2026, 11, 30)];
    let locked = [day(2026, 11, 18)];
    // A slot's view takes what it uses by value, so it gets a string of its own.
    let trip_popup_id = format!("{trip_id}-popup");

    Ok(view! {
        page_header(slug: "date-picker")
        example(
            title: "One date, in a field",
            <form id="start-form">
                field(
                    field_label(target: &start_id, "Start date")
                    date_picker(
                        id: &start_id,
                        name: "start",
                        label: "Start date",
                        today: Some(today),
                        min: Some(day(2026, 11, 2)),
                        attrs: attributes! { aria-describedby=(&start_help_id) }
                    )
                    field_description(
                        id: &start_help_id,
                        "The first day of the rollout."
                    )
                )
            </form>
        )
        example(
            title: "A range, with presets",
            <form id="period-form">
                date_picker(
                    id: &period_id,
                    name: "period",
                    label: "Report period",
                    mode: DatePickerMode::Range,
                    number_of_months: 2,
                    today: Some(today),
                    max: Some(today),
                    presets: &presets
                )
            </form>
        )
        example(
            title: "A range with a value, kept open, with a footer",
            date_picker(
                id: &trip_id,
                name: "trip",
                label: "Trip",
                mode: DatePickerMode::Range,
                today: Some(today),
                selected: &trip,
                keep_open: true,
                footer: view! {
                    button(
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        attrs: attributes! { popovertarget=(trip_popup_id) popovertargetaction="hide" },
                        "Done"
                    )
                }
            )
        )
        example(
            title: "Small, and full width",
            date_picker(
                id: &small_id,
                name: "due",
                label: "Due date",
                size: DatePickerSize::Sm,
                today: Some(today),
                selected: &due,
                format: "{day} {month} {year}"
            )
            date_picker(
                id: &wide_id,
                name: "birthday",
                label: "Birthday",
                block: true,
                dropdown: true,
                year_range: Some((1940, 2026)),
                today: Some(today),
                max: Some(today)
            )
        )
        example(
            title: "Invalid and disabled",
            date_picker(
                id: &invalid_id,
                name: "expiry",
                label: "Expiry date",
                invalid: true,
                today: Some(today)
            )
            date_picker(
                id: &locked_id,
                name: "locked",
                label: "Locked date",
                disabled: true,
                today: Some(today),
                selected: &locked
            )
        )
    })
}
