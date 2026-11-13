export default async ({ page, base, check, focused, setDir }) => {
  const bar = page.getByRole("group", { name: "Filters", exact: true });
  const results = () => page.locator("[data-results] li").allTextContents();
  const status = bar.getByRole("radiogroup", { name: "Status" });
  const chips = () => bar.getByRole("list", { name: "Active filters" }).getByRole("link").evaluateAll((all) => all.map((a) => [a.textContent.replace(/\s+/g, " ").trim(), a.getAttribute("aria-label")]));
  const url = () => page.evaluate(() => location.pathname + location.search);
  const focusedValue = (value) => page.waitForFunction((v) => document.activeElement?.value === v, value);

  // The bar is a group; each facet's control is named by its visible label, and the
  // counts are read after the options.
  check("statusOptions", await status.getByRole("radio").evaluateAll((all) => all.map((r) => [r.closest("label").textContent.replace(/\s+/g, " ").trim(), r.value, r.checked])), [["All", "", true], ["Ready 3", "ready", false], ["Failed 2", "failed", false], ["Skipped 1", "skipped", false]]);
  check("owner", await bar.getByRole("combobox", { name: "Owner" }).evaluate((s) => [...s.options].map((o) => o.textContent.replace(/\s+/g, " ").trim())), ["Any", "Ada Lovelace", "Grace Hopper", "Alan Turing"]);
  check("dates", await bar.getByRole("radiogroup", { name: "Updated" }).getByRole("radio").evaluateAll((all) => all.map((r) => [r.closest("label").textContent.trim(), r.value])), [["Today", "today"], ["Last 7 days", "7d"], ["Last 30 days", "30d"], ["Last 90 days", "90d"], ["Custom", "custom"]]);
  // As upstream's: no "Any time" item, and no range picker until Custom is chosen.
  check("customPickerHidden", await bar.locator("[data-filter-custom-picker]").isHidden());
  check("all", [await results(), await page.locator("[data-count]").textContent(), await bar.getByRole("list", { name: "Active filters" }).count()], [["handbook.pdf", "pricing.docx", "wifi-setup.pdf", "blog/launch", "wifi-faq", "notes.docx"], "6 documents", 0]);
  // With the script a change submits at once, so Apply is hidden.
  check("enhanced", [await bar.getAttribute("data-enhanced"), await bar.locator("[data-filter-apply]").isVisible(), await bar.getByRole("button", { name: "Export" }).isVisible()], ["", false, true]);
  // The chosen item is not shown by colour alone: it is raised, with a shadow.
  check("chosenLook", await status.locator("label").evaluateAll((all) => all.slice(0, 2).map((l) => getComputedStyle(l).boxShadow !== "none")), [true, false]);
  const toggleBox = await status.boundingBox();
  check("toggleHeight", Math.round(toggleBox.height), 32);

  // Keyboard: the arrow keys move through the radio group, and the choice applies.
  await page.goto(`${base}/c/filter-bar?sort=name&page=3`);
  await status.getByRole("radio", { name: "All" }).focus();
  await page.keyboard.press("ArrowRight");
  await page.waitForURL("**/c/filter-bar?sort=name&status=ready");
  check("statusApplied", [await results(), await page.locator("[data-count]").textContent()], [["handbook.pdf", "pricing.docx", "notes.docx"], "3 documents"]);
  // Other parameters are kept, `page` is dropped, and focus is back on the choice.
  await focusedValue("ready");
  check("focusReturns", await page.evaluate(() => [document.activeElement.name, document.activeElement.checked]), ["status", true]);
  // The other facets' counts follow; a facet's own counts do not shrink to its choice.
  check("counts", [await bar.locator("details label").evaluateAll((all) => all.map((l) => l.textContent.replace(/\s+/g, " ").trim())), await status.locator("label").nth(2).textContent().then((t) => t.replace(/\s+/g, " ").trim())], [["PDF 1", "Word 2", "Web page 0"], "Failed 2"]);
  // A single-choice toggle has no chip, but "Clear all" is there.
  check("noChipForSingleToggle", await chips(), [["Clear all", null]]);

  // Several choices: a dropdown of checkboxes with its own Apply.
  const kind = bar.locator("details");
  const summary = kind.locator("summary");
  check("dropdownButton", await summary.textContent().then((t) => t.replace(/\s+/g, " ").trim()), "Kind: Any");
  await summary.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#doc-filters details").open);
  check("dropdownGroup", await kind.getByRole("group", { name: "Kind" }).getByRole("checkbox").count(), 3);
  await page.keyboard.press("Tab");
  await page.keyboard.press("Space");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Space");
  check("staysOpenWhileChoosing", [await url(), await kind.evaluate((d) => d.open), await kind.getByRole("checkbox").evaluateAll((all) => all.map((c) => c.checked))], ["/c/filter-bar?sort=name&status=ready", true, [true, true, false]]);
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  check("applyInDropdown", await focused(), "Apply");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/filter-bar?sort=name&status=ready&kind=pdf&kind=doc");
  check("kindApplied", [await results(), await summary.textContent().then((t) => t.replace(/\s+/g, " ").trim())], [["handbook.pdf", "pricing.docx", "notes.docx"], "Kind: PDF, Word"]);
  await page.waitForFunction(() => document.activeElement?.tagName === "SUMMARY");
  check("focusOnDropdownButton", await page.evaluate(() => document.activeElement.closest("details").open), false);
  check("chips", await chips(), [["Kind: PDF", "Remove filter Kind: PDF"], ["Kind: Word", "Remove filter Kind: Word"], ["Clear all", null]]);

  // A chip is a link that removes its filter; focus moves to the chip that takes its place.
  const chipList = bar.getByRole("list", { name: "Active filters" });
  await chipList.getByRole("link", { name: "Remove filter Kind: PDF" }).focus();
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/filter-bar?sort=name&status=ready&kind=doc");
  await page.waitForFunction(() => document.activeElement?.matches("[data-filter-chip]"));
  check("chipRemoved", [await results(), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], [["pricing.docx", "notes.docx"], "Remove filter Kind: Word"]);
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/filter-bar?sort=name&status=ready");
  // No chip is left: focus goes to "Clear all".
  await page.waitForFunction(() => document.activeElement?.matches("[data-filter-clear]"));
  check("lastChipRemoved", await focused(), "Clear all");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/filter-bar?sort=name");
  // Nothing is active: focus goes to the first control.
  await page.waitForFunction(() => document.activeElement?.id === "doc-search");
  check("cleared", [await results().then((r) => r.length), await bar.getByRole("list", { name: "Active filters" }).count()], [6, 0]);

  // The dropdown closes on Escape, with focus back on its button, and on a click outside.
  await summary.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#doc-filters details").open);
  await page.keyboard.press("Tab");
  await page.keyboard.press("Escape");
  check("escapeCloses", [await kind.evaluate((d) => d.open), await page.evaluate(() => document.activeElement.tagName)], [false, "SUMMARY"]);
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#doc-filters details").open);
  await page.locator("h1").click();
  check("outsideClickCloses", await kind.evaluate((d) => d.open), false);

  // A select, a date preset and the search box; each gets a chip.
  await bar.getByRole("combobox", { name: "Owner" }).selectOption("grace");
  await page.waitForURL("**/c/filter-bar?sort=name&owner=grace");
  await bar.getByRole("radio", { name: "Last 7 days" }).check();
  await page.waitForURL("**/c/filter-bar?sort=name&owner=grace&updated=7d");
  check("dateApplied", await results(), ["handbook.pdf", "pricing.docx", "wifi-faq"]);
  await page.getByRole("searchbox", { name: "Search documents" }).fill("wifi");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/filter-bar?sort=name&q=wifi&owner=grace&updated=7d");
  check("searchApplied", [await results(), await chips()], [["wifi-faq"], [
    ["Search: wifi", "Remove filter Search: wifi"], ["Owner: Grace Hopper", "Remove filter Owner: Grace Hopper"], ["Updated: Last 7 days", "Remove filter Updated: Last 7 days"], ["Clear all", null],
  ]]);
  await page.waitForFunction(() => document.activeElement?.id === "doc-search");
  // "Clear all" clears the search with the facets, and keeps the other parameters.
  await chipList.getByRole("link", { name: "Clear all" }).click();
  await page.waitForURL("**/c/filter-bar?sort=name");
  check("clearAll", await results().then((r) => r.length), 6);

  // A custom range in the URL is read, shown as a chip, and kept when another facet changes.
  await page.goto(`${base}/c/filter-bar?updated=2026-09-01/2026-09-25&status=bogus`);
  check("customRange", [await results(), await chips().then((c) => c[0][0]), await status.getByRole("radio", { name: "All" }).isChecked()], [["pricing.docx", "wifi-setup.pdf", "wifi-faq"], "Updated: Sep 1, 2026 – Sep 25, 2026", true]);
  // It is shown as "Custom", with the range in the picker beside the presets.
  const customPicker = bar.locator("[data-filter-custom-picker] [data-date-picker-trigger]");
  check("customShown", [await bar.getByRole("radio", { name: "Custom" }).isChecked(), await customPicker.isVisible(), await customPicker.getAttribute("aria-label")], [true, true, "Updated: custom Sep 1, 2026 – Sep 25, 2026"]);
  await status.getByRole("radio", { name: /Failed/ }).check();
  // The browser writes the slash of the range as %2F when it submits the form.
  await page.waitForURL("**/c/filter-bar?status=failed&updated=2026-09-01%2F2026-09-25");
  check("customRangeKept", await results(), ["wifi-setup.pdf"]);
  await bar.getByRole("radio", { name: "Today" }).check();
  await page.waitForURL("**/c/filter-bar?status=failed&updated=today");
  check("presetReplacesRange", [await results(), await bar.locator("[data-filter-custom-picker]").isHidden()], [[], true]);

  // "Custom" opens a range picker; nothing is filtered until both ends are picked.
  await page.goto(`${base}/c/filter-bar?status=failed`);
  await bar.getByRole("radio", { name: "Custom" }).check();
  const rangePopup = bar.locator("[data-filter-custom-picker] [popover]");
  await page.waitForFunction(() => document.querySelector("[data-filter-custom-picker] [popover]").matches(":popover-open") && document.querySelector("[data-filter-custom-picker] [popover]").contains(document.activeElement));
  check("customOpens", [await url(), await rangePopup.getAttribute("aria-label"), await rangePopup.locator("[data-calendar-month]").count()], ["/c/filter-bar?status=failed", "Choose dates", 2]);
  const days = rangePopup.locator("[data-day]");
  const from = await days.nth(1).getAttribute("data-day");
  const to = await days.nth(4).getAttribute("data-day");
  await days.nth(1).click();
  check("halfRangeWaits", [await url(), await rangePopup.evaluate((p) => p.matches(":popover-open"))], ["/c/filter-bar?status=failed", true]);
  await days.nth(4).click();
  await page.waitForURL(`**/c/filter-bar?status=failed&updated=${from}%2F${to}`);
  check("customApplied", [await bar.getByRole("radio", { name: "Custom" }).isChecked(), (await chips())[0][1].startsWith("Remove filter Updated: ")], [true, true]);
  await focusedValue("custom");
  // Pressed again, Custom clears the range.
  await bar.locator("label", { hasText: "Custom" }).click();
  await page.waitForURL("**/c/filter-bar?status=failed");
  check("customCleared", await bar.getByRole("radio", { name: "Custom" }).isChecked(), false);

  // The second bar: medium; a single choice with no "All" clears when chosen again.
  await page.goto(`${base}/c/filter-bar`);
  const more = page.getByRole("group", { name: "More filters" });
  const plan = more.getByRole("radiogroup", { name: "Plan" });
  check("medium", [await more.getAttribute("data-size"), Math.round((await plan.boundingBox()).height)], ["md", 36]);
  await plan.locator("label", { hasText: "Free" }).click();
  await page.waitForURL("**/c/filter-bar?plan=free");
  check("chipsOff", [await plan.getByRole("radio", { name: "Free" }).isChecked(), await more.getByRole("list").count()], [true, 0]);
  await plan.locator("label", { hasText: "Free" }).click();
  await page.waitForURL(`${base}/c/filter-bar`);
  check("clickAgainClears", await plan.getByRole("radio", { name: "Free" }).isChecked(), false);
  await plan.getByRole("radio", { name: "Team" }).focus();
  await page.keyboard.press("Space");
  await page.waitForURL("**/c/filter-bar?plan=team");
  await focusedValue("team");
  await page.keyboard.press("Space");
  await page.waitForURL(`${base}/c/filter-bar`);
  check("spaceAgainClears", await plan.getByRole("radio", { name: "Team" }).isChecked(), false);
  // Several choices in a toggle are checkboxes; "Any" clears them.
  const tags = more.getByRole("group", { name: "Tags" });
  check("tagBoxes", await tags.getByRole("checkbox").evaluateAll((all) => all.map((c) => [c.closest("label").textContent.trim(), c.checked])), [["Any", true], ["New", false], ["Pinned", false], ["Shared", false]]);
  await tags.getByRole("checkbox", { name: "New" }).check();
  await page.waitForURL("**/c/filter-bar?tag=new");
  await tags.getByRole("checkbox", { name: "Shared" }).check();
  await page.waitForURL("**/c/filter-bar?tag=new&tag=shared");
  check("tagsApplied", await tags.getByRole("checkbox").evaluateAll((all) => all.map((c) => c.checked)), [false, true, false, true]);
  // The first bar keeps the second bar's parameters.
  await status.getByRole("radio", { name: /Skipped/ }).check();
  await page.waitForURL("**/c/filter-bar?tag=new&tag=shared&status=skipped");
  await tags.getByRole("checkbox", { name: "Any" }).check();
  await page.waitForURL("**/c/filter-bar?status=skipped");
  check("anyClears", await tags.getByRole("checkbox").evaluateAll((all) => all.map((c) => c.checked)), [true, false, false, false]);

  // Right to left: the facets run from the right, and a chip's cross is on its left.
  await page.goto(`${base}/c/filter-bar?kind=pdf`);
  await setDir("rtl");
  const statusBox = await status.boundingBox();
  const searchBox = await page.getByRole("searchbox", { name: "Search documents" }).boundingBox();
  const chip = bar.getByRole("link", { name: "Remove filter Kind: PDF" });
  const cross = await chip.locator("[data-icon]").boundingBox();
  const chipBox = await chip.boundingBox();
  check("rtl", [searchBox.x > statusBox.x + statusBox.width - 1, cross.x - chipBox.x < 12], [true, true]);

  // Without a script: choose, then press Apply. Empty parameters are in the URL and are ignored.
  const plain = await page.context().browser().newContext({ javaScriptEnabled: false });
  const still = await plain.newPage();
  await still.goto(`${base}/c/filter-bar`);
  const stillBar = still.getByRole("group", { name: "Filters", exact: true });
  await stillBar.getByRole("radio", { name: /Failed/ }).check();
  check("noScriptWaits", [await still.evaluate(() => location.search), await stillBar.getByRole("button", { name: "Apply" }).first().isVisible()], ["", true]);
  await stillBar.locator("[data-filter-apply]").click();
  await still.waitForURL("**/c/filter-bar?q=&status=failed&owner=");
  // "Custom" needs the script.
  check("noScriptNoCustom", await stillBar.locator("[data-filter-custom-item]").isHidden());
  check("noScriptApplies", await still.locator("[data-results] li").allTextContents(), ["wifi-setup.pdf", "blog/launch"]);
  await stillBar.getByRole("link", { name: "Clear all" }).click();
  await still.waitForURL(`${base}/c/filter-bar`);
  check("noScriptClears", await still.locator("[data-results] li").count(), 6);
  await plain.close();
};
