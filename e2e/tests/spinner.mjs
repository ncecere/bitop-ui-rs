export default async ({ page, check }) => {
  check("labelled", await page.getByRole("status").filter({ hasText: "Saving" }).count(), 1);
  check("decorativeHidden", await page.locator("main span[data-size][aria-hidden=true]").count(), 3);
  check("loading", await page.getByRole("status").filter({ hasText: "Loading…" }).isVisible());
  const sizes = await page.locator("main section [data-size]").evaluateAll((all) => all.slice(0, 3).map((s) => s.getBoundingClientRect().width));
  check("sizesGrow", sizes[0] < sizes[1] && sizes[1] < sizes[2]);
  check("spins", await page.locator("main svg").first().evaluate((s) => getComputedStyle(s).animationName !== "none"));
};
