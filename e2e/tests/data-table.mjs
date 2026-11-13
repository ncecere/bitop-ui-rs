export default async ({ page, base, check, focused, settled, setDir }) => {
  const root = page.locator("#invoices");
  const table = page.getByRole("table", { name: "Invoices", exact: true });
  const numbers = () => table.getByRole("rowheader").allTextContents().then((all) => all.map((t) => t.trim()));
  const header = (name) => table.getByRole("columnheader", { name, exact: true });
  await page.evaluate(() => localStorage.removeItem("gallery-invoices-columns"));

  // Sorting: headers are links; the server sorts and marks the column with aria-sort.
  check("unsorted", [await numbers(), await header("Invoice").getAttribute("aria-sort"), await header("Status").getAttribute("aria-sort")], [["INV-001", "INV-002", "INV-003", "INV-004", "INV-005"], "none", null]);
  check("sortLinks", await table.locator("thead a").evaluateAll((all) => all.map((a) => [a.textContent.trim(), a.getAttribute("href"), a.querySelector("[data-icon]").dataset.icon])), [
    ["Invoice", "/c/data-table?sort=number&dir=asc", "chevrons-up-down"],
    ["Customer", "/c/data-table?sort=customer&dir=asc", "chevrons-up-down"],
    ["Amount", "/c/data-table?sort=amount&dir=asc", "chevrons-up-down"],
  ]);
  await header("Amount").getByRole("link").focus();
  check("sortFocus", await focused(), "Amount");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/data-table?sort=amount&dir=asc");
  check("ascending", [await numbers(), await header("Amount").getAttribute("aria-sort"), await header("Amount").locator("[data-icon]").getAttribute("data-icon"), await header("Invoice").getAttribute("aria-sort")], [["INV-008", "INV-012", "INV-004", "INV-007", "INV-010"], "ascending", "arrow-up", "none"]);
  await header("Amount").getByRole("link").click();
  await page.waitForURL("**/c/data-table?sort=amount&dir=desc");
  check("descending", [await numbers(), await header("Amount").getAttribute("aria-sort"), await header("Amount").locator("[data-icon]").getAttribute("data-icon")], [["INV-006", "INV-009", "INV-003", "INV-001", "INV-005"], "descending", "arrow-down"]);
  // Paging keeps the sort; the summary follows.
  const pages = page.getByRole("navigation", { name: "Invoices pages" });
  await pages.getByRole("link", { name: "Page 3" }).click();
  await page.waitForURL("**/c/data-table?sort=amount&dir=desc&page=3");
  check("paged", [await numbers(), await root.getByText("Rows 11–12 of 12").count(), await pages.locator("[aria-current=page]").textContent()], [["INV-012", "INV-008"], 1, "3"]);
  // The third click clears the sort.
  await header("Amount").getByRole("link").click();
  await page.waitForURL(`${base}/c/data-table`);
  check("cleared", [await numbers(), await header("Amount").getAttribute("aria-sort")], [["INV-001", "INV-002", "INV-003", "INV-004", "INV-005"], "none"]);
  // A numeric header puts the arrow before the text, at the end edge.
  const amountLink = header("Amount").getByRole("link");
  check("numericHeader", await amountLink.evaluate((a) => { const i = a.querySelector("[data-icon]").getBoundingClientRect(); const s = a.querySelector("span").getBoundingClientRect(); return i.x < s.x; }));

  // Selection: checkboxes named after their rows, with a form name and value.
  const boxes = table.locator("input[data-row-select]");
  const all = table.getByRole("checkbox", { name: "Select all rows" });
  const bulk = root.getByRole("group", { name: "Bulk actions" });
  check("rowBoxes", await boxes.evaluateAll((a) => a.slice(0, 2).map((b) => [b.getAttribute("aria-label"), b.name, b.value])), [["Select INV-001", "selected", "i1"], ["Select INV-002", "selected", "i2"]]);
  check("noneSelected", [await all.isChecked(), await all.evaluate((b) => b.indeterminate), await bulk.isVisible(), await root.getByText("0 of 12 selected").count()], [false, false, false, 1]);
  await table.getByRole("checkbox", { name: "Select INV-002" }).focus();
  await page.keyboard.press("Space");
  check("oneSelected", [await all.isChecked(), await all.evaluate((b) => b.indeterminate), await bulk.isVisible(), await bulk.locator("[data-selected-count]").textContent().then((t) => t.trim()), await root.getByText("1 of 12 selected").count()], [false, true, true, "1 selected", 1]);
  check("rowTinted", await table.locator("tbody tr").evaluateAll((rows) => rows.map((r) => r.hasAttribute("data-selected"))), [false, true, false, false, false]);
  check("mixedIsAnnounced", await all.evaluate((b) => b.matches(":indeterminate")));
  // The header checkbox selects every row, then none.
  await all.focus();
  await page.keyboard.press("Space");
  check("allSelected", [await all.isChecked(), await all.evaluate((b) => b.indeterminate), await boxes.evaluateAll((a) => a.every((b) => b.checked)), await bulk.locator("[data-selected-count]").textContent().then((t) => t.trim())], [true, false, true, "5 selected"]);
  await page.keyboard.press("Space");
  check("noneAgain", [await all.isChecked(), await boxes.evaluateAll((a) => a.some((b) => b.checked)), await bulk.isVisible()], [false, false, false]);
  // Clear selection empties it and moves focus to the header checkbox, since the bar goes.
  await table.getByRole("checkbox", { name: "Select INV-001" }).check();
  await table.getByRole("checkbox", { name: "Select INV-003" }).check();
  await bulk.getByRole("button", { name: "Clear selection" }).click();
  check("clearSelection", [await boxes.evaluateAll((a) => a.some((b) => b.checked)), await bulk.isVisible(), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], [false, false, "Select all rows"]);
  // The selection submits with the form, with the action chosen in the bulk bar.
  await table.getByRole("checkbox", { name: "Select INV-002" }).check();
  await table.getByRole("checkbox", { name: "Select INV-004" }).check();
  await bulk.getByRole("button", { name: "Archive" }).click();
  await page.waitForURL("**/c/data-table?action=Archived&selected=i2&selected=i4");
  check("submitted", await page.locator("[data-result]").textContent(), "Archived: INV-002, INV-004");

  // Columns menu: checkbox items show and hide columns; the choice is remembered.
  const trigger = root.getByRole("button", { name: "Columns" });
  const menu = page.locator("#invoices-columns");
  const item = (name) => menu.getByRole("menuitemcheckbox", { name });
  const shown = () => table.getByRole("columnheader").evaluateAll((a) => a.filter((th) => !th.hidden).map((th) => th.textContent.trim()));
  check("startsHidden", [await shown(), await table.locator("tbody tr").first().locator("td, th").evaluateAll((a) => a.map((c) => c.hidden))], [["", "Invoice", "Customer", "Amount", "Status", "Actions"], [false, false, false, false, false, true, false]]);
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitemcheckbox");
  await settled(menu);
  check("menuItems", await menu.getByRole("menuitemcheckbox").evaluateAll((a) => a.map((i) => [i.textContent.trim(), i.getAttribute("aria-checked")])), [["Customer", "true"], ["Amount", "true"], ["Status", "true"], ["Due", "false"]]);
  check("menuGroup", await menu.getByRole("group", { name: "Show columns" }).count(), 1);
  await page.keyboard.press("Space");
  check("hidden", [await item("Customer").getAttribute("aria-checked"), await shown(), await menu.evaluate((m) => m.matches(":popover-open"))], ["false", ["", "Invoice", "Amount", "Status", "Actions"], true]);
  check("cellsHidden", await table.locator("tbody tr").first().locator("td, th").evaluateAll((a) => a.map((c) => c.hidden)), [false, false, true, false, false, true, false]);
  await page.keyboard.press("End");
  await page.keyboard.press("Enter");
  check("shownAgain", [await item("Due").getAttribute("aria-checked"), await shown()], ["true", ["", "Invoice", "Amount", "Status", "Due", "Actions"]]);
  check("stored", await page.evaluate(() => JSON.parse(localStorage.getItem("gallery-invoices-columns"))), ["customer"]);
  await page.keyboard.press("Escape");
  check("focusReturns", await focused(), "Columns");
  await page.reload();
  check("remembered", await shown(), ["", "Invoice", "Amount", "Status", "Due", "Actions"]);
  check("menuFollowsStorage", await menu.locator("[role=menuitemcheckbox]").evaluateAll((a) => a.map((i) => i.getAttribute("aria-checked"))), ["false", "true", "true", "true"]);
  await page.evaluate(() => localStorage.removeItem("gallery-invoices-columns"));
  await page.reload();

  // The filter is a search form of its own, with a search icon as upstream's; Enter
  // submits it; the result count is announced.
  check("searchIcon", await page.getByRole("searchbox", { name: "Filter invoices" }).evaluate((i) => [i.parentElement.querySelector("[data-icon]")?.dataset.icon, Math.round(i.getBoundingClientRect().height)]), ["search", 32]);
  await page.getByRole("searchbox", { name: "Filter invoices" }).fill("in");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/data-table?q=in");
  check("filtered", [await numbers(), await root.getByRole("status").textContent(), await root.getByText("Rows 1–5 of 12").count()], [["INV-001", "INV-002", "INV-003", "INV-004", "INV-005"], "12 rows match the filter", 1]);
  await page.goto(`${base}/c/data-table?q=init`);
  check("filteredToOne", [await numbers(), await root.getByRole("status").textContent(), await pages.getByRole("link").count()], [["INV-003"], "1 row matches the filter", 0]);
  await page.goto(`${base}/c/data-table?q=zzz`);
  check("noResults", [await table.locator("tbody tr").count(), await table.getByText("No invoices match the filter.").isVisible()], [1, true]);
  // The row that spans the table spans the columns that are shown.
  check("emptySpansShown", await table.locator("tbody td").evaluate((td) => [td.colSpan, td.getBoundingClientRect().width > 300]), [6, true]);
  await page.goto(`${base}/c/data-table`);

  // A row opens its record: the link in the row header, or a click elsewhere on the row.
  const row = table.getByRole("row", { name: /INV-003/ });
  await row.getByText("pay@initech.example").click();
  check("rowClickOpens", await page.evaluate(() => location.hash), "#INV-003");
  await page.evaluate(() => history.replaceState(null, "", location.pathname));
  await row.getByRole("button", { name: "Refund INV-003" }).click();
  await row.getByRole("checkbox").click();
  check("controlsDoNotOpen", [await page.evaluate(() => location.hash), await row.getByRole("checkbox").isChecked()], ["", true]);
  await row.getByRole("link", { name: "INV-003" }).focus();
  await page.keyboard.press("Enter");
  check("rowLinkOpens", await page.evaluate(() => location.hash), "#INV-003");
  // Two lines in a cell, separated by a space for assistive technology.
  check("cellText", await row.getByRole("cell").nth(1).textContent().then((t) => t.trim()), "Initech pay@initech.example");

  // A row selected by the server; a disabled row is left out of select-all.
  const members = page.locator("#members");
  const membersAll = members.getByRole("checkbox", { name: "Select all rows" });
  check("serverSelected", [await membersAll.evaluate((b) => b.indeterminate), await members.getByRole("group", { name: "Bulk actions" }).isVisible(), await members.getByText("1 of 40 selected").count(), await members.locator("tbody tr").nth(1).getAttribute("data-selected")], [true, true, 1, ""]);
  await membersAll.check();
  check("disabledLeftOut", [await members.locator("input[data-row-select]").evaluateAll((a) => a.map((b) => b.checked)), await membersAll.isChecked(), await members.locator("[data-selected-count]").textContent().then((t) => t.trim())], [[true, true, false], true, "2 selected"]);
  const cursor = page.getByRole("navigation", { name: "Members pages" });
  check("cursor", [await cursor.getByRole("button", { name: "Go to previous page" }).isDisabled(), await cursor.getByRole("link", { name: "Go to next page" }).getAttribute("href"), await members.getByText("Showing 1–3").count()], [true, "/c/data-table?after=m3", 1]);

  // Loading: the table is busy, the rows are placeholders, and a status says so.
  const loading = page.locator("#loading");
  check("loading", [await loading.getByRole("table").getAttribute("aria-busy"), await loading.locator("tbody tr[aria-hidden=true]").count(), await loading.locator("[data-shape=text]").count(), await loading.getByRole("status").textContent(), await loading.getAttribute("data-loading")], ["true", 3, 9, "Loading rows…", ""]);
  // Refreshing: the rows it has stay, dimmed, with the error above and Retry as a link.
  const stale = page.locator("#stale");
  await page.waitForFunction(() => getComputedStyle(document.querySelector("#stale tbody")).opacity === "0.6");
  const alertBox = await stale.getByRole("alert").boundingBox();
  const tableBox = await stale.getByRole("table").boundingBox();
  check("stale", [await stale.getAttribute("data-stale"), await stale.getByRole("rowheader").count(), alertBox.y < tableBox.y, await stale.getByRole("alert").locator("p").textContent(), await stale.getByRole("link", { name: "Retry" }).getAttribute("href")], ["", 2, true, "Couldn't load rows", "/c/data-table"]);
  const more = stale.getByRole("link", { name: "Load more" });
  const moreBox = await more.boundingBox();
  check("loadMore", [await more.getAttribute("href"), Math.abs(moreBox.x + moreBox.width / 2 - (tableBox.x + tableBox.width / 2)) <= 1], ["/c/data-table?more=1", true]);
  // Empty: a message across the table; no toolbar, bar or footer takes space.
  const empty = page.locator("#empty");
  check("empty", [await empty.getByText("No rows.").isVisible(), await empty.locator("> div").evaluateAll((a) => a.filter((d) => getComputedStyle(d).display !== "none").length)], [true, 1]);
  check("hiddenLabel", await members.getByRole("columnheader", { name: "Last seen" }).locator(".sr-only").count(), 1);

  // Without a script the bulk bar still appears and the selection still submits.
  const plain = await page.context().browser().newContext({ javaScriptEnabled: false });
  const still = await plain.newPage();
  await still.goto(`${base}/c/data-table`);
  const stillRoot = still.locator("#invoices");
  check("noScriptBarHidden", await stillRoot.getByRole("group", { name: "Bulk actions" }).isVisible(), false);
  await stillRoot.getByRole("checkbox", { name: "Select INV-005" }).check();
  check("noScriptBarShown", await stillRoot.getByRole("group", { name: "Bulk actions" }).isVisible());
  await stillRoot.getByRole("button", { name: "Delete" }).click();
  await still.waitForURL("**/c/data-table?action=Deleted&selected=i5");
  check("noScriptSubmits", await still.locator("[data-result]").textContent(), "Deleted: INV-005");
  await stillRoot.getByRole("columnheader", { name: "Customer" }).getByRole("link").click();
  await still.waitForURL("**/c/data-table?sort=customer&dir=asc");
  check("noScriptSorts", await stillRoot.getByRole("rowheader").first().textContent().then((t) => t.trim()), "INV-001");
  await plain.close();

  // Right to left: the checkbox column is on the right, the actions on the left.
  await setDir("rtl");
  const first = table.locator("tbody tr").first();
  const box = await first.getByRole("checkbox").boundingBox();
  const name = await first.getByRole("rowheader").boundingBox();
  const refund = await first.getByRole("button").boundingBox();
  check("rtl", [box.x > name.x, refund.x < name.x], [true, true]);
};
