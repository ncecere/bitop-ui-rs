export default async ({ page, check, button }) => {
  check("variants", await page.locator("main button[data-variant]").evaluateAll((all) => [...new Set(all.map((b) => b.dataset.variant))]), ["solid", "outline", "ghost", "danger"]);
  check("typeIsButton", await button("Solid").getAttribute("type"), "button");
  check("disabled", await button("Disabled").isDisabled());
  const heights = await page.locator("main button[data-size]").evaluateAll((all) => Object.fromEntries(all.map((b) => [b.dataset.size, b.getBoundingClientRect().height])));
  check("sizes", heights, { md: 36, sm: 32, lg: 40 });
};
