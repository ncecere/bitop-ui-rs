use bitop_ui::components::{
    direction::{Direction, direction},
    layout::{inline, stack},
    menu::{menu, menu_item, menu_submenu, menu_trigger},
    tabs::{tab, tab_list, tab_panel, tabs},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, element_id, view},
};

use crate::app::kit::{example, page_header};

const FRAME: &str = "flex: 1; min-inline-size: 16rem; padding: var(--space-4); border: 1px solid var(--color-border); border-radius: var(--radius-control)";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let menu_id = element_id(cx);
    let more_id = element_id(cx);

    Ok(view! {
        page_header(slug: "direction")
        example(
            title: "A right-to-left section in a left-to-right page",
            direction(
                dir: Direction::of_language("ar"),
                lang: "ar",
                attrs: attributes! { data-demo="rtl" style=(FRAME) },
                stack(
                    <p>
                        "هذا القسم يُقرأ من اليمين إلى اليسار."
                    </p>
                    tabs(
                        tab_list(
                            label: "الإعدادات",
                            tab(panel: "dir-general", selected: true, "عام")
                            tab(panel: "dir-models", "النماذج")
                            tab(panel: "dir-members", "الأعضاء")
                        )
                        tab_panel(
                            id: "dir-general",
                            selected: true,
                            "الإعدادات العامة."
                        )
                        tab_panel(id: "dir-models", "إعدادات النماذج.")
                        tab_panel(id: "dir-members", "إعدادات الأعضاء.")
                    )
                    inline(
                        menu_trigger(target: &menu_id, "خيارات")
                        menu(
                            id: &menu_id,
                            menu_item("إعادة تسمية")
                            menu_submenu(
                                id: &more_id,
                                label: "المزيد",
                                menu_item("أرشفة")
                                menu_item("حذف")
                            )
                        )
                    )
                )
            )
            direction(
                dir: Direction::Ltr,
                attrs: attributes! { data-demo="ltr" style=(FRAME) },
                stack(
                    <p>
                        "This section reads from left to right, whatever the page does."
                    </p>
                    tabs(
                        tab_list(
                            label: "Settings",
                            tab(panel: "dir-en-general", selected: true, "General")
                            tab(panel: "dir-en-models", "Models")
                        )
                        tab_panel(
                            id: "dir-en-general",
                            selected: true,
                            "General settings."
                        )
                        tab_panel(id: "dir-en-models", "Model settings.")
                    )
                )
            )
        )
        example(
            title: "Auto: the direction comes from the text",
            direction(
                dir: Direction::Auto,
                attrs: attributes! { data-demo="auto-rtl" style=(FRAME) },
                <p>"שלום עולם"</p>
            )
            direction(
                dir: Direction::Auto,
                attrs: attributes! { data-demo="auto-ltr" style=(FRAME) },
                <p>"Hello, world"</p>
            )
        )
    })
}
