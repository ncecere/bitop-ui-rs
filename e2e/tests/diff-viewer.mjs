export default async ({ page, check, focused, setDir }) => {
  const viewer = (id) => page.locator(`#${id}`);
  const shown = (id) => viewer(id).locator("table").evaluateAll((all) => all.filter((t) => t.checkVisibility()).map((t) => t.dataset.table));
  const rows = (id, layout) => viewer(id).locator(`table[data-table=${layout}] tbody tr`).evaluateAll((all) => all.filter((r) => !r.hidden).map((r) => [...r.cells].map((c) => c.textContent.replace(/\s+/g, " ").trim())));

  // Text, unified: the summary says how much changed, and one table is shown.
  const prompt = viewer("prompt");
  check("summary", await prompt.locator("p").first().textContent(), "3 lines added, 2 removed");
  check("oneTable", [await shown("prompt"), await prompt.getAttribute("data-mode")], [["unified"], "unified"]);
  check("named", [await prompt.getByRole("region", { name: "System prompt" }).count(), await prompt.getByRole("table", { name: "System prompt" }).count()], [1, 1]);
  // Line numbers of both sides, a sign, and a word for assistive technology: never colour alone.
  // (The sign is hidden from assistive technology, so the text here runs it into the word.)
  check("unifiedRows", await rows("prompt", "unified"), [
    ["1", "", "−Removed: You are a support agent."],
    ["", "1", "+Added: You are a support agent for Acme."],
    ["2", "2", "Answer in one paragraph."],
    ["3", "3", "Be polite."],
    ["4", "4", "Cite the handbook."],
    ["Show 5 unchanged lines"],
    ["10", "10", "Offer a follow-up."],
    ["11", "11", "Sign off with your name."],
    ["12", "12", "Log the conversation."],
    ["13", "", "−Removed: Tag the ticket."],
    ["", "13", "+Added: Tag the ticket with a topic."],
    ["", "14", "+Added: Set the priority."],
    ["14", "15", "Close the ticket."],
  ]);
  check("signsHidden", await prompt.locator("table[data-table=unified] tr[data-kind=add] span[aria-hidden=true]").first().textContent(), "+");
  const tints = await prompt.locator("table[data-table=unified] tbody tr").evaluateAll((all) => ["remove", "add", "equal"].map((k) => getComputedStyle(all.find((r) => r.dataset.kind === k).cells[2]).backgroundColor));
  check("tints", new Set(tints).size, 3);
  check("hiddenHeaders", await prompt.locator("table[data-table=unified]").getByRole("columnheader").allTextContents(), ["Before line", "After line", "Content"]);
  check("action", await prompt.getByRole("button", { name: "Copy new" }).isVisible());

  // The fold button shows the unchanged lines, in both layouts, and focus goes to the region.
  await prompt.getByRole("button", { name: "Show 5 unchanged lines" }).focus();
  await page.keyboard.press("Enter");
  check("unfolded", [await rows("prompt", "unified").then((r) => r.slice(5, 10).map((c) => c[2])), await prompt.getByRole("button", { name: /unchanged/ }).count(), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], [
    ["Never share internal links.", "Ask before closing a ticket.", "Escalate billing questions.", "Use the customer's name.", "Keep answers short."], 0, "System prompt",
  ]);

  // The toggle switches the layout; it is one tab stop with arrow keys, and one item is always pressed.
  const modes = prompt.getByRole("group", { name: "Diff layout" });
  const pressed = () => modes.getByRole("button").evaluateAll((all) => all.map((b) => b.getAttribute("aria-pressed")));
  check("toggleStart", await pressed(), ["true", "false"]);
  await modes.getByRole("button", { name: "Unified" }).focus();
  await page.keyboard.press("ArrowRight");
  check("arrowMoves", await focused(), "Split");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#prompt").dataset.mode === "split" && document.querySelector("#prompt [data-value=split]").getAttribute("aria-pressed") === "true");
  check("split", [await shown("prompt"), await pressed()], [["split"], ["false", "true"]]);
  const split = await rows("prompt", "split");
  check("splitRows", [split.length, split[0], split[13], split[14]], [15,
    ["1", "−Removed: You are a support agent.", "1", "+Added: You are a support agent for Acme."],
    ["", "", "14", "+Added: Set the priority."],
    ["14", "Close the ticket.", "15", "Close the ticket."],
  ]);
  check("splitHeaders", await prompt.locator("table[data-table=split]").getByRole("columnheader").allTextContents().then((all) => all.map((t) => t.trim())), ["Before line", "Before", "After line", "After"]);
  // The two sides are side by side and as wide as each other.
  const cells = await prompt.locator("table[data-table=split] tbody tr").first().locator("td").evaluateAll((all) => all.map((c) => { const b = c.getBoundingClientRect(); return [Math.round(b.x), Math.round(b.width)]; }));
  check("splitColumns", [cells[1][0] < cells[3][0], Math.abs(cells[1][1] - cells[3][1]) <= 1, cells[0][1], cells[2][1]], [true, true, 48, 48]);
  check("emptySide", await prompt.locator("table[data-table=split] tbody tr").nth(13).locator("td").evaluateAll((all) => all.map((c) => c.dataset.kind)), ["none", "none", "add", "add"]);
  // Pressing the pressed item again keeps it pressed.
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#prompt [data-value=split]").getAttribute("aria-pressed") === "true");
  check("staysPressed", [await pressed(), await shown("prompt")], [["false", "true"], ["split"]]);
  await modes.getByRole("button", { name: "Unified" }).click();
  await page.waitForFunction(() => document.querySelector("#prompt [data-value=unified]").getAttribute("aria-pressed") === "true");
  check("backToUnified", await shown("prompt"), ["unified"]);

  // JSON, split: one row for each changed key, with its path and both values.
  const settings = viewer("settings");
  check("jsonSummary", await settings.locator("p").first().textContent(), "5 changes: 1 added, 2 removed, 2 changed");
  check("jsonSplit", [await shown("settings"), await rows("settings", "split")], [["split"], [
    ["~Changed: model", '"small"', '"large"'],
    ["~Changed: limits.maxPages", "10", "25"],
    ["−Removed: tags[1]", '"beta"', "—Not set"],
    ["−Removed: owner", '"ada"', "—Not set"],
    ["+Added: notify", "—Not set", '{ "email": true }'],
  ]]);
  check("jsonRowHeaders", [await settings.locator("table[data-table=split]").getByRole("rowheader").count(), await settings.locator("table[data-table=split]").getByRole("columnheader").allTextContents()], [5, ["Field", "Before", "After"]]);
  check("jsonIndented", await settings.locator("table[data-table=split] tbody tr").last().locator("td").last().locator("span").textContent(), '{\n  "email": true\n}');
  await settings.getByRole("button", { name: "Unified" }).click();
  await page.waitForFunction(() => document.querySelector("#settings").dataset.mode === "unified");
  check("jsonUnified", await rows("settings", "unified").then((r) => r.slice(0, 5)), [
    ["−Changed, before: model", '"small"'],
    ["+Changed, after: model", '"large"'],
    ["−Changed, before: limits.maxPages", "10"],
    ["+Changed, after: limits.maxPages", "25"],
    ["−Removed: tags[1]", '"beta"'],
  ]);

  // Long lines do not wrap: the area scrolls both ways, from the keyboard, and stops at its height.
  const config = viewer("config");
  const region = config.getByRole("region", { name: "Configuration" });
  check("noToggle", [await config.getByRole("group").count(), await config.evaluate((c) => c.hasAttribute("data-wrap"))], [0, false]);
  check("scrolls", await region.evaluate((r) => [r.scrollWidth > r.clientWidth, r.scrollHeight > r.clientHeight, Math.round(r.getBoundingClientRect().height), r.tabIndex]), [true, true, 128, 0]);
  await region.focus();
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.waitForFunction(() => document.querySelector("#config [role=region]").scrollTop > 0);
  check("pageDoesNotWiden", await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth));
  // No context: every unchanged run is folded.
  check("noContext", await config.getByRole("button", { name: /unchanged/ }).allTextContents().then((all) => all.map((t) => t.trim())), ["Show 2 unchanged lines"]);
  // With wrapping on, long lines wrap and nothing scrolls sideways.
  check("wraps", await prompt.getByRole("region").evaluate((r) => r.scrollWidth <= r.clientWidth));

  // No changes: the summary says so, and there is no table and no toggle.
  check("same", [await viewer("same").locator("p").first().textContent(), await viewer("same").locator("table").count(), await viewer("same").getByRole("group").count()], ["No changes", 0, 0]);
  // A value that was not there before is one change at the root.
  check("addedRoot", [await viewer("added").locator("p").first().textContent(), await rows("added", "unified")], ["1 change: 1 added, 0 removed, 0 changed", [["+Added: (root)", '{ "enabled": true }']]]);

  // Strings.
  const german = viewer("german");
  check("strings", [await german.locator("p").first().textContent(), await german.getByRole("group", { name: "Ansicht" }).getByRole("button").allTextContents().then((all) => all.map((t) => t.trim())), await rows("german", "unified").then((r) => r.slice(1).map((c) => c[2])), await german.locator("table[data-table=unified] th").allTextContents()], [
    "1 Zeile hinzugefügt, 1 entfernt", ["Vereint", "Geteilt"], ["−Entfernt: zwei", "+Hinzugefügt: drei"], ["Zeile Vorher", "Zeile Nachher", "Inhalt"],
  ]);

  // Right to left: the line numbers are on the right of the text.
  await setDir("rtl");
  const numberBox = await prompt.locator("table[data-table=unified] tbody td").first().boundingBox();
  const textBox = await prompt.locator("table[data-table=unified] tbody td").nth(2).boundingBox();
  check("rtl", numberBox.x > textBox.x);
};
