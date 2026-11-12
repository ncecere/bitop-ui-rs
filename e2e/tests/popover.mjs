export default async ({ page, check, button, settled, setDir }) => {
  const trigger = button("Filters");
  const popover = page.locator("[popover]", { hasText: "Anchored by the browser" });
  await trigger.click();
  check("open", await popover.evaluate((p) => p.matches(":popover-open")));
  await settled(popover);
  let t = await trigger.boundingBox();
  let p = await popover.boundingBox();
  check(
    "anchored",
    { below: p.y >= t.y + t.height, gap: +(p.y - t.y - t.height).toFixed(1), aligned: Math.abs(p.x - t.x) < 2 },
    { below: true, gap: 4, aligned: true },
  );
  await page.keyboard.press("Escape");
  check("closedByEsc", await popover.evaluate((el) => !el.matches(":popover-open")));

  await setDir("rtl");
  await trigger.click();
  await settled(popover);
  t = await trigger.boundingBox();
  p = await popover.boundingBox();
  check("rtlAligned", Math.abs(p.x + p.width - (t.x + t.width)) < 2);
};
