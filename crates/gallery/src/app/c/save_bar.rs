use bitop_ui::components::{
    button::{ButtonVariant, button},
    field::{field, field_label},
    input::{input, textarea},
    save_bar::save_bar,
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
        page_header(slug: "save-bar")
        example(
            title: "Opens when the form changes, closes when it is reset",
            <div
                data-demo="scroller"
                tabindex="0"
                role="region"
                aria-label="Profile form"
                style="position: relative; inline-size: 100%; block-size: 18rem; overflow: auto; padding: var(--space-4); border: 1px solid var(--color-border); border-radius: var(--radius-control)"
            >
                <form
                    action="/c/save-bar"
                    data-demo="form"
                    style="display: grid; gap: var(--space-4)"
                >
                    field(
                        field_label(target: "save-name", "Name")
                        input(
                            attrs: attributes! { id="save-name" name="name" value="Ada" }
                        )
                    )
                    field(
                        field_label(target: "save-email", "Email")
                        input(
                            attrs: attributes! {
                                id="save-email"
                                name="email"
                                type="email"
                                value="ada@example.com"
                            }
                        )
                    )
                    field(
                        field_label(target: "save-team", "Team")
                        input(
                            attrs: attributes! { id="save-team" name="team" value="Engines" }
                        )
                    )
                    field(
                        field_label(target: "save-bio", "About")
                        textarea(
                            attrs: attributes! { id="save-bio" name="bio" rows="6" },
                            "Writes programs."
                        )
                    )
                    save_bar(
                        auto: true,
                        button(
                            variant: ButtonVariant::Ghost,
                            attrs: attributes! { type="reset" },
                            "Discard"
                        )
                        button(attrs: attributes! { type="submit" }, "Save")
                    )
                </form>
            </div>
        )
        example(
            title: "Open from the start, with another message",
            <form action="/c/save-bar" data-demo="open" style="inline-size: 100%">
                save_bar(
                    open: true,
                    message: "2 unsaved changes",
                    attrs: attributes! { style="margin-block-start: 0" },
                    button(
                        variant: ButtonVariant::Ghost,
                        attrs: attributes! { type="reset" },
                        "Discard"
                    )
                    button(attrs: attributes! { type="submit" }, "Save changes")
                )
            </form>
        )
    })
}
