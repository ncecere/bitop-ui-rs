export default async ({ page, check }) => {
  const separators = page.locator("main [role=separator]");
  check("orientations", await separators.evaluateAll((all) => all.map((s) => s.getAttribute("aria-orientation"))), ["horizontal", "vertical"]);
  const sizes = await separators.evaluateAll((all) => all.map((s) => { const b = s.getBoundingClientRect(); return b.width > b.height; }));
  check("shape", sizes, [true, false]);
  check("label", await page.getByText("or", { exact: true }).isVisible());
};
