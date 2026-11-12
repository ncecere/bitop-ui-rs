export default async ({ page, check }) => {
  const select = page.getByLabel("Retention");
  check("value", await select.inputValue(), "90");
  await select.selectOption("365");
  check("changes", await select.inputValue(), "365");
  check("height", await select.evaluate((s) => s.getBoundingClientRect().height), 36);
};
