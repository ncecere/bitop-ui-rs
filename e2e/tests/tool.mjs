export default async ({ page, check }) => {
  const name = (id) => page.locator(`#${id} > summary`).evaluate((s) => s.textContent.replace(/\s+/g, " ").trim());

  // Completed and open: the name, the state as a word, the summary, input and output.
  const done = page.locator("#completed-tool");
  check("name", await name("completed-tool"), "search_documents Completed 2 results");
  check("states", await page.locator("details[data-tool] > summary").evaluateAll((all) => all.map((s) => s.dataset.state)), ["completed", "pending", "running", "error", "completed", "running"]);
  const labels = await done.locator("section > p").allTextContents();
  check("sections", labels, ["Parameters", "Result"]);
  check("sectionCaps", await done.locator("section > p").first().evaluate((p) => getComputedStyle(p).textTransform), "uppercase");
  check("input", await done.locator("section").first().locator("code").textContent(), '{\n  "query": "refund policy",\n  "limit": 5\n}');
  check("json", await done.locator("section code").evaluateAll((all) => all.map((c) => c.className)), ["language-json", "language-json"]);
  check("output", (await done.locator("section").nth(1).locator("code").textContent()).includes("policies/refunds.md"), true);
  // The code scrolls sideways from the keyboard, and copies.
  check("preFocusable", await done.locator("pre").first().getAttribute("tabindex"), "0");
  check("copy", await done.getByRole("button", { name: "Copy JSON code" }).count(), 2);

  // Every state has its own word and colour.
  check("pending", await name("pending-tool"), "send_email Pending");
  check("running", await name("running-tool"), "Searching the web Running");
  check("errorName", await name("error-tool"), "read_file Error");
  const badges = page.locator("[data-example=states] summary > span[data-state]");
  check("badgeColours", await badges.evaluateAll((all) => new Set(all.map((b) => getComputedStyle(b).backgroundColor)).size), 3);
  check("pulse", await page.locator("#running-tool summary span[data-state] > span").evaluate((d) => getComputedStyle(d, "::after").animationName !== "none"), true);
  check("noPulse", await page.locator("#pending-tool summary span[data-state] > span").evaluate((d) => getComputedStyle(d, "::after").content), "none");
  check("ownIcon", await page.locator("#running-tool summary [data-icon]").first().getAttribute("data-icon"), "globe");

  // The keyboard opens a failed call; the error replaces the result.
  const failed = page.locator("#error-tool");
  await failed.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("error-tool").open);
  const error = failed.getByText("Permission denied");
  await error.waitFor();
  check("error", [await failed.locator("section > p").allTextContents(), await failed.locator("[data-tool-output=error] [data-icon]").getAttribute("data-icon")], [["Parameters", "Error"], "circle-alert"]);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => !document.getElementById("error-tool").open);
  check("closes", await failed.evaluate((d) => d.open), false);

  // Text input and a custom result.
  const custom = page.locator("[data-example=custom]");
  check("textInput", await custom.locator("pre code").getAttribute("class"), "language-text");
  check("customOutput", await custom.locator("[data-custom-output]").textContent(), "Refunds are due within 30 days of a return.");
  check("stateLabel", await custom.locator("summary span[data-state]").textContent(), "Done");

  // Right to left: the chevron goes to the left end.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const chevronLeft = await done.locator("summary").evaluate((s) => {
    const chevron = s.querySelector("[data-icon=chevron-down]").getBoundingClientRect();
    return chevron.left - s.getBoundingClientRect().left < 40;
  });
  check("rtlChevron", chevronLeft, true);

  check("german", [await page.locator("[data-example=strings] summary span[data-state]").textContent(), await page.locator("[data-example=strings] section > p").textContent()], ["Läuft", "Parameter"]);
};
