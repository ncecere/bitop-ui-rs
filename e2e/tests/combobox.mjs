export default async ({ page, check, axe, settled, visible }) => {
  const combo = page.getByRole("combobox", { name: "Model" });
  const popup = page.locator("[data-combobox] > [popover]");
  const value = () => page.locator("input[type=hidden][name=model]").inputValue();
  const options = () => visible(popup.locator("[role=option]:not([hidden])"));
  const active = () => visible(popup.locator("[data-active]"));

  await combo.click();
  check("opens", await popup.evaluate((p) => p.matches(":popover-open")));
  check("expanded", await combo.getAttribute("aria-expanded"), "true");
  await settled(popup);
  const i = await combo.boundingBox();
  const p = await popup.boundingBox();
  check(
    "anchored",
    { below: p.y >= i.y + i.height, sameWidth: Math.abs(p.width - i.width) < 1, aligned: Math.abs(p.x - i.x) < 1 },
    { below: true, sameWidth: true, aligned: true },
  );
  check("axeOpen", await axe(), []);
  await page.keyboard.type("atl");
  check("filters", await options(), ["Atlas Large", "Atlas Small"]);
  check("firstActive", await active(), ["Atlas Large"]);
  await page.keyboard.press("ArrowDown");
  check("arrow", await active(), ["Atlas Small"]);
  check(
    "activeDescendant",
    await combo.evaluate((c) => document.getElementById(c.getAttribute("aria-activedescendant"))?.textContent.trim()),
    "Atlas Small",
  );
  await page.keyboard.press("Enter");
  check("chosen", [await combo.inputValue(), await value()], ["Atlas Small", "atlas-small"]);
  check("closed", await popup.evaluate((el) => !el.matches(":popover-open")));
  check("keepsFocus", await page.evaluate(() => document.activeElement.getAttribute("role")), "combobox");
  await page.keyboard.press("ArrowDown");
  check("reopensOnSelected", await active(), ["Atlas Small"]);
  await combo.fill("zzz");
  check("empty", await popup.locator("[data-combobox-empty]").isVisible());
  await page.keyboard.press("Escape");
  check("escReverts", [await combo.inputValue(), await value()], ["Atlas Small", "atlas-small"]);
  await combo.click();
  await popup.getByText("Cirrus").click({ force: true });
  check("skipsDisabled", await value(), "atlas-small");
  await popup.getByText("Meridian").click();
  check("clickChooses", [await combo.inputValue(), await value()], ["Meridian", "meridian"]);
  await combo.fill("");
  await page.keyboard.press("Tab");
  check("clears", [await combo.inputValue(), await value()], ["", ""]);
};
