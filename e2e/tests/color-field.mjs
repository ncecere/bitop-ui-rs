export default async ({ page, check }) => {
  const root = page.locator("#accent-form [data-color-field]");
  const hex = page.getByLabel("Accent colour");
  const picker = root.locator("input[type=color]");
  const line = root.locator("p");
  const message = () => line.textContent().then((t) => t.trim());
  const formValue = () => page.evaluate(() => new FormData(document.getElementById("accent-form")).get("accent"));
  const swatch = () => root.locator("[data-color-shown]").evaluateAll((all) => all.map((s) => getComputedStyle(s).backgroundColor));
  const pressed = () => root.locator("[aria-pressed=true]").evaluateAll((all) => all.map((b) => b.getAttribute("aria-label")));

  // Empty means the default colour, which the swatch, the picker and the check use.
  check("initial", [await hex.inputValue(), await hex.getAttribute("placeholder"), await picker.inputValue(), await formValue()], ["", "#1d4ed8 (default)", "#1d4ed8", ""]);
  check("initialContrast", [await message(), await line.getAttribute("data-pass")], ["Contrast with white text 6.7:1: meets WCAG AA (4.5:1).", ""]);
  check("describes", await hex.evaluate((i) => document.getElementById(i.getAttribute("aria-describedby")).tagName), "P");
  check("pickerNamed", await page.getByLabel("Pick a colour").count(), 5);
  check("presets", [await root.getByRole("group", { name: "Preset colours" }).locator("button").count(), await pressed()], [4, []]);

  // Typing a colour updates everything; a failing colour says so in words.
  await hex.fill("#FDE047");
  check("typed", [await picker.inputValue(), await swatch(), await pressed()], ["#fde047", ["rgb(253, 224, 71)", "rgb(253, 224, 71)"], ["Yellow"]]);
  check("fails", [await message(), await line.getAttribute("data-pass")], ["Contrast with white text 1.3:1: below 4.5:1. Choose a darker colour.", null]);
  check("icons", await line.locator("[data-icon]").evaluateAll((all) => all.filter((i) => getComputedStyle(i).display !== "none").map((i) => i.dataset.icon)), ["circle-alert"]);

  await hex.fill("abc");
  check("shortHex", [await picker.inputValue(), await hex.getAttribute("aria-invalid")], ["#aabbcc", null]);
  await hex.fill("#12");
  check("invalid", [await hex.getAttribute("aria-invalid"), await message(), await picker.inputValue()], ["true", "Enter a hex colour such as #1d4ed8.", "#1d4ed8"]);

  // A preset is a button: the keyboard presses it.
  await root.getByRole("button", { name: "Green" }).focus();
  await page.keyboard.press("Enter");
  check("preset", [await hex.inputValue(), await pressed(), await formValue(), await hex.getAttribute("aria-invalid")], ["#166534", ["Green"], "#166534", null]);
  check("presetContrast", await message(), "Contrast with white text 7.1:1: meets WCAG AA (4.5:1).");
  check("checkShown", await root.getByRole("button", { name: "Green" }).locator("[data-icon]").evaluate((i) => getComputedStyle(i).display !== "none"));
  check("checkHidden", await root.getByRole("button", { name: "Blue" }).locator("[data-icon]").evaluate((i) => getComputedStyle(i).display), "none");

  // The native picker writes the hex input.
  await picker.fill("#b91c1c");
  check("picker", [await hex.inputValue(), await pressed(), await formValue()], ["#b91c1c", ["Red"], "#b91c1c"]);

  await hex.fill("");
  check("clearedIsDefault", [await picker.inputValue(), await pressed(), await message()], ["#1d4ed8", [], "Contrast with white text 6.7:1: meets WCAG AA (4.5:1)."]);

  const badge = page.getByLabel("Badge colour");
  const badgeLine = badge.locator("xpath=../..").locator("p");
  check("darkTextPasses", (await badgeLine.textContent()).trim(), "Contrast with dark text 13.4:1: meets WCAG AA (4.5:1).");
  await badge.fill("#1f2937");
  check("darkTextFails", (await badgeLine.textContent()).trim(), "Contrast with dark text 1.2:1: below 4.5:1. Choose a lighter colour.");

  const border = page.getByLabel("Border colour").locator("xpath=../..");
  check("noCheck", [await border.locator("p").isVisible(), await border.locator("[data-color-shown]").count()], [false, 1]);
  const bad = page.getByLabel("Link colour");
  check("serverInvalid", [await bad.getAttribute("aria-invalid"), (await bad.locator("xpath=../..").locator("p").textContent()).trim()], ["true", "Enter a hex colour such as #1d4ed8."]);

  const locked = page.getByLabel("Locked colour");
  const lockedRoot = locked.locator("xpath=../..");
  check("disabled", [await locked.isDisabled(), await lockedRoot.locator("input[type=color]").isDisabled(), await lockedRoot.locator("button").evaluateAll((all) => all.every((b) => b.disabled))], [true, true, true]);
  check("serverPressed", await lockedRoot.locator("[aria-pressed=true]").getAttribute("aria-label"), "Green");
};
