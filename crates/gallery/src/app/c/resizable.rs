use bitop_ui::components::resizable::{
    ResizableOrientation, resizable_handle, resizable_panel, resizable_panel_group,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const FRAME: &str = "inline-size: 100%; block-size: 12rem; border: 1px solid var(--color-border); border-radius: var(--radius-control); overflow: hidden";
const PANE: &str = "padding: var(--space-3)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "resizable")
        example(
            title: "Side by side: a sidebar between 20% and 50%",
            <div style=(FRAME)>
                resizable_panel_group(
                    attrs: attributes! { data-demo="horizontal" },
                    resizable_panel(
                        id: "resize-sidebar",
                        default_size: 30.0,
                        min_size: 20.0,
                        max_size: 50.0,
                        <div style=(PANE)>"Sidebar"</div>
                    )
                    resizable_handle(label: "Resize sidebar", with_handle: true)
                    resizable_panel(
                        id: "resize-editor",
                        <div style=(PANE)>"Editor"</div>
                    )
                )
            </div>
        )
        example(
            title: "Stacked, moving ten percent for each key press",
            <div style=(FRAME)>
                resizable_panel_group(
                    orientation: ResizableOrientation::Vertical,
                    keyboard_step: 10.0,
                    attrs: attributes! { data-demo="vertical" },
                    resizable_panel(id: "resize-code", <div style=(PANE)>"Code"</div>)
                    resizable_handle(label: "Resize output")
                    resizable_panel(
                        id: "resize-output",
                        default_size: 40.0,
                        min_size: 20.0,
                        <div style=(PANE)>"Output"</div>
                    )
                )
            </div>
        )
        example(
            title: "Three panels, one group inside another, kept in this browser",
            <div style=(FRAME)>
                resizable_panel_group(
                    storage: "bitop-gallery-resizable",
                    attrs: attributes! { data-demo="nested" },
                    resizable_panel(
                        id: "resize-files",
                        default_size: 25.0,
                        min_size: 10.0,
                        <div style=(PANE)>"Files"</div>
                    )
                    resizable_handle(label: "Resize files", with_handle: true)
                    resizable_panel(
                        id: "resize-main",
                        min_size: 30.0,
                        resizable_panel_group(
                            orientation: ResizableOrientation::Vertical,
                            attrs: attributes! { data-demo="inner" },
                            resizable_panel(
                                id: "resize-source",
                                default_size: 60.0,
                                <div style=(PANE)>"Source"</div>
                            )
                            resizable_handle(
                                label: "Resize terminal",
                                with_handle: true
                            )
                            resizable_panel(
                                id: "resize-terminal",
                                <div style=(PANE)>"Terminal"</div>
                            )
                        )
                    )
                    resizable_handle(label: "Resize preview", disabled: true)
                    resizable_panel(
                        id: "resize-preview",
                        default_size: 25.0,
                        <div style=(PANE)>"Preview: its handle is disabled"</div>
                    )
                )
            </div>
        )
    })
}
