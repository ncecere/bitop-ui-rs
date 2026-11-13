export default async ({ page, check, axe, button, focusedLabel: focused, settled, setDir }) => {
  const area = page.getByRole("group", { name: "Canvas" });
  const popup = page.locator("[data-context-menu-popup]", { hasText: "Paste in place" });
  const open = (p = popup) => p.evaluate((el) => el.matches(":popover-open"));
  // Waits, with the usual time limit, for a popover to be open or closed.
  const untilOpen = async (popup, want) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await popup.elementHandle(), want]);
  const a = await area.boundingBox();

  // Right-click opens the menu at the pointer, with focus on the menu, as upstream:
  // no item is highlighted until a key or the pointer moves to one.
  await page.mouse.click(a.x + 60, a.y + 40, { button: "right" });
  await untilOpen(popup, true);
  await settled(popup);
  let p = await popup.boundingBox();
  check("atPointer", [Math.round(p.x - a.x), Math.round(p.y - a.y)], [60, 40]);
  check("menu", await page.getByRole("menu", { name: "Canvas actions" }).count(), 1);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  check("menuFocused", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Canvas actions");
  await page.keyboard.press("ArrowDown");
  check("firstFocus", await focused(), "Copy");
  check("itemIcon", await page.evaluate(() => document.activeElement.querySelector("[data-icon]")?.dataset.icon), "copy");
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
  const report = page.locator("[data-context-menu-popup]:has([aria-label=\"Report actions\"])");
  await button("Quarterly report").focus();
  await page.keyboard.press("Shift+F10");
  await untilOpen(report, true);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menuitem");
  check("fromInnerControl", await focused(), "Open");
  await page.keyboard.press("Escape");
  await untilOpen(report, false);
  check("innerFocusReturned", await focused(), "Quarterly report");

  // Checkbox items, radio items and a submenu are the menu's, and work here.
  const layer = page.getByRole("group", { name: "Layer" });
  const layerMenu = page.locator("[data-context-menu-popup]", { hasText: "Visible" });
  const arrange = page.locator("[data-submenu]", { hasText: "Bring to front" });
  const untilFocused = (text) => page.waitForFunction((t) => document.activeElement?.textContent.trim() === t, text);
  await layer.click({ button: "right", position: { x: 30, y: 30 } });
  await untilOpen(layerMenu, true);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  await page.keyboard.press("ArrowDown");
  await untilFocused("Visible");
  const visibleItem = layerMenu.getByRole("menuitemcheckbox", { name: "Visible" });
  await page.keyboard.press("Enter");
  check("checkbox", [await visibleItem.getAttribute("aria-checked"), await open(layerMenu)], ["false", true]);
  await layerMenu.getByRole("menuitemradio", { name: "Multiply" }).click();
  check("radio", [await layerMenu.locator("[role=menuitemradio]").evaluateAll((all) => all.map((r) => r.getAttribute("aria-checked"))), await open(layerMenu)], [["false", "true"], true]);
  check("group", await layerMenu.getByRole("group", { name: "Order" }).count(), 1);
  await page.keyboard.press("End");
  check("submenuItem", await focused(), "Arrange");
  await page.keyboard.press("ArrowRight");
  await untilOpen(arrange, true);
  await untilFocused("Bring to front");
  await settled(arrange);
  const arrangeItem = await layerMenu.getByRole("menuitem", { name: "Arrange" }).boundingBox();
  const arrangeBox = await arrange.boundingBox();
  check("submenuBeside", { beside: arrangeBox.x >= arrangeItem.x + arrangeItem.width - 1, level: Math.abs(arrangeBox.y - arrangeItem.y) < 8 }, { beside: true, level: true });
  check("axeSubmenu", await axe(), []);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(arrange, false);
  check("leftClosesSubmenu", [await open(layerMenu), await focused()], [true, "Arrange"]);
  await page.keyboard.press("Enter");
  await untilOpen(arrange, true);
  await untilFocused("Bring to front");
  await page.keyboard.press("Enter");
  await untilOpen(layerMenu, false);
  await page.waitForFunction(() => !document.querySelector("[data-popup-open]"));
  check("chooseInSubmenuClosesAll", [await open(arrange), await layer.evaluate((el) => el.hasAttribute("data-popup-open"))], [false, false]);

  // Upstream's file row: icons, shortcut hints, a link, a submenu with an icon, danger.
  const file = page.locator("[data-context-menu]", { hasText: "quarterly-report.pdf" });
  const fileMenu = page.locator("[data-context-menu-popup]", { hasText: "Move to trash" });
  await file.click({ button: "right", position: { x: 40, y: 10 } });
  await untilOpen(fileMenu, true);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  check("fileItems", await fileMenu.locator(":scope > [role=menu] > [role=menuitem]").evaluateAll((all) => all.map((el) => [el.querySelector("[data-icon]")?.dataset.icon, el.querySelector("[aria-hidden=true]:not([data-icon])")?.textContent ?? ""].join(" ").trim())), [
    "pencil F2", "copy ⌘D", "download", "share-2", "trash-2 ⌫",
  ]);
  check("fileDanger", await fileMenu.getByRole("menuitem", { name: "Move to trash" }).getAttribute("data-tone"), "danger");
  await page.keyboard.press("End");
  check("fileEnd", await focused(), "Move to trash");
  await page.keyboard.press("Escape");
  await untilOpen(fileMenu, false);

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
