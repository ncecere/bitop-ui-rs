export default async ({ page, check, button, setTheme }) => {
  const toggle = button("Dark mode");
  const other = button("Dark theme");
  const select = page.getByRole("combobox", { name: "Color mode" });
  const theme = () => page.evaluate(() => document.documentElement.dataset.theme);
  const stored = () => page.evaluate(() => localStorage.getItem("bitop-color-mode"));
  const shown = (control) => control.locator("[data-icon]").evaluateAll((all) => all.filter((i) => getComputedStyle(i).display !== "none").map((i) => i.dataset.icon));

  // The controls show the mode the page loaded with.
  await setTheme("light");
  check("light", [await theme(), await toggle.getAttribute("aria-pressed"), await shown(toggle), await select.inputValue()], ["light", "false", ["sun"], "light"]);

  // The toggle switches between light and dark, and stores the choice.
  await toggle.focus();
  await page.keyboard.press("Enter");
  check("toDark", [await theme(), await stored(), await toggle.getAttribute("aria-pressed"), await shown(toggle)], ["dark", "dark", "true", ["moon"]]);
  check("othersFollow", [await other.getAttribute("aria-pressed"), await select.inputValue()], ["true", "dark"]);
  await page.keyboard.press("Space");
  check("toLight", [await theme(), await stored(), await toggle.getAttribute("aria-pressed")], ["light", "light", "false"]);
  await other.click();
  check("anyToggle", [await theme(), await toggle.getAttribute("aria-pressed")], ["dark", "true"]);

  // The select has the system setting too.
  await select.selectOption("light");
  check("selectLight", [await theme(), await stored(), await toggle.getAttribute("aria-pressed")], ["light", "light", "false"]);
  await page.emulateMedia({ colorScheme: "dark" });
  await select.selectOption("system");
  // WebKit tells the page about the emulated setting a moment later.
  await page.waitForFunction(() => document.documentElement.dataset.theme === "dark");
  check("selectSystem", [await theme(), await stored(), await toggle.getAttribute("aria-pressed"), await select.inputValue()], ["dark", null, "true", "system"]);
  // The system changes: the controls follow.
  await page.emulateMedia({ colorScheme: "light" });
  await page.waitForFunction(() => document.documentElement.dataset.theme === "light");
  check("followsSystem", [await toggle.getAttribute("aria-pressed"), await shown(toggle), await select.inputValue()], ["false", ["sun"], "system"]);

  // A script that sets the mode is followed as well.
  await setTheme("dark");
  check("followsScript", [await toggle.getAttribute("aria-pressed"), await select.inputValue()], ["true", "dark"]);
  // The choice survives a reload, with no flash: the theme is set before the page paints.
  await page.reload();
  check("survivesReload", [await theme(), await toggle.getAttribute("aria-pressed"), await select.inputValue()], ["dark", "true", "dark"]);
  const size = await toggle.boundingBox();
  check("iconButton", [Math.round(size.width), Math.round(size.height)], [36, 36]);
};
