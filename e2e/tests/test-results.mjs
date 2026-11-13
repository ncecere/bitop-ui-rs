export default async ({ page, check, setDir }) => {
  const text = (locator) => locator.evaluate((e) => e.textContent.replace(/\s+/g, " ").trim());
  const run = page.locator("#run");

  // The counts are written out; zero counts are left out elsewhere.
  check("badges", await run.locator("[data-count]").evaluateAll((all) => all.map((b) => [b.dataset.count, b.textContent.trim(), b.querySelector("[data-icon]").dataset.icon])), [
    ["passed", "18 passed", "circle-check"],
    ["failed", "1 failed", "circle-x"],
    ["skipped", "1 skipped", "circle-minus"],
  ]);
  check("duration", await text(run.locator("[data-count]").first().locator("xpath=../..").locator(":scope > span").last()), "Duration: 4.21s");

  // The pass rate is a meter with its value in words; the bar has three segments.
  const meter = page.getByRole("meter", { name: "18/20 tests passed" });
  check("meter", [await meter.getAttribute("aria-valuenow"), await meter.getAttribute("aria-valuemax"), await meter.getAttribute("aria-valuetext")], ["18", "20", "18 of 20 (90%)"]);
  check("meterText", await meter.locator("[aria-hidden=true] > span").allTextContents(), ["18/20 tests passed", "90%"]);
  const widths = await meter.evaluate(async (m) => {
    const track = m.firstElementChild;
    await Promise.all(m.getAnimations({ subtree: true }).map((a) => a.finished));
    return [...track.children].map((s) => Math.round((s.getBoundingClientRect().width / track.getBoundingClientRect().width) * 100));
  });
  check("segments", widths, [90, 5, 5]);

  // A suite is a disclosure; its tests are a list named by the suite, each status an icon
  // and a word.
  const auth = page.locator("#auth-suite");
  const summary = auth.locator("> summary");
  check("suiteOpen", await auth.evaluate((d) => d.open), true);
  check("suiteName", await text(summary), "Failed: auth.test.ts 3 passed, 1 failed");
  check("suiteStats", await summary.locator("[data-stat]").evaluateAll((all) => all.map((s) => s.dataset.stat)), ["passed", "failed"]);
  const tests = page.getByRole("list", { name: "auth.test.ts" });
  check("tests", await tests.locator(":scope > li").evaluateAll((all) => all.map((li) => li.dataset.status)), ["passed", "failed", "passed", "passed"]);
  check("testRow", await text(tests.locator(":scope > li").first()), "Passed: refreshes the token 12ms");
  check("durations", await tests.locator("li span:not(:has(*)):last-child").evaluateAll((all) => all.map((s) => s.textContent).filter((t) => /^\d[\d.]*(ms|s)$/.test(t))), ["12ms", "40ms", "8ms", "1.53s"]);
  check("statusColours", await tests.locator("[data-test-status]").evaluateAll((all) => new Set(all.map((s) => getComputedStyle(s).color)).size), 2);

  // A failed test starts open; its error and stack are shown; the stack scrolls from the keyboard.
  const failed = page.locator("#failed-test details");
  check("failedOpen", await failed.evaluate((d) => d.open), true);
  check("error", await text(page.locator("#failed-test p")), "Expected 401, got 200");
  const stack = page.locator("#failed-test pre");
  check("stack", [await stack.getAttribute("tabindex"), await stack.getAttribute("dir"), (await stack.textContent()).startsWith("AssertionError")], ["0", "ltr", true]);
  // The keyboard closes it and opens it again.
  await failed.locator("> summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => !document.querySelector("#failed-test details").open);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector("#failed-test details").open);
  check("toggles", await failed.evaluate((d) => d.open), true);
  // Tab moves from the test's row into its stack.
  await stack.waitFor();
  await page.keyboard.press("Tab");
  check("tabToStack", await page.evaluate(() => document.activeElement.tagName), "PRE");

  // A closed suite opens from the keyboard and turns its chevron.
  const billing = page.locator("#billing-suite");
  check("closedSuite", await billing.evaluate((d) => d.open), false);
  await billing.locator("> summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("billing-suite").open);
  const turned = await billing.locator("> summary > [data-icon]").evaluate(async (i) => {
    await Promise.all(i.getAnimations().map((a) => a.finished));
    return getComputedStyle(i).rotate;
  });
  check("chevron", turned, "90deg");
  check("skippedIcon", await page.locator("[data-test-status][data-status=skipped] [data-icon]").first().getAttribute("data-icon"), "circle-minus");

  // A running test spins.
  const running = page.locator("[data-example=running] li[data-status=running] [data-icon]");
  check("spins", [await running.getAttribute("data-icon"), await running.evaluate((i) => getComputedStyle(i).animationName !== "none")], ["loader-circle", true]);
  check("runningWord", await text(page.locator("[data-example=running] li[data-status=running]")), "Running: pages the results");

  // All green: no failed or skipped badges; minutes.
  const green = page.locator("[data-example=green]");
  check("green", [await green.locator("[data-count]").count(), await text(green.locator("[data-count]")), await green.getByRole("meter", { name: "42/42 checks passed" }).getAttribute("aria-valuetext")], [1, "42 passed", "42 of 42 (100%)"]);
  check("minutes", await text(green.locator("[data-count]").locator("xpath=../..").locator(":scope > span").last()), "Duration: 1m 5s");

  const german = page.locator("[data-example=strings]");
  check("german", [await german.locator("[data-count]").allTextContents(), await german.getByRole("meter").getAttribute("aria-label"), await text(german.locator("summary").first())], [
    [" 18 bestanden", " 1 fehlgeschlagen", " 1 übersprungen"].map((t) => t.trim()),
    "18/20 tests bestanden",
    "Fehlgeschlagen: konto.test.ts 1 fehlgeschlagen",
  ]);

  // Right to left: the stats go to the left and the open chevron still points down.
  await setDir("rtl");
  const rtl = await summary.evaluate(async (s) => {
    const chevron = s.querySelector("[data-icon]");
    await Promise.all(chevron.getAnimations().map((a) => a.finished));
    const name = s.querySelector("[dir=ltr]").getBoundingClientRect();
    const stats = s.querySelector("[data-stat]").parentElement.getBoundingClientRect();
    return [stats.right <= name.left, getComputedStyle(chevron).rotate, getComputedStyle(chevron).scale];
  });
  check("rtl", rtl, [true, "-90deg", "-1 1"]);
};
