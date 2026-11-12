export default async ({ page, check, focused, setDir }) => {
  const card = (label) => page.locator("[data-stat-card]").filter({ has: page.locator("dt", { hasText: label }) });
  // A <dl>: the label is the term, the value and the rest are definitions.
  check("semantics", await card("Requests").locator("dl > *").evaluateAll((all) => all.map((e) => e.tagName)), ["DT", "DD", "DD", "DD", "DD"]);
  check("value", await card("Requests").locator("dd").first().textContent(), "12,480");
  check("tabularValue", await card("Requests").locator("dd").first().evaluate((d) => getComputedStyle(d).fontVariantNumeric), "tabular-nums");

  // The change: an arrow, words for assistive technology, and a sentiment.
  const delta = (label) => card(label).locator("[data-sentiment]");
  const read = async (label) => [
    await delta(label).getAttribute("data-sentiment"),
    await delta(label).locator("[data-icon]").getAttribute("data-icon"),
    (await delta(label).textContent()).replace(/\s+/g, " ").trim(),
  ];
  check("up", await read("Requests"), ["positive", "arrow-up-right", "Increased by +12%"]);
  check("down", await read("Conversion"), ["negative", "arrow-down-right", "Decreased by -0.4%"]);
  check("flat", await read("Latency"), ["neutral", "arrow-right", "Unchanged: 0%"]);
  check("upIsBad", await read("Errors"), ["negative", "arrow-up-right", "Increased by +9"]);
  const colours = await Promise.all(["Requests", "Conversion", "Latency"].map((l) => delta(l).evaluate((d) => getComputedStyle(d).backgroundColor)));
  check("sentimentColours", new Set(colours).size, 3);
  check("deltaLabelAndHint", [await card("Requests").getByText("vs last week").count(), await card("Latency").getByText("p50, last hour").count()], [1, 1]);

  // The icon sits in the top corner at the end, and is decorative.
  const icon = card("Requests").locator("span[aria-hidden=true]").first();
  const corner = async () => {
    const c = await card("Requests").boundingBox();
    const i = await icon.boundingBox();
    return [Math.round(c.x + c.width - (i.x + i.width)), Math.round(i.x - c.x), Math.round(i.width)];
  };
  check("iconAtEnd", (await corner()).filter((_, n) => n !== 1), [16, 32]);

  // Empty slots take no space; a card with nothing optional has two rows.
  check("emptySlotsHidden", await card("Plain").locator("dd:empty").evaluateAll((all) => all.map((d) => getComputedStyle(d).display)), ["none", "none"]);
  check("chart", await card("Storage").getByRole("img", { name: "Storage over the last week" }).isVisible());

  // A linked card: one link, named by the label, stretched over the card.
  const linked = card("Sources");
  check("linked", [await linked.getAttribute("data-linked"), await linked.locator("dt a").getAttribute("href")], ["", "#sources"]);
  const cover = await linked.locator("dt a").evaluate((a) => { const p = getComputedStyle(a, "::after"); return [p.position, p.inset ?? p.top]; });
  check("stretched", cover, ["absolute", "0px"]);
  const box = await linked.boundingBox();
  await page.mouse.click(box.x + box.width - 10, box.y + 12);
  check("clickAnywhereFollows", await page.evaluate(() => location.hash), "#sources");
  // The link inside the details stays above the stretched link.
  await linked.getByRole("link", { name: "4 archived" }).click();
  check("innerLinkWins", await page.evaluate(() => location.hash), "#archived");
  await linked.getByRole("link", { name: "Sources" }).focus();
  check("focus", await focused(), "Sources");
  await page.keyboard.press("Tab");
  check("tabToInnerLink", await focused(), "4 archived");
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Enter");
  check("enterFollows", await page.evaluate(() => location.hash), "#sources");
  check("notLinked", [await card("Plain").getAttribute("data-linked"), await card("Plain").getByRole("link").count()], [null, 0]);

  // Right to left: the icon moves to the other corner and the arrows mirror.
  await setDir("rtl");
  const rtl = await corner();
  check("rtlIcon", [rtl[1], rtl[2]], [16, 32]);
  check("rtlArrow", await delta("Requests").locator("[data-icon]").evaluate((i) => getComputedStyle(i).scale), "-1 1");
};
