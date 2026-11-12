use bitop_ui::components::accordion::{
    AccordionHeadingLevel, AccordionVariant, accordion, accordion_item, accordion_panel,
    accordion_trigger,
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
        page_header(slug: "accordion")
        example(
            title: "Plain: one panel open at a time",
            accordion(
                attrs: attributes! { data-demo="single" },
                accordion_item(
                    name: "faq",
                    open: true,
                    accordion_trigger("Billing")
                    accordion_panel(
                        <p>"Invoices are sent on the first of the month."</p>
                        <p>"Change the card under Settings, Payment."</p>
                    )
                )
                accordion_item(
                    name: "faq",
                    accordion_trigger(meta: "3", "Members")
                    accordion_panel(<p>"Owners can invite and remove people."</p>)
                )
                accordion_item(
                    name: "faq",
                    accordion_trigger(disabled: true, "Audit log")
                    accordion_panel(<p>"Available on the enterprise plan."</p>)
                )
                accordion_item(
                    name: "faq",
                    accordion_trigger("Security")
                    accordion_panel(
                        <p>
                            "Two-factor sign-in is required for owners. "
                            <a href="/c/accordion">"Read the policy"</a>
                        </p>
                    )
                )
            )
        )
        example(
            title: "Outline: any number open, with level 4 headings",
            <h3 style="inline-size: 100%; font-size: var(--font-size-md)">"Project"</h3>
            accordion(
                variant: AccordionVariant::Outline,
                attrs: attributes! { data-demo="multiple" },
                accordion_item(
                    open: true,
                    accordion_trigger(
                        level: AccordionHeadingLevel::H4,
                        meta: "Daily",
                        "Backups"
                    )
                    accordion_panel(<p>"Kept for thirty days."</p>)
                )
                accordion_item(
                    open: true,
                    accordion_trigger(level: AccordionHeadingLevel::H4, "Regions")
                    accordion_panel(<p>"Frankfurt and Virginia."</p>)
                )
                accordion_item(
                    accordion_trigger(level: AccordionHeadingLevel::H4, "Limits")
                    accordion_panel(<p>"A thousand requests a minute."</p>)
                )
            )
        )
    })
}
