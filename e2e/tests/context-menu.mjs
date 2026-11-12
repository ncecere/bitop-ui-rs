export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const area = page.getByRole("group", { name: "Canvas" });
  const popup = page.locator("[data-context-menu-popup]", { hasText: "Duplicate" });
  const open = (p = popup) => p.evaluate((el) => el.matches(":popover-open"));
  // Waits, with the usual time limit, for a popover to be open or closed.
  const untilOpen = async (popup, want) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await popup.elementHandle(), want]);
  const a = await area.boundingBox();

  // Right-click opens the menu at the pointer, with focus on the first item.
  await page.mouse.click(a.x + 60, a.y + 40, { button: "right" });
  await untilOpen(popup, true);
  await settled(popup);
  let p = await popup.boundingBox();
  check("atPointer", [Math.round(p.x - a.x), Math.round(p.y - a.y)], [60, 40]);
  check("menu", await page.getByRole("menu", { name: "Canvas actions" }).count(), 1);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitem");
  check("firstFocus", await focused(), "Copy");
  check("triggerMarked", await area.evaluate((el) => el.hasAttribute("data-popup-open")));
  check("axeOpen", await axe(), []);

  // The menu's keyboard pattern.
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("skipsDisabled", await focused(), "Duplicate");
  await page.keyboard.press("End");
  check("end", await focused(), "Delete");
  await page.keyboard.press("c");
  check("typeahead", await focused(), "Copy");
  await page.keyboard.press("Escape");
  await untilOpen(popup, false);
  check("closedByEsc", await open(), false);
  await page.waitForFunction(() => !document.querySelector("[data-popup-open]"));
  check("triggerUnmarked", await area.evaluate((el) => el.hasAttribute("data-popup-open")), false);

  // Right-clicking again moves it; choosing an item closes it.
  await page.mouse.click(a.x + 60, a.y + 40, { button: "right" });
  await untilOpen(popup, true);
  await page.mouse.click(a.x + 20, a.y + 20, { button: "right" });
  await page.waitForFunction((x) => Math.round(document.querySelector("[data-context-menu-popup]:popover-open")?.getBoundingClientRect().x) === Math.round(x), a.x + 20);
  await settled(popup);
  p = await popup.boundingBox();
  check("moves", [Math.round(p.x - a.x), Math.round(p.y - a.y)], [20, 20]);
  await popup.getByRole("menuitem", { name: "Duplicate" }).click();
  check("closedOnSelect", await open(), false);
  await page.mouse.click(a.x + 20, a.y + 20, { button: "right" });
  await untilOpen(popup, true);
  await page.mouse.click(5, 5);
  check("closedByClickOutside", await open(), false);

  // Near the edge of the window the menu opens on the other side of the pointer.
  const view = page.viewportSize();
  await area.evaluate((el) => {
    el.style.position = "fixed";
    el.style.right = "0";
    el.style.bottom = "0";
  });
  const edge = await area.boundingBox();
  await page.mouse.click(view.width - 10, view.height - 10, { button: "right" });
  await untilOpen(popup, true);
  await settled(popup);
  p = await popup.boundingBox();
  check("flipsAtEdge", [Math.round(p.x + p.width), Math.round(p.y + p.height)], [view.width - 10, view.height - 10]);
  check("edgeArea", edge.x + edge.width > view.width - 1);
  await page.keyboard.press("Escape");
  await area.evaluate((el) => {
    el.style.position = "";
    el.style.right = "";
    el.style.bottom = "";
  });

  // From the keyboard: Shift+F10 or the context-menu key, at the focused element.
  await area.focus();
  await page.keyboard.press("Shift+F10");
  await untilOpen(popup, true);
  await settled(popup);
  p = await popup.boundingBox();
  const now = await area.boundingBox();
  check("keyboardAtElement", [Math.round(p.x - now.x - now.width / 2), Math.round(p.y - now.y - now.height / 2)], [0, 0]);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitem");
  check("keyboardFocus", await focused(), "Copy");
  await page.keyboard.press("Escape");
  await untilOpen(popup, false);
  check("focusReturned", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Canvas");
  await page.keyboard.press("ContextMenu");
  await untilOpen(popup, true);
  check("contextMenuKey", await open());
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitem");
  await page.keyboard.press("Enter");
  check("enterChooses", await open(), false);

  // Around a control: the key works from the control inside the area.
  const report = page.locator("[data-context-menu-popup]", { hasText: "Rename" });
  await button("Quarterly report").focus();
  await page.keyboard.press("Shift+F10");
  await untilOpen(report, true);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitem");
  check("fromInnerControl", await focused(), "Open");
  await page.keyboard.press("Escape");
  await untilOpen(report, false);
  check("innerFocusReturned", await focused(), "Quarterly report");

  // Disabled: nothing of ours opens, and the event is left to the browser.
  const prevented = await page.getByText("Nothing of ours opens here").evaluate((el) => {
    const event = new MouseEvent("contextmenu", { bubbles: true, cancelable: true });
    el.dispatchEvent(event);
    return event.defaultPrevented;
  });
  check("disabled", [prevented, await page.locator("[data-context-menu-popup]:popover-open").count()], [false, 0]);

  // Right to left: the menu opens to the left of the pointer.
  await setDir("rtl");
  const r = await area.boundingBox();
  await page.mouse.click(r.x + 260, r.y + 40, { button: "right" });
  await untilOpen(popup, true);
  await settled(popup);
  p = await popup.boundingBox();
  check("rtlOpensToTheLeft", [Math.round(p.x + p.width - r.x), Math.round(p.y - r.y)], [260, 40]);
  await page.keyboard.press("Escape");
};
