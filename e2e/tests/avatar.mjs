export default async ({ page, check }) => {
  const avatar = (name) => page.getByRole("img", { name });
  check("keepsGoodImage", await avatar("With Picture").locator("img").count(), 1);
  check("dropsBrokenImage", await avatar("Broken Picture").locator("img").count(), 0);
  check("initials", (await avatar("Broken Picture").textContent()).trim(), "BP");
  check("sizes", await page.locator("main [role=img][data-size]").evaluateAll((all) => all.slice(0, 3).map((a) => a.getBoundingClientRect().width)), [24, 32, 44]);
};
