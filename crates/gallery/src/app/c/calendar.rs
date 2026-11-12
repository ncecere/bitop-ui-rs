use bitop_ui::components::calendar::{CalendarMode, Date, calendar};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

/// The gallery's "today", fixed so that the page and its checks do not change by the
/// day. It is a Thursday.
pub const TODAY: (i32, u8, u8) = (2026, 11, 12);

fn day(year: i32, month: u8, day: u8) -> Date {
    Date::new(year, month, day).expect("a real date")
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let today = day(TODAY.0, TODAY.1, TODAY.2);
    let single_id = element_id(cx);
    let range_id = element_id(cx);
    let multiple_id = element_id(cx);
    let limits_id = element_id(cx);
    let dropdown_id = element_id(cx);
    let monday_id = element_id(cx);
    let disabled_id = element_id(cx);
    // Slices are borrowed by the view, so they are bound before it.
    let one_day = [day(2026, 11, 18)];
    let stay = [day(2026, 11, 24), day(2026, 12, 3)];
    let shifts = [day(2026, 11, 20), day(2026, 11, 5)];
    let holidays = [day(2026, 11, 26)];

    Ok(view! {
        page_header(slug: "calendar")
        example(
            title: "One day",
            <form id="day-form">
                calendar(
                    id: &single_id,
                    name: "day",
                    today: Some(today),
                    selected: &one_day,
                    attrs: attributes! { aria-label="Delivery day" role="group" }
                )
            </form>
        )
        example(
            title: "A range, over two months",
            <form id="stay-form">
                calendar(
                    id: &range_id,
                    name: "stay",
                    mode: CalendarMode::Range,
                    number_of_months: 2,
                    today: Some(today),
                    selected: &stay
                )
            </form>
        )
        example(
            title: "Several days, three at most",
            <form id="shifts-form">
                calendar(
                    id: &multiple_id,
                    name: "shifts",
                    mode: CalendarMode::Multiple,
                    max_selected: 3,
                    today: Some(today),
                    selected: &shifts
                )
            </form>
        )
        example(
            title: "A minimum, a maximum, and no weekends or holidays",
            calendar(
                id: &limits_id,
                name: "appointment",
                today: Some(today),
                required: true,
                min: Some(day(2026, 11, 10)),
                max: Some(day(2026, 12, 20)),
                disabled_weekdays: &[0, 6],
                disabled_dates: &holidays
            )
        )
        example(
            title: "Month and year selects",
            calendar(
                id: &dropdown_id,
                name: "birthday",
                today: Some(today),
                month: Some(day(1990, 6, 1)),
                dropdown: true,
                year_range: Some((1980, 2026)),
                max: Some(today)
            )
        )
        example(
            title: "Weeks from Monday, without the neighbouring months' days",
            calendar(
                id: &monday_id,
                today: Some(today),
                week_starts_on: 1,
                hide_outside_days: true
            )
        )
        example(
            title: "Disabled",
            calendar(
                id: &disabled_id,
                today: Some(today),
                selected: &one_day,
                disabled: true
            )
        )
    })
}
