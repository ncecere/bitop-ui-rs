export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const trigger = button("What is a preview deployment?");
  const popover = page.locator("[data-popover]", { hasText: "Every pull request" });
  const isOpen = (p) => p.evaluate((el) => el.matches(":popover-open"));
  const centre = (box) => box.x + box.width / 2;

  check("haspopup", await trigger.getAttribute("aria-haspopup"), "dialog");
  await trigger.click();
  check("open", await isOpen(popover));
  await settled(popover);
  check("named", await page.getByRole("dialog", { name: "Preview deployments" }).count(), 1);
  check("described", await popover.evaluate((p) => document.getElementById(p.getAttribute("aria-describedby"))?.textContent), "Every pull request gets its own URL, so reviewers can try changes before they are merged.");
  // Focus moves to the first control in it.
  await page.waitForFunction(() => document.activeElement?.closest("[data-popover]"));
  check("focusMovesIn", await focused(), "Read more");
  let t = await trigger.boundingBox();
  let p = await popover.boundingBox();
  check(
    "anchored",
    { below: p.y >= t.y + t.height, gap: Math.round(p.y - t.y - t.height), centred: Math.abs(centre(p) - centre(t)) < 2 },
    { below: true, gap: 8, centred: true },
  );
  check("noBorder", await popover.evaluate((el) => getComputedStyle(el).borderTopWidth), "0px");
  check("axeOpen", await axe(), []);
  await page.keyboard.press("Escape");
  check("closedByEsc", !(await isOpen(popover)));
  check("focusReturned", await focused(), "What is a preview deployment?");
  await trigger.click();
  await page.mouse.click(5, 5);
  check("closedByOutsideClick", !(await isOpen(popover)));

  await setDir("rtl");
  await trigger.click();
  await settled(popover);
  t = await trigger.boundingBox();
  p = await popover.boundingBox();
  check("rtlCentred", Math.abs(centre(p) - centre(t)) < 2);
  await page.keyboard.press("Escape");
  await setDir("ltr");

  // Opens on hover, on the right, without taking focus; closes when the pointer leaves.
  const hoverTrigger = button("Build minutes");
  const infotip = page.locator("[data-popover]", { hasText: "1,240 of 6,000" });
  await button("What is a preview deployment?").focus();
  await hoverTrigger.hover();
  await infotip.waitFor({ state: "visible" });
  await settled(infotip);
  t = await hoverTrigger.boundingBox();
  p = await infotip.boundingBox();
  check("rightSide", { gap: Math.round(p.x - t.x - t.width), middle: Math.abs(p.y + p.height / 2 - (t.y + t.height / 2)) < 2 }, { gap: 8, middle: true });
  check("hoverKeepsFocus", await focused(), "What is a preview deployment?");
  // Into the popup, across the gap: it stays.
  await page.mouse.move(p.x + 10, p.y + p.height / 2, { steps: 4 });
  await page.waitForTimeout(300);
  check("staysOverPopup", await isOpen(infotip));
  await page.mouse.move(5, 5);
  await page.waitForTimeout(400);
  check("closesOnLeave", !(await isOpen(infotip)));
  // A click after hover keeps it open.
  await hoverTrigger.hover();
  await infotip.waitFor({ state: "visible" });
  await hoverTrigger.click();
  await page.mouse.move(5, 5);
  await page.waitForTimeout(400);
  check("clickKeepsOpen", await isOpen(infotip));
  await page.keyboard.press("Escape");
  // And a click alone opens it and moves focus in, as any popover.
  await page.mouse.move(5, 5);
  await hoverTrigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.activeElement?.closest("[data-popover]"));
  check("keyboardOpens", [await isOpen(infotip), await page.evaluate(() => document.activeElement.getAttribute("role"))], [true, "dialog"]);
  await page.keyboard.press("Escape");
};
