export default async ({ page, check, setDir }) => {
  const style = (id) => page.locator(`#${id}`).evaluate((s) => { const c = getComputedStyle(s); return { active: s.hasAttribute("data-active"), duration: c.animationDuration, name: c.animationName !== "none", count: c.animationIterationCount, clip: c.backgroundClip || c.webkitBackgroundClip, gradient: c.backgroundImage.includes("gradient"), direction: c.animationDirection }; });
  // Active: the text is painted by a gradient that sweeps without end.
  check("active", await style("active"), { active: true, duration: "2s", name: true, count: "infinite", clip: "text", gradient: true, direction: "normal" });
  check("text", await page.locator("#active").textContent(), "Thinking…");
  // The duration is an argument, and the caller's style is kept beside it.
  const slow = await style("slow");
  check("duration", [slow.duration, await page.locator("#slow").evaluate((s) => [s.style.getPropertyValue("--shimmer-duration"), s.style.fontWeight !== ""])], ["4s", ["4s", true]]);
  // Not active: plain muted text, the same element.
  const idle = await style("idle");
  check("idle", [idle.active, idle.name, idle.gradient], [false, false, false]);
  const muted = await page.locator("#idle").evaluate((s) => getComputedStyle(s).color);
  check("idleIsMuted", muted !== await page.evaluate(() => getComputedStyle(document.body).color));

  // The sweep moves: the background position changes from frame to frame.
  const moved = await page.locator("#active").evaluate(async (s) => {
    const read = () => getComputedStyle(s).backgroundPositionX;
    const first = read();
    for (let i = 0; i < 120; i++) {
      await new Promise((r) => requestAnimationFrame(r));
      if (read() !== first) return true;
    }
    return false;
  });
  check("sweeps", moved);

  // Right to left: the sweep runs the other way.
  await setDir("rtl");
  check("rtl", (await style("active")).direction, "reverse");
  await setDir("ltr");

  // Reduced motion: no sweep, and the text is muted, not transparent.
  await page.emulateMedia({ reducedMotion: "reduce" });
  // The page hears of the change a moment later, and the style follows a frame after that.
  await page.waitForFunction(() => { const s = getComputedStyle(document.getElementById("active")); return s.animationName === "none" && s.color !== "rgba(0, 0, 0, 0)"; });
  check("reducedMotion", await page.locator("#active").evaluate((s) => [getComputedStyle(s).backgroundImage, getComputedStyle(s).color]), ["none", muted]);
  await page.emulateMedia({ reducedMotion: "no-preference" });
};
