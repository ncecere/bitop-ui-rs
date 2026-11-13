export default async ({ page, check }) => {
  const loaders = page.locator("main [data-loader]");
  // With a label it is a status; without one it is hidden from assistive technology.
  check("roles", await loaders.evaluateAll((all) => all.map((l) => [l.dataset.size, l.getAttribute("role"), l.getAttribute("aria-hidden")])), [
    ["sm", "status", null], ["md", "status", null], ["lg", "status", null], ["md", "status", null], ["md", null, "true"],
  ]);
  check("named", await page.getByRole("status").filter({ hasText: /^Loading$/ }).count(), 1);
  check("dotsHidden", await loaders.evaluateAll((all) => all.every((l) => l.firstElementChild.getAttribute("aria-hidden") === "true" && l.firstElementChild.children.length === 3)));

  // Sizes: the dots grow.
  const dot = (n) => loaders.nth(n).locator("span > span").first().evaluate((d) => parseFloat(getComputedStyle(d).width));
  check("sizes", [await dot(0), await dot(1), await dot(2)], [5, 6, 8]);

  // The label is hidden unless asked for.
  const hidden = await loaders.nth(1).getByText("Loading").boundingBox();
  const shown = await loaders.nth(3).getByText("Thinking").boundingBox();
  check("labelHidden", [Math.round(hidden.width), shown.width > 20], [1, true]);

  // The dots pulse one after another, without end.
  const animation = await loaders.nth(1).locator("span > span").evaluateAll((all) => all.map((d) => { const s = getComputedStyle(d); return [s.animationIterationCount, s.animationDelay, s.animationName !== "none"]; }));
  check("pulse", animation, [["infinite", "0s", true], ["infinite", "0.15s", true], ["infinite", "0.3s", true]]);
  // The animation really runs: the keyframes are found, and the opacity changes.
  const pulses = await loaders.nth(1).locator("span > span").first().evaluate(async (d) => {
    const first = getComputedStyle(d).opacity;
    for (let i = 0; i < 120; i++) {
      await new Promise((r) => requestAnimationFrame(r));
      if (getComputedStyle(d).opacity !== first) return true;
    }
    return false;
  });
  check("pulses", pulses);

  // Reduced motion: the dots stop and stay visible.
  await page.emulateMedia({ reducedMotion: "reduce" });
  // The page hears of the change a moment later, and the style follows a frame after that.
  await page.waitForFunction(() => { const s = getComputedStyle(document.querySelector("[data-loader] span > span")); return s.animationName === "none" && s.opacity === "0.7"; });
  check("reducedMotion", await loaders.nth(1).locator("span > span").first().evaluate((d) => [getComputedStyle(d).animationName, getComputedStyle(d).opacity]), ["none", "0.7"]);
  await page.emulateMedia({ reducedMotion: "no-preference" });
};
