export default async ({ page, check }) => {
  check("named", await page.getByRole("region", { name: "Members" }).count(), 1);
  check("heading", await page.getByRole("heading", { name: "Members", level: 2 }).count(), 1);
  check("action", await page.getByRole("button", { name: "Invite" }).isVisible());
  // A header without actions does not leave an empty slot taking space.
  check("emptySlotsHidden", await page.locator("main section section > header > div:empty").evaluateAll((all) => all.every((d) => getComputedStyle(d).display === "none")));
};
