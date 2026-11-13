export default async ({ page, check }) => {
  const details = (example, n = 0) => page.locator(`[data-example=${example}] details`).nth(n);
  const label = (locator) => locator.locator("summary [data-reasoning-label]").evaluate((l) => l.textContent.trim());
  const state = (locator) => locator.evaluate((d) => [d.open, d.hasAttribute("data-streaming")]);

  // Streaming: open, "Thinking…" shimmering.
  const live = details("streaming");
  check("streamingOpen", await state(live), [true, true]);
  check("thinking", [await label(live), await live.locator("[data-shimmer]").evaluate((s) => s.hasAttribute("data-active"))], ["Thinking…", true]);
  check("icons", await live.locator("summary [data-icon]").evaluateAll((all) => all.map((i) => i.dataset.icon)), ["brain", "chevron-down"]);

  // The reader is in the panel when streaming ends: the label says how long it took,
  // the panel closes a second later, and focus moves to the trigger instead of the page.
  await live.locator("summary").focus();
  await page.keyboard.press("Tab");
  check("focusInPanel", await page.evaluate(() => document.activeElement.textContent), "The 2025 policy");
  await page.evaluate(() => {
    const d = document.getElementById("live-reasoning");
    window.closeTimes = { ended: performance.now() };
    // The first close, not the first toggle: the toggle event of the open is queued
    // and may arrive now.
    d.addEventListener("toggle", (e) => e.newState === "closed" && (window.closeTimes.closed ??= performance.now()));
    d.removeAttribute("data-streaming");
  });
  await page.waitForFunction(() => !document.getElementById("live-reasoning").querySelector("[data-shimmer][data-active]"));
  check("measured", /^Thought for \d+ seconds?$/.test(await label(live)), true);
  check("stillOpen", await live.evaluate((d) => d.open), true);
  await page.waitForFunction(() => !document.getElementById("live-reasoning").open);
  check("closedAfterDelay", await page.evaluate(() => window.closeTimes.closed - window.closeTimes.ended >= 950), true);
  check("focusToTrigger", await page.evaluate(() => document.activeElement.tagName), "SUMMARY");
  // Streaming again opens it; it closes on its own only once.
  await page.evaluate(() => document.getElementById("live-reasoning").setAttribute("data-streaming", ""));
  await page.waitForFunction(() => document.getElementById("live-reasoning").open);
  check("reopens", await label(live), "Thinking…");
  await page.evaluate(() => {
    window.secondEnd = performance.now();
    document.getElementById("live-reasoning").removeAttribute("data-streaming");
  });
  await page.waitForFunction(() => performance.now() - window.secondEnd > 1500);
  check("closesOnce", await live.evaluate((d) => d.open), true);

  // Once the reader has toggled it, it stays as they left it.
  const german = details("strings");
  check("germanThinking", await label(german), "Denkt nach…");
  await german.locator("summary").click();
  await page.waitForFunction(() => !document.getElementById("german-reasoning").open);
  await page.evaluate(() => document.getElementById("german-reasoning").removeAttribute("data-streaming"));
  await page.waitForFunction(() => !document.querySelector("#german-reasoning [data-shimmer][data-active]"));
  check("germanDone", /^\d+ Sekunden? nachgedacht$/.test(await label(german)), true);
  await page.evaluate(() => document.getElementById("german-reasoning").setAttribute("data-streaming", ""));
  await page.waitForFunction(() => document.querySelector("#german-reasoning [data-shimmer][data-active]"));
  check("userChoiceKept", await german.evaluate((d) => d.open), false);

  // Finished: closed, with the given duration; the keyboard opens it.
  const done = details("done");
  check("done", [await state(done), await label(done)], [[false, false], "Thought for 12 seconds"]);
  await done.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-example=done] details").open);
  await done.getByText("Checked three policies").waitFor();
  check("keyboardOpens", await done.getByText("Checked three policies").isVisible(), true);

  // Open with no duration, one second, and a label of its own.
  check("unknown", [await state(details("open")), await label(details("open"))], [[true, false], "Thought for a few seconds"]);
  check("oneSecond", await label(details("labels")), "Thought for 1 second");
  check("ownLabel", await label(details("labels", 1)), "Show the plan");
};
