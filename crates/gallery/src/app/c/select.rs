use bitop_ui::components::{
    button::{ButtonVariant, button},
    icon::Icon,
    select::{SelectItem, SelectSize, native_select, select},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const MACHINES: [SelectItem<'static>; 3] = [
    SelectItem::new("standard", "Standard")
        .icon(Icon::Cpu)
        .hint("2 vCPU"),
    SelectItem::new("performance", "Performance")
        .icon(Icon::Cpu)
        .hint("4 vCPU"),
    SelectItem::new("turbo", "Turbo")
        .icon(Icon::Cpu)
        .hint("8 vCPU")
        .disabled(),
];

const RETENTION: [SelectItem<'static>; 4] = [
    SelectItem::new("7", "7 days"),
    SelectItem::new("30", "30 days"),
    SelectItem::new("90", "90 days"),
    SelectItem::new("365", "1 year"),
];

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "select")
        example(
            title: "With icons and hints",
            select(
                id: "select-machine",
                label: "Machine",
                name: "machine",
                items: &MACHINES,
                value: "standard"
            )
        )
        example(
            title: "A placeholder, a small size and a hidden label, in a form",
            <form
                action="/c/select"
                data-select-form=""
                style="display: flex; flex-wrap: wrap; align-items: end; gap: var(--space-4)"
            >
                select(
                    id: "select-retention",
                    label: "Retention",
                    name: "retention",
                    items: &RETENTION,
                    placeholder: "Choose a period…",
                    required: true
                )
                select(
                    id: "select-small",
                    label: "Small select",
                    hide_label: true,
                    name: "small",
                    size: SelectSize::Sm,
                    items: &RETENTION,
                    value: "30"
                )
                button(
                    variant: ButtonVariant::Primary,
                    attrs: attributes! { type="submit" },
                    "Save"
                )
            </form>
        )
        example(
            title: "Disabled",
            select(
                id: "select-disabled",
                label: "Region",
                items: &RETENTION,
                value: "90",
                disabled: true
            )
        )
        example(
            title: "Native select",
            native_select(
                attrs: attributes! { name="native" aria-label="Native select" },
                <option value="30">"30 days"</option>
                <option value="90" selected=(true)>"90 days"</option>
                <option value="365">"1 year"</option>
            )
        )
    })
}
