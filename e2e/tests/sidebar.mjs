export default async ({ page, check, axe, settled, setDir }) => {
  const layout = page.locator("[data-sidebar-layout]").first();
  const aside = page.getByRole("complementary", { name: "Sidebar", exact: true });
  const drawer = page.locator("[data-sidebar-drawer]").first();
  const toggle = page.locator("header [data-sidebar-toggle]").first();
  const nav = page.getByRole("navigation", { name: "Components" });
  const width = (locator) => locator.evaluate((e) => Math.round(e.getBoundingClientRect().width));
  const toggleState = () => toggle.evaluate((t) => [t.getAttribute("aria-label"), t.getAttribute("aria-expanded")]);
  const cookie = () => page.evaluate(() => document.cookie.match(/bitop_sidebar=(\w+)/)?.[1] ?? null);

  // The shell: landmarks, the brand, sections and the current page.
  check("landmarks", [await aside.count(), await nav.count(), await page.getByRole("banner").count(), await page.getByRole("main").count()], [1, 1, 1, 1]);
  check("brand", await aside.getByRole("link", { name: "bitop-ui" }).getAttribute("href"), "/");
  check("current", await nav.locator("[aria-current=page]").allTextContents(), ["Sidebar"]);
  check("sectionNamesList", await page.getByRole("list", { name: "Overlays" }).count(), 1);
  check("expanded", [await width(aside), ...(await toggleState())], [240, "Collapse sidebar", "true"]);

  // The skip link is the first stop and moves focus to the main content.
  await page.keyboard.press("Tab");
  check("skipFirst", await page.evaluate(() => document.activeElement.textContent), "Skip to content");
  await page.keyboard.press("Enter");
  check("skipToMain", await page.evaluate(() => document.activeElement.tagName), "MAIN");

  // Collapsed: an icon rail, remembered in a cookie. Labels are hidden but still named.
  await toggle.click();
  await page.waitForFunction(() => Math.round(document.querySelector("aside").getBoundingClientRect().width) === 56);
  check("collapses", [await layout.getAttribute("data-collapsed"), ...(await toggleState()), await cookie()], ["", "Expand sidebar", "false", "collapsed"]);
  check("labelsNamed", await nav.getByRole("link", { name: "Tooltip", exact: true }).count(), 1);
  const home = aside.getByRole("link", { name: "All components" });
  check("labelHidden", await home.locator("[data-sidebar-label]").evaluate((l) => l.getBoundingClientRect().width <= 1));
  await page.reload();
  check("remembered", [await layout.getAttribute("data-collapsed"), ...(await toggleState()), await width(aside)], ["", "Expand sidebar", "false", 56]);

  // Hovering an item on the rail shows its label as a tooltip beside it.
  const tip = aside.locator("[data-sidebar-tip]");
  await home.hover();
  await tip.waitFor();
  await settled(tip);
  check("tipText", await tip.textContent(), "All components");
  const [itemBox, tipBox] = [await home.boundingBox(), await tip.boundingBox()];
  check("tipBeside", tipBox.x >= itemBox.x + itemBox.width && Math.abs(tipBox.y + tipBox.height / 2 - (itemBox.y + itemBox.height / 2)) < 2);
  check("tipHidden", await tip.getAttribute("aria-hidden"), "true");
  await page.mouse.move(600, 600);
  await tip.waitFor({ state: "hidden" });
  // Keyboard focus shows it at once.
  // The content scrolls, so it is a focusable region between the brand and the items.
  await aside.getByRole("region", { name: "Sidebar navigation" }).focus();
  await page.keyboard.press("Tab");
  check("tipOnFocus", await tip.textContent(), "All components");
  check("tipOpenOnFocus", await tip.evaluate((t) => t.matches(":popover-open")));
  await page.keyboard.press("Escape");
  check("tipEscape", await tip.evaluate((t) => t.matches(":popover-open")), false);

  await toggle.click();
  await page.waitForFunction(() => Math.round(document.querySelector("aside").getBoundingClientRect().width) === 240);
  check("expands", [...(await toggleState()), await cookie()], ["Collapse sidebar", "true", "expanded"]);
  check("noTipExpanded", await (async () => {
    await home.hover();
    await page.waitForTimeout(800);
    return tip.evaluate((t) => t.matches(":popover-open"));
  })(), false);

  // The example shell: its own toggle, not remembered.
  const demo = page.locator("[data-example=dashboard]");
  const demoAside = page.getByRole("complementary", { name: "App sidebar (example)" });
  const demoToggle = demo.locator("[data-sidebar-toggle]");
  check("demoExpanded", await width(demoAside), 240);
  check("modeSwitch", await demo.getByRole("navigation", { name: "Portal" }).locator("[aria-current=page]").textContent().then((t) => t.trim()), "Workspace");
  check("switcherName", await demo.getByRole("button", { name: /^Current workspace: Acme Inc\s*Owner$/ }).count(), 1);
  check("userName", await demo.getByRole("button", { name: /^Account: Ada Lovelace\s*ada@example.com$/ }).count(), 1);
  check("trailing", await demoAside.getByRole("link", { name: /Deployments/ }).textContent().then((t) => t.trim()), "Deployments12");
  await demoToggle.click();
  await page.waitForFunction(() => Math.round(document.querySelector('[aria-label="App sidebar (example)"]').getBoundingClientRect().width) === 56);
  check("demoNotRemembered", [await cookie(), await layout.getAttribute("data-collapsed")], ["expanded", null]);
  check("railHidesTrailing", await demoAside.locator("a", { hasText: "Deployments" }).locator("[class*=trailing]").evaluate((t) => getComputedStyle(t).display), "none");
  const tokens = demoAside.getByRole("link", { name: "API tokens" });
  check("dotOnIcon", await tokens.locator("span[aria-hidden=true]:not([data-icon])").last().evaluate((d) => getComputedStyle(d).position), "absolute");
  check("modeStacked", await demo.getByRole("navigation", { name: "Portal" }).evaluate((n) => getComputedStyle(n).gridAutoFlow), "row");
  // The workspace menu still opens from the rail.
  await demo.locator("button[aria-haspopup=menu]").first().click();
  await page.locator("[role=menu]", { hasText: "Side project" }).waitFor();
  check("switcherOpens", await demo.locator("button[aria-haspopup=menu]").first().getAttribute("aria-expanded"), "true");
  await page.keyboard.press("Escape");
  await demoToggle.click();

  // Collapsible sections and two-line items.
  const sections = page.getByRole("complementary", { name: "Sidebar with collapsible sections (example)" });
  const people = sections.getByRole("button", { name: "People" });
  const peopleList = page.locator(`#${await people.getAttribute("aria-controls")}`);
  check("sectionClosed", [await people.getAttribute("aria-expanded"), await peopleList.isVisible()], ["false", false]);
  await people.click();
  check("sectionOpens", [await people.getAttribute("aria-expanded"), await peopleList.isVisible()], ["true", true]);
  check("sectionLabelled", await sections.getByRole("list", { name: "People" }).count(), 1);
  await people.click();
  check("sectionCloses", await peopleList.isVisible(), false);
  check("twoLineNames", await sections.getByRole("link", { name: /^Quarterly report, / }).allTextContents().then((all) => all.length), 2);
  check("twoLineName", await sections.getByRole("link", { name: "Quarterly report, Finance · yesterday" }).count(), 1);

  await setDir("rtl");
  const box = await aside.boundingBox();
  check("rtlOnRight", Math.abs(box.x + box.width - page.viewportSize().width) < 1);
  check("rtlNoOverflow", await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth));
  check("axeRtl", await axe(), []);
  await setDir("ltr");

  // Narrow: a modal drawer over the page, with its full labels.
  await page.setViewportSize({ width: 500, height: 800 });
  await page.waitForFunction(() => document.querySelector("header [data-sidebar-toggle]").getAttribute("aria-label") === "Open navigation");
  // The sidebar leaves the layout with a transition.
  await aside.waitFor({ state: "hidden" });
  check("drawerHidden", [await aside.isVisible(), ...(await toggleState())], [false, "Open navigation", "false"]);
  await toggle.click();
  check("drawerOpens", await drawer.evaluate((d) => d.matches(":popover-open")));
  await settled(drawer);
  check("drawerAtEdge", await drawer.boundingBox().then((b) => [Math.round(b.x), Math.round(b.height)]), [0, 800]);
  check("drawerDialog", await page.getByRole("dialog", { name: "Navigation" }).count(), 1);
  check("drawerToggle", await toggle.getAttribute("aria-expanded"), "true");
  await page.waitForFunction(() => document.querySelector("[data-sidebar-drawer]").contains(document.activeElement));
  check("focusOnCurrent", await page.evaluate(() => document.activeElement.textContent.trim()), "Sidebar");
  check("restInert", await page.evaluate(() => document.querySelector("main").closest("[inert]") !== null));
  // Tab stays in the drawer.
  const inside = [];
  for (let i = 0; i < 4; i += 1) {
    await page.keyboard.press("Shift+Tab");
    inside.push(await page.evaluate(() => document.querySelector("[data-sidebar-drawer]").contains(document.activeElement)));
  }
  check("trapBack", inside, [true, true, true, true]);
  await drawer.getByRole("button", { name: "Close navigation" }).focus();
  await page.keyboard.press("Tab");
  check("trapWraps", await page.evaluate(() => document.querySelector("[data-sidebar-drawer]").contains(document.activeElement)));
  check("axeDrawerOpen", await axe(), []);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.querySelector("[data-sidebar-drawer]").matches(":popover-open"));
  check("drawerEsc", [await page.evaluate(() => document.activeElement === document.querySelector("header [data-sidebar-toggle]")), await page.evaluate(() => document.querySelector("[inert]") === null)], [true, true]);
  check("narrowNoOverflow", await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth));

  // The close button closes it.
  await toggle.click();
  await settled(drawer);
  await drawer.getByRole("button", { name: "Close navigation" }).click();
  await page.waitForFunction(() => !document.querySelector("[data-sidebar-drawer]").matches(":popover-open"));
  check("closeButton", await toggle.getAttribute("aria-expanded"), "false");

  // Following a link in the example's drawer closes it.
  const demoDrawer = demo.locator("[data-sidebar-drawer]");
  await demoToggle.click();
  await settled(demoDrawer);
  await demoDrawer.getByRole("link", { name: "Projects" }).click();
  await page.waitForFunction(() => !document.querySelector("[data-example=dashboard] [data-sidebar-drawer]").matches(":popover-open"));
  check("linkCloses", await page.evaluate(() => document.querySelector("[inert]") === null));

  // Widening the window closes an open drawer and brings back the sidebar.
  await toggle.click();
  await settled(drawer);
  await page.setViewportSize({ width: 1000, height: 800 });
  await page.waitForFunction(() => !document.querySelector("[data-sidebar-drawer]").matches(":popover-open"));
  check("wideAgain", [await aside.isVisible(), ...(await toggleState()), await page.evaluate(() => document.querySelector("[inert]") === null)], [true, "Collapse sidebar", "true", true]);
};
