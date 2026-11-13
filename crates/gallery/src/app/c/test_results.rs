use bitop_ui_ai::components::test_results::{
    TestCountLabels, TestRunSummary, TestStatus, TestStatusLabels, test, test_error,
    test_error_message, test_error_stack, test_results, test_results_content,
    test_results_duration, test_results_header, test_results_progress, test_results_summary,
    test_suite, test_suite_content, test_suite_name, test_suite_stats,
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

use crate::app::kit::{example, page_header};

const RUN: TestRunSummary = TestRunSummary {
    passed: 18,
    failed: 1,
    skipped: 1,
    total: 20,
    duration: Some(4210.0),
};
const GREEN: TestRunSummary = TestRunSummary {
    passed: 42,
    failed: 0,
    skipped: 0,
    total: 42,
    duration: Some(65_000.0),
};
const STACK: &str = "AssertionError: expected 200 to equal 401\n    at Context.<anonymous> (test/auth.test.ts:42:18)\n    at processImmediate (node:internal/timers:476:21)";
const GERMAN_COUNTS: TestCountLabels<'static> = TestCountLabels {
    passed: "{n} bestanden",
    failed: "{n} fehlgeschlagen",
    skipped: "{n} übersprungen",
};
const GERMAN_STATUS: TestStatusLabels<'static> = TestStatusLabels {
    passed: "Bestanden:",
    failed: "Fehlgeschlagen:",
    skipped: "Übersprungen:",
    running: "Läuft:",
};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "test-results")
        example(
            title: "A run with a failure",
            <div data-example="run" style="inline-size: 100%; max-inline-size: 40rem">
                test_results(
                    summary: RUN,
                    attrs: attributes! { id="run" },
                    test_results_content(
                        test_suite(
                            status: TestStatus::Failed,
                            open: true,
                            attrs: attributes! { id="auth-suite" },
                            test_suite_name(
                                name: "auth.test.ts",
                                status: TestStatus::Failed,
                                test_suite_stats(passed: 3, failed: 1)
                            )
                            test_suite_content(
                                name: "auth.test.ts",
                                test(
                                    name: "refreshes the token",
                                    status: TestStatus::Passed,
                                    duration: 12.0
                                )
                                test(
                                    name: "rejects expired tokens",
                                    status: TestStatus::Failed,
                                    duration: 40.0,
                                    details: true,
                                    attrs: attributes! { id="failed-test" },
                                    test_error(
                                        test_error_message("Expected 401, got 200")
                                        test_error_stack((STACK))
                                    )
                                )
                                test(
                                    name: "logs out on every device",
                                    status: TestStatus::Passed,
                                    duration: 8.0
                                )
                                test(
                                    name: "rotates the signing key",
                                    status: TestStatus::Passed,
                                    duration: 1530.0
                                )
                            )
                        )
                        test_suite(
                            status: TestStatus::Passed,
                            attrs: attributes! { id="billing-suite" },
                            test_suite_name(
                                name: "billing.test.ts",
                                status: TestStatus::Passed,
                                test_suite_stats(passed: 14)
                            )
                            test_suite_content(
                                name: "billing.test.ts",
                                test(
                                    name: "charges the card once",
                                    status: TestStatus::Passed,
                                    duration: 30.0
                                )
                            )
                        )
                        test_suite(
                            status: TestStatus::Skipped,
                            test_suite_name(
                                name: "export.test.ts",
                                status: TestStatus::Skipped,
                                test_suite_stats(skipped: 1)
                            )
                            test_suite_content(
                                name: "export.test.ts",
                                test(name: "exports to PDF", status: TestStatus::Skipped)
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "Still running",
            <div
                data-example="running"
                style="inline-size: 100%; max-inline-size: 40rem"
            >
                test_results(
                    test_results_content(
                        test_suite(
                            status: TestStatus::Running,
                            open: true,
                            test_suite_name(
                                name: "search.test.ts",
                                status: TestStatus::Running
                            )
                            test_suite_content(
                                name: "search.test.ts",
                                test(
                                    name: "ranks exact matches first",
                                    status: TestStatus::Passed,
                                    duration: 22.0
                                )
                                test(
                                    name: "pages the results",
                                    status: TestStatus::Running
                                )
                            )
                        )
                    )
                )
            </div>
        )
        example(
            title: "All green, composed from the parts",
            <div data-example="green" style="inline-size: 100%; max-inline-size: 40rem">
                test_results(
                    test_results_header(
                        test_results_summary(summary: GREEN)
                        test_results_duration(duration: 65_000.0)
                    )
                    test_results_progress(summary: GREEN, label: "Checks passed")
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
                test_results(
                    summary: RUN,
                    count_labels: GERMAN_COUNTS,
                    duration_label: "Dauer: ",
                    progress_label: "Tests bestanden",
                    test_results_content(
                        test_suite(
                            status: TestStatus::Failed,
                            test_suite_name(
                                name: "konto.test.ts",
                                status: TestStatus::Failed,
                                status_labels: GERMAN_STATUS,
                                test_suite_stats(failed: 1, labels: GERMAN_COUNTS)
                            )
                            test_suite_content(
                                name: "konto.test.ts",
                                test(
                                    name: "sperrt das Konto",
                                    status: TestStatus::Failed,
                                    status_labels: GERMAN_STATUS
                                )
                            )
                        )
                    )
                )
            </div>
        )
    })
}
