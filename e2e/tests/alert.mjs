export default async ({ page, check }) => {
  const alerts = page.locator("main [data-alert]");
  check("roles", await alerts.evaluateAll((all) => all.map((a) => `${a.dataset.tone}:${a.getAttribute("role")}`)), [
    "info:status", "success:status", "warning:status", "danger:alert", "danger:alert", "info:status",
  ]);
  check("icons", await alerts.nth(3).locator("[data-icon]").getAttribute("data-icon"), "circle-alert");
  check("iconHidden", await alerts.last().locator("[data-icon=info]").count(), 0);
  check("action", await page.getByRole("button", { name: "Try again" }).isVisible());
  await page.getByRole("button", { name: "Dismiss" }).click();
  check("dismissed", await alerts.count(), 5);
};
