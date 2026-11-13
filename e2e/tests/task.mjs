export default async ({ page, check }) => {
  const details = (example) => page.locator(`[data-example=${example}] details`);
  const open = details("open");

  // Open: a list of what it did, with file chips.
  check("open", await open.evaluate((d) => d.open), true);
  check("items", await open.locator("li").count(), 3);
  check("files", await open.locator("li").nth(1).locator("[data-icon=file-text]").count(), 2);
  check("triggerIcons", await open.locator("summary [data-icon]").evaluateAll((all) => all.map((i) => i.dataset.icon)), ["search", "chevron-down"]);
  check("chevronTurned", await open.locator("summary [data-icon=chevron-down]").evaluate((i) => getComputedStyle(i).rotate), "180deg");

  // Running: a loader, and "(in progress)" is part of the trigger's name.
  const running = details("running");
  check("runningName", await running.locator("summary").evaluate((s) => s.textContent.replace(/\s+/g, " ").trim()), "Reading the policy library (in progress)");
  check("loader", await running.locator("summary [data-loader]").getAttribute("aria-hidden"), "true");
  check("runningClosed", await running.evaluate((d) => d.open), false);
  check("notRunning", await open.locator("[data-loader]").count(), 0);

  // The keyboard opens and closes it.
  const closed = details("closed");
  await closed.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("closed-task").open);
  await closed.getByText("offices.csv").waitFor();
  check("keyboardOpens", await closed.getByText("offices.csv").isVisible(), true);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => !document.getElementById("closed-task").open);
  check("keyboardCloses", await closed.evaluate((d) => d.open), false);
  check("ownIcon", await closed.locator("summary [data-icon]").first().getAttribute("data-icon"), "book-open");

  // A long title is cut with an ellipsis, and its full text is its title attribute.
  const title = closed.locator("summary span[title]");
  check("ellipsis", await title.evaluate((t) => t.scrollWidth > t.clientWidth && getComputedStyle(t).textOverflow === "ellipsis"), true);

  // Right to left: the guide line moves to the right of the items.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const list = open.locator("ul");
  check("rtlLine", await list.evaluate((l) => [getComputedStyle(l).borderRightWidth, getComputedStyle(l).borderLeftWidth]), ["2px", "0px"]);

  check("german", await details("strings").locator("summary .sr-only").textContent(), "(läuft)");
};
