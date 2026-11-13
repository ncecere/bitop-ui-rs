export default async ({ page, check }) => {
  check("title", await page.getByText("No projects yet").isVisible());
  check("action", await page.getByRole("button", { name: "New project" }).isVisible());
  check("secondary", await page.getByRole("button", { name: "Read the guide" }).isVisible());
  const inbox = page.locator("main [data-icon=inbox]");
  check("iconDecorative", await inbox.locator("xpath=..").getAttribute("aria-hidden"), "true");
  // 22px icon in a 48px circle, as upstream.
  check("iconSize", await inbox.evaluate((i) => [i.getBoundingClientRect().width, i.parentElement.getBoundingClientRect().width]), [22, 48]);
  check("compact", await page.locator("main [data-size=compact]").count(), 1);
};
