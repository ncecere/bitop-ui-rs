use bitop_ui::components::input::textarea;
use bitop_ui_ai::components::snippet::snippet;
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
        page_header(slug: "snippet")
        example(
            title: "An install command",
            <div
                data-example="install"
                style="inline-size: 100%; max-inline-size: 28rem"
            >
                snippet(code: "cargo add bitop-ui-ai")
            </div>
        )
        example(
            title: "A long command scrolls",
            <div data-example="long" style="inline-size: 100%; max-inline-size: 20rem">
                snippet(
                    code: "docker run --rm -p 3000:3000 -e DATABASE_URL=postgres://localhost/app ghcr.io/example/app:latest"
                )
            </div>
        )
        example(
            title: "Another prompt, or none",
            <div
                data-example="prompts"
                style="display: grid; gap: var(--space-2); inline-size: 100%; max-inline-size: 28rem"
            >
                snippet(
                    code: "SELECT count(*) FROM users;",
                    prefix: ">",
                    label: "query"
                )
                snippet(
                    code: "https://example.com/invite/4f9a2c",
                    prefix: "",
                    label: "link"
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 28rem"
            >
                snippet(
                    code: "npm install bitop-ui",
                    label: "Befehl",
                    copy_label: "{label} kopieren",
                    copied_label: "{label} kopiert"
                )
            </div>
        )
        example(
            title: "Paste here to try copying",
            <div style="inline-size: 100%; max-inline-size: 24rem">
                textarea(attrs: attributes! { aria-label="Paste target" })
            </div>
        )
    })
}
