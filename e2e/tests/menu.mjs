export default async ({ page, check, axe, button, focusedLabel: focused, settled, setDir }) => {
  const actions = page.locator("[popover]", { hasText: "View audit log" });
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
  const trigger = button("Actions");

  // The trigger says whether the menu is open; Enter opens on the first item.
  check("triggerClosed", [await trigger.getAttribute("aria-haspopup"), await trigger.getAttribute("aria-expanded")], ["menu", "false"]);
  await trigger.focus();
  await page.keyboard.press("Enter");
  await untilOpen(actions);
  await untilFocused("Rename");
  check("firstFocus", [await focused(), await trigger.getAttribute("aria-expanded")], ["Rename", "true"]);
  await settled(actions);
  // Upstream's rows: 32px high, 8px each side, the base text size, an icon of 16px.
  const rename = actions.getByRole("menuitem", { name: "Rename" });
  check("row", await rename.evaluate((el) => {
    const style = getComputedStyle(el);
    const icon = el.querySelector("[data-icon]").getBoundingClientRect();
    return [Math.round(el.getBoundingClientRect().height), style.paddingInlineStart, style.paddingInlineEnd, style.fontSize, Math.round(icon.width)];
  }), [32, "8px", "8px", "14px", 16]);
  check("shortcutHidden", await rename.locator("[aria-hidden=true]").last().textContent(), "R");
  check("nameWithoutShortcut", await page.getByRole("menuitem", { name: "Rename", exact: true }).count(), 1);
  const t = await trigger.boundingBox();
  const p = await actions.boundingBox();
  check("anchored", { gap: Math.round(p.y - t.y - t.height), aligned: Math.abs(p.x - t.x) < 2 }, { gap: 6, aligned: true });
  await page.keyboard.press("ArrowDown");
  check("down", await focused(), "Share");
  await page.keyboard.press("ArrowDown");
  check("link", [await focused(), await page.evaluate(() => document.activeElement.tagName)], ["View audit log", "A"]);
  await page.keyboard.press("ArrowDown");
  check("skipsDisabled", await focused(), "Delete");
  check("danger", await page.evaluate(() => document.activeElement.dataset.tone), "danger");
  await page.keyboard.press("ArrowDown");
  check("wraps", await focused(), "Rename");
  await page.keyboard.press("ArrowUp");
  check("wrapsUp", await focused(), "Delete");
  await page.keyboard.press("Home");
  check("home", await focused(), "Rename");
  await page.keyboard.press("End");
  check("end", await focused(), "Delete");
  await page.keyboard.press("s");
  check("typeahead", await focused(), "Share");
  check("axeOpen", await axe(), []);
  await page.keyboard.press("Enter");
  await untilOpen(actions, false);
  check("closedOnSelect", [await open(actions), await trigger.getAttribute("aria-expanded")], [false, "false"]);
  check("focusReturned", await focused(), "Actions");

  // ArrowDown on the trigger opens on the first item, ArrowUp on the last.
  await page.keyboard.press("ArrowDown");
  await untilOpen(actions);
  await untilFocused("Rename");
  check("arrowDownOpens", await focused(), "Rename");
  await page.keyboard.press("Escape");
  await untilOpen(actions, false);
  check("escReturnsFocus", await focused(), "Actions");
  await page.keyboard.press("ArrowUp");
  await untilOpen(actions);
  await untilFocused("Delete");
  check("arrowUpOpensOnLast", await focused(), "Delete");
  await page.keyboard.press("Escape");
  await untilOpen(actions, false);

  // The pointer: a click focuses the menu, not an item; pointing moves the highlight.
  await trigger.click();
  await untilOpen(actions);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  check("clickFocusesMenu", await page.evaluate(() => document.activeElement.getAttribute("role")), "menu");
  await page.keyboard.press("ArrowDown");
  check("keysAfterClick", await focused(), "Rename");
  await actions.getByRole("menuitem", { name: "Share" }).hover();
  check("hoverHighlights", await focused(), "Share");
  check("oneHighlight", await actions.locator("[role=menuitem]").evaluateAll((all) => all.filter((el) => getComputedStyle(el).backgroundColor !== "rgba(0, 0, 0, 0)").length), 1);
  const disabled = actions.getByRole("menuitem", { name: "Settings (no access)" });
  check("disabledLook", await disabled.evaluate((el) => getComputedStyle(el).cursor), "not-allowed");
  await disabled.click({ force: true });
  check("disabledDoesNothing", await open(actions), true);
  await page.mouse.move(5, 5);
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "menu");
  check("leavingClearsHighlight", await page.evaluate(() => document.activeElement.getAttribute("role")), "menu");
  await page.keyboard.press("Escape");
  await untilOpen(actions, false);

  // Checkbox items, radio items and a submenu, in a form.
  const view = page.locator("[popover]", { hasText: "Comfortable" });
  const more = page.locator("[data-submenu]", { hasText: "Reset columns" });
  const item = (role, name) => view.getByRole(role, { name, exact: true });
  const formData = () => page.locator("[data-menu-form]").evaluate((form) => [...new FormData(form)].map((pair) => pair.join("=")));
  await page.evaluate(() => {
    window.menuChanges = [];
    document.addEventListener("change", (event) => window.menuChanges.push(event.target.textContent.trim()));
  });

  check("submitsInitially", await formData(), ["status=on", "branch=on", "density=comfortable"]);
  await button("View").focus();
  await page.keyboard.press("Enter");
  await untilOpen(view);
  await untilFocused("Status");
  check("checkboxRole", await item("menuitemcheckbox", "Status").getAttribute("aria-checked"), "true");
  await page.keyboard.press("Space");
  check("spaceUnchecks", [await item("menuitemcheckbox", "Status").getAttribute("aria-checked"), await open(view)], ["false", true]);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  check("enterChecks", [await item("menuitemcheckbox", "Author").getAttribute("aria-checked"), await open(view)], ["true", true]);
  check("radioGroup", await view.getByRole("group", { name: "Density" }).getByRole("menuitemradio").count(), 2);
  const radios = () => view.locator("[role=menuitemradio]").evaluateAll((all) => all.map((r) => r.getAttribute("aria-checked")));
  check("radioInitial", await radios(), ["true", "false"]);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("toRadio", await focused(), "Compact");
  await page.keyboard.press("Enter");
  check("radioChooses", [await radios(), await open(view)], [["false", "true"], true]);
  await page.keyboard.press("Enter");
  check("radioStaysChecked", await radios(), ["false", "true"]);
  await item("menuitemradio", "Comfortable").click();
  check("radioClick", [await radios(), await open(view)], [["true", "false"], true]);
  check("changeEvents", await page.evaluate(() => window.menuChanges), ["Status", "Author", "Compact", "Comfortable"]);
  check("submits", await formData(), ["branch=on", "author=on", "density=comfortable"]);
  check("groupNamed", await view.getByRole("group", { name: "Columns" }).getByRole("menuitemcheckbox").count(), 3);

  // A submenu with an icon: the arrow key that points at it opens it, the other closes it.
  const moreItem = item("menuitem", "More options");
  await page.keyboard.press("End");
  check("submenuItem", [await focused(), await moreItem.getAttribute("aria-haspopup"), await moreItem.getAttribute("aria-expanded")], ["More options", "menu", "false"]);
  check("submenuIcon", await moreItem.locator("[data-icon]").first().getAttribute("data-icon"), "sliders-horizontal");
  await page.keyboard.press("ArrowRight");
  await untilOpen(more);
  await untilFocused("Reset columns");
  check("rightOpens", [await moreItem.getAttribute("aria-expanded"), await open(view)], ["true", true]);
  await settled(more);
  let mt = await moreItem.boundingBox();
  let mp = await more.boundingBox();
  check("submenuBeside", { beside: mp.x >= mt.x + mt.width - 1, near: mp.x - mt.x - mt.width < 12, level: Math.abs(mp.y - mt.y) < 8 }, { beside: true, near: true, level: true });
  check("submenuNamed", await page.getByRole("menu", { name: "More options" }).count(), 1);
  check("axeSubmenu", await axe(), []);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("submenuWrapsInItself", await focused(), "Reset columns");
  await page.keyboard.press("ArrowLeft");
  await untilOpen(more, false);
  check("leftCloses", [await focused(), await open(view)], ["More options", true]);
  await page.keyboard.press("Enter");
  await untilOpen(more);
  await untilFocused("Reset columns");
  await page.keyboard.press("Escape");
  await untilOpen(more, false);
  check("escClosesOneLevel", [await focused(), await open(view)], ["More options", true]);
  await page.keyboard.press("Enter");
  await untilOpen(more);
  await untilFocused("Reset columns");
  await page.keyboard.press("Enter");
  await untilOpen(view, false);
  await untilFocused("View");
  check("chooseClosesAll", [await open(more), await focused()], [false, "View"]);

  // A header, nested submenus and disabled items.
  const account = page.locator("[popover]", { hasText: "ada@example.com" });
  const exportMenu = page.locator("[data-submenu]", { hasText: "Markdown" });
  const imageMenu = page.locator("[data-submenu]", { hasText: "PNG" }).last();
  const exportItem = account.getByRole("menuitem", { name: "Export as" });
  await button("Account").focus();
  await page.keyboard.press("Enter");
  await untilOpen(account);
  await untilFocused("Export as");
  check("headerIsNotAnItem", await focused(), "Export as");
  check("header", await account.locator("strong").textContent(), "Ada Lovelace");
  await page.keyboard.press("End");
  check("endSkipsDisabledSubmenu", await focused(), "Home");
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("ArrowRight");
  await untilOpen(exportMenu);
  await untilFocused("PDF");
  await page.keyboard.press("End");
  await page.keyboard.press("ArrowRight");
  await untilOpen(imageMenu);
  await untilFocused("PNG");
  check("nested", [await open(account), await open(exportMenu), await open(imageMenu)], [true, true, true]);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(imageMenu, false);
  check("leftClosesNested", [await focused(), await open(exportMenu)], ["Image", true]);
  await page.keyboard.press("Escape");
  await untilOpen(exportMenu, false);
  await page.keyboard.press("Escape");
  await untilOpen(account, false);

  // The pointer and submenus: pointing at the item opens it, pointing at a sibling closes it.
  await button("Account").click();
  await untilOpen(account);
  await exportItem.hover();
  await untilOpen(exportMenu);
  check("hoverOpens", [await exportItem.getAttribute("aria-expanded"), await page.evaluate(() => !!document.activeElement.closest("[data-submenu]"))], ["true", false]);
  await exportItem.click();
  check("clickKeepsOpen", await open(exportMenu));
  await exportMenu.getByRole("menuitem", { name: "Markdown" }).hover();
  check("hoverInSubmenu", await focused(), "Markdown");
  await account.getByRole("menuitem", { name: "Home" }).hover();
  await untilOpen(exportMenu, false);
  check("hoverSiblingCloses", [await open(account), await exportItem.getAttribute("aria-expanded")], [true, "false"]);
  await exportItem.hover();
  await untilOpen(exportMenu);
  await exportMenu.getByRole("menuitem", { name: "Markdown" }).click();
  await untilOpen(account, false);
  check("clickInSubmenuClosesAll", [await open(account), await open(exportMenu)], [false, false]);
  await button("Account").click();
  await untilOpen(account);
  const publish = account.locator("[data-submenu-trigger][aria-disabled=true]");
  await publish.click({ force: true });
  check("disabledSubmenu", [await publish.textContent().then((t) => t.trim()), await publish.getAttribute("aria-expanded")], ["Publish to", "false"]);
  await page.keyboard.press("Escape");
  await untilOpen(account, false);

  // Right to left: the submenu opens to the left, and the arrow keys swap.
  await setDir("rtl");
  await button("Account").focus();
  await page.keyboard.press("Enter");
  await untilOpen(account);
  await untilFocused("Export as");
  await page.keyboard.press("ArrowRight");
  check("rtlRightDoesNotOpen", await open(exportMenu), false);
  await page.keyboard.press("ArrowLeft");
  await untilOpen(exportMenu);
  await untilFocused("PDF");
  await settled(exportMenu);
  mt = await exportItem.boundingBox();
  mp = await exportMenu.boundingBox();
  check("rtlSubmenuOnTheLeft", mp.x + mp.width <= mt.x + 1);
  await page.keyboard.press("ArrowRight");
  await untilOpen(exportMenu, false);
  check("rtlRightCloses", [await focused(), await open(account)], ["Export as", true]);
  await page.keyboard.press("Escape");
};
