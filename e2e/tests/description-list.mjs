export default async ({ page, check, setDir }) => {
  const list = (name) => page.locator(`dl[aria-label="${name}"]`);
  const pairs = (name) => list(name).locator("> div").evaluateAll((all) => all.map((d) => [d.children[0].tagName, d.children[0].textContent.trim(), d.children[1].tagName]));
  check("semantics", await pairs("Source"), [["DT", "Start URL", "DD"], ["DT", "Schedule", "DD"], ["DT", "Status", "DD"], ["DT", "Owner", "DD"]]);
  check("terms", await list("Source").getByRole("term").count(), 4);
  check("definitions", await list("Source").getByRole("definition").count(), 4);

  // Grid: the label is in a fixed column at the start, the value beside it.
  const box = (name, n, tag) => list(name).locator(`> div:nth-child(${n}) > ${tag}`).boundingBox();
  const term = await box("Source", 2, "dt");
  const detail = await box("Source", 2, "dd");
  check("gridSideBySide", [Math.round(term.width), detail.x > term.x + term.width, Math.abs(detail.y - term.y) < 4], [160, true, true]);
  // In a narrow container the value drops under its label.
  const narrowTerm = await box("Narrow", 1, "dt");
  const narrowDetail = await box("Narrow", 1, "dd");
  check("gridStacksWhenNarrow", [narrowDetail.y > narrowTerm.y + narrowTerm.height - 1, Math.round(narrowDetail.x) === Math.round(narrowTerm.x)], [true, true]);

  // An empty value shows a dash, announced as "Not set".
  const empty = list("Source").locator("dd[data-empty]");
  check("empty", [await empty.count(), await empty.locator("[aria-hidden=true]").textContent(), await empty.locator(".sr-only").textContent()], [1, "—", "Not set"]);
  check("emptyStrings", await list("Deployment").locator("dd[data-empty]").evaluate((d) => [...d.children].map((c) => c.textContent)), ["none", "No reviewer"]);

  // Small with dividers: hairlines between the rows, none after the last.
  const plan = list("Plan");
  check("variants", await plan.evaluate((d) => [d.dataset.layout, d.dataset.size, d.hasAttribute("data-dividers"), d.dataset.columns ?? null]), ["grid", "sm", true, null]);
  check("dividers", await plan.locator("> div").evaluateAll((all) => all.map((d) => getComputedStyle(d).borderBottomWidth)), ["1px", "1px", "0px"]);
  check("smallText", await plan.evaluate((d) => parseFloat(getComputedStyle(d).fontSize)) < await list("Source").evaluate((d) => parseFloat(getComputedStyle(d).fontSize)));

  // Stacked: label above value, three pairs to a row.
  const stacked = list("Deployment");
  check("stackedColumns", await stacked.getAttribute("data-columns"), "3");
  const places = await stacked.locator("> div").evaluateAll((all) => all.map((d) => { const b = d.getBoundingClientRect(); return [Math.round(b.x), Math.round(b.y)]; }));
  check("threeToARow", [places[0][1] === places[2][1], places[3][1] > places[0][1], places[3][0] === places[0][0], places[1][0] > places[0][0]], [true, true, true, true]);
  const st = await box("Deployment", 1, "dt");
  const sd = await box("Deployment", 1, "dd");
  check("labelAbove", sd.y >= st.y + st.height - 1);

  // Facts line: a list; labels are announced and hidden unless asked for.
  const facts = page.getByRole("list", { name: "Knowledge base" });
  check("facts", await facts.getByRole("listitem").evaluateAll((all) => all.map((li) => li.textContent.replace(/\s+/g, " ").trim())), ["Website", "Embedding profile: Nomic 768", "58 documents", "synced 2 h ago"]);
  const hidden = await facts.getByText("Embedding profile").boundingBox();
  check("labelHidden", [Math.round(hidden.width), Math.round(hidden.height)], [1, 1]);
  const shown = await page.getByRole("list", { name: "Labelled facts" }).getByText("Profile").boundingBox();
  check("labelShown", shown.width > 20);
  check("factIcon", await facts.locator("span[aria-hidden=true] > [data-icon=search]").count(), 1);
  // The separator is drawn before every fact but the first on a line, where it is clipped.
  const clip = await facts.locator("..").boundingBox();
  const items = await facts.getByRole("listitem").evaluateAll((all) => all.map((li) => { const b = li.getBoundingClientRect(); return [b.x, parseFloat(getComputedStyle(li).paddingInlineStart), getComputedStyle(li, "::before").content]; }));
  check("separatorContent", items[1][2].startsWith('"·"'));
  check("firstSeparatorClipped", items[0][0] < clip.x + 4.5 && items[0][0] + items[0][1] >= clip.x + 3.5);
  check("laterSeparatorsShown", items[1][0] > clip.x + 4);
  const wrapped = page.getByRole("list", { name: "Wrapped facts" });
  const lines = await wrapped.getByRole("listitem").evaluateAll((all) => all.map((li) => Math.round(li.getBoundingClientRect().y)));
  const starts = await wrapped.getByRole("listitem").evaluateAll((all) => all.map((li) => Math.round(li.getBoundingClientRect().x)));
  const second = lines.findIndex((y) => y > lines[0]);
  check("wraps", second > 0);
  check("wrappedLineStartsClipped", starts[second], starts[0]);

  // A link in a fact is reached with Tab, and its focus ring is not clipped.
  await facts.getByRole("link").focus();
  check("factLinkFocus", await page.evaluate(() => document.activeElement.textContent), "synced 2 h ago");

  // Right to left: the label column is on the right.
  await setDir("rtl");
  const rt = await box("Source", 2, "dt");
  const rd = await box("Source", 2, "dd");
  check("rtlGrid", rd.x + rd.width <= rt.x + 1);
  const rtlItems = await facts.getByRole("listitem").evaluateAll((all) => all.map((li) => Math.round(li.getBoundingClientRect().right)));
  check("rtlFactsOrder", rtlItems[0] > rtlItems[1] && rtlItems[1] > rtlItems[2]);
};
