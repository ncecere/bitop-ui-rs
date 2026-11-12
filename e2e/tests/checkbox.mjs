export default async ({ page, check }) => {
  const terms = page.getByRole("checkbox", { name: "Accept the policy" });
  await terms.focus();
  await page.keyboard.press("Space");
  check("checkboxToggles", await terms.isChecked());
  const memory = page.getByRole("switch", { name: "Use memory" });
  await page.getByText("Use memory").click();
  check("switchToggles", await memory.isChecked());
  check("switchCount", await page.getByRole("switch").count(), 3);
};
