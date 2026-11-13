export default async ({ page, check }) => {
  const card = page.locator("[data-example=full] [data-agent]");
  check("named", [await card.evaluate((c) => c.tagName), await card.getAttribute("aria-label")], ["SECTION", "Research assistant"]);
  check("model", await card.locator("[data-agent-model]").evaluate((b) => b.textContent), "Model: claude-sonnet-4");
  check("icon", await card.locator("[data-icon]").first().getAttribute("data-icon"), "bot");
  check("groups", await card.locator("[role=group]").evaluateAll((all) => all.map((g) => g.getAttribute("aria-label"))), ["Instructions", "Tools", "Output schema"]);
  // Line breaks in the instructions are kept.
  check("preWrap", await card.locator("[role=group]").first().locator("div").evaluate((d) => getComputedStyle(d).whiteSpace), "pre-wrap");

  // Tools: a heading in each trigger; one open at a time.
  const tools = card.locator("[data-agent-tool]");
  check("headings", await tools.locator("summary h3").allTextContents(), [
    "search_webSearch the web for recent pages",
    "fetch_pageRead one page",
    "current_timeThe time now, in UTC",
    "delete_filesNot allowed for this agent",
  ]);
  check("firstOpen", await tools.evaluateAll((all) => all.map((d) => d.open)), [true, false, false, false]);
  check("schema", await tools.first().locator("[data-code-block]").evaluate((b) => [b.dataset.language, b.textContent.includes("Input schema")]), ["json", true]);

  await tools.nth(1).locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-agent-tool=fetch_page]").open);
  check("exclusive", await tools.evaluateAll((all) => all.map((d) => d.open)), [false, true, false, false]);
  // Content that has just been shown cannot take focus at once; wait for it.
  await tools.nth(1).locator("pre").waitFor();
  await page.keyboard.press("Tab");
  // The code in the open panel is in the tab order (it scrolls), then the copy button.
  const next = await page.evaluate(() => document.activeElement.closest("[data-agent-tool]")?.dataset.agentTool);
  check("tabIntoPanel", next, "fetch_page");

  // A tool without a schema says so.
  const timeTool = card.locator("[data-agent-tool=current_time]");
  await timeTool.locator("summary").click();
  await timeTool.getByText("No input parameters.").waitFor();
  check("empty", await timeTool.getByText("No input parameters.").isVisible(), true);

  // A disabled tool cannot be focused or opened.
  const disabled = card.locator("[data-agent-tool=delete_files] summary");
  check("disabled", [await disabled.getAttribute("aria-disabled"), await disabled.getAttribute("tabindex")], ["true", "-1"]);
  await disabled.click({ force: true });
  check("staysClosed", await card.locator("[data-agent-tool=delete_files]").evaluate((d) => d.open), false);

  // The output schema.
  check("output", await card.locator("[role=group]").last().locator("code").getAttribute("class"), "language-typescript");

  // Right to left: the chevrons go to the left end.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const left = await tools.first().locator("summary").evaluate((s) => s.querySelector("[data-icon=chevron-down]").getBoundingClientRect().left - s.getBoundingClientRect().left < 30);
  check("rtlChevron", left, true);

  const german = page.locator("[data-example=strings] [data-agent]");
  check("german", [await german.locator("[data-agent-model]").textContent(), await german.locator("summary h3").count(), await german.locator("[role=group]").evaluateAll((all) => all.map((g) => g.getAttribute("aria-label")))], ["Modell: small-1", 1, ["Anweisungen", "Werkzeuge"]]);
};
