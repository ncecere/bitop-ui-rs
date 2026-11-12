export default async ({ page, check }) => {
  check("title", await page.getByText("No chats yet").isVisible());
  check("action", await page.getByRole("button", { name: "New chat" }).isVisible());
  check("iconDecorative", await page.locator("main [data-icon=inbox]").getAttribute("aria-hidden"), "true");
  check("compact", await page.locator("main [data-size=compact]").count(), 1);
};
