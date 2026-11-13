export default async ({ page, check }) => {
  const badges = page.locator("main span[data-tone][data-variant]");
  check("tones", await badges.evaluateAll((all) => all.slice(0, 5).map((b) => b.dataset.tone)), ["neutral", "info", "success", "warning", "danger"]);

  const style = (text, read) => page.locator("main span[data-variant]", { hasText: new RegExp(`^${text}$`) }).first().evaluate(read);
  const box = (b) => [Math.round(b.getBoundingClientRect().height), getComputedStyle(b).fontSize];
  check("md", await style("Info", box), [22, "12px"]);
  check("sm", await style("sm", box), [18, "11px"]);

  // The thin ring is in the tone's colour; the outline variant has the surface and a hairline.
  const ring = (b) => getComputedStyle(b).boxShadow;
  check("ringInTone", (await style("Info", ring)) !== (await style("Success", ring)));
  const surface = await page.evaluate(() => {
    const probe = document.createElement("div");
    probe.style.background = "var(--color-surface)";
    document.body.append(probe);
    const color = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return color;
  });
  check("outlineSurface", await style("Beta", (b) => getComputedStyle(b).backgroundColor), surface);
  check("outlineKeepsToneText", (await style("Beta", (b) => getComputedStyle(b).color)) === (await style("Info", (b) => getComputedStyle(b).color)));

  // Status badges lead with a decorative dot; Building pulses.
  const live = page.locator("main span[data-variant]", { hasText: "Live" });
  check("statusDot", await live.locator("> span[aria-hidden=true]").count(), 1);
  const building = page.locator("main span[data-variant]", { hasText: "Building" });
  check("pulse", await building.locator("> span[data-pulse]").evaluate((d) => getComputedStyle(d, "::after").animationName !== "none"));
  check("noPulse", await live.locator("> span").evaluate((d) => d.hasAttribute("data-pulse")), false);

  // A lone dot is decorative unless it has a label.
  check("dotLabelled", await page.getByRole("img", { name: "Gateway down" }).count(), 1);
  check("dotDecorative", await page.locator("main span[aria-hidden=true][data-tone=success]:not([data-variant])").count(), 1);

  // Under reduced motion the pulse is not drawn.
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.waitForFunction(() => matchMedia("(prefers-reduced-motion: reduce)").matches);
  check("reducedMotion", await building.locator("> span[data-pulse]").evaluate((d) => getComputedStyle(d, "::after").display), "none");
  await page.emulateMedia({ reducedMotion: null });
};
