export default async ({ page, check }) => {
  const notes = page.getByRole("region", { name: "Release notes" });
  check("maxHeight", Math.round(await notes.evaluate((r) => r.getBoundingClientRect().height)), 224);
  check("verticalOnly", await notes.evaluate((r) => getComputedStyle(r).overflowX), "hidden");
  await notes.focus();
  await page.keyboard.press("End");
  await page.waitForFunction(() => document.querySelector('[aria-label="Release notes"]').scrollTop > 0);
  check("keyboardScrolls", await notes.evaluate((r) => r.scrollTop > 0));

  const tags = page.getByRole("region", { name: "Project tags" });
  check("horizontalOnly", await tags.evaluate((r) => getComputedStyle(r).overflowY), "hidden");
  check("horizontalOverflows", await tags.evaluate((r) => r.scrollWidth > r.clientWidth));
  await tags.focus();
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => document.querySelector('[aria-label="Project tags"]').scrollLeft > 0);
  check("horizontalScrolls", await tags.evaluate((r) => r.scrollLeft > 0));
};
