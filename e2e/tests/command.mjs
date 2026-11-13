export default async ({ page, check, axe, settled }) => {
  const palette = page.locator("dialog[data-command-palette]");
  const search = palette.getByRole("combobox");
  const toasts = page.locator("[data-toast]");
  const labels = () => palette.locator("[role=option]:not([hidden]) [data-command-label]").allTextContents();
  const active = () => palette.locator("[data-active] [data-command-label]").allTextContents();
  const groups = () => palette.locator("[role=group]:not([hidden])").evaluateAll((all) => all.map((g) => g.getAttribute("aria-label")));
  const trigger = page.getByRole("button", { name: "Search or jump to…" });

  // The trigger looks like a search field with its shortcut.
  check("trigger", [await trigger.getAttribute("aria-keyshortcuts"), (await trigger.locator("kbd").allTextContents()).map((t) => t.trim())], ["Meta+K Control+K", ["CtrlCommand or Control", "K"]]);

  await page.keyboard.press("Control+k");
  check("shortcut", await palette.evaluate((d) => d.open && d.matches(":modal")));
  check("focus", await page.evaluate(() => document.activeElement.getAttribute("role")), "combobox");
  check("firstActive", await active(), ["Projects"]);
  await settled(palette);
  const box = await palette.boundingBox();
  check("centredAcross", Math.abs(box.x * 2 + box.width - page.viewportSize().width) < 2);
  check("item", await palette.locator("[role=option]").nth(1).evaluate((o) => [o.querySelector("[data-icon]").dataset.icon, o.querySelector("[data-command-label] + span").textContent]), ["arrow-right", "Page"]);
  check("shortcutHint", (await palette.getByRole("option", { name: /Toggle dark mode/ }).locator("kbd").allTextContents()).map((t) => t.trim()), ["ShiftShift", "D"]);
  check("esc", [await palette.getByRole("button", { name: "Esc — Close command palette" }).textContent()], ["Esc"]);
  check("footer", (await palette.locator("[aria-hidden=true]", { hasText: "to navigate" }).count()) === 1 && (await search.getAttribute("aria-describedby")) !== null);
  check("axeOpen", await axe(), []);

  // Every word must match, in the singular or the plural, in the label or keywords.
  await page.keyboard.type("invoices");
  check("keywords", await labels(), ["Billing"]);
  check("hidesEmptyGroups", await groups(), ["Jump to"]);
  await search.fill("members");
  check("plural", await labels(), ["Add member"]);
  await search.fill("policies");
  check("pluralIes", await labels(), ["Retention policy"]);
  await search.fill("add mem");
  check("eachWord", await labels(), ["Add member"]);

  // Ranking: an exact label first, then a label starting with the query, then word
  // starts, then keywords; the group with the best match moves up.
  await search.fill("usage");
  check("exactFirst", [await labels(), await groups()], [["Usage"], ["Settings"]]);
  await search.fill("s");
  check("prefixFirst", [(await labels())[0], (await groups())[0]], ["Show a toast", "Actions"]);
  await search.fill("toast");
  check("groupMovesUp", await groups(), ["Actions"]);
  await search.fill("legal");
  check("rankAndHighlight", [await labels(), await active()], [["Legal holds"], ["Legal holds"]]);
  await search.fill("o");
  const ranked = await labels();
  check("bestGroupFirst", (await groups())[0], "Jump to");
  check("rankedInGroup", ranked.indexOf("Toggle dark mode") > -1);

  await search.fill("nothing here");
  check("empty", await palette.locator("[data-command-empty]").textContent(), "No results found.");
  await search.fill("show a toast");
  await page.keyboard.press("Enter");
  await page.waitForTimeout(300);
  check("runsAction", [await palette.evaluate((d) => !d.open), await toasts.allTextContents().then((t) => t.some((x) => x.includes("Hello from the palette")))], [true, true]);
  await trigger.click();
  check("resets", [await search.inputValue(), await labels()], ["", ["Projects", "Billing", "Show a toast", "Toggle dark mode", "Add member", "Retention policy", "Usage", "Legal holds"]]);
  await page.keyboard.press("ArrowUp");
  check("arrowWraps", await active(), ["Legal holds"]);
  await page.keyboard.press("ArrowDown");
  check("arrowWrapsDown", await active(), ["Projects"]);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(300);
  check("escCloses", await palette.evaluate((d) => !d.open));
  await page.keyboard.press("Control+k");
  await palette.getByRole("button", { name: "Esc — Close command palette" }).click();
  await page.waitForTimeout(300);
  check("escButton", await palette.evaluate((d) => !d.open));
  await trigger.click();
  await page.keyboard.press("Enter");
  await page.waitForURL((url) => url.pathname === "/");
  check("followsLink", new URL(page.url()).pathname, "/");
};
