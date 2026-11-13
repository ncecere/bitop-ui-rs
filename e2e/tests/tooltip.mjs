export default async ({ page, check, button, focused, settled }) => {
  const bell = button("Notifications");
  const copy = button("Copy link");
  const tipOf = (trigger) => trigger.evaluate((b) => b.getAttribute("aria-describedby")).then((id) => page.locator(`#${id}`));
  const open = (tip) => tip.evaluate((t) => t.matches(":popover-open"));
  const centre = (box) => box.x + box.width / 2;

  const copyTip = await tipOf(copy);
  check("describes", await copyTip.getAttribute("role"), "tooltip");
  check("shortcut", (await copyTip.textContent()).replace(/\s+/g, " ").trim(), "Copy link⌘C");
  // Opens at once on keyboard focus, above and centred.
  await bell.focus();
  await page.keyboard.press("Tab");
  check("onFocus", [await focused(), await open(copyTip)], ["", true]);
  await settled(copyTip);
  let t = await copy.boundingBox();
  let p = await copyTip.boundingBox();
  check("above", { gap: Math.round(t.y - p.y - p.height), centred: Math.abs(centre(p) - centre(t)) < 2 }, { gap: 6, centred: true });
  // inline-flex, made a block in the top layer.
  check("chip", await copyTip.evaluate((el) => getComputedStyle(el).display), "flex");
  await page.keyboard.press("Escape");
  check("esc", !(await open(copyTip)));

  // side: bottom.
  const bellTip = await tipOf(bell);
  await page.mouse.move(0, 0);
  await bell.hover();
  await page.waitForTimeout(300);
  check("waitsForDelay", !(await open(bellTip)));
  await bellTip.waitFor({ state: "visible" });
  await settled(bellTip);
  t = await bell.boundingBox();
  p = await bellTip.boundingBox();
  check("below", Math.round(p.y - t.y - t.height), 6);
  await page.mouse.move(0, 0);
  await page.waitForTimeout(100);
  check("hidesOnLeave", !(await open(bellTip)));

  // delay: the time between the pointer arriving and the tooltip opening.
  await bellTip.evaluate((tip) => (tip.dataset.delay = "100"));
  const opened = page.evaluate(
    (id) =>
      new Promise((resolve) => {
        const tip = document.getElementById(id);
        const start = performance.now();
        tip.addEventListener("toggle", (e) => e.newState === "open" && resolve(performance.now() - start));
      }),
    await bell.getAttribute("aria-describedby"),
  );
  await bell.hover();
  const waited = await opened;
  check("customDelay", waited >= 90 && waited < 500);
  await page.mouse.move(0, 0);

  // tooltip_text: focusable plain text described by its tooltip.
  const text = page.getByText("82%", { exact: true });
  const textTip = await tipOf(text);
  check("textFocusable", await text.getAttribute("tabindex"), "0");
  check("textDecoration", await text.evaluate((el) => getComputedStyle(el).textDecorationStyle), "dotted");
  await text.focus();
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Tab");
  check("textOnFocus", await open(textTip));
};
