use bitop_ui::components::{
    button::button,
    radio_group::{RadioGroupVariant, radio, radio_group},
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
    let plan_id = element_id(cx);
    let density_id = element_id(cx);
    let tier_id = element_id(cx);
    let region_id = element_id(cx);
    let locked_id = element_id(cx);
    Ok(view! {
        page_header(slug: "radio-group")
        example(
            title: "Vertical, with descriptions",
            <form id="plan-form" style="display: grid; gap: var(--space-4)">
                radio_group(
                    id: &plan_id,
                    legend: "Plan",
                    description: "You can change this later.",
                    radio(name: "plan", value: "free", checked: true, "Free")
                    radio(
                        name: "plan",
                        value: "team",
                        description: "For up to 20 people.",
                        "Team"
                    )
                    radio(
                        name: "plan",
                        value: "enterprise",
                        disabled: true,
                        description: "Ask sales.",
                        "Enterprise"
                    )
                    radio(name: "plan", value: "custom", "Custom")
                )
                <div>button(attrs: attributes! { type="submit" }, "Save")</div>
            </form>
        )
        example(
            title: "Horizontal",
            radio_group(
                id: &density_id,
                legend: "Density",
                horizontal: true,
                radio(name: "density", value: "compact", "Compact")
                radio(name: "density", value: "normal", checked: true, "Normal")
                radio(name: "density", value: "roomy", "Roomy")
            )
        )
        example(
            title: "Cards",
            <div style="inline-size: 100%">
                radio_group(
                    id: &tier_id,
                    legend: "Model tier",
                    variant: RadioGroupVariant::Card,
                    radio(
                        name: "tier",
                        value: "small",
                        checked: true,
                        description: "Fast and cheap.",
                        "Small"
                    )
                    radio(
                        name: "tier",
                        value: "large",
                        description: "Best answers.",
                        "Large"
                    )
                )
            </div>
        )
        example(
            title: "With an error",
            radio_group(
                id: &region_id,
                legend: "Region",
                error: "Choose a region.",
                radio(name: "region", value: "eu", "Europe")
                radio(name: "region", value: "us", "United States")
            )
        )
        example(
            title: "Disabled",
            radio_group(
                id: &locked_id,
                legend: "Billing",
                attrs: attributes! { disabled=(true) },
                radio(name: "billing", value: "monthly", checked: true, "Monthly")
                radio(name: "billing", value: "yearly", "Yearly")
            )
        )
    })
}
