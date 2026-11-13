export default async ({ page, check, axe, focused, settled, setDir }) => {
  const trail = (name) => page.getByRole("navigation", { name, exact: true });
  const cut = (locator) => locator.evaluate((el) => el.scrollWidth > el.clientWidth);
  const untilOpen = async (p, want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await p.elementHandle(), want]);

  // A trail: a landmark with an ordered list, links, then the current page.
  const plain = trail("Breadcrumb");
  check("list", [await plain.locator("ol > li").count(), await plain.getByRole("link").allTextContents()], [4, ["Acme", "Projects", "Web"]]);
  const current = plain.locator("[aria-current=page]");
  check("current", [await current.textContent(), await current.evaluate((el) => el.tagName), await plain.locator("li").last().locator("[data-icon]").count()], ["v2.1", "SPAN", 0]);
  check("hrefs", await plain.getByRole("link").evaluateAll((all) => all.map((a) => a.getAttribute("href"))), ["/", "/c/breadcrumbs?p=projects", "/c/breadcrumbs?p=web"]);
  check("separators", await plain.locator("li > [data-icon=chevron-right]").evaluateAll((all) => all.map((i) => i.getAttribute("aria-hidden"))), ["true", "true", "true"]);
  check("icon", await plain.getByRole("link", { name: "Acme" }).locator("[data-icon=inbox]").count(), 1);
  check("titles", await plain.getByRole("link", { name: "Projects" }).getAttribute("title"), "Projects");
  const boxes = await plain.locator("ol > li").evaluateAll((all) => all.map((li) => li.getBoundingClientRect()).map((b) => [Math.round(b.x), Math.round(b.y)]));
  check("oneLineInOrder", [new Set(boxes.map((b) => b[1])).size, boxes.every((b, i) => i === 0 || b[0] > boxes[i - 1][0])], [1, true]);

  // From the keyboard: Tab goes from link to link and skips the current page.
  await plain.getByRole("link", { name: "Acme" }).focus();
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  check("tabThroughLinks", await focused(), "Web");
  await page.keyboard.press("Tab");
  check("currentNotFocusable", await page.evaluate(() => document.activeElement.closest("nav")?.getAttribute("aria-label")), "Collapsed breadcrumb");
  await page.keyboard.press("Tab");

  // The collapsed middle: a button that opens a menu of the hidden crumbs.
  const collapsed = trail("Collapsed breadcrumb");
  const more = collapsed.getByRole("button", { name: "Show 3 hidden levels" });
  const popup = collapsed.locator("[popover]");
  check("more", [await more.getAttribute("aria-haspopup"), await more.getAttribute("title"), await more.locator("[aria-hidden=true]").textContent()], ["menu", "Projects › Web › Releases", "…"]);
  check("focusedMore", await page.evaluate(() => document.activeElement.getAttribute("title")), "Projects › Web › Releases");
  await page.keyboard.press("Enter");
  await untilOpen(popup);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitem");
  check("menuFirst", await focused(), "Projects");
  await settled(popup);
  const m = await more.boundingBox();
  const p = await popup.boundingBox();
  check("menuUnderButton", [p.y >= m.y + m.height, Math.abs(p.x - m.x) < 2], [true, true]);
  check("menuLinks", await popup.getByRole("menuitem").evaluateAll((all) => all.map((a) => `${a.tagName} ${a.getAttribute("href")}`)), [
    "A /c/breadcrumbs?p=projects", "A /c/breadcrumbs?p=web", "A /c/breadcrumbs?p=releases",
  ]);
  check("menuNamed", await page.getByRole("menu", { name: "Show 3 hidden levels" }).count(), 1);
  check("axeOpen", await axe(), []);
  await page.keyboard.press("End");
  check("menuEnd", await focused(), "Releases");
  await page.keyboard.press("Escape");
  await untilOpen(popup, false);
  check("escReturnsFocus", await page.evaluate(() => document.activeElement.getAttribute("title")), "Projects › Web › Releases");
  await more.click();
  await untilOpen(popup);
  // A click focuses the menu itself, as upstream's; the first ArrowDown goes to the first item.
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/breadcrumbs?p=web");
  check("menuNavigates", new URL(page.url()).search, "?p=web");

  // Too long: one line, the current page cut first, a short crumb never.
  const long = trail("Long breadcrumb");
  await page.waitForFunction(() => document.querySelector('[aria-label="Long breadcrumb"] li').style.getPropertyValue("--crumb-width") !== "");
  const room = await long.evaluate((el) => Math.round(el.parentElement.getBoundingClientRect().width));
  const list = await long.locator("ol").boundingBox();
  check("longOneLine", [room, Math.round(list.width) <= room, list.height < 30], [352, true, true]);
  const longCurrent = long.locator("[aria-current=page]");
  check("currentCut", [await cut(longCurrent), await longCurrent.evaluate((el) => getComputedStyle(el).textOverflow), await longCurrent.getAttribute("title")], [true, "ellipsis", "connection-eu-central-1-production-readonly-replica"]);
  check("shortCrumbWhole", await cut(long.getByRole("link", { name: "Org" })), false);
  const widths = await long.locator("ol > li").evaluateAll((all) => all.map((li) => Math.round(li.getBoundingClientRect().width)));
  // The floors: 5.5rem for a crumb before the current page, 25vw (under 12rem here)
  // for the current page; a crumb shorter than its floor keeps its own width.
  check("floors", [widths[1] >= 88, widths[3] >= 192, widths[0] < 88], [true, true, true]);
  check("measured", await long.locator("ol > li").evaluateAll((all) => all.every((li) => /^\d+px$/.test(li.style.getPropertyValue("--crumb-width")))));

  // Wrapping: more lines, nothing cut.
  const wrap = trail("Wrapping breadcrumb");
  const wrapped = await wrap.locator("ol").boundingBox();
  check("wraps", [await wrap.getAttribute("data-wrap"), wrapped.height > 30, await cut(wrap.locator("[aria-current=page]"))], ["", true, false]);

  // Right to left: the trail starts on the right and the separators point left.
  await setDir("rtl");
  const rtl = await plain.locator("ol > li").evaluateAll((all) => all.map((li) => Math.round(li.getBoundingClientRect().x)));
  check("rtlOrder", rtl.every((x, i) => i === 0 || x < rtl[i - 1]));
  check("rtlSeparators", await plain.locator("li > [data-icon=chevron-right]").first().evaluate((el) => getComputedStyle(el).scale), "-1 1");
};
