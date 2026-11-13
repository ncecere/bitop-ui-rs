export default async ({ page, check, settled, axe }) => {
  const card = (id) => page.locator(`#${id}`);
  const isOpen = (id) => card(id).evaluate((c) => c.matches(":popover-open"));
  const active = () => page.evaluate(() => document.activeElement.id || document.activeElement.getAttribute("aria-label") || document.activeElement.tagName);
  const waitOpen = (id, open = true) => page.waitForFunction(([i, o]) => document.getElementById(i).matches(":popover-open") === o, [id, open]);

  // The chip's name says what is cited; the card is closed.
  const one = page.getByRole("button", { name: "Source 1: Annual report 2024", exact: true });
  check("chip", [(await one.textContent()).trim(), await one.getAttribute("popovertarget"), await isOpen("cite-one")], ["1", "cite-one", false]);
  check("chipNames", await page.locator("[data-example=basic] [data-citation-trigger]").evaluateAll((all) => all.map((c) => [c.textContent.trim(), c.getAttribute("aria-label")])), [
    ["1", "Source 1: Annual report 2024"],
    ["1, 2, 3", "Sources 1, 2, 3: Annual report 2024 and 2 more"],
    ["data.example.net +1", "data.example.net +1: Climate data portal and 1 more"],
  ]);

  // A press opens the card above the chip and moves focus into it; Escape closes it and
  // returns focus to the chip.
  await one.focus();
  await page.keyboard.press("Enter");
  await waitOpen("cite-one");
  await page.waitForFunction(() => document.activeElement?.id === "cite-one");
  check("focusIn", await active(), "cite-one");
  await settled(card("cite-one"));
  const c = await card("cite-one").boundingBox();
  const t = await one.boundingBox();
  check("above", Math.round(c.y + c.height) <= Math.round(t.y), true);
  check("dialog", [await card("cite-one").getAttribute("role"), await card("cite-one").evaluate((d) => document.getElementById(d.getAttribute("aria-labelledby")).textContent.trim())], [
    "dialog", "Annual report 2024 (opens in a new tab)",
  ]);
  check("content", await card("cite-one").evaluate((d) => [...d.querySelectorAll("[data-citation-page] > *")].map((e) => e.textContent.trim())), [
    "example.com", "Annual report 2024 (opens in a new tab)", "Revenue, margins and headcount for the fiscal year.", "Revenue grew 12% year over year, driven by subscriptions.",
  ]);
  check("axeOpen", await axe(), []);
  await page.keyboard.press("Tab");
  check("tabToLink", await page.evaluate(() => document.activeElement.getAttribute("href")), "https://www.example.com/annual-report-2024");
  await page.keyboard.press("Escape");
  await waitOpen("cite-one", false);
  check("escapeReturns", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Source 1: Annual report 2024");

  // Several sources: the pager shows one at a time.
  const several = page.getByRole("button", { name: "Sources 1, 2, 3: Annual report 2024 and 2 more" });
  await several.click();
  await waitOpen("cite-several");
  const pager = card("cite-several");
  const state = () => pager.evaluate((d) => [
    d.querySelector("[data-citation-index]").textContent.trim(),
    document.getElementById(d.getAttribute("aria-labelledby")).textContent.trim(),
    [...d.querySelectorAll("[data-citation-step]")].map((b) => b.disabled),
  ]);
  check("firstPage", await state(), ["1 of 3", "Annual report 2024 (opens in a new tab)", [true, false]]);
  check("live", await pager.locator("[data-citation-index]").getAttribute("aria-live"), "polite");
  await pager.getByRole("button", { name: "Next source" }).click();
  check("secondPage", await state(), ["2 of 3", "Q4 earnings call", [false, false]]);
  await pager.getByRole("button", { name: "Next source" }).focus();
  await page.keyboard.press("Enter");
  check("lastPage", await state(), ["3 of 3", "Analyst note (opens in a new tab)", [false, true]]);
  check("focusKept", await active(), "Previous source");
  // The sources shrink while the card is open: the page is clamped to the last one.
  await pager.evaluate((d) => d.querySelectorAll("[data-citation-page]").forEach((p, i) => i > 0 && p.remove()));
  await page.keyboard.press("Enter");
  check("clamped", await state(), ["1 of 1", "Annual report 2024 (opens in a new tab)", [true, true]]);
  check("clampKeepsFocus", await active(), "cite-several");
  await page.keyboard.press("Escape");
  await waitOpen("cite-several", false);

  // Hover opens the card after a moment without moving focus, and leaving closes it.
  const site = page.getByRole("button", { name: "data.example.net +1: Climate data portal and 1 more" });
  await site.hover();
  await waitOpen("cite-site");
  check("hoverKeepsFocus", await active(), "Sources 1, 2, 3: Annual report 2024 and 2 more");
  await settled(card("cite-site"));
  const below = await card("cite-site").boundingBox();
  const siteBox = await site.boundingBox();
  check("bottomSide", Math.round(below.y) >= Math.round(siteBox.y + siteBox.height), true);
  // The pointer can move onto the card.
  await page.mouse.move(below.x + 20, below.y + 20, { steps: 4 });
  await page.waitForTimeout(400);
  check("staysOverCard", await isOpen("cite-site"), true);
  await page.mouse.move(5, 5);
  await waitOpen("cite-site", false);
  // A press while hover has it open keeps it open and moves focus in.
  await site.hover();
  await waitOpen("cite-site");
  await site.click();
  await page.waitForFunction(() => document.activeElement?.id === "cite-site");
  await page.mouse.move(5, 5);
  await page.waitForTimeout(400);
  check("pressPins", await isOpen("cite-site"), true);
  await page.keyboard.press("Escape");
  await waitOpen("cite-site", false);

  // Checked claims: the note is in the name and heads the card.
  check("verified", await page.locator("[data-example=verified] [data-citation-trigger]").evaluateAll((all) => all.map((c) => [c.dataset.verification, c.querySelector("[data-icon]").dataset.icon, c.getAttribute("aria-label")])), [
    ["verified", "check", "Source 1: Annual report 2024. Verified: the source supports this"],
    ["unsupported", "triangle-alert", "Source 2: Annual report 2024. Not supported by this source"],
    ["contradicted", "triangle-alert", "Source 3: Annual report 2024. The report gives 9%, not 12%"],
  ]);
  check("noteInCard", (await card("cite-contradicted").locator("[data-verification]").textContent()).trim(), "The report gives 9%, not 12%");

  // A chip that goes to the source listed below: no popup; the list opens and the card takes focus.
  const jump = page.locator("[data-citation-trigger=cite-jump]");
  check("noPopupButton", await jump.getAttribute("popovertarget"), null);
  await jump.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.activeElement?.id === "listed-source-1");
  check("jumped", [await page.locator("[data-example=listed] details").evaluate((d) => d.open), await isOpen("cite-jump")], [true, false]);
  // The source action closes the card and goes to its source.
  await page.locator("[data-example=listed] details").evaluate((d) => (d.open = false));
  await page.locator("[data-citation-trigger=cite-action]").click();
  await waitOpen("cite-action");
  await card("cite-action").getByRole("button", { name: "Show source 2 below" }).click();
  await page.waitForFunction(() => document.activeElement?.id === "listed-source-2");
  check("actionGoes", [await isOpen("cite-action"), await page.locator("[data-example=listed] details").evaluate((d) => d.open)], [false, true]);

  // Other strings.
  const german = page.locator("[data-citation-trigger=cite-german]");
  check("stringsName", await german.getAttribute("aria-label"), "Quellen 1, 2, 3: Annual report 2024 und 2 weitere");
  await german.click();
  await waitOpen("cite-german");
  check("stringsPager", [
    (await card("cite-german").locator("[data-citation-index]").textContent()).trim(),
    await card("cite-german").locator("[data-citation-step]").evaluateAll((all) => all.map((b) => b.getAttribute("aria-label"))),
    (await card("cite-german").locator("a").first().textContent()).trim(),
  ], ["1 von 3", ["Vorherige Quelle", "Nächste Quelle"], "Annual report 2024 (öffnet in einem neuen Tab)"]);
};
