use bitop_ui::components::{
    avatar::avatar,
    badge::badge,
    button::{ButtonSize, ButtonVariant, button},
    icon::{Icon, icon},
    item::{
        ItemMediaVariant, ItemSize, ItemVariant, item, item_actions, item_content,
        item_description, item_footer, item_group, item_header, item_media, item_separator,
        item_title,
    },
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "item")
        example(
            title: "Variants",
            item_group(
                label: "Integrations",
                item(
                    in_group: true,
                    item_media(variant: ItemMediaVariant::Icon, icon(name: Icon::Inbox))
                    item_content(
                        item_title("Default")
                        item_description("No border and no fill.")
                    )
                    item_actions(
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            "Connect"
                        )
                    )
                )
                item(
                    in_group: true,
                    variant: ItemVariant::Outline,
                    item_media(
                        variant: ItemMediaVariant::Icon,
                        icon(name: Icon::Upload)
                    )
                    item_content(
                        item_title("Outline")
                        item_description("A bordered row on the surface.")
                    )
                    item_actions(
                        button(
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            "Manage"
                        )
                    )
                )
                item(
                    in_group: true,
                    variant: ItemVariant::Muted,
                    item_media(
                        variant: ItemMediaVariant::Icon,
                        icon(name: Icon::Search)
                    )
                    item_content(
                        item_title("Muted")
                        item_description("A sunken fill, for a row inside a card.")
                    )
                )
            )
        )
        example(
            title: "Sizes",
            item_group(
                label: "Sizes",
                item(
                    in_group: true,
                    variant: ItemVariant::Outline,
                    item_media(icon(name: Icon::CalendarDays))
                    item_content(item_title("Medium"))
                )
                item(
                    in_group: true,
                    variant: ItemVariant::Outline,
                    size: ItemSize::Sm,
                    item_media(icon(name: Icon::CalendarDays))
                    item_content(item_title("Small"))
                )
                item(
                    in_group: true,
                    variant: ItemVariant::Outline,
                    size: ItemSize::Xs,
                    item_media(icon(name: Icon::CalendarDays))
                    item_content(item_title("Extra small"))
                )
            )
        )
        example(
            title: "Link rows with a separator",
            item_group(
                label: "Settings",
                item(
                    in_group: true,
                    href: "#billing",
                    size: ItemSize::Sm,
                    item_content(
                        item_title("Billing")
                        item_description("Invoices, payment method and plan.")
                    )
                    item_actions(icon(name: Icon::ChevronRight))
                )
                item_separator(in_group: true)
                item(
                    in_group: true,
                    href: "#members",
                    size: ItemSize::Sm,
                    item_content(
                        item_title("Members")
                        item_description("People with access to this workspace.")
                    )
                    item_actions(icon(name: Icon::ChevronRight))
                )
            )
        )
        example(
            title: "Image media, header and footer",
            item(
                variant: ItemVariant::Outline,
                item_header(
                    <span>"Pinned"</span>
                    badge("New")
                )
                item_media(
                    variant: ItemMediaVariant::Image,
                    avatar(name: "Ada Lovelace")
                )
                item_content(
                    item_title("Ada Lovelace")
                    item_description(
                        "Wrote the first published algorithm intended for a machine, a long description that is clamped to two lines however much text there is in it, so that every row in a list keeps a similar height and the list stays easy to scan from top to bottom without one row taking over."
                    )
                )
                item_content(<span>"Owner"</span>)
                item_actions(button(size: ButtonSize::Sm, "Message"))
                item_footer(<span>"Joined in 1843"</span>)
            )
        )
    })
}
