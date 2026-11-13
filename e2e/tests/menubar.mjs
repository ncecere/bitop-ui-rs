export default async ({ page, check, axe, button, focusedLabel: focused, settled, setDir }) => {
  const bar = page.getByRole("menubar", { name: "Editor" });
  const item = (name) => bar.getByRole("menuitem", { name, exact: true });
  const popup = (text) => page.locator("[data-menubar-popup]", { hasText: text });
  const fileMenu = popup("New file");
  const editMenu = popup("Undo");
  const viewMenu = popup("Word wrap");
  const helpMenu = popup("Documentation");
  const open = (p) => p.evaluate((el) => el.matches(":popover-open"));
  const untilOpen = async (p, want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await p.elementHandle(), want]);
  // Focus moves into a menu a moment after it opens.
  const untilFocused = (text) =>
    page.waitForFunction((t) => {
      const el = document.activeElement?.cloneNode(true);
      if (!el) return false;
      for (const hidden of el.querySelectorAll('[aria-hidden="true"]')) hidden.remove();
      return el.textContent.trim() === t;
    }, text);
  const stops = () => bar.locator("[data-menubar-trigger][tabindex='0']").allTextContents();

  // The bar is one tab stop, and its triggers are menu items with a popup.
  check("roles", await bar.locator("> [role=menuitem]").evaluateAll((all) => all.map((b) => [b.textContent.trim(), b.getAttribute("aria-haspopup"), b.getAttribute("aria-expanded")].join(" "))), [
    "File menu false", "Edit menu false", "View menu false", "Help menu false",
  ]);
  check("oneTabStop", await stops(), ["File"]);
  await button("Collapse sidebar").focus();
  for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
  check("tabIntoBar", await focused(), "File");
  await page.keyboard.press("Tab");
  check("tabLeavesBar", await page.evaluate(() => !document.activeElement.closest("[data-menubar]") || document.activeElement.closest("[aria-label=Tools]") !== null));
  await page.keyboard.press("Shift+Tab");
  check("shiftTabReturns", await focused(), "File");

  // Arrow keys move along the bar and wrap.
  await page.keyboard.press("ArrowRight");
  check("right", [await focused(), await stops()], ["Edit", ["Edit"]]);
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowRight");
  check("toLast", await focused(), "Help");
  await page.keyboard.press("ArrowRight");
  check("wraps", await focused(), "File");
  await page.keyboard.press("ArrowLeft");
  check("left", await focused(), "Help");
  await page.keyboard.press("Home");
  check("home", await focused(), "File");
  await page.keyboard.press("End");
  check("end", await focused(), "Help");
  await page.keyboard.press("v");
  check("letter", await focused(), "View");
  await page.keyboard.press("Home");

  // ArrowDown opens on the first item, ArrowUp on the last.
  await page.keyboard.press("ArrowDown");
  await untilOpen(fileMenu);
  await untilFocused("New file");
  check("downOpensFirst", [await focused(), await item("File").getAttribute("aria-expanded")], ["New file", "true"]);
  await settled(fileMenu);
  const t = await item("File").boundingBox();
  const p = await fileMenu.boundingBox();
  // Upstream's alignOffset: the popup starts 4px before the button, so that the items'
  // text lines up with the button's.
  check("anchored", { below: p.y >= t.y + t.height, gap: Math.round(p.y - t.y - t.height), offset: Math.round(p.x - t.x) }, { below: true, gap: 6, offset: -4 });
  check("menuNamed", await page.getByRole("menu", { name: "File" }).count(), 1);
  check("axeOpen", await axe(), []);
  check("shortcutHint", await page.evaluate(() => document.activeElement.querySelector("[aria-hidden=true]:not([data-icon])")?.textContent), "⌘N");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("menuSubmenuItem", await focused(), "Open recent");
  await page.keyboard.press("Escape");
  await untilOpen(fileMenu, false);
  check("escReturnsFocus", [await focused(), await item("File").getAttribute("aria-expanded")], ["File", "false"]);
  await page.keyboard.press("ArrowUp");
  await untilOpen(fileMenu);
  await untilFocused("Close editor");
  check("upOpensLast", [await focused(), await page.evaluate(() => document.activeElement.dataset.tone)], ["Close editor", "danger"]);

  // Inside an open menu, the bar's arrows open the neighbouring menus.
  await page.keyboard.press("ArrowRight");
  await untilOpen(editMenu);
  await untilFocused("Undo");
  check("rightOpensNext", [await open(fileMenu), await focused(), await stops()], [false, "Undo", ["Edit"]]);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(fileMenu);
  await untilFocused("New file");
  check("leftOpensPrevious", [await open(editMenu), await focused()], [false, "New file"]);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(helpMenu);
  await untilFocused("Documentation");
  check("wrapsToLast", await focused(), "Documentation");
  await page.keyboard.press("Escape");
  await untilOpen(helpMenu, false);
  check("escAfterMoving", await focused(), "Help");

  // Enter and Space open; choosing an item closes and returns focus.
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Enter");
  await untilOpen(editMenu);
  await untilFocused("Undo");
  await page.keyboard.press("ArrowDown");
  check("menuSkipsDisabled", await focused(), "Cut");
  await page.keyboard.press("End");
  check("menuEnd", await focused(), "Paste");
  await page.keyboard.press("Enter");
  await untilOpen(editMenu, false);
  check("chooseCloses", await focused(), "Edit");
  await page.keyboard.press("Space");
  await untilOpen(editMenu);
  await untilFocused("Undo");
  // Tab closes the menu and leaves the bar.
  await page.keyboard.press("Tab");
  check("tabClosesAndLeaves", [await open(editMenu), await page.evaluate(() => !!document.activeElement.closest("[aria-label=Editor]"))], [false, false]);

  // The pointer: a click opens, with focus on the menu rather than an item, and with
  // one menu open, pointing at another opens it.
  await item("View").click();
  await untilOpen(viewMenu);
  check("clickOpens", await item("View").getAttribute("aria-expanded"), "true");
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  check("clickFocusesMenu", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "View");
  await item("Help").hover();
  await untilOpen(helpMenu);
  check("hoverSwitches", [await open(viewMenu), await item("Help").getAttribute("aria-expanded")], [false, "true"]);
  await page.waitForFunction(() => document.querySelector("[data-menubar-trigger][aria-expanded=true]")?.textContent.trim() === "Help" && document.querySelectorAll("[data-menubar-trigger][aria-expanded=true]").length === 1);
  check("oneExpanded", await bar.locator("[aria-expanded=true]").allTextContents(), ["Help"]);
  // Escape returns focus to the trigger even when a click, which in Safari does not
  // focus a button, opened the menu.
  await page.keyboard.press("Escape");
  check("escAfterPointer", [await open(helpMenu), await focused()], [false, "Help"]);
  await item("Help").click();
  await untilOpen(helpMenu);
  await item("Help").click();
  await untilOpen(helpMenu, false);
  check("clickCloses", await open(helpMenu), false);
  await item("File").hover();
  check("hoverAloneDoesNotOpen", await open(fileMenu), false);
  // A pointer moving along the bar, as a person's does, switches at each menu.
  const centre = async (locator) => {
    const box = await locator.boundingBox();
    return [box.x + box.width / 2, box.y + box.height / 2];
  };
  await page.mouse.move(...(await centre(item("File"))));
  await page.mouse.down();
  await page.mouse.up();
  await untilOpen(fileMenu);
  await page.mouse.move(...(await centre(fileMenu.getByRole("menuitem", { name: "Open…" }))), { steps: 5 });
  await page.mouse.move(...(await centre(item("Edit"))), { steps: 10 });
  await untilOpen(editMenu);
  await page.mouse.move(...(await centre(item("View"))), { steps: 10 });
  await untilOpen(viewMenu);
  check("pointerSwitches", [await open(fileMenu), await open(editMenu), await open(viewMenu)], [false, false, true]);
  await page.keyboard.press("Escape");
  await untilOpen(viewMenu, false);
  check("disabledMenu", await page.getByRole("menubar", { name: "Document" }).getByRole("menuitem", { name: "Window" }).isDisabled());

  // Vertical: up and down move, the arrow that points at the popup opens it.
  const tools = page.getByRole("menubar", { name: "Tools" });
  const drawMenu = popup("Pencil");
  check("vertical", await tools.getAttribute("aria-orientation"), "vertical");
  await tools.getByRole("menuitem", { name: "Draw" }).focus();
  await page.keyboard.press("ArrowDown");
  check("verticalDown", await focused(), "Text");
  await page.keyboard.press("ArrowUp");
  check("verticalUp", await focused(), "Draw");
  await page.keyboard.press("ArrowRight");
  await untilOpen(drawMenu);
  await untilFocused("Pencil");
  await settled(drawMenu);
  const d = await tools.getByRole("menuitem", { name: "Draw" }).boundingBox();
  const dp = await drawMenu.boundingBox();
  check("verticalPopupBeside", dp.x >= d.x + d.width);
  await page.keyboard.press("ArrowLeft");
  check("verticalArrowCloses", [await open(drawMenu), await focused()], [false, "Draw"]);

  // Checkbox items, radio items and submenus are the menu's, and work in a bar.
  const doc = page.getByRole("menubar", { name: "Document" });
  const docItem = (name) => doc.getByRole("menuitem", { name, exact: true });
  const formatMenu = popup("Wrap lines");
  const insertMenu = popup("Footnote");
  const reviewMenu = popup("Spelling");
  const tableMenu = page.locator("[data-submenu]", { hasText: "Two columns" });
  await docItem("Format").focus();
  await page.keyboard.press("ArrowDown");
  await untilOpen(formatMenu);
  await untilFocused("Wrap lines");
  const wrap = formatMenu.getByRole("menuitemcheckbox", { name: "Wrap lines" });
  await page.keyboard.press("Space");
  check("checkboxInBar", [await wrap.getAttribute("aria-checked"), await open(formatMenu), await docItem("Format").getAttribute("aria-expanded")], ["false", true, "true"]);
  await page.keyboard.press("End");
  check("radioIsLastItem", await focused(), "Tabs");
  await page.keyboard.press("Enter");
  check("radioInBar", [await formatMenu.locator("[role=menuitemradio]").evaluateAll((all) => all.map((r) => r.getAttribute("aria-checked"))), await open(formatMenu)], [["false", "true"], true]);
  check("groupInBar", await formatMenu.getByRole("group", { name: "Indent with" }).count(), 1);
  await page.keyboard.press("Escape");
  await untilOpen(formatMenu, false);
  await page.keyboard.press("ArrowUp");
  await untilOpen(formatMenu);
  await untilFocused("Tabs");
  check("upOpensOnLastOfAnyKind", await focused(), "Tabs");
  // ArrowRight on a submenu's item opens the submenu; on any other item, the next menu.
  await page.keyboard.press("ArrowRight");
  await untilOpen(insertMenu);
  await untilFocused("Image");
  await page.keyboard.press("ArrowDown");
  check("submenuItemInBar", [await focused(), await insertMenu.getByRole("menuitem", { name: "Table" }).getAttribute("aria-haspopup")], ["Table", "menu"]);
  await page.keyboard.press("ArrowRight");
  await untilOpen(tableMenu);
  await untilFocused("Two columns");
  check("rightOpensSubmenu", [await open(insertMenu), await open(reviewMenu), await focused()], [true, false, "Two columns"]);
  await settled(tableMenu);
  check("axeSubmenu", await axe(), []);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(tableMenu, false);
  check("leftClosesSubmenu", [await open(insertMenu), await open(formatMenu), await focused()], [true, false, "Table"]);
  await page.keyboard.press("ArrowRight");
  await untilOpen(tableMenu);
  await untilFocused("Two columns");
  await page.keyboard.press("Escape");
  await untilOpen(tableMenu, false);
  check("escClosesSubmenuOnly", [await open(insertMenu), await focused()], [true, "Table"]);
  await page.keyboard.press("ArrowRight");
  await untilOpen(tableMenu);
  await untilFocused("Two columns");
  await page.keyboard.press("ArrowRight");
  await untilOpen(reviewMenu);
  await untilFocused("Spelling");
  check("rightInSubmenuOpensNextMenu", [await open(insertMenu), await open(tableMenu), await focused()], [false, false, "Spelling"]);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(insertMenu);
  await untilFocused("Image");
  await page.keyboard.press("End");
  check("endInBarMenu", await focused(), "Footnote");
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("Enter");
  await untilOpen(tableMenu);
  await untilFocused("Two columns");
  await page.keyboard.press("Enter");
  await untilOpen(insertMenu, false);
  check("chooseInSubmenuCloses", [await open(tableMenu), await focused()], [false, "Insert"]);
  await docItem("Insert").click();
  await untilOpen(insertMenu);
  await insertMenu.getByRole("menuitem", { name: "Table" }).hover();
  await untilOpen(tableMenu);
  check("hoverOpensSubmenu", await open(insertMenu));
  await page.keyboard.press("Escape");
  await untilOpen(insertMenu, false);

  // Right to left: the arrows follow the visual order, and the popup aligns to the right.
  await setDir("rtl");
  await item("File").focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlLeftIsNext", await focused(), "Edit");
  await page.keyboard.press("ArrowRight");
  check("rtlRightIsPrevious", await focused(), "File");
  await page.keyboard.press("ArrowDown");
  await untilOpen(fileMenu);
  await untilFocused("New file");
  await settled(fileMenu);
  const rt = await item("File").boundingBox();
  const rp = await fileMenu.boundingBox();
  check("rtlAligned", Math.round(rp.x + rp.width - (rt.x + rt.width)), 4);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(editMenu);
  check("rtlLeftOpensNext", await open(editMenu));
  await page.keyboard.press("Escape");
  await untilOpen(editMenu, false);
  await docItem("Insert").focus();
  await page.keyboard.press("ArrowDown");
  await untilOpen(insertMenu);
  await untilFocused("Image");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowLeft");
  await untilOpen(tableMenu);
  await untilFocused("Two columns");
  check("rtlLeftOpensSubmenu", [await open(insertMenu), await focused()], [true, "Two columns"]);
  await page.keyboard.press("ArrowRight");
  await untilOpen(tableMenu, false);
  check("rtlRightClosesSubmenu", [await open(insertMenu), await focused()], [true, "Table"]);
  await page.keyboard.press("Escape");
};
