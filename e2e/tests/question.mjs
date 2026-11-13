export default async ({ page, check, base }) => {
  const single = page.locator("#single-question");
  const group = single.getByRole("radiogroup");
  const error = single.locator("[data-question-error]");

  // A form named by its prompt, with a radio group named by the same.
  check("form", [await single.evaluate((f) => f.tagName), await single.getAttribute("aria-label"), await single.getAttribute("data-state")], ["FORM", "Which environment should I deploy to?", "pending"]);
  check("legend", await group.locator("legend").textContent(), "Which environment should I deploy to?");
  check("options", await group.locator("input").evaluateAll((all) => all.map((i) => [i.value, i.disabled])), [["staging", false], ["production", false], ["legacy", true], ["__other__", false]]);
  check("described", await group.getAttribute("aria-describedby"), "environment-description");
  check("otherHidden", await single.locator("[data-question-other-text]").isVisible(), false);

  // Submitting without an answer shows an error and moves focus to the group.
  await single.getByRole("button", { name: "Submit" }).click();
  await error.waitFor();
  check("error", [await error.textContent(), await group.getAttribute("aria-invalid"), await group.getAttribute("aria-describedby")], ["Choose an answer.", "true", "environment-description environment-error"]);
  check("focusFirst", await page.evaluate(() => document.activeElement.value), "staging");
  check("notSent", page.url(), `${base}/c/question`);

  // "Other" shows a text box; choosing it alone is not an answer until there is text.
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("skipsDisabled", await page.evaluate(() => document.activeElement.value), "__other__");
  const other = single.getByRole("textbox", { name: "Your answer" });
  await other.waitFor();
  check("otherShown", [await other.isVisible(), await other.isDisabled()], [true, false]);
  await single.getByRole("button", { name: "Submit" }).click();
  await error.waitFor();
  check("otherNeedsText", [await error.isVisible(), await page.evaluate(() => document.activeElement.value)], [true, "__other__"]);
  await other.fill("QA");
  check("errorCleared", [await error.isVisible(), await group.getAttribute("aria-invalid")], [false, null]);
  // Back to a listed choice: the text box hides and is not sent.
  await group.getByRole("radio", { name: "Staging" }).check();
  check("otherOff", [await other.isVisible(), await single.evaluate((f) => [...new FormData(f).keys()])], [false, ["environment"]]);

  // Sent with the keyboard: the server renders it answered and locked.
  await page.evaluate(() => {
    window.questions = [];
    document.addEventListener("bitop:question-submit", (e) => window.questions.push([e.target.id, e.detail]));
  });
  await group.getByRole("radio", { name: "Production" }).focus();
  await page.keyboard.press("Space");
  await single.getByRole("button", { name: "Submit" }).focus();
  await Promise.all([page.waitForURL(/environment=production/), page.keyboard.press("Enter")]);
  check("answered", [await single.getAttribute("data-state"), await single.locator("[role=status]").textContent(), await single.getByRole("button").count()], ["answered", "Answered", 0]);
  check("locked", await single.locator("fieldset").evaluate((f) => [f.disabled, f.querySelector("input:checked").value]), [true, "production"]);
  check("answeredIcon", await single.locator("[data-icon]").evaluateAll((all) => all.filter((i) => i.checkVisibility()).map((i) => i.dataset.icon)), ["circle-check"]);

  // Skip needs no answer.
  await page.goto(`${base}/c/question`);
  await single.getByRole("button", { name: "Skip" }).click();
  await page.waitForURL(/action=skip/);
  check("skipped", [await single.getAttribute("data-state"), await single.locator("[role=status]").textContent()], ["skipped", "Skipped"]);
  await page.goto(`${base}/c/question`);
  await page.evaluate(() => {
    window.questions = [];
    document.addEventListener("bitop:question-submit", (e) => window.questions.push([e.target.id, e.detail]));
  });

  // Several choices, answered in script: checkboxes, the event, and the focus.
  const multiple = page.locator("#multiple-question");
  const boxes = multiple.getByRole("checkbox");
  check("checkboxes", await boxes.evaluateAll((all) => all.map((b) => [b.value, b.checked])), [["tests", true], ["lint", false], ["docs", false], ["__other__", false]]);
  check("groupNotRadio", await multiple.locator("fieldset").getAttribute("role"), null);
  await multiple.getByRole("checkbox", { name: "Lint the code" }).focus();
  await page.keyboard.press("Space");
  await multiple.getByRole("checkbox", { name: "Other" }).check();
  await multiple.getByRole("textbox", { name: "Your answer" }).fill("Smoke test");
  await multiple.getByRole("button", { name: "Submit" }).focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("multiple-question").dataset.state === "answered");
  check("event", await page.evaluate(() => window.questions), [["multiple-question", { skipped: false, selected: ["tests", "lint"], text: "Smoke test" }]]);
  check("scriptLocked", [await multiple.locator("fieldset").evaluate((f) => f.disabled), await multiple.locator("[data-question-other-text]").isDisabled()], [true, true]);
  check("announced", await multiple.locator("[role=status]").textContent(), "Answered");
  check("focusToForm", await page.evaluate(() => document.activeElement.id), "multiple-question");
  check("extra", await multiple.getByText("I'll wait for your answer").count(), 1);

  // Free text, with a heading: an empty answer is refused, then skipped in script.
  const freeform = page.locator("#freeform-question");
  check("heading", await freeform.locator("h3").evaluate((h) => [h.textContent, h.tabIndex]), ["Anything I should mention in the release notes?", -1]);
  const box = freeform.getByRole("textbox", { name: "Anything I should mention in the release notes?" });
  check("textbox", await box.getAttribute("placeholder"), "For example: the new export button");
  await freeform.getByRole("button", { name: "Submit" }).click();
  await freeform.locator("[data-question-error]").waitFor();
  check("freeError", [await freeform.locator("[data-question-error]").textContent(), await page.evaluate(() => document.activeElement.tagName), await box.getAttribute("aria-invalid")], ["Type an answer.", "TEXTAREA", "true"]);
  await freeform.getByRole("button", { name: "Skip" }).click();
  await page.waitForFunction(() => document.getElementById("freeform-question").dataset.state === "skipped");
  check("skipEvent", await page.evaluate(() => window.questions.at(-1)), ["freeform-question", { skipped: true, selected: [], text: "" }]);
  check("skipLocked", [await box.isDisabled(), await freeform.locator("[data-question-error]").isVisible(), await freeform.locator("[role=status]").textContent()], [true, false, "Skipped"]);

  // Rendered answered and skipped.
  const done = page.locator("[data-example=done] form");
  check("done", await done.evaluateAll((all) => all.map((f) => [f.dataset.state, f.querySelector("[role=status]").textContent, f.querySelectorAll("button").length > 0 && [...f.querySelectorAll("button")].some((b) => b.checkVisibility())])), [["answered", "Answered", false], ["skipped", "Skipped", false]]);

  // Other strings.
  const german = page.locator("#german-question");
  await german.getByRole("button", { name: "Senden" }).click();
  await german.locator("[data-question-error]").waitFor();
  check("german", [await german.locator("[data-question-error]").textContent(), await german.getByRole("radio", { name: "Andere" }).count()], ["Wähle eine Antwort.", 1]);

  // Right to left: the icon is at the start, and the arrow keys follow the page.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  await german.getByRole("radio", { name: "Englisch" }).focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlArrows", await page.evaluate(() => document.activeElement.value), "de");
  check("rtlIcon", await german.evaluate((f) => f.firstElementChild.getBoundingClientRect().left > f.lastElementChild.getBoundingClientRect().left), true);
};
