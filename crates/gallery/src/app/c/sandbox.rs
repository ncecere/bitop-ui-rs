use bitop_ui::components::button::{ButtonSize, ButtonVariant, button};
use bitop_ui_ai::components::sandbox::{
    SandboxState, sandbox, sandbox_code, sandbox_content, sandbox_header, sandbox_output,
    sandbox_tab, sandbox_tab_panel, sandbox_tabs, sandbox_tabs_list,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const CODE: &str = "import pandas as pd\n\nsales = pd.read_csv(\"sales.csv\")\nprint(sales.groupby(\"region\")[\"total\"].sum())\n";
const OUTPUT: &str =
    "region\nEast     1204.50\nNorth     980.00\nWest     1411.25\nName: total, dtype: float64\n";
const FAILING: &str =
    "import json\n\nwith open(\"config.json\") as f:\n    config = json.load(f)\n";
const TRACEBACK: &str = "Traceback (most recent call last):\n  File \"main.py\", line 3, in <module>\nFileNotFoundError: [Errno 2] No such file or directory: 'config.json'";

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "sandbox")
        example(
            title: "Completed, with its code and output",
            <div
                data-example="completed"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                sandbox(
                    attrs: attributes! { id="completed-sandbox" },
                    sandbox_header(
                        title: "analyze.py",
                        state: SandboxState::Completed,
                        summary: "1.2 s"
                    )
                    sandbox_content(
                        sandbox_tabs(
                            sandbox_tabs_list(
                                label: "Run of analyze.py",
                                actions: view! {
                                    button(
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::Sm,
                                        "Run again"
                                    )
                                },
                                sandbox_tab(panel: "analyze-code", selected: true, "Code")
                                sandbox_tab(panel: "analyze-output", "Output")
                            )
                            sandbox_tab_panel(
                                id: "analyze-code",
                                selected: true,
                                sandbox_code(
                                    code: CODE,
                                    language: "python",
                                    filename: "analyze.py"
                                )
                            )
                            sandbox_tab_panel(
                                id: "analyze-output",
                                sandbox_output(output: OUTPUT)
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Failed, on its output",
            <div data-example="error" style="inline-size: 100%; max-inline-size: 40rem">
                sandbox(
                    attrs: attributes! { id="error-sandbox" },
                    sandbox_header(
                        title: "load_config.py",
                        state: SandboxState::Error,
                        summary: "exit code 1"
                    )
                    sandbox_content(
                        sandbox_tabs(
                            sandbox_tabs_list(
                                label: "Run of load_config.py",
                                sandbox_tab(panel: "config-code", "Code")
                                sandbox_tab(
                                    panel: "config-output",
                                    selected: true,
                                    "Output"
                                )
                            )
                            sandbox_tab_panel(
                                id: "config-code",
                                sandbox_code(code: FAILING, language: "python")
                            )
                            sandbox_tab_panel(
                                id: "config-output",
                                selected: true,
                                sandbox_output(error_text: TRACEBACK)
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Pending and running, closed",
            <div
                data-example="states"
                style="display: grid; gap: var(--space-2); inline-size: 100%; max-inline-size: 40rem"
            >
                sandbox(
                    open: false,
                    attrs: attributes! { id="pending-sandbox" },
                    sandbox_header(
                        title: "Python sandbox",
                        state: SandboxState::Pending
                    )
                    sandbox_content(sandbox_output())
                )
                sandbox(
                    open: false,
                    attrs: attributes! { id="running-sandbox" },
                    sandbox_header(title: "train.py", state: SandboxState::Running)
                    sandbox_content(sandbox_output(output: "epoch 1/10 loss=0.42\n"))
                )
            </div>
        )
        example(
            title: "Other strings",
            <div
                data-example="strings"
                lang="de"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                sandbox(
                    sandbox_header(
                        title: "rechnen.py",
                        state: SandboxState::Running,
                        running_label: "Läuft"
                    )
                    sandbox_content(
                        sandbox_tabs(
                            sandbox_tabs_list(
                                label: "Ausführung",
                                sandbox_tab(
                                    panel: "german-output",
                                    selected: true,
                                    "Ausgabe"
                                )
                            )
                            sandbox_tab_panel(
                                id: "german-output",
                                selected: true,
                                sandbox_output(empty_text: "Noch keine Ausgabe")
                            )
                        )
                    )
                )
            </div>
        )
    })
}
