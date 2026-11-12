export default async ({ page, check, axe, button, settled, setDir }) => {
  const nav = page.locator("nav[popover]");
  const toggle = button("Toggle sidebar");

  // Wide: part of the layout, collapsed by the toggle and remembered in a cookie.
  check("landmark", await page.getByRole("navigation", { name: "Components" }).count(), 1);
  check("current", await nav.locator("[aria-current=page]").allTextContents(), ["Sidebar"]);
  check("shown", [await nav.isVisible(), await toggle.getAttribute("aria-expanded")], [true, "true"]);
  await toggle.click();
  check("collapses", [await nav.isVisible(), await toggle.getAttribute("aria-expanded")], [false, "false"]);
  await page.reload();
  check("remembered", [await nav.isVisible(), await toggle.getAttribute("aria-expanded")], [false, "false"]);
  await toggle.click();
  check("expands", await nav.isVisible());

  await setDir("rtl");
  const box = await nav.boundingBox();
  check("rtlOnRight", Math.abs(box.x + box.width - page.viewportSize().width) < 1);
  check("rtlNoOverflow", await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth));
  check("axeRtl", await axe(), []);
  await setDir("ltr");

  // Narrow: a drawer over the page. The sidebar leaves the layout with a transition.
  await page.setViewportSize({ width: 500, height: 800 });
  await page.waitForTimeout(400);
  check("drawerHidden", await nav.isVisible(), false);
  await toggle.click();
  check("drawerOpens", await nav.evaluate((n) => n.matches(":popover-open")));
  await settled(nav);
  check("drawerAtEdge", await nav.boundingBox().then((b) => [b.x, b.height]), [0, 800]);
  check("axeDrawerOpen", await axe(), []);
  await page.keyboard.press("Escape");
  check("drawerEsc", await nav.evaluate((n) => !n.matches(":popover-open")));
  check("narrowNoOverflow", await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth));
};
