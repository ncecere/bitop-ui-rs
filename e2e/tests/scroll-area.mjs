export default async ({ page, check }) => {
  const notes = page.getByRole("region", { name: "Release notes" });
  await notes.focus();
  await page.keyboard.press("End");
  await page.waitForTimeout(300);
  check("keyboardScrolls", await notes.evaluate((r) => r.scrollTop > 0));
};
