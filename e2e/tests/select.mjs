export default async ({ page, check, axe, settled }) => {
  const trigger = page.getByRole("combobox", { name: "Machine" });
  const popup = page.locator("#select-machine-popup");
  const value = () => page.locator("[data-select-value][name=machine]").inputValue();
  const isOpen = () => popup.evaluate((p) => p.matches(":popover-open"));
  const untilOpen = async (want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await popup.elementHandle(), want]);
  const focusedText = () => page.evaluate(() => document.activeElement?.dataset.label ?? document.activeElement?.textContent.trim());

  // The trigger: a combobox button named by the label, showing the chosen label.
  check("trigger", [await trigger.getAttribute("aria-haspopup"), await trigger.getAttribute("aria-expanded"), (await trigger.textContent()).trim(), await value()], ["listbox", "false", "Standard", "standard"]);
  check("chevron", await trigger.locator("[data-icon]").getAttribute("data-icon"), "chevrons-up-down");

  // Opening from the keyboard: focus goes to the chosen option.
  await trigger.focus();
  await page.keyboard.press("ArrowDown");
  await untilOpen();
  await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "option");
  check("opens", [await trigger.getAttribute("aria-expanded"), await focusedText()], ["true", "Standard"]);
  await settled(popup);
  const t = await trigger.boundingBox();
  const p = await popup.boundingBox();
  check("anchored", { gap: Math.round(p.y - t.y - t.height), aligned: Math.abs(p.x - t.x) < 2, wide: p.width >= t.width - 1 }, { gap: 6, aligned: true, wide: true });
  const standard = popup.getByRole("option", { name: /Standard/ });
  check("option", await standard.evaluate((o) => [o.querySelector("[data-icon]").dataset.icon, o.children[2].textContent, getComputedStyle(o.lastElementChild).display, Math.round(o.getBoundingClientRect().height)]), ["cpu", "2 vCPU", "flex", 32]);
  check("checkOnlyOnChosen", await popup.locator("[role=option]").evaluateAll((all) => all.map((o) => getComputedStyle(o.lastElementChild).display)), ["flex", "none", "none"]);
  check("axeOpen", await axe(), []);
  await page.keyboard.press("ArrowDown");
  check("down", await focusedText(), "Performance");
  await page.keyboard.press("ArrowDown");
  check("disabledReachable", await focusedText(), "Turbo");
  await page.keyboard.press("ArrowDown");
  check("noWrap", await focusedText(), "Turbo");
  await page.keyboard.press("Enter");
  check("disabledNotChosen", [await value(), await isOpen()], ["standard", true]);
  await page.keyboard.press("Home");
  check("home", await focusedText(), "Standard");
  await page.keyboard.press("p");
  check("typeahead", await focusedText(), "Performance");
  await page.evaluate(() => {
    window.selectChanges = [];
    document.addEventListener("change", (event) => window.selectChanges.push(event.target.value));
    document.addEventListener("bitop:change", (event) => window.selectChanges.push(`bitop:${event.detail.value}`));
  });
  await page.keyboard.press("Enter");
  await untilOpen(false);
  check("chosen", [await value(), (await trigger.textContent()).trim(), await page.evaluate(() => document.activeElement.id)], ["performance", "Performance", "select-machine"]);
  check("events", await page.evaluate(() => window.selectChanges), ["performance", "bitop:performance"]);

  // Escape closes without choosing; typing on the closed button chooses.
  await page.keyboard.press("Space");
  await untilOpen();
  await page.keyboard.press("Escape");
  await untilOpen(false);
  check("escape", [await value(), await page.evaluate(() => document.activeElement.id)], ["performance", "select-machine"]);
  // Letters typed within half a second make one search; start a new one.
  await page.waitForTimeout(600);
  await page.keyboard.press("s");
  check("typeOnButton", [await value(), (await trigger.textContent()).trim(), await isOpen()], ["standard", "Standard", false]);

  // The pointer: a click opens, a click on an option chooses, the label focuses.
  await trigger.click();
  await untilOpen();
  await popup.getByRole("option", { name: /Performance/ }).hover();
  check("hoverHighlights", await focusedText(), "Performance");
  await popup.getByRole("option", { name: /Performance/ }).click();
  await untilOpen(false);
  check("clickChooses", await value(), "performance");
  await page.getByText("Machine", { exact: true }).click();
  check("labelFocuses", [await page.evaluate(() => document.activeElement.id), await isOpen()], ["select-machine", false]);

  // In a form: the placeholder, a required value, and the values submitted.
  const retention = page.getByRole("combobox", { name: "Retention" });
  check("placeholder", await retention.locator("[data-placeholder]").textContent(), "Choose a period…");
  const form = page.locator("[data-select-form]");
  const valid = () => form.evaluate((f) => f.checkValidity());
  check("requiredInvalid", await valid(), false);
  check("small", await page.locator("#select-small").evaluate((b) => [b.dataset.size, Math.round(b.getBoundingClientRect().height)]), ["sm", 32]);
  check("hiddenLabel", await page.getByRole("combobox", { name: "Small select" }).count(), 1);
  await retention.click();
  await page.locator("#select-retention-popup").getByRole("option", { name: "1 year" }).click();
  check("formData", await form.evaluate((f) => [...new FormData(f)].map((pair) => pair.join("="))), ["retention=365", "small=30"]);
  check("requiredValid", await valid(), true);

  // Disabled: the button cannot be used, and the value is not submitted.
  check("disabled", await page.getByRole("combobox", { name: "Region" }).isDisabled());
};
