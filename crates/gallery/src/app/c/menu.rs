use bitop_ui::components::menu::{MenuItemVariant, menu, menu_item, menu_separator, menu_trigger};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, element_id, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let menu_id = element_id(cx);

    Ok(view! {
        page_header(slug: "menu")
        example(
            title: "Actions and a link",
            menu_trigger(target: &menu_id, "Options")
            menu(
                id: &menu_id,
                menu_item("Rename")
                menu_item("Duplicate")
                menu_item(disabled: true, "Archive")
                menu_item(href: "/", "Home")
                menu_separator()
                menu_item(variant: MenuItemVariant::Danger, "Delete")
            )
        )
    })
}
