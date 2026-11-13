export default async ({ page, check, settled }) => {
  const trigger = page.getByRole("button", { name: "Context window: 21% used, 42,000 of 200,000 tokens" });
  const card = page.locator("#context-full");
  check("trigger", [await trigger.count(), await trigger.textContent(), await trigger.getAttribute("data-level"), await trigger.getAttribute("aria-expanded")], [1, "21%", "normal", "false"]);
  check("ring", await trigger.locator("svg").evaluate((s) => s.style.getPropertyValue("--ratio")), "0.2100");

  // Enter opens the card; it holds the meter, the model, the breakdown and the cost.
  await trigger.focus();
  await page.keyboard.press("Enter");
  await settled(card);
  check("open", [await card.evaluate((c) => c.matches(":popover-open")), await trigger.getAttribute("aria-expanded")], [true, "true"]);
  // Focus moves into the card, as upstream's.
  await page.waitForFunction(() => document.activeElement?.id === "context-full");
  check("focusInCard", await page.evaluate(() => document.activeElement.id), "context-full");
  check("dialog", [await card.getAttribute("role"), await card.getAttribute("aria-label")], ["dialog", "Context window"]);
  const meter = card.getByRole("meter", { name: "Context window" });
  check("meter", [await meter.getAttribute("aria-valuenow"), await meter.getAttribute("aria-valuemax"), await meter.getAttribute("aria-valuetext")], ["42000", "200000", "21% · 42K / 200K"]);
  check("model", await card.getByText("Claude Sonnet").count(), 1);
  check("rows", await card.locator("dl > div").evaluateAll((all) => all.map((r) => [...r.querySelectorAll("dt, dd > span")].map((e) => e.textContent))), [
    ["Input", "30,000 tokens", "$0.09"],
    ["Output", "9,000 tokens", "$0.135"],
    ["Reasoning", "2,000 tokens"],
    ["Cached", "1,000 tokens"],
  ]);
  check("total", await card.locator("dl + div").evaluate((d) => d.textContent), "Total cost$0.225");
  // Above its trigger. (The examples leave room above each one: in Firefox, a card
  // that position-try moves from off-screen stops its entry transition just short of
  // the end, and would never settle.)
  const above = await page.evaluate(() => document.getElementById("context-full").getBoundingClientRect().bottom <= document.querySelector('[popovertarget="context-full"]').getBoundingClientRect().top + 1);
  check("above", above, true);

  // Escape closes it and focus goes back to the trigger.
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.getElementById("context-full").matches(":popover-open"));
  check("closed", [await trigger.getAttribute("aria-expanded"), await page.evaluate(() => document.activeElement.dataset.contextTrigger)], ["false", ""]);

  // Hover opens it after a moment, and leaving closes it again.
  const hover = await page.evaluate(() => matchMedia("(hover: hover)").matches);
  if (hover) {
    await page.evaluate(() => {
      const card = document.getElementById("context-full");
      // Not { once: true }: the toggle event of the close by Escape above is queued and
      // may arrive first, which would use the listener up.
      card.addEventListener("toggle", (e) => e.newState === "open" && (window.openedAt ??= performance.now()));
      document.querySelector('[popovertarget="context-full"]').addEventListener("pointerover", () => (window.hoveredAt ??= performance.now()));
    });
    await trigger.hover();
    await page.waitForFunction(() => document.getElementById("context-full").matches(":popover-open"));
    check("hoverDelay", await page.evaluate(() => window.openedAt - window.hoveredAt >= 150), true);
    // The pointer moves onto the card: it stays.
    await settled(card);
    const box = await card.boundingBox();
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2, { steps: 4 });
    await page.waitForTimeout(400);
    check("staysOnCard", await card.evaluate((c) => c.matches(":popover-open")), true);
    await page.mouse.move(5, 890);
    await page.waitForFunction(() => !document.getElementById("context-full").matches(":popover-open"));
    check("leaveCloses", await trigger.getAttribute("aria-expanded"), "false");
    // A click after hover keeps it open.
    await trigger.hover();
    await page.waitForFunction(() => document.getElementById("context-full").matches(":popover-open"));
    await trigger.click();
    await page.mouse.move(5, 890);
    await page.waitForTimeout(400);
    check("clickKeeps", await card.evaluate((c) => c.matches(":popover-open")), true);
    await page.keyboard.press("Escape");
    await page.waitForFunction(() => !document.getElementById("context-full").matches(":popover-open"));
  }

  // Levels, which the ring's colour and the name both carry.
  const levels = page.locator("[data-example=levels] [data-context-trigger]");
  check("levels", await levels.evaluateAll((all) => all.map((b) => [b.dataset.level, b.textContent])), [["normal", "6%"], ["high", "80%"], ["critical", "95%"]]);
  check("ringColours", await levels.evaluateAll((all) => new Set(all.map((b) => getComputedStyle(b.querySelector("circle:last-child")).stroke)).size), 3);
  await page.locator('[popovertarget="context-critical"]').click();
  const critical = page.locator("#context-critical");
  await settled(critical);
  check("criticalText", await critical.getByRole("meter").getAttribute("aria-valuetext"), "95% · 1.9M / 2M, almost full");

  // Right to left: the card still opens over its trigger.
  await page.keyboard.press("Escape");
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  await page.locator('[popovertarget="context-german"]').click();
  const german = page.locator("#context-german");
  await settled(german);
  const aligned = await page.evaluate(() => {
    const card = document.getElementById("context-german").getBoundingClientRect();
    const trigger = document.querySelector('[popovertarget="context-german"]').getBoundingClientRect();
    return Math.abs(card.right - trigger.right) <= 1;
  });
  check("rtlAligned", aligned, true);
  check("german", [await page.locator('[popovertarget="context-german"]').getAttribute("aria-label"), await german.locator("dl + div, div:last-child").last().textContent()], ["Kontextfenster: 50 % belegt, 50,000 von 100,000 Tokens", "Gesamtkosten1.50 €"]);
};
