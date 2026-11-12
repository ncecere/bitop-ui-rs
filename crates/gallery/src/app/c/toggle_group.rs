use bitop_ui::components::{
    icon::{Icon, icon},
    toggle::{ToggleSize, ToggleVariant},
    toggle_group::{toggle_group, toggle_group_item},
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
        page_header(slug: "toggle-group")
        example(
            title: "One at a time",
            <form id="align-form">
                toggle_group(
                    label: "Text alignment",
                    toggle_group_item(
                        value: "left",
                        name: "align",
                        pressed: true,
                        "Left"
                    )
                    toggle_group_item(value: "center", name: "align", "Centre")
                    toggle_group_item(value: "right", name: "align", "Right")
                    toggle_group_item(
                        value: "justify",
                        name: "align",
                        disabled: true,
                        "Justify"
                    )
                )
            </form>
        )
        example(
            title: "Multiple, outline",
            <form id="style-form">
                toggle_group(
                    label: "Text style",
                    multiple: true,
                    variant: ToggleVariant::Outline,
                    toggle_group_item(value: "bold", name: "style", "Bold")
                    toggle_group_item(
                        value: "italic",
                        name: "style",
                        pressed: true,
                        "Italic"
                    )
                    toggle_group_item(
                        value: "underline",
                        name: "style",
                        pressed: true,
                        "Underline"
                    )
                )
            </form>
        )
        example(
            title: "Joined",
            toggle_group(
                label: "Period",
                joined: true,
                toggle_group_item(value: "day", "Day")
                toggle_group_item(value: "week", pressed: true, "Week")
                toggle_group_item(value: "month", "Month")
            )
            toggle_group(
                label: "Period, small",
                joined: true,
                size: ToggleSize::Sm,
                toggle_group_item(value: "day", pressed: true, "Day")
                toggle_group_item(value: "week", "Week")
                toggle_group_item(value: "month", "Month")
            )
        )
        example(
            title: "Joined, scrolling when too wide",
            <div style="inline-size: 14rem">
                toggle_group(
                    label: "Range",
                    joined: true,
                    scroll: true,
                    toggle_group_item(value: "1h", pressed: true, "Last hour")
                    toggle_group_item(value: "24h", "Last day")
                    toggle_group_item(value: "7d", "Last week")
                    toggle_group_item(value: "30d", "Last month")
                )
            </div>
        )
        example(
            title: "Vertical, icon only",
            toggle_group(
                label: "Tool",
                vertical: true,
                size: ToggleSize::Sm,
                variant: ToggleVariant::Outline,
                toggle_group_item(
                    value: "search",
                    icon_only: true,
                    pressed: true,
                    attrs: attributes! { aria-label="Search" },
                    icon(name: Icon::Search)
                )
                toggle_group_item(
                    value: "add",
                    icon_only: true,
                    attrs: attributes! { aria-label="Add" },
                    icon(name: Icon::Plus)
                )
                toggle_group_item(
                    value: "remove",
                    icon_only: true,
                    attrs: attributes! { aria-label="Remove" },
                    icon(name: Icon::Minus)
                )
            )
        )
    })
}
