use bitop_ui::components::{
    badge::badge,
    button::{ButtonVariant, button},
    layout::{ContainerSize, FlexAlign, FlexJustify, container, inline, stack},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const BOX: &str = "padding: var(--space-2) var(--space-3); border: 1px solid var(--color-border-strong); border-radius: var(--radius-control); background: var(--color-surface-sunken)";
const FRAME: &str = "inline-size: 100%; border: 1.5px dashed var(--color-border-strong); border-radius: var(--radius-control)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "layout")
        example(
            title: "Stack: a column, with the default gap and a wider one",
            stack(
                attrs: attributes! { data-demo="stack" style="flex: 1" },
                <div style=(BOX)>"One"</div>
                <div style=(BOX)>"Two"</div>
                <div style=(BOX)>"Three"</div>
            )
            stack(
                gap: 8,
                align: FlexAlign::Start,
                attrs: attributes! { data-demo="stack-wide" style="flex: 1" },
                <div style=(BOX)>"One"</div>
                <div style=(BOX)>"Two"</div>
                <div style=(BOX)>"Three"</div>
            )
        )
        example(
            title: "Stack as a list, aligned to the end",
            stack(
                list: true,
                gap: 1,
                align: FlexAlign::End,
                attrs: attributes! { data-demo="stack-list" style="inline-size: 100%" },
                <li style=(BOX)>"First item"</li>
                <li style=(BOX)>"Second"</li>
            )
        )
        example(
            title: "Inline: a row that wraps",
            inline(
                attrs: attributes! { data-demo="inline" style="inline-size: 16rem" },
                badge("Rust")
                badge("Topcoat")
                badge("CSS Modules")
                badge("Popover API")
                badge("Anchor positioning")
            )
        )
        example(
            title: "Inline: spread, on one line, on a baseline",
            inline(
                justify: FlexJustify::Between,
                nowrap: true,
                attrs: attributes! { data-demo="inline-between" style="inline-size: 100%" },
                <strong>"3 members"</strong>
                inline(
                    gap: 3,
                    button(variant: ButtonVariant::Ghost, "Cancel")
                    button("Save")
                )
            )
            inline(
                gap: 4,
                align: FlexAlign::Baseline,
                justify: FlexJustify::Center,
                attrs: attributes! { data-demo="inline-baseline" style="inline-size: 100%" },
                <span style="font-size: var(--font-size-2xl)">"Large"</span>
                <span style="font-size: var(--font-size-xs)">"small"</span>
            )
            inline(
                gap: 0,
                justify: FlexJustify::End,
                align: FlexAlign::Stretch,
                attrs: attributes! { data-demo="inline-end" style="inline-size: 100%" },
                <div style=(BOX)>"A"</div>
                <div style=(BOX)>"B"</div>
            )
        )
        example(
            title: "Container: the page column, in three sizes",
            <div
                data-demo="containers"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                <div style=(FRAME)>
                    container(
                        size: ContainerSize::Md,
                        <div style=(BOX)>"Medium: up to 56rem"</div>
                    )
                </div>
                <div style=(FRAME)>
                    container(<div style=(BOX)>"Large: up to 1200 pixels"</div>)
                </div>
                <div style=(FRAME)>
                    container(
                        size: ContainerSize::Full,
                        <div style=(BOX)>"Full: no limit"</div>
                    )
                </div>
            </div>
        )
    })
}
