export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const nav = page.getByRole("navigation", { name: "Main" });
  const products = nav.getByRole("button", { name: "Products" });
  const resources = nav.getByRole("button", { name: "Resources" });
  const panel = (text) => page.locator("[data-navigation-content]", { hasText: text });
  const productsPanel = panel("Deployments");
  const resourcesPanel = panel("Changelog");
  const open = (p) => p.evaluate((el) => el.matches(":popover-open"));
  const untilOpen = async (p, want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await p.elementHandle(), want]);
  const untilExpanded = async (trigger, want) =>
    page.waitForFunction(([el, w]) => el.getAttribute("aria-expanded") === w, [await trigger.elementHandle(), want]);
  // The first line of a link: its title, without the description.
  const focusedTitle = () => page.evaluate(() => (document.activeElement.querySelector("span > span:not([data-icon])") ?? document.activeElement).textContent.trim());

  // A list of buttons and links in a navigation landmark.
  check("structure", await nav.locator("> ul > li > *:first-child").evaluateAll((all) => all.map((el) => el.tagName)), ["BUTTON", "BUTTON", "A", "A"]);
  check("collapsed", [await products.getAttribute("aria-expanded"), await products.evaluate((b) => document.getElementById(b.getAttribute("aria-controls")).hasAttribute("data-navigation-content"))], ["false", true]);
  check("current", await nav.locator("> ul > li > a[aria-current=page]").allTextContents(), ["Gallery"]);

  // A click opens the panel under its button.
  await products.click();
  await untilOpen(productsPanel);
  await untilExpanded(products, "true");
  check("clickOpens", await products.getAttribute("aria-expanded"), "true");
  await settled(productsPanel);
  let t = await products.boundingBox();
  let p = await productsPanel.boundingBox();
  check("anchored", { below: p.y >= t.y + t.height, gap: Math.round(p.y - t.y - t.height), aligned: Math.abs(p.x - t.x) < 2 }, { below: true, gap: 8, aligned: true });
  const links = productsPanel.locator("a");
  const first = await links.nth(0).boundingBox();
  const second = await links.nth(1).boundingBox();
  check("twoColumns", [Math.round(first.y) === Math.round(second.y), second.x > first.x], [true, true]);
  check("axeOpen", await axe(), []);
  await products.click();
  await untilOpen(productsPanel, false);
  await untilExpanded(products, "false");
  check("clickCloses", await products.getAttribute("aria-expanded"), "false");

  // Keyboard: Enter opens, Tab moves into the panel, Escape returns to the button.
  await products.focus();
  await page.keyboard.press("Enter");
  await untilOpen(productsPanel);
  check("enterKeepsFocus", await focused(), "Products");
  await page.keyboard.press("Tab");
  check("tabIntoPanel", await focusedTitle(), "Deployments");
  await page.keyboard.press("ArrowDown");
  check("down", await focusedTitle(), "Storage");
  await page.keyboard.press("End");
  check("end", await focusedTitle(), "Status");
  await page.keyboard.press("Home");
  check("home", await focusedTitle(), "Deployments");
  await page.keyboard.press("Escape");
  check("escReturnsFocus", [await open(productsPanel), await focused()], [false, "Products"]);
  // ArrowDown opens and moves in; ArrowUp from the first link goes back.
  await page.keyboard.press("ArrowDown");
  await untilOpen(productsPanel);
  check("downOpensAndEnters", await focusedTitle(), "Deployments");
  await page.keyboard.press("ArrowUp");
  check("upReturnsToButton", [await focused(), await open(productsPanel)], ["Products", true]);
  // Tabbing through the panel and out of it closes it.
  for (let i = 0; i < 5; i++) await page.keyboard.press("Tab");
  check("tabOutCloses", [await focused(), await open(productsPanel)], ["Resources", false]);
  await untilExpanded(products, "false");

  // The arrows move along the top level.
  await page.keyboard.press("ArrowRight");
  check("right", await focused(), "Gallery");
  await page.keyboard.press("End");
  check("topEnd", await focused(), "Pricing");
  await page.keyboard.press("Home");
  check("topHome", await focused(), "Products");
  await page.keyboard.press("ArrowLeft");
  check("stopsAtStart", await focused(), "Products");
  await page.evaluate(() => document.activeElement.blur());

  // Hover opens after a moment, the pointer can cross into the panel, and leaving closes.
  await page.mouse.move(0, 0);
  await resources.hover();
  await untilOpen(resourcesPanel);
  await untilExpanded(resources, "true");
  check("hoverOpens", await resources.getAttribute("aria-expanded"), "true");
  await settled(resourcesPanel);
  check("currentInPanel", await resourcesPanel.locator("a[aria-current=page]").evaluate((a) => a.querySelector("span > span:not([data-icon])").textContent), "Components");
  // A click straight after hovering keeps it open.
  await resources.click();
  check("clickAfterHoverKeepsOpen", await open(resourcesPanel));
  await resourcesPanel.getByRole("link", { name: "Guides" }).hover();
  await page.waitForTimeout(300);
  check("staysWhileInPanel", await open(resourcesPanel));
  await resources.hover();
  await page.mouse.move(0, 0);
  await page.waitForTimeout(400);
  check("clickedPanelStays", await open(resourcesPanel));
  await page.keyboard.press("Escape");
  await untilOpen(resourcesPanel, false);
  await products.hover();
  await untilOpen(productsPanel);
  await resources.hover();
  await untilOpen(resourcesPanel);
  check("hoverSwitches", await open(productsPanel), false);
  await page.mouse.move(0, 0);
  await untilOpen(resourcesPanel, false);
  await untilExpanded(resources, "false");
  check("leaveCloses", await resources.getAttribute("aria-expanded"), "false");
  await products.click();
  await untilOpen(productsPanel);
  await page.mouse.click(5, 5);
  check("clickOutsideCloses", await open(productsPanel), false);

  // Panels aligned to the end of their button.
  const account = page.getByRole("navigation", { name: "Account" }).getByRole("button", { name: "Account" });
  const accountPanel = panel("Billing");
  await account.click();
  await untilOpen(accountPanel);
  await settled(accountPanel);
  t = await account.boundingBox();
  p = await accountPanel.boundingBox();
  check("alignEnd", Math.abs(p.x + p.width - (t.x + t.width)) < 2);
  await page.keyboard.press("Escape");

  // Right to left: the panel aligns to the right edge, and the arrows follow the visual order.
  await setDir("rtl");
  await products.click();
  await untilOpen(productsPanel);
  await settled(productsPanel);
  t = await products.boundingBox();
  p = await productsPanel.boundingBox();
  check("rtlAligned", Math.abs(p.x + p.width - (t.x + t.width)) < 2);
  await page.keyboard.press("Escape");
  await products.focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlLeftIsNext", await focused(), "Resources");
};
