use bitop_ui::components::{
    combobox::{ComboboxOption, ComboboxSize, combobox},
    field::{field, field_description, field_label},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const WIDE: &str = "inline-size: 100%";

const TIMEZONES: [ComboboxOption<'static>; 8] = [
    ComboboxOption::new("America/New_York", "New York").hint("UTC−5"),
    ComboboxOption::new("America/Chicago", "Chicago").hint("UTC−6"),
    ComboboxOption::new("America/Denver", "Denver").hint("UTC−7"),
    ComboboxOption::new("America/Los_Angeles", "Los Angeles").hint("UTC−8"),
    ComboboxOption::new("Europe/London", "London").hint("UTC+0"),
    ComboboxOption::new("Europe/Berlin", "Berlin").hint("UTC+1"),
    ComboboxOption::new("Asia/Tokyo", "Tokyo").hint("UTC+9"),
    ComboboxOption::new("Australia/Sydney", "Sydney").hint("UTC+10"),
];

const REGIONS: [ComboboxOption<'static>; 8] = [
    ComboboxOption::new("iad1", "Washington, D.C.")
        .hint("iad1")
        .group("Americas"),
    ComboboxOption::new("sfo1", "San Francisco")
        .hint("sfo1")
        .group("Americas"),
    ComboboxOption::new("gru1", "São Paulo")
        .hint("gru1")
        .group("Americas"),
    ComboboxOption::new("fra1", "Frankfurt")
        .hint("fra1")
        .group("Europe"),
    ComboboxOption::new("lhr1", "London")
        .hint("lhr1")
        .group("Europe"),
    ComboboxOption::new("cdg1", "Paris")
        .hint("cdg1")
        .group("Europe"),
    ComboboxOption::new("hnd1", "Tokyo")
        .hint("hnd1")
        .group("Asia Pacific"),
    ComboboxOption::new("syd1", "Sydney")
        .hint("syd1")
        .group("Asia Pacific")
        .disabled(),
];

const PEOPLE: [ComboboxOption<'static>; 6] = [
    ComboboxOption::new("ana", "Ana Silva"),
    ComboboxOption::new("ben", "Ben Okafor"),
    ComboboxOption::new("chen", "Chen Wei"),
    ComboboxOption::new("dara", "Dara Novak"),
    ComboboxOption::new("eli", "Eli Cohen"),
    ComboboxOption::new("fatima", "Fatima Zahra"),
];

const LABELS: [ComboboxOption<'static>; 4] = [
    ComboboxOption::new("bug", "bug"),
    ComboboxOption::new("docs", "documentation"),
    ComboboxOption::new("perf", "performance"),
    ComboboxOption::new("a11y", "accessibility"),
];

const GROUPS: [ComboboxOption<'static>; 3] = [
    ComboboxOption::new("registrar-staff", "registrar-staff").hint("12 people"),
    ComboboxOption::new("advising-staff", "advising-staff").hint("8 people"),
    ComboboxOption::new("it-admins", "it-admins").hint("3 people"),
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let reviewers: [&str; 2] = ["ana", "chen"];
    Ok(view! {
        page_header(slug: "combobox")
        example(
            title: "Single selection with a clear button",
            field(
                attrs: attributes! { style=(WIDE) },
                field_label(target: "combobox-zone", "Time zone")
                combobox(
                    id: "combobox-zone",
                    name: "zone",
                    items: &TIMEZONES,
                    value: "Europe/Berlin",
                    placeholder: "Search cities…",
                    clearable: true,
                    attrs: attributes! { aria-describedby="combobox-zone-description" }
                )
                field_description(
                    id: "combobox-zone-description",
                    "Used for scheduled deployments."
                )
            )
        )
        example(
            title: "Groups, disabled options and an empty state",
            field(
                attrs: attributes! { style=(WIDE) },
                field_label(target: "combobox-region", "Function region")
                combobox(
                    id: "combobox-region",
                    name: "region",
                    items: &REGIONS,
                    placeholder: "Search regions…",
                    empty: Some("No region matches that name."),
                    auto_highlight: true
                )
            )
        )
        example(
            title: "Multiple selection with chips",
            <form
                action="/c/combobox"
                data-combobox-form=""
                style="display: flex; flex-direction: column; gap: var(--space-5); inline-size: 100%"
            >
                field(
                    field_label(target: "combobox-reviewers", "Reviewers")
                    combobox(
                        id: "combobox-reviewers",
                        name: "reviewers",
                        multiple: true,
                        items: &PEOPLE,
                        values: &reviewers,
                        chips_label: "Selected reviewers",
                        placeholder: "Add people…",
                        clearable: true,
                        attrs: attributes! { aria-describedby="combobox-reviewers-description" }
                    )
                    field_description(
                        id: "combobox-reviewers-description",
                        "Everyone listed must approve before merging."
                    )
                )
                field(
                    field_label(target: "combobox-labels", "Labels")
                    combobox(
                        id: "combobox-labels",
                        name: "labels",
                        multiple: true,
                        size: ComboboxSize::Sm,
                        items: &LABELS,
                        chips_label: "Selected labels"
                    )
                )
            </form>
        )
        example(
            title: "At most three matches shown (limit)",
            field(
                attrs: attributes! { style=(WIDE) },
                field_label(target: "combobox-limited", "City")
                combobox(
                    id: "combobox-limited",
                    name: "city",
                    items: &TIMEZONES,
                    limit: Some(3),
                    placeholder: "Search cities…"
                )
            )
        )
        example(
            title: "Free text with suggestions (free_text)",
            field(
                attrs: attributes! { style=(WIDE) },
                field_label(target: "combobox-group", "Directory group")
                combobox(
                    id: "combobox-group",
                    name: "group",
                    free_text: true,
                    items: &GROUPS,
                    placeholder: "registrar-staff",
                    clearable: true,
                    attrs: attributes! { aria-describedby="combobox-group-description" }
                )
                field_description(
                    id: "combobox-group-description",
                    "Pick a group seen at sign-in, or type any group name."
                )
            )
        )
    })
}
