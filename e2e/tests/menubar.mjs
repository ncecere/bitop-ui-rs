export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const bar = page.getByRole("menubar", { name: "Editor" });
  const item = (name) => bar.getByRole("menuitem", { name, exact: true });
  const popup = (text) => page.locator("[data-menubar-popup]", { hasText: text });
  const fileMenu = popup("New file");
  const editMenu = popup("Undo");
  const viewMenu = popup("Zoom in");
  const helpMenu = popup("Documentation");
  const open = (p) => p.evaluate((el) => el.matches(":popover-open"));
  const untilOpen = async (p, want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await p.elementHandle(), want]);
  // Focus moves into a menu a moment after it opens.
  const untilFocused = (text) => page.waitForFunction((t) => document.activeElement?.textContent.trim() === t, text);
  const stops = () => bar.locator("[data-menubar-trigger][tabindex='0']").allTextContents();

  // The bar is one tab stop, and its triggers are menu items with a popup.
  check("roles", await bar.locator("> [role=menuitem]").evaluateAll((all) => all.map((b) => [b.textContent.trim(), b.getAttribute("aria-haspopup"), b.getAttribute("aria-expanded")].join(" "))), [
    "File menu false", "Edit menu false", "View menu false", "Window menu false", "Help menu false",
  ]);
  check("oneTabStop", await stops(), ["File"]);
  await button("Toggle sidebar").focus();
  for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
  check("tabIntoBar", await focused(), "File");
  await page.keyboard.press("Tab");
  check("tabLeavesBar", await page.evaluate(() => !document.activeElement.closest("[data-menubar]") || document.activeElement.closest("[aria-label=Tools]") !== null));
  await page.keyboard.press("Shift+Tab");
  check("shiftTabReturns", await focused(), "File");

  // Arrow keys move along the bar, wrap, and skip a disabled menu.
  await page.keyboard.press("ArrowRight");
  check("right", [await focused(), await stops()], ["Edit", ["Edit"]]);
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowRight");
  check("skipsDisabled", await focused(), "Help");
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
  check("anchored", { below: p.y >= t.y + t.height, gap: Math.round(p.y - t.y - t.height), aligned: Math.abs(p.x - t.x) < 2 }, { below: true, gap: 6, aligned: true });
  check("menuNamed", await page.getByRole("menu", { name: "File" }).count(), 1);
  check("axeOpen", await axe(), []);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("menuSkipsDisabled", await focused(), "Close");
  await page.keyboard.press("Escape");
  await untilOpen(fileMenu, false);
  check("escReturnsFocus", [await focused(), await item("File").getAttribute("aria-expanded")], ["File", "false"]);
  await page.keyboard.press("ArrowUp");
  await untilOpen(fileMenu);
  await untilFocused("Close");
  check("upOpensLast", await focused(), "Close");

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
  check("wrapsPastDisabled", await focused(), "Documentation");
  await page.keyboard.press("Escape");
  await untilOpen(helpMenu, false);
  check("escAfterMoving", await focused(), "Help");

  // Enter and Space open; choosing an item closes and returns focus.
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Enter");
  await untilOpen(editMenu);
  await untilFocused("Undo");
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

  // The pointer: a click opens, and with one menu open, pointing at another opens it.
  await item("View").click();
  await untilOpen(viewMenu);
  check("clickOpens", await item("View").getAttribute("aria-expanded"), "true");
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
  check("disabledMenu", await item("Window").isDisabled());

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
  check("rtlAligned", Math.abs(rp.x + rp.width - (rt.x + rt.width)) < 2);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(editMenu);
  check("rtlLeftOpensNext", await open(editMenu));
  await page.keyboard.press("Escape");
};
