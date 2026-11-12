export default async ({ page, check }) => {
  const all = page.locator("main [data-shape]");
  check("hidden", await all.evaluateAll((list) => list.every((s) => s.getAttribute("aria-hidden") === "true")));
  check("circle", await all.first().evaluate((s) => { const b = s.getBoundingClientRect(); return [b.width, b.height]; }), [40, 40]);
  const lines = await page.locator("main [data-shape=text]").evaluateAll((list) => list.map((s) => s.getBoundingClientRect().width));
  check("lastLineShorter", lines.length === 3 && lines[2] < lines[0]);
};
