//! A test run: the counts, the duration and a bar of the pass rate, then the suites,
//! each opening to show its tests, whose error details open in turn.
//!
//! A suite and a test with details are native `<details>`, so the browser opens and
//! closes them on click, Enter and Space; a failed test starts open. Every status has
//! its own icon shape and a hidden word ("Failed:"), and counts are written out ("1
//! failed"), so nothing relies on colour. The pass rate is a `role="meter"`.
//!
//! ```ignore
//! let summary = TestRunSummary { passed: 18, failed: 1, skipped: 1, total: 20,
//!     duration: Some(4210.0) };
//! view! {
//!     test_results(summary: summary,
//!         test_results_content(
//!             test_suite(name: "auth.test.ts", status: TestStatus::Failed, open: true,
//!                 test_suite_name(name: "auth.test.ts", status: TestStatus::Failed,
//!                     test_suite_stats(passed: 3, failed: 1))
//!                 test_suite_content(name: "auth.test.ts",
//!                     test(name: "refreshes the token", status: TestStatus::Passed,
//!                         duration: 12.0)
//!                     test(name: "rejects expired tokens", status: TestStatus::Failed,
//!                         details: true,
//!                         test_error(test_error_message("Expected 401, got 200")))
//!                 )
//!             )
//!         )
//!     )
//! }
//! ```
//!
//! Differences from upstream: there is no context, so the parts take the summary, the
//! suite's name and status themselves. [`test_results`] renders the header and the bar
//! when it is given a `summary`, before its child. A suite is a `<details>` (a
//! `<summary>` is announced as upstream's collapsible button is) and there is no
//! `onOpenChange`: listen for the native `toggle` event. A test with details says so
//! with `details: true`, as a part cannot tell whether it was given a child.

use bitop_ui::{
    components::icon::{Icon, icon},
    css::badge as badge_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::test_results as css;

/// A duration in milliseconds as text: `12ms`, `4.21s`, `1m 5s`.
///
/// ```
/// use bitop_ui_ai::components::test_results::format_duration;
///
/// assert_eq!(format_duration(12.0), "12ms");
/// assert_eq!(format_duration(4210.0), "4.21s");
/// assert_eq!(format_duration(65_000.0), "1m 5s");
/// ```
#[must_use]
pub fn format_duration(ms: f64) -> String {
    if ms < 1000.0 {
        return format!("{}ms", ms.round());
    }
    if ms < 60_000.0 {
        let text = format!("{:.2}", ms / 1000.0);
        let text = text.trim_end_matches('0').trim_end_matches('.');
        return format!("{text}s");
    }
    let whole = ms.round();
    let minutes = (whole / 60_000.0).floor();
    let seconds = ((whole % 60_000.0) / 1000.0).round();
    if seconds > 0.0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{minutes}m")
    }
}

/// The status of a test or a suite. Rendered as `data-status`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Running,
}

impl TestStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::Running => "running",
        }
    }

    fn icon(self) -> Icon {
        match self {
            Self::Passed => Icon::CircleCheck,
            Self::Failed => Icon::CircleX,
            Self::Skipped => Icon::CircleMinus,
            Self::Running => Icon::LoaderCircle,
        }
    }
}

/// The words read with a status icon, each ending in ":".
#[derive(Clone, Copy, Debug)]
pub struct TestStatusLabels<'a> {
    pub passed: &'a str,
    pub failed: &'a str,
    pub skipped: &'a str,
    pub running: &'a str,
}

impl TestStatusLabels<'_> {
    /// The English words.
    pub const ENGLISH: TestStatusLabels<'static> = TestStatusLabels {
        passed: "Passed:",
        failed: "Failed:",
        skipped: "Skipped:",
        running: "Running:",
    };

    fn of(&self, status: TestStatus) -> &str {
        match status {
            TestStatus::Passed => self.passed,
            TestStatus::Failed => self.failed,
            TestStatus::Skipped => self.skipped,
            TestStatus::Running => self.running,
        }
    }
}

impl Default for TestStatusLabels<'_> {
    fn default() -> Self {
        TestStatusLabels::ENGLISH
    }
}

/// The words of the counts, with `{n}` replaced: "18 passed".
#[derive(Clone, Copy, Debug)]
pub struct TestCountLabels<'a> {
    pub passed: &'a str,
    pub failed: &'a str,
    pub skipped: &'a str,
}

impl TestCountLabels<'_> {
    /// The English words.
    pub const ENGLISH: TestCountLabels<'static> = TestCountLabels {
        passed: "{n} passed",
        failed: "{n} failed",
        skipped: "{n} skipped",
    };
}

impl Default for TestCountLabels<'_> {
    fn default() -> Self {
        TestCountLabels::ENGLISH
    }
}

/// The totals of a run.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TestRunSummary {
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub total: u32,
    /// Milliseconds.
    pub duration: Option<f64>,
}

impl TestRunSummary {
    /// The share of `count` in the total, as a percentage; 0 when there are no tests.
    #[must_use]
    pub fn percent(&self, count: u32) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            f64::from(count) / f64::from(self.total) * 100.0
        }
    }
}

/// A status icon with its word for assistive technology, which `decorative` leaves out
/// when the status is said elsewhere. The running icon spins unless reduced motion is
/// asked for.
#[component]
pub async fn test_status_icon(
    cx: &Cx,
    status: TestStatus,
    #[default] decorative: bool,
    #[default] labels: TestStatusLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-status=(status.as_str())
            data-test-status=""
            class=(class!(css::STATUS_ICON, attrs.remove("class")))
            (attrs)
        >
            icon(name: status.icon())
            if !decorative {
                <span class="sr-only">(labels.of(status))</span>
            }
        </span>
    })
}

/// The run.
///
/// - `summary` renders the header (counts and duration) and the pass-rate bar first.
/// - `count_labels`, `duration_label` and `progress_label` are those of
///   [`test_results_summary`], [`test_results_duration`] and [`test_results_progress`].
/// - `child` follows: a [`test_results_content`].
#[component]
pub async fn test_results(
    cx: &Cx,
    #[default]
    #[into]
    summary: Option<TestRunSummary>,
    #[default] count_labels: TestCountLabels<'_>,
    #[default("Duration: ")] duration_label: &str,
    #[default("Tests passed")] progress_label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-test-results=""
            class=(class!(css::ROOT, attrs.remove("class")))
            (attrs)
        >
            if let Some(summary) = summary {
                test_results_header(
                    test_results_summary(summary: summary, labels: count_labels)
                    if let Some(ms) = summary.duration {
                        test_results_duration(duration: ms, label: duration_label)
                    }
                )
                test_results_progress(summary: summary, label: progress_label)
            }
            (child)
        </div>
    })
}

/// The header row: the counts and the duration.
#[component]
pub async fn test_results_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::HEADER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The counts as badges: "18 passed", "1 failed", "1 skipped". A failed or skipped
/// count of zero is left out.
#[component]
pub async fn test_results_summary(
    cx: &Cx,
    summary: TestRunSummary,
    #[default] labels: TestCountLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let count = |template: &str, n: u32| template.replace("{n}", &n.to_string());
    let passed = count(labels.passed, summary.passed);
    let failed = count(labels.failed, summary.failed);
    let skipped = count(labels.skipped, summary.skipped);
    Ok(view! {
        <div class=(class!(css::SUMMARY, attrs.remove("class"))) (attrs)>
            <span
                data-tone="success"
                data-count="passed"
                class=(class!(badge_css::ROOT, css::SUMMARY_BADGE))
            >
                icon(name: Icon::CircleCheck)
                (passed)
            </span>
            if summary.failed > 0 {
                " "
                <span
                    data-tone="danger"
                    data-count="failed"
                    class=(class!(badge_css::ROOT, css::SUMMARY_BADGE))
                >
                    icon(name: Icon::CircleX)
                    (failed)
                </span>
            }
            if summary.skipped > 0 {
                " "
                <span
                    data-tone="warning"
                    data-count="skipped"
                    class=(class!(badge_css::ROOT, css::SUMMARY_BADGE))
                >
                    icon(name: Icon::CircleMinus)
                    (skipped)
                </span>
            }
        </div>
    })
}

/// The run's duration in milliseconds, through [`format_duration`], after `label` for
/// assistive technology.
#[component]
pub async fn test_results_duration(
    cx: &Cx,
    duration: f64,
    #[default("Duration: ")] label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::DURATION, attrs.remove("class"))) (attrs)>
            <span class="sr-only">(label)</span>
            (format_duration(duration))
        </span>
    })
}

/// The pass rate: a meter of passed tests, with failed and skipped tests as more
/// segments of the bar.
///
/// - `label` is shown as "18/20 tests passed" (in lower case after the counts) and
///   names the meter.
/// - `value_text` is what the meter's value is read as: `{passed}`, `{total}` and
///   `{percent}` are replaced.
#[component]
pub async fn test_results_progress(
    cx: &Cx,
    summary: TestRunSummary,
    #[default("Tests passed")] label: &str,
    #[default("{passed} of {total} ({percent}%)")] value_text: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let percent = summary.percent(summary.passed).round();
    let shown = format!(
        "{}/{} {}",
        summary.passed,
        summary.total,
        label.to_lowercase()
    );
    let spoken = value_text
        .replace("{passed}", &summary.passed.to_string())
        .replace("{total}", &summary.total.to_string())
        .replace("{percent}", &percent.to_string());
    let style = bitop_ui::merge_style(
        &mut attrs,
        &format!(
            "--passed: {}%; --failed: {}%; --skipped: {}%",
            summary.percent(summary.passed),
            summary.percent(summary.failed),
            summary.percent(summary.skipped)
        ),
    );
    Ok(view! {
        <div style=(style) class=(class!(css::PROGRESS, attrs.remove("class"))) (attrs)>
            <div
                role="meter"
                aria-label=(&shown)
                aria-valuemin="0"
                aria-valuemax=(summary.total.max(1))
                aria-valuenow=(summary.passed)
                aria-valuetext=(spoken)
                class=(css::METER)
            >
                <div class=(css::TRACK)>
                    <span class=(css::PASSED_SEGMENT)></span>
                    <span aria-hidden="true" class=(css::FAILED_SEGMENT)></span>
                    <span aria-hidden="true" class=(css::SKIPPED_SEGMENT)></span>
                </div>
                <div aria-hidden="true" class=(css::PROGRESS_TEXT)>
                    <span class=(css::PROGRESS_LABEL)>(&shown)</span>
                    <span>(format!("{percent}%"))</span>
                </div>
            </div>
        </div>
    })
}

/// The suites, in a column.
#[component]
pub async fn test_results_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::CONTENT, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// A suite: a `<details>`. Put a [`test_suite_name`] first, then a
/// [`test_suite_content`]. `open` opens it.
#[component]
pub async fn test_suite(
    cx: &Cx,
    status: TestStatus,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            open=(open)
            data-status=(status.as_str())
            data-test-suite=""
            class=(class!(css::SUITE, attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The suite's `<summary>`: a chevron, the status, the name and `child`, such as
/// [`test_suite_stats`].
#[component]
pub async fn test_suite_name(
    cx: &Cx,
    name: &str,
    status: TestStatus,
    #[default] status_labels: TestStatusLabels<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary class=(class!(css::SUITE_TRIGGER, attrs.remove("class"))) (attrs)>
            icon(name: Icon::ChevronRight, attrs: attributes! { class=(css::CHEVRON) })
            test_status_icon(status: status, labels: status_labels)
            " "
            <span dir="ltr" class=(css::SUITE_NAME)>(name)</span>
            " "
            (child)
        </summary>
    })
}

/// A suite's counts: "3 passed, 1 failed". Zero counts are left out.
#[component]
pub async fn test_suite_stats(
    cx: &Cx,
    #[default] passed: u32,
    #[default] failed: u32,
    #[default] skipped: u32,
    #[default] labels: TestCountLabels<'_>,
    #[default(", ")] separator: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let parts: Vec<(String, &'static str, &'static str)> = [
        (labels.passed, passed, css::STAT_PASSED, "passed"),
        (labels.failed, failed, css::STAT_FAILED, "failed"),
        (labels.skipped, skipped, css::STAT_SKIPPED, "skipped"),
    ]
    .into_iter()
    .filter(|(_, n, _, _)| *n > 0)
    .map(|(template, n, class, kind)| (template.replace("{n}", &n.to_string()), class, kind))
    .collect();
    Ok(view! {
        <span class=(class!(css::STATS, attrs.remove("class"))) (attrs)>
            for (index, (text, class, kind)) in parts.into_iter().enumerate() {
                <span class=(class) data-stat=(kind)>
                    if index > 0 {
                        <span class="sr-only">(separator)</span>
                    }
                    (text)
                </span>
            }
        </span>
    })
}

/// The suite's tests: a `<ul>` named by the suite's `name`.
#[component]
pub async fn test_suite_content(
    cx: &Cx,
    name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::PANEL, attrs.remove("class"))) (attrs)>
            <ul aria-label=(name) class=(css::TESTS)>(child)</ul>
        </div>
    })
}

/// One test: its status, name and duration in milliseconds.
///
/// With `details`, the row is a `<summary>` that shows and hides `child`, such as a
/// [`test_error`]; it starts open when `open` says so, or when the test failed.
#[component]
pub async fn test(
    cx: &Cx,
    name: &str,
    status: TestStatus,
    #[default]
    #[into]
    duration: Option<f64>,
    #[default] details: bool,
    #[default]
    #[into]
    open: Option<bool>,
    #[default] status_labels: TestStatusLabels<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let open = open.unwrap_or(status == TestStatus::Failed);
    let row_class = format!("{} {}", css::TEST_ROW, css::TEST_TRIGGER);
    let time = duration.map(format_duration);
    Ok(view! {
        <li
            data-status=(status.as_str())
            data-test=""
            class=(class!(css::TEST, attrs.remove("class")))
            (attrs)
        >
            if details {
                <details open=(open) class=(css::TEST_DETAILS_ROOT)>
                    <summary class=(row_class.as_str())>
                        icon(
                            name: Icon::ChevronRight,
                            attrs: attributes! { class=(css::CHEVRON) }
                        )
                        test_status_icon(status: status, labels: status_labels)
                        " "
                        <span class=(css::TEST_NAME)>(name)</span>
                        if let Some(time) = &time {
                            " "
                            <span class=(css::TEST_DURATION)>(time)</span>
                        }
                    </summary>
                    <div class=(css::TEST_DETAILS)>(child)</div>
                </details>
            } else {
                <span class=(css::TEST_ROW)>
                    <span aria-hidden="true" class=(css::SPACER)></span>
                    test_status_icon(status: status, labels: status_labels)
                    " "
                    <span class=(css::TEST_NAME)>(name)</span>
                    if let Some(time) = &time {
                        " "
                        <span class=(css::TEST_DURATION)>(time)</span>
                    }
                </span>
            }
        </li>
    })
}

/// A test's error: a tinted box for a [`test_error_message`] and a
/// [`test_error_stack`].
#[component]
pub async fn test_error(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(css::ERROR, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

/// The error's message.
#[component]
pub async fn test_error_message(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!(css::ERROR_MESSAGE, attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// The error's stack or diff, as code. It scrolls sideways, so it is in the tab order,
/// and stays left to right.
#[component]
pub async fn test_error_stack(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <pre
            tabindex="0"
            dir="ltr"
            class=(class!(css::ERROR_STACK, attrs.remove("class")))
            (attrs)
        >
            (child)
        </pre>
    })
}

#[cfg(test)]
mod tests {
    use super::{TestRunSummary, format_duration};

    #[test]
    fn durations() {
        assert_eq!(format_duration(0.0), "0ms");
        assert_eq!(format_duration(999.4), "999ms");
        assert_eq!(format_duration(1000.0), "1s");
        assert_eq!(format_duration(1500.0), "1.5s");
        assert_eq!(format_duration(4210.0), "4.21s");
        assert_eq!(format_duration(59_999.0), "60s");
        assert_eq!(format_duration(60_000.0), "1m");
        assert_eq!(format_duration(65_000.0), "1m 5s");
        assert_eq!(format_duration(125_400.0), "2m 5s");
    }

    #[test]
    fn percent() {
        let summary = TestRunSummary {
            passed: 18,
            failed: 1,
            skipped: 1,
            total: 20,
            duration: None,
        };
        assert!((summary.percent(18) - 90.0).abs() < f64::EPSILON);
        assert!((TestRunSummary::default().percent(0)).abs() < f64::EPSILON);
    }
}
