export default async ({ page, check }) => {
  check("variants", await page.locator("main span[data-variant]").evaluateAll((all) => all.map((b) => b.dataset.variant)), ["neutral", "accent", "success", "warning", "danger"]);
};
