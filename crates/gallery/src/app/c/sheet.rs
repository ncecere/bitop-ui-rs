use bitop_ui::components::{
    button::ButtonVariant,
    field::{field, field_label},
    icon::Icon,
    input::input,
    sheet::{SheetSide, SheetSize, sheet, sheet_close, sheet_trigger},
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
    let filters_id = element_id(cx);
    let name_id = element_id(cx);
    let owner_id = element_id(cx);
    let left_id = element_id(cx);
    let top_id = element_id(cx);
    let bottom_id = element_id(cx);
    let small_id = element_id(cx);
    let wide_id = element_id(cx);
    let full_id = element_id(cx);
    let terms_id = element_id(cx);
    // A slot's view takes what it uses by value, so each footer gets an ID of its own.
    let filters_footer_id = filters_id.clone();
    let top_footer_id = top_id.clone();
    let bottom_footer_id = bottom_id.clone();
    let terms_footer_id = terms_id.clone();

    Ok(view! {
        page_header(slug: "sheet")
        example(
            title: "From the right, with fields and a footer",
            sheet_trigger(target: &filters_id, "Filters")
            sheet(
                id: &filters_id,
                title: "Filters",
                description: "Narrow the list of deployments.",
                footer: view! {
                    sheet_close(target: &filters_footer_id, "Cancel")
                    sheet_close(
                        target: &filters_footer_id,
                        variant: ButtonVariant::Solid,
                        "Apply"
                    )
                },
                field(
                    field_label(target: &name_id, "Name contains")
                    input(attrs: attributes! { id=(&name_id) name="name" })
                )
                field(
                    field_label(target: &owner_id, "Owner")
                    input(attrs: attributes! { id=(&owner_id) name="owner" })
                )
            )
        )
        example(
            title: "Other sides",
            sheet_trigger(target: &left_id, "Open left")
            sheet(
                id: &left_id,
                title: "Navigation",
                description: "A sheet on the left edge.",
                side: SheetSide::Left,
                <p>"The contents mirror in a right-to-left page; the side does not."</p>
            )
            sheet_trigger(target: &top_id, "Open top")
            sheet(
                id: &top_id,
                title: "Announcement",
                description: "A sheet on the top edge spans the viewport.",
                side: SheetSide::Top,
                footer: view! { sheet_close(target: &top_footer_id, "Dismiss") }
            )
            sheet_trigger(target: &bottom_id, "Open bottom")
            sheet(
                id: &bottom_id,
                title: "Share",
                description: "A sheet on the bottom edge spans the viewport.",
                side: SheetSide::Bottom,
                hide_close: true,
                footer: view! { sheet_close(target: &bottom_footer_id, "Done") }
            )
        )
        example(
            title: "Sizes",
            sheet_trigger(target: &small_id, "Open small")
            sheet(
                id: &small_id,
                title: "Small",
                description: "Twenty rems wide.",
                size: SheetSize::Sm
            )
            sheet_trigger(target: &wide_id, "Open extra large")
            sheet(
                id: &wide_id,
                title: "Extra large",
                description: "Forty-two rems wide, or the viewport less a margin.",
                size: SheetSize::Xl
            )
            sheet_trigger(target: &full_id, "Open full screen")
            sheet(
                id: &full_id,
                title: "Source",
                description: "A full-screen sheet has its close button first, like a back button.",
                size: SheetSize::Full,
                close_label: "Back",
                close_icon: Icon::ChevronLeft,
                <p>"A side panel's stand-in on a phone."</p>
            )
        )
        example(
            title: "Text to read: focus starts on the title, and the body scrolls",
            sheet_trigger(target: &terms_id, "Read the terms")
            sheet(
                id: &terms_id,
                title: "Terms of service",
                description: "Last updated in November.",
                focus_title: true,
                hide_close: true,
                footer: view! {
                    sheet_close(
                        target: &terms_footer_id,
                        variant: ButtonVariant::Solid,
                        "Accept"
                    )
                },
                for number in 1..=30 {
                    <p>
                        "Clause "
                        (number)
                        ". The service is provided as it is, and this paragraph is here to make the body long enough to scroll."
                    </p>
                }
            )
        )
    })
}
