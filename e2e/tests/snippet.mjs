export default async ({ page, check, button, setDir }) => {
  const target = page.getByLabel("Paste target");
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };

  // The prompt is decorative and the command is what is copied.
  const install = page.locator("[data-example=install] [data-snippet]");
  check("prefix", [await install.locator("[aria-hidden=true]").first().textContent(), await install.locator("code").textContent()], ["$", "cargo add bitop-ui-ai"]);
  await install.getByRole("button", { name: "Copy command" }).click();
  await install.locator("[data-copy][data-state=copied]").waitFor();
  check("copiedName", await install.locator("[data-copy]").getAttribute("aria-label"), "Copied command");
  check("clipboard", await pasted(), "cargo add bitop-ui-ai");

  // A long command scrolls sideways and is reached from the keyboard.
  const long = page.locator("[data-example=long] code");
  check("focusable", await long.getAttribute("tabindex"), "0");
  check("overflows", await long.evaluate((c) => c.scrollWidth > c.clientWidth + 1), true);
  await long.focus();
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("End");
  await page.waitForFunction(() => document.querySelector("[data-example=long] code").scrollLeft > 0);
  check("scrolled", await long.evaluate((c) => c.scrollLeft > 0), true);
  // Tab goes from the command to its copy button.
  await page.keyboard.press("Tab");
  check("tabToCopy", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Copy command");
  await page.keyboard.press("Enter");
  await page.locator("[data-example=long] [data-copy][data-state=copied]").waitFor();
  check("keyboardCopy", (await pasted()).startsWith("docker run --rm"), true);

  // Another prompt, no prompt, and another label.
  const prompts = page.locator("[data-example=prompts] [data-snippet]");
  check("otherPrompt", await prompts.first().locator("[aria-hidden=true]").first().textContent(), ">");
  check("noPrompt", await prompts.nth(1).locator("span[aria-hidden=true]:not([data-icon])").count(), 0);
  check("labels", [await prompts.first().locator("[data-copy]").getAttribute("aria-label"), await prompts.nth(1).locator("[data-copy]").getAttribute("aria-label")], ["Copy query", "Copy link"]);
  check("german", await button("Befehl kopieren").count(), 1);

  // Right to left: the copy button moves to the left, and the command stays left to right
  // with its prompt before it.
  await setDir("rtl");
  const geometry = await install.evaluate((root) => {
    const prompt = root.querySelector("[aria-hidden=true]").getBoundingClientRect();
    const code = root.querySelector("code").getBoundingClientRect();
    const copy = root.querySelector("[data-copy]").getBoundingClientRect();
    return { copyLeft: copy.right <= code.left + 1, promptBefore: prompt.right <= code.left + 1, direction: getComputedStyle(root.querySelector("code")).direction };
  });
  check("rtl", geometry, { copyLeft: true, promptBefore: true, direction: "ltr" });
};
