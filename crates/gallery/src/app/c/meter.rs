use bitop_ui::components::meter::{MeterMarker, MeterSize, meter};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let queries = element_id(cx);
    let resets = element_id(cx);
    Ok(view! {
        page_header(slug: "meter")
        example(
            title: "Levels",
            <div style="display: grid; gap: var(--space-5); inline-size: 100%">
                meter(label: "Documents", value: 1200.0, max: 5000.0)
                meter(label: "Storage", value: 8.6, max: 10.0, unit: "GB")
                meter(label: "Seats", value: 20.0, max: 20.0)
                meter(label: "Tokens", value: 5400.0, max: 5000.0)
            </div>
        )
        example(
            title: "A marker and a description",
            <div style="display: grid; gap: var(--space-5); inline-size: 100%">
                meter(
                    label: "Queries today",
                    value: 4120.0,
                    max: 5000.0,
                    warning_at: 0.9,
                    marker: MeterMarker { value: 4000.0, label: "Team limit" },
                    description: "Resets at midnight UTC",
                    description_id: &queries
                )
                meter(
                    label: "Builds",
                    value: 3.0,
                    max: 10.0,
                    description: "Resets on the first of the month",
                    description_id: &resets
                )
            </div>
        )
        example(
            title: "Small, a hidden label, a range, and no status",
            <div style="display: grid; gap: var(--space-5); inline-size: 100%">
                meter(
                    label: "Memory",
                    value: 61.0,
                    max: 64.0,
                    unit: "GB",
                    size: MeterSize::Sm
                )
                meter(label: "Disk", value: 40.0, max: 100.0, hide_label: true)
                meter(label: "Temperature", value: 180.0, min: 100.0, max: 200.0)
                meter(label: "Uploads", value: 95.0, max: 100.0, show_status: false)
                meter(
                    label: "Budget",
                    value: 110.0,
                    max: 100.0,
                    critical_at: 1.2,
                    value_text: "110% of the monthly budget"
                )
            </div>
        )
        example(
            title: "No limit",
            meter(
                label: "API calls",
                value: 58210.0,
                max: None,
                description: "Your plan has no limit."
            )
        )
        example(
            title: "Every string is an argument",
            meter(
                label: "Speicher",
                value: 9.5,
                max: 10.0,
                unit: "GB",
                of_label: "von",
                warning_label: "Fast voll"
            )
        )
    })
}
