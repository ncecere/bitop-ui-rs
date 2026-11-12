export default async ({ page, check }) => {
  const icons = page.locator("main [data-icon]");
  check("count", await icons.count(), 15);
  check("hidden", await icons.evaluateAll((all) => all.every((i) => i.getAttribute("aria-hidden") === "true")));
  check("masked", await icons.evaluateAll((all) => all.every((i) => (getComputedStyle(i).maskImage || getComputedStyle(i).webkitMaskImage).startsWith("url("))));
  check("square", await icons.first().evaluate((i) => { const b = i.getBoundingClientRect(); return [b.width, b.height]; }), [20, 20]);
  const colour = await icons.last().evaluate((i) => [getComputedStyle(i).backgroundColor, getComputedStyle(i.parentElement).color]);
  check("takesTextColour", colour[0], colour[1]);
};
