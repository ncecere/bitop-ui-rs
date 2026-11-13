export default async ({ page, check }) => {
  const form = page.locator("#setup");
  const visibleSteps = () => form.locator(":scope > [data-questionnaire-step]").evaluateAll((all) => all.filter((s) => !s.hidden).map((s) => s.dataset.questionnaireStep));
  const bar = form.getByRole("progressbar");
  const progress = async () => [await bar.getAttribute("aria-label"), await bar.getAttribute("aria-valuenow"), await bar.getAttribute("aria-valuemax")];
  const buttons = () => form.locator(":scope > div:last-child > button").evaluateAll((all) => all.filter((b) => b.checkVisibility()).map((b) => b.textContent.trim()));
  const focusedHeading = () => page.evaluate(() => /^H[2-6]$/.test(document.activeElement.tagName) && document.activeElement.textContent);

  // The first step: one question, the progress, and Next.
  check("named", await form.getAttribute("aria-label"), "Project setup");
  check("firstStep", [await visibleSteps(), await progress(), await buttons()], [["0"], ["Question 1 of 3", "1", "3"], ["Next"]]);
  check("heading", await form.locator("h3").first().textContent(), "Where should it run?");

  // Next without an answer: an error, and focus on the first choice.
  await form.getByRole("button", { name: "Next" }).click();
  const error = form.locator("[data-questionnaire-step='0'] [data-question-error]");
  await error.waitFor();
  check("required", [await error.textContent(), await visibleSteps(), await page.evaluate(() => document.activeElement.value)], ["Answer this question to continue.", ["0"], "eu"]);

  // Answer with the keyboard and go on: focus moves to the next step's heading.
  await page.keyboard.press("Space");
  check("errorGone", await error.isVisible(), false);
  await page.keyboard.press("Tab");
  check("tabToNext", await page.evaluate(() => document.activeElement.textContent.trim()), "Next");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#setup").dataset.step === "1");
  check("secondStep", [await visibleSteps(), await progress(), await buttons(), await focusedHeading()], [["1"], ["Question 2 of 3", "2", "3"], ["Back", "Next"], "What do you need?"]);

  // Several choices and Other.
  const step1 = form.locator("[data-questionnaire-step='1']");
  await step1.getByRole("checkbox", { name: "Billing" }).check();
  await step1.getByRole("checkbox", { name: "Other" }).check();
  await step1.getByRole("textbox", { name: "Your answer" }).fill("Analytics");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#setup").dataset.step === "2");
  check("thirdStep", [await progress(), await buttons(), await focusedHeading()], [["Question 3 of 3", "3", "3"], ["Back", "Skip", "Next"], "Anything else?"]);

  // Skip the optional question: the review lists every answer.
  await form.locator("[data-questionnaire-step='2'] textarea").fill("Will be dropped");
  await form.getByRole("button", { name: "Skip" }).click();
  await page.waitForFunction(() => document.querySelector("#setup").dataset.step === "3");
  check("review", [await progress(), await buttons(), await focusedHeading()], [["Review your answers", "3", "3"], ["Back", "Submit"], "Review your answers"]);
  check("answers", await form.locator("[data-questionnaire-answer]").evaluateAll((all) => all.map((a) => [a.textContent, a.hasAttribute("data-skipped")])), [["Europe", false], ["Billing, Analytics", false], ["Skipped", true]]);
  check("skippedCleared", await form.locator("[data-questionnaire-step='2'] textarea").inputValue(), "");

  // Edit returns to the question, with its answer.
  await form.getByRole("button", { name: "Edit What do you need?" }).click();
  await page.waitForFunction(() => document.querySelector("#setup").dataset.step === "1");
  check("edit", [await focusedHeading(), await step1.getByRole("checkbox", { name: "Billing" }).isChecked()], ["What do you need?", true]);
  await form.getByRole("button", { name: "Back" }).click();
  await page.waitForFunction(() => document.querySelector("#setup").dataset.step === "0");
  check("back", [await focusedHeading(), await buttons()], ["Where should it run?", ["Next"]]);

  // Through to the end: the event, then the form is sent.
  await page.evaluate(() => {
    window.answers = [];
    document.addEventListener("bitop:questionnaire-submit", (e) => window.answers.push(e.detail.answers));
  });
  for (const step of ["1", "2", "3"]) {
    await form.getByRole("button", { name: "Next" }).click();
    await page.waitForFunction((s) => document.querySelector("#setup").dataset.step === s, step);
  }
  await Promise.all([page.waitForURL(/region=eu/), form.getByRole("button", { name: "Submit" }).click()]);
  const query = new URL(page.url()).searchParams;
  check("sent", [query.get("region"), query.getAll("features"), query.get("features-other"), query.get("notes")], ["eu", ["billing", "__other__"], "Analytics", ""]);

  // Starting at the review, sent in script.
  const team = page.locator("#team");
  check("startsAtReview", [await team.getByRole("progressbar").getAttribute("aria-label"), await team.locator("[data-questionnaire-answer]").allTextContents()], ["Review your answers", ["Up to 10", "Skipped"]]);
  await team.getByRole("button", { name: "Submit" }).click();
  await page.waitForFunction(() => document.querySelector("#team").dataset.sent);
  check("scriptAnswers", JSON.parse(await team.getAttribute("data-sent")), { size: { selected: ["small"] }, deadline: null });

  // Without a review, in German: the last question's button sends it.
  const short = page.locator("#short");
  await short.getByRole("button", { name: "Weiter" }).click();
  await page.waitForFunction(() => document.querySelector("#short").dataset.step === "1");
  check("germanLast", await short.locator(":scope > div:last-child > button").evaluateAll((all) => all.filter((b) => b.checkVisibility()).map((b) => b.textContent.trim())), ["Zurück", "Überspringen", "Absenden"]);
  check("germanProgress", await short.getByRole("progressbar").getAttribute("aria-label"), "Frage 2 von 2");
  await short.getByRole("button", { name: "Überspringen" }).click();
  await page.waitForFunction(() => document.querySelector("#short").dataset.sent);
  check("germanSent", JSON.parse(await short.getAttribute("data-sent")), { size: { selected: ["small"] }, deadline: null });

  // Right to left: Back sits at the right.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const backRight = await short.evaluate((f) => {
    const back = f.querySelector("[data-questionnaire-back]").getBoundingClientRect();
    const next = f.querySelector("[data-questionnaire-next]").getBoundingClientRect();
    return back.left > next.left;
  });
  check("rtlBack", backRight, true);
};
