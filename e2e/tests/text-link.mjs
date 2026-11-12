export default async ({ page, check }) => {
  const external = page.getByRole("link", { name: /API reference/ });
  check("external", await external.evaluate((a) => [a.target, a.rel]), ["_blank", "noreferrer noopener"]);
  check("externalAnnounced", await external.textContent().then((t) => t.replace(/\s+/g, " ").trim()), "API reference (opens in a new tab)");
  check("internal", await page.getByRole("link", { name: "getting started guide" }).getAttribute("target"), null);
  check("muted", await page.getByRole("link", { name: "Privacy" }).getAttribute("data-tone"), "muted");
};
