use bitop_ui::components::{
    button::{ButtonSize, ButtonVariant, button},
    disclosure::disclosure,
};
use bitop_ui_ai::{
    components::{
        inline_citation::CitationSource,
        response::{ResponseStrings, response, response_block, response_blocks},
    },
    markdown::{Block, ResponseImages, ResponseStream},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const ANSWER: &str = r#"# Parental leave

Under the **2025 policy**, eligible employees get *16 weeks* of ~~unpaid~~ fully paid leave [1]. It can be split into blocks [2, 3].

| | Details |
| --- | ---: |
| Duration | 16 weeks |
| Split | Up to 3 blocks<br>of 2+ weeks [2] |

1. Tell your manager.
2. Submit the request in the [HR portal](https://hr.example.com/portal).

- [x] Read the policy
- [ ] Book the leave

> Requests need **8 weeks'** notice.

Run `leave --request` or:

```bash
hr leave request --weeks 16
```

Questions? Write to <hr@example.com>. An unknown marker [7] stays text, and so does a[1]."#;

const UNTRUSTED: &str = r#"<script>alert("injected")</script>

Raw <b>HTML</b> and <img src=x onerror=alert(1)> show as text.

[A script link](javascript:alert(1)), [a data link](data:text/html;base64,PHNjcmlwdD4=), [a relative link](/admin/delete) and [a safe one](https://example.com/docs).

![An SVG](data:image/svg+xml;base64,PHN2Zz4=) ![A tracking pixel](https://tracker.example.net/p.png?user=42) ![A dot](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==)"#;

/// Where the streaming snapshots stop, in characters of [`ANSWER`].
const CUTS: [&str; 5] = [
    "Under the **2025 pol",
    "Under the **2025 policy**, eligible employees get *16 weeks* of ~~unpaid~~ fully paid leave [1",
    "2. Submit the request in the [HR portal](https://hr.exa",
    "Run `leave --req",
    "```bash\nhr leave request --we",
];

fn prefix_through(cut: &str) -> &'static str {
    let end = ANSWER.find(cut).map_or(ANSWER.len(), |at| at + cut.len());
    &ANSWER[..end]
}

/// One step of the replay: the blocks that changed, and how many there are.
struct Stage {
    changed: Vec<(usize, Block)>,
    len: usize,
}

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    let sources = [
        CitationSource::new("Parental leave policy 2025")
            .href("https://hr.example.com/policies/leave")
            .description("Sixteen weeks of fully paid leave."),
        CitationSource::new("Benefits FAQ").site_name("HR help centre"),
        CitationSource::new("Leave request guide").href("https://hr.example.com/guide"),
    ];
    let snapshots: Vec<(String, &'static str)> = CUTS
        .iter()
        .enumerate()
        .map(|(n, cut)| (format!("snapshot-{n}"), prefix_through(cut)))
        .collect();
    let strings = ResponseStrings::default();
    let mut stream = ResponseStream::new();
    let mut stages = Vec::new();
    let chars: Vec<(usize, char)> = ANSWER.char_indices().collect();
    let mut cut = 0;
    while cut < chars.len() {
        cut = (cut + 24).min(chars.len());
        let end = chars.get(cut).map_or(ANSWER.len(), |(at, _)| *at);
        let done = cut == chars.len();
        let blocks = response_blocks(
            &ANSWER[..end],
            "replay",
            !done,
            sources.len(),
            2,
            None,
            false,
            ResponseImages::Click,
            &strings,
        );
        let update = stream.update(&blocks);
        stages.push(Stage {
            changed: update
                .changed
                .iter()
                .map(|&index| (index, blocks[index].clone()))
                .collect(),
            len: update.len,
        });
    }
    let german = ResponseStrings {
        new_tab_label: "(öffnet in einem neuen Tab)",
        load_image_label: "Bild von {host} laden",
        done_label: "Erledigt: ",
        todo_label: "Offen: ",
        code_label: "Quelltext",
        copy_code_label_plain: "Quelltext",
        copy_label: "{label} kopieren",
        copied_label: "{label} kopiert",
        copied_status: "{label} kopiert",
        ..ResponseStrings::default()
    };
    Ok(view! {
        page_header(slug: "response")
        example(
            title: "An answer with citations",
            <div data-example="answer" style="inline-size: 100%">
                response(id: "answer", source: ANSWER, citations: &sources)
            </div>
        )
        example(
            title: "Streaming: half-written syntax renders as it will",
            <div
                data-example="streaming"
                style="display: grid; gap: var(--space-4); inline-size: 100%"
            >
                for (snapshot_id, snapshot) in &snapshots {
                    <div
                        data-snapshot=""
                        style="padding-block-end: var(--space-4); border-block-end: 1px solid var(--color-border)"
                    >
                        response(
                            id: snapshot_id,
                            source: snapshot,
                            streaming: true,
                            citations: &sources
                        )
                    </div>
                }
            </div>
        )
        example(
            title: "Streaming block by block",
            <div
                data-example="replay"
                style="display: grid; gap: var(--space-3); inline-size: 100%"
            >
                <div>
                    button(
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        attrs: attributes! { data-gallery-replay="replay" },
                        "Replay the answer"
                    )
                </div>
                response(id: "replay", source: "", streaming: true, citations: &sources)
                for (n, stage) in stages.iter().enumerate() {
                    <template data-replay-stage=(n) data-len=(stage.len)>
                        for (index, block) in &stage.changed {
                            <template data-index=(*index)>
                                response_block(block: block, citations: &sources)
                            </template>
                        }
                    </template>
                }
            </div>
        )
        example(
            title: "Untrusted output",
            <div data-example="untrusted" style="inline-size: 100%">
                response(source: UNTRUSTED)
            </div>
        )
        example(
            title: "Images shown, images as text, and HTML dropped",
            <div
                data-example="modes"
                style="display: grid; gap: var(--space-4); inline-size: 100%"
            >
                // Closed, so that the lazy image is not requested until it is opened.
                disclosure(
                    title: "An image from another host, shown at once",
                    response(
                        source: "![A dot](https://images.example.com/dot.png) is shown at once.",
                        images: ResponseImages::Show,
                        attrs: attributes! { data-mode="show" }
                    )
                )
                response(
                    source: "![A chart](https://images.example.com/chart.png) and ![](https://images.example.com/x.png) are text.",
                    images: ResponseImages::Alt,
                    attrs: attributes! { data-mode="alt" }
                )
                response(
                    source: "Raw <b>HTML</b> is dropped.",
                    skip_html: true,
                    attrs: attributes! { data-mode="skip" }
                )
            </div>
        )
        example(
            title: "Headings at one level, for quoted text",
            <div data-example="headings" style="inline-size: 100%">
                response(source: "# Policy\n\n### Scope\n\nText.", heading_level: 3)
            </div>
        )
        example(
            title: "Other strings",
            <div data-example="strings" lang="de" style="inline-size: 100%">
                response(
                    source: "- [x] Antrag gestellt\n- [ ] Urlaub gebucht\n\nSiehe [das Portal](https://hr.example.de).\n\n![Ein Diagramm](https://bilder.example.de/d.png)\n\n```\nhallo\n```",
                    strings: german
                )
            </div>
        )
    })
}
