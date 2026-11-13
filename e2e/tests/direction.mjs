export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const rtl = page.locator('[data-demo="rtl"]');
  const ltr = page.locator('[data-demo="ltr"]');
  const dir = (locator) => locator.evaluate((el) => getComputedStyle(el).direction);
  const untilOpen = async (p, want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await p.elementHandle(), want]);
  const untilFocused = (text) => page.waitForFunction((t) => document.activeElement?.textContent.trim() === t, text);

  // The part is a <div> with dir, and lang when given.
  check("attributes", [await rtl.getAttribute("dir"), await rtl.getAttribute("lang"), await ltr.getAttribute("dir"), await ltr.getAttribute("lang")], ["rtl", "ar", "ltr", null]);
  check("computed", [await dir(rtl), await dir(ltr), await dir(page.locator("main"))], ["rtl", "ltr", "ltr"]);
  check("auto", [await dir(page.locator('[data-demo="auto-rtl"]')), await dir(page.locator('[data-demo="auto-ltr"]'))], ["rtl", "ltr"]);

  // Inside the right-to-left section the layout mirrors: the first tab is on the right.
  const tabs = rtl.getByRole("tab");
  const first = await tabs.nth(0).boundingBox();
  const second = await tabs.nth(1).boundingBox();
  const section = await rtl.boundingBox();
  check("rtlTabsMirror", [first.x > second.x, Math.round(section.x + section.width - first.x - first.width) < 40], [true, true]);
  const text = await rtl.locator("p").first().evaluate((el) => getComputedStyle(el).textAlign);
  check("rtlTextStart", text, "start");

  // And so do the arrow keys, which scripts read from the computed direction. They move
  // focus; the tabs are activated by hand, as upstream's.
  const selected = () => page.evaluate(() => document.activeElement.id);
  await tabs.nth(0).focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlLeftIsNext", await selected(), "dir-models-tab");
  await page.keyboard.press("ArrowRight");
  check("rtlRightIsPrevious", await selected(), "dir-general-tab");
  await ltr.getByRole("tab").nth(0).focus();
  await page.keyboard.press("ArrowRight");
  check("ltrRightIsNext", await selected(), "dir-en-models-tab");

  // A popup belongs to its section: a submenu opens with ArrowLeft, on the left.
  const menu = rtl.locator("[popover]").first();
  const submenu = rtl.locator("[data-submenu]");
  await rtl.getByRole("button", { name: "خيارات" }).focus();
  await page.keyboard.press("Enter");
  await untilOpen(menu);
  await untilFocused("إعادة تسمية");
  check("popupDirection", await dir(menu), "rtl");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowRight");
  check("rtlRightDoesNotOpenSubmenu", await submenu.evaluate((el) => el.matches(":popover-open")), false);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(submenu);
  await untilFocused("أرشفة");
  await settled(submenu);
  const item = await menu.getByRole("menuitem", { name: "المزيد" }).boundingBox();
  const sub = await submenu.boundingBox();
  check("rtlSubmenuOnTheLeft", sub.x + sub.width <= item.x + 1);
  check("axeOpen", await axe(), []);
  await page.keyboard.press("ArrowRight");
  await untilOpen(submenu, false);
  await page.keyboard.press("Escape");
  await untilOpen(menu, false);

  // A right-to-left page does not change a section that says left to right.
  await setDir("rtl");
  const a = await ltr.getByRole("tab").nth(0).boundingBox();
  const b = await ltr.getByRole("tab").nth(1).boundingBox();
  check("ltrSectionInRtlPage", [await dir(ltr), a.x < b.x, await dir(page.locator("main"))], ["ltr", true, "rtl"]);
  await ltr.getByRole("tab").nth(1).focus();
  await page.keyboard.press("ArrowLeft");
  check("ltrKeysInRtlPage", await selected(), "dir-en-general-tab");
};
