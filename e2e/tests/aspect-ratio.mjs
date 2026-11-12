export default async ({ page, check }) => {
  const boxes = page.locator("main [style*='--aspect-ratio']");
  const ratio = (i) => boxes.nth(i).evaluate((b) => { const r = b.getBoundingClientRect(); return +(r.width / r.height).toFixed(2); });
  check("wide", await ratio(0), 1.78);
  check("square", await ratio(1), 1);
  check("mediaFills", await page.getByRole("img", { name: "A blue square" }).evaluate((i) => { const a = i.getBoundingClientRect(), b = i.parentElement.getBoundingClientRect(); return Math.abs(a.width - b.width) < 1 && Math.abs(a.height - b.height) < 1; }));
};
