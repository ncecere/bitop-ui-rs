use bitop_ui::components::tabs::{tab, tab_list, tab_panel, tabs};
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
        page_header(slug: "tabs")
        example(
            title: "Three panels",
            tabs(
                attrs: attributes! { style="inline-size: 100%" },
                tab_list(
                    label: "Settings",
                    tab(panel: "tab-general", selected: true, "General")
                    tab(panel: "tab-models", "Models")
                    tab(panel: "tab-members", "Members")
                )
                tab_panel(id: "tab-general", selected: true, "General settings.")
                tab_panel(id: "tab-models", "Model settings.")
                tab_panel(id: "tab-members", "Member settings.")
            )
        )
    })
}
