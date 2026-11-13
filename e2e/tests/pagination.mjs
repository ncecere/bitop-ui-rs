export default async ({ page, base, check, focused, setDir }) => {
  const nav = (name) => page.getByRole("navigation", { name, exact: true });
  const entries = (locator) =>
    locator.locator("li").evaluateAll((all) => all.map((li) => li.firstElementChild.getAttribute("aria-label") ?? li.textContent.trim()));
  const main = nav("Pagination");

  // The server renders the list for the current page: first, last, the pages around
  // the current one, and an ellipsis where pages are skipped.
  check("range", await entries(main), ["Go to previous page", "Page 1", "More pages", "Page 4", "Page 5", "Page 6", "More pages", "Page 20", "Go to next page"]);
  const current = main.locator("[aria-current=page]");
  check("current", [await current.count(), await current.textContent(), await current.getAttribute("aria-label"), await current.getAttribute("data-variant")], [1, "5", "Page 5", "secondary"]);
  check("links", await main.getByRole("link").evaluateAll((all) => all.map((a) => a.getAttribute("href").split("=")[1])), ["4", "1", "4", "5", "6", "20", "6"]);
  check("rel", [await main.getByRole("link", { name: "Go to previous page" }).getAttribute("rel"), await main.getByRole("link", { name: "Go to next page" }).getAttribute("rel")], ["prev", "next"]);
  check("stepText", [await main.getByRole("link", { name: "Go to previous page" }).textContent(), await main.getByRole("link", { name: "Go to next page" }).textContent()], ["Previous", "Next"]);
  const ellipsis = main.locator("li > span").first();
  check("ellipsis", [await ellipsis.locator("[data-icon=ellipsis]").count(), await ellipsis.locator(".sr-only").textContent(), await ellipsis.evaluate((el) => el.tabIndex)], [1, "More pages", -1]);
  const size = await current.boundingBox();
  check("pageSize", [Math.round(size.width), Math.round(size.height)], [36, 36]);
  const ys = await main.locator("li").evaluateAll((all) => all.map((li) => Math.round(li.getBoundingClientRect().y)));
  check("oneRowCentred", new Set(ys).size, 1);

  // From the keyboard: Tab moves over the links, the ellipsis is skipped, Enter follows.
  await main.getByRole("link", { name: "Page 1", exact: true }).focus();
  await page.keyboard.press("Tab");
  check("tabSkipsEllipsis", await focused(), "4");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/pagination?page=6");
  check("enterFollows", [await main.locator("[aria-current=page]").textContent(), await entries(main)], ["6", ["Go to previous page", "Page 1", "More pages", "Page 5", "Page 6", "Page 7", "More pages", "Page 20", "Go to next page"]]);
  await main.getByRole("link", { name: "Go to next page" }).click();
  await page.waitForURL("**/c/pagination?page=7");
  check("nextLink", await main.locator("[aria-current=page]").textContent(), "7");
  await main.getByRole("link", { name: "Page 20" }).click();
  await page.waitForURL("**/c/pagination?page=20");
  check("lastPage", await entries(main), ["Go to previous page", "Page 1", "More pages", "Page 16", "Page 17", "Page 18", "Page 19", "Page 20", "Go to next page"]);

  // At either end the step is a disabled button, not a link: it cannot be focused.
  const next = main.getByRole("button", { name: "Go to next page" });
  check("nextDisabledAtEnd", [await next.isDisabled(), await main.getByRole("link", { name: "Go to next page" }).count()], [true, 0]);
  await main.getByRole("link", { name: "Page 20" }).focus();
  await page.keyboard.press("Tab");
  check("disabledSkipped", await page.evaluate(() => document.activeElement.closest("nav")?.getAttribute("aria-label")), "First page");
  check("firstPage", [await nav("First page").getByRole("button", { name: "Go to previous page" }).isDisabled(), await entries(nav("First page"))], [true, ["Go to previous page", "Page 1", "Page 2", "Page 3", "Page 4", "Page 5", "More pages", "Page 10", "Go to next page"]]);
  check("lastOfTen", await nav("Last page").getByRole("button", { name: "Go to next page" }).isDisabled());
  await page.goto(`${base}/c/pagination?page=99`);
  check("clamped", await main.locator("[aria-current=page]").textContent(), "20");

  // It is links: the same click works with scripts turned off.
  const plain = await page.context().browser().newContext({ javaScriptEnabled: false });
  const still = await plain.newPage();
  await still.goto(`${base}/c/pagination`);
  await still.getByRole("navigation", { name: "Pagination", exact: true }).getByRole("link", { name: "Go to next page" }).click();
  await still.waitForURL("**/c/pagination?page=6");
  check("worksWithoutScript", await still.getByRole("navigation", { name: "Pagination", exact: true }).locator("[aria-current=page]").textContent(), "6");
  await plain.close();

  // Sizes and wider ranges.
  const small = nav("Small pagination");
  check("smallRange", await entries(small), [
    "Go to previous page", "Page 1", "Page 2", "More pages", "Page 8", "Page 9", "Page 10", "Page 11", "Page 12", "More pages", "Page 19", "Page 20", "Go to next page",
  ]);
  const s = await small.locator("[aria-current=page]").boundingBox();
  const one = await small.getByRole("link", { name: "Page 1", exact: true }).boundingBox();
  check("smallSize", [await small.locator("[aria-current=page]").getAttribute("data-size"), Math.round(s.height), Math.round(one.width), s.width >= 32], ["sm", 32, 32, true]);

  // Every string is an argument.
  const german = nav("Seitennavigation");
  check("strings", await entries(german), ["Zurück zur vorherigen Seite", "Seite 1", "Seite 2", "Seite 3", "Weiter zur nächsten Seite"]);
  check("stringsText", [await german.getByRole("link", { name: "Zurück zur vorherigen Seite" }).textContent(), await german.getByRole("link", { name: "Weiter zur nächsten Seite" }).textContent()], ["Zurück", "Weiter"]);
  // Compact: on a narrow screen the text is clipped, and still names the link.
  const text = german.getByRole("link", { name: "Weiter zur nächsten Seite" }).locator("span").first();
  check("compactWide", (await text.boundingBox()).width > 20);
  await page.setViewportSize({ width: 500, height: 900 });
  check("compactNarrow", [Math.round((await text.boundingBox()).width), await german.getByRole("link", { name: "Weiter zur nächsten Seite" }).count()], [1, 1]);
  const notCompact = main.getByRole("link", { name: "Go to previous page" }).locator("span").last();
  check("notCompactNarrow", (await notCompact.boundingBox()).width > 20);
  await page.setViewportSize({ width: 1000, height: 900 });

  // Previous and next alone, and the parts put together by hand.
  const cursor = nav("Cursor pagination");
  check("cursor", [await entries(cursor), await cursor.getByRole("button", { name: "Go to previous page" }).isDisabled(), await cursor.getByRole("link", { name: "Go to next page" }).getAttribute("href")], [["Go to previous page", "Go to next page"], true, "/c/pagination?after=abc"]);
  const composed = nav("Composed pagination");
  check("composed", [await entries(composed), await composed.locator("[aria-current=page]").textContent()], [["Go to previous page", "Page 1", "Page 2", "More pages", "Page 9", "Go to next page"], "2"]);

  // Right to left: previous is on the right, and the chevrons point the other way.
  await setDir("rtl");
  const previous = await main.getByRole("link", { name: "Go to previous page" }).boundingBox();
  const last = await main.getByRole("button", { name: "Go to next page" }).boundingBox();
  check("rtlOrder", previous.x > last.x);
  check("rtlChevrons", await main.locator("[data-icon=chevron-left]").evaluate((el) => getComputedStyle(el).scale), "-1 1");
};
