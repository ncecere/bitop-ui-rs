export default async ({ page, check, setDir }) => {
  const table = (name) => page.getByRole("table", { name, exact: true });
  const wrap = (name) => table(name).locator("xpath=..");
  const frame = (name) => wrap(name).locator("xpath=..");

  // Semantics: a caption, column headers with scope, a row header, and aria-sort.
  const docs = table("Documents");
  check("caption", await docs.locator("caption").evaluate((c) => [c.textContent.trim(), c.className]), ["Documents", "sr-only"]);
  check("columns", await docs.getByRole("columnheader").evaluateAll((all) => all.map((th) => [th.textContent.trim(), th.scope, th.getAttribute("aria-sort")])), [["Name", "col", "ascending"], ["Size", "col", null], ["Status", "col", null], ["Actions", "col", null]]);
  check("blankHeaderHidden", await docs.getByRole("columnheader", { name: "Actions" }).locator("span").getAttribute("class"), "sr-only");
  check("rowHeaders", await docs.getByRole("rowheader").evaluateAll((all) => all.map((th) => [th.textContent.trim(), th.scope])), [["handbook.pdf", "row"], ["pricing.xlsx", "row"], ["roadmap.md", "row"], ["interview-notes.docx", "row"]]);
  check("cellByHeaders", await docs.getByRole("row", { name: /pricing\.xlsx/ }).getByRole("cell").first().textContent(), "84 kB");

  // Cell variants.
  const size = docs.getByRole("cell", { name: "1.2 MB" });
  check("numeric", await size.evaluate((td) => [getComputedStyle(td).textAlign, getComputedStyle(td).fontVariantNumeric, getComputedStyle(td).whiteSpace]), ["end", "tabular-nums", "nowrap"]);
  check("numericHeader", await docs.getByRole("columnheader", { name: "Size" }).evaluate((th) => getComputedStyle(th).textAlign), "end");
  const body = await page.evaluate(() => getComputedStyle(document.body).color);
  check("muted", await docs.locator("td[data-muted]").first().evaluate((td) => getComputedStyle(td).color) !== body);
  check("selected", await docs.locator("tbody tr").evaluateAll((all) => all.map((tr) => tr.hasAttribute("data-selected"))), [false, true, false, false]);
  const rowColours = await docs.locator("tbody tr").evaluateAll((all) => all.map((tr) => getComputedStyle(tr).backgroundColor));
  check("selectedTinted", rowColours[1] !== rowColours[0]);
  // The actions sit at the end of their cell.
  const cell = await docs.locator("tbody tr").first().locator("td").last().boundingBox();
  const open = await page.getByRole("button", { name: "Open handbook.pdf" }).boundingBox();
  check("actionsAtEnd", Math.round(cell.x + cell.width - (open.x + open.width)), 20);
  check("framed", await frame("Documents").evaluate((f) => [f.hasAttribute("data-framed"), getComputedStyle(f).boxShadow !== "none"]), [true, true]);

  // A table that fits is not a tab stop and is not a region.
  check("fitsIsNotFocusable", await wrap("Documents").evaluate((w) => [w.hasAttribute("tabindex"), w.getAttribute("role"), w.hasAttribute("data-overflowing")]), [false, null, false]);

  // Compact, striped, a visible caption, a width, and a hidden header label.
  const usage = table("Usage by model");
  check("density", [await usage.getAttribute("data-density"), await docs.getAttribute("data-density")], ["compact", "comfortable"]);
  const rowHeight = (t) => t.locator("tbody tr").first().evaluate((tr) => tr.getBoundingClientRect().height);
  check("compactIsShorter", (await rowHeight(usage)) < (await rowHeight(docs)));
  const stripes = await usage.locator("tbody tr").evaluateAll((all) => all.map((tr) => getComputedStyle(tr).backgroundColor));
  check("striped", [stripes[0] === stripes[2], stripes[1] === stripes[3], stripes[0] !== stripes[1]], [true, true, true]);
  check("captionShown", (await usage.locator("caption").boundingBox()).height > 10);
  const widths = await usage.getByRole("columnheader").evaluateAll((all) => all.map((th) => th.getBoundingClientRect().width));
  check("columnWidth", Math.abs(widths[0] / (widths[0] + widths[1] + widths[2]) - 0.5) < 0.01);
  check("hiddenLabel", await usage.getByRole("columnheader", { name: "Share" }).locator("span.sr-only").count(), 1);

  // A sticky header: the wrapper scrolls, is a focusable region named by the caption,
  // and the header stays at the top while the keyboard scrolls the body.
  const events = wrap("Events");
  await page.waitForFunction(() => document.querySelector("[data-label=Events]").hasAttribute("data-overflowing"));
  check("scrollRegion", await events.evaluate((w) => [w.tabIndex, w.getAttribute("role"), w.getAttribute("aria-label"), w.style.maxHeight]), [0, "region", "Events", "12rem"]);
  check("regionRole", await page.getByRole("region", { name: "Events" }).count(), 1);
  await events.focus();
  check("focusRing", await events.evaluate((w) => getComputedStyle(w).outlineStyle !== "none"));
  for (let i = 0; i < 6; i++) await page.keyboard.press("ArrowDown");
  await page.waitForFunction(() => document.querySelector("[data-label=Events]").scrollTop > 50);
  const head = await table("Events").getByRole("columnheader", { name: "Event" }).boundingBox();
  const box = await events.boundingBox();
  check("headerStays", Math.abs(head.y - box.y) <= 1);
  check("headerCovers", await table("Events").getByRole("columnheader", { name: "Event" }).evaluate((th) => [getComputedStyle(th).position, getComputedStyle(th).backgroundColor !== "rgba(0, 0, 0, 0)"]), ["sticky", true]);

  // Wider than its container: it scrolls sideways inside the wrapper, not the page.
  const wide = wrap("Deployments");
  await page.waitForFunction(() => document.querySelector("[data-label=Deployments]").hasAttribute("data-overflowing"));
  check("wideScrolls", await wide.evaluate((w) => [w.scrollWidth > w.clientWidth, document.documentElement.scrollWidth <= document.documentElement.clientWidth, w.tabIndex, w.getAttribute("role")]), [true, true, 0, "region"]);
  // The pinned column stays at the end edge while the others scroll under it.
  const pinned = page.getByRole("button", { name: "Roll back deployment 1" });
  const gap = async () => { const w = await wide.boundingBox(); const b = await pinned.boundingBox(); return Math.round(w.x + w.width - (b.x + b.width)); };
  const before = await gap();
  const commit = table("Deployments").getByRole("cell", { name: "671a1f1" });
  const commitX = (await commit.boundingBox()).x;
  await wide.focus();
  for (let i = 0; i < 5; i++) await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => document.querySelector("[data-label=Deployments]").scrollLeft > 50);
  check("pinnedStays", [await gap(), (await commit.boundingBox()).x < commitX], [before, true]);
  check("pinnedShadow", await table("Deployments").locator("td[data-sticky-end]").first().evaluate((td) => [getComputedStyle(td).position, getComputedStyle(td).boxShadow !== "none"]), ["sticky", true]);
  // The frame shows a shadow on the side that hides columns; the pinned column casts the end one.
  const edges = (name) => frame(name).evaluate((f) => [...f.querySelectorAll(":scope > span")].map((s) => getComputedStyle(s).display));
  check("edgesPinned", [await frame("Deployments").evaluate((f) => [f.hasAttribute("data-overflow-start"), f.hasAttribute("data-overflow-end")]), await edges("Deployments")], [[true, true], ["block", "none"]]);
  const regions = wrap("Regions");
  await page.waitForFunction(() => document.querySelector("[data-label=Regions]").hasAttribute("data-overflowing"));
  check("edgeAtStart", await edges("Regions"), ["none", "block"]);
  await regions.evaluate((w) => w.scrollTo({ left: w.scrollWidth, behavior: "instant" }));
  await page.waitForFunction(() => !document.querySelector("[data-label=Regions]").parentElement.hasAttribute("data-overflow-end"));
  check("edgeAtEnd", await edges("Regions"), ["block", "none"]);
  // Tab reaches the scroll regions, in order, and skips the tables that fit.
  await page.getByRole("button", { name: "Open interview-notes.docx" }).focus();
  await page.keyboard.press("Tab");
  check("tabToRegion", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Events");

  // Empty: one row across every column, centred on the visible width.
  const invoices = table("Invoices");
  check("empty", [await invoices.locator("tbody tr").count(), await invoices.locator("tbody td").getAttribute("colspan"), await invoices.getByText("No invoices yet").isVisible()], [1, "3", true]);
  const title = await invoices.getByText("No invoices yet").boundingBox();
  const inv = await wrap("Invoices").boundingBox();
  check("emptyCentred", Math.abs(title.x + title.width / 2 - (inv.x + inv.width / 2)) <= 1);

  // Right to left: numbers align to the left, and the pinned column is on the left.
  await setDir("rtl");
  const rtlSize = await size.boundingBox();
  const rtlName = await docs.getByRole("rowheader", { name: "handbook.pdf" }).boundingBox();
  check("rtlColumns", rtlSize.x < rtlName.x);
  await wide.evaluate((w) => w.scrollTo({ left: -60, behavior: "instant" }));
  const w = await wide.boundingBox();
  const b = await pinned.boundingBox();
  check("rtlPinned", b.x - w.x < 30);
  await page.waitForFunction(() => document.querySelector("[data-label=Deployments]").parentElement.hasAttribute("data-overflow-start"));
  check("rtlEdges", await frame("Deployments").evaluate((f) => [f.hasAttribute("data-overflow-start"), f.hasAttribute("data-overflow-end")]), [true, true]);
  await setDir("ltr");

  // Stacked: on a narrow screen each row is a block and each cell names its column.
  const settings = table("Notification settings");
  check("stackLabels", await settings.locator("tbody tr").first().locator("th, td").evaluateAll((all) => all.map((c) => c.dataset.label ?? null)), [null, "Email", "Push", "Remarks"]);
  check("wideIsATable", await settings.locator("tbody tr").first().evaluate((tr) => getComputedStyle(tr).display), "table-row");
  await page.setViewportSize({ width: 500, height: 900 });
  await page.waitForFunction(() => getComputedStyle(document.querySelector("table[data-stack] tbody tr")).display === "grid");
  const first = settings.locator("tbody tr").first();
  const cells = await first.locator("th, td").evaluateAll((all) => all.map((c) => { const b = c.getBoundingClientRect(); return [Math.round(b.x), Math.round(b.y), getComputedStyle(c, "::before").content]; }));
  check("stacked", [cells[1][1] > cells[0][1], cells[2][1] === cells[1][1], cells[2][0] > cells[1][0], cells[1][2], cells[0][2]], [true, true, true, '"Email"', "none"]);
  check("stackedHeadersStillAnnounced", await settings.getByRole("columnheader").count(), 4);
  check("stackedFits", await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth));
  // A row added later is named too.
  await settings.evaluate((t) => { const row = t.tBodies[0].rows[1].cloneNode(true); for (const c of row.cells) delete c.dataset.label; t.tBodies[0].append(row); });
  await page.waitForFunction(() => document.querySelector("table[data-stack]").tBodies[0].rows[2].cells[1].dataset.label === "Email");
  check("addedRowLabelled", await settings.locator("tbody tr").nth(2).locator("td").nth(1).getAttribute("data-label"), "Push");
  await page.setViewportSize({ width: 1000, height: 900 });
  // The region comes and goes with the overflow: squeeze a table that fits, then let it go.
  await frame("Documents").evaluate((f) => (f.style.inlineSize = "12rem"));
  await page.waitForFunction(() => document.querySelector("[data-label=Documents]").hasAttribute("data-overflowing"));
  check("squeezedIsARegion", await wrap("Documents").evaluate((w) => [w.tabIndex, w.getAttribute("role"), w.getAttribute("aria-label")]), [0, "region", "Documents"]);
  await frame("Documents").evaluate((f) => (f.style.inlineSize = ""));
  await page.waitForFunction(() => !document.querySelector("[data-label=Documents]").hasAttribute("data-overflowing"));
  check("releasedIsNot", await wrap("Documents").evaluate((w) => [w.hasAttribute("tabindex"), w.getAttribute("role")]), [false, null]);
};
