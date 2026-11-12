export default async ({ page, check }) => {
  check("keys", await page.locator("main kbd").count(), 7);
  // Symbols are hidden from screen readers and spelled out instead.
  const mod = page.locator("main kbd", { has: page.locator("[data-kbd-key=mod]") }).first();
  check("spoken", await mod.locator(".sr-only").textContent(), "Command or Control");
  check("shown", await mod.locator("[aria-hidden]").textContent().then((t) => t.trim()), "Ctrl");
};
