use bitop_ui::components::input::textarea;
use bitop_ui_ai::components::stack_trace::{InternalFrames, StackTraceLabels, stack_trace};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const NODE: &str = "TypeError: Cannot read properties of undefined (reading 'id')
    at getUser (/app/src/users.ts:42:18)
    at handler (/app/src/routes/profile.ts:17:22)
    at Layer.handle [as handle_request] (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:149:13)
    at Route.dispatch (/app/node_modules/express/lib/router/route.js:119:3)
    at processTicksAndRejections (node:internal/process/task_queues:95:5)
    at async main (/app/src/server.ts:8:3)";

const FIREFOX: &str = "RangeError [ERR_OUT_OF_RANGE]: The value of \"offset\" is out of range.
Received 12
readUInt32@http://localhost:3000/assets/buffer.js:210:11
parseHeader@http://localhost:3000/assets/parser.js:33:20
@http://localhost:3000/assets/main.js:5:1";

const GERMAN_TRACE: &str = "TypeError: Eigenschaft 'id' von undefined kann nicht gelesen werden
    at getUser (/app/src/users.ts:42:18)
    at next (/app/node_modules/express/lib/router/route.js:149:13)
    at processTicksAndRejections (node:internal/process/task_queues:95:5)";

const GERMAN: StackTraceLabels<'static> = StackTraceLabels {
    fallback: "Fehler",
    at: "bei ",
    empty: "Keine Stack-Frames",
    internal_one: "{n} interner Frame",
    internal_other: "{n} interne Frames",
    copy: "Stacktrace",
    copy_label: "{label} kopieren",
    copied_label: "{label} kopiert",
};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    // The same frames as the first, with another message: two regions named alike would
    // not be told apart.
    let hidden_trace = NODE.replace("reading 'id'", "reading 'name'");
    Ok(view! {
        page_header(slug: "stack-trace")
        example(
            title: "Closed",
            <div
                data-example="closed"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                stack_trace(id: "node-trace", trace: NODE)
            </div>
        )
        example(
            title: "Open, with internal frames folded and locations that send events",
            <div data-example="open" style="inline-size: 100%; max-inline-size: 44rem">
                stack_trace(
                    id: "open-trace",
                    trace: NODE,
                    open: true,
                    frame_events: true,
                    attrs: attributes! { data-gallery-frames="" }
                )
                <p data-frame-clicked="" style="margin-block-start: var(--space-2)"></p>
            </div>
        )
        example(
            title: "Firefox's format, a message of two lines, internal frames shown",
            <div
                data-example="firefox"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                stack_trace(
                    id: "firefox-trace",
                    trace: FIREFOX,
                    open: true,
                    internal_frames: InternalFrames::Show
                )
            </div>
        )
        example(
            title: "Internal frames hidden, a lower height",
            <div
                data-example="hidden"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                stack_trace(
                    id: "hidden-trace",
                    trace: &hidden_trace,
                    open: true,
                    internal_frames: InternalFrames::Hide,
                    max_height: "4rem"
                )
            </div>
        )
        example(
            title: "No frames",
            <div data-example="empty" style="inline-size: 100%; max-inline-size: 44rem">
                stack_trace(
                    id: "empty-trace",
                    trace: "Something went wrong",
                    open: true
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 44rem"
            >
                stack_trace(
                    id: "german-trace",
                    trace: GERMAN_TRACE,
                    open: true,
                    labels: GERMAN
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
