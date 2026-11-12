export default async ({ page, check, button }) => {
  const target = page.getByLabel("Paste target");
  // Reads the clipboard the way a person would: by pasting.
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };
  const status = (name) => button(name).locator("xpath=..").locator("[role=status]").textContent();

  const command = button("Copy command");
  check("idle", [await command.getAttribute("data-state"), await command.locator("[data-icon]:visible").getAttribute("data-icon")], ["idle", "copy"]);
  check("statusMounted", await page.locator("main [role=status]").count(), 3);
  await command.click();
  await page.locator("main [data-state=copied]").first().waitFor();
  const copied = button("Copied command");
  check("copied", [await copied.count(), await copied.locator("[data-icon]:visible").getAttribute("data-icon")], [1, "check"]);
  check("announced", await status("Copied command"), "Copied command to clipboard");
  check("clipboard", await pasted(), "npm install bitop-ui");

  // The keyboard works, and the text of another element is read when the button is pressed.
  await page.locator("main code").last().evaluate((code) => (code.textContent = "cargo add bitop-ui --features ai"));
  const code = button("Copy code");
  await code.focus();
  await page.keyboard.press("Enter");
  await page.locator("main [data-copy-target][data-state=copied]").waitFor();
  check("copiedText", [await page.locator("main [data-copy-target]").getAttribute("aria-label"), (await page.locator("main [data-copy-target]").textContent()).trim()], ["Copied code", "Copied"]);
  // It goes back to idle by itself.
  await page.locator("main [data-copy-target][data-state=idle]").waitFor();
  check("backToIdle", [await code.count(), (await code.textContent()).trim(), await status("Copy code")], [1, "Copy", ""]);
  check("clipboardFromTarget", await pasted(), "cargo add bitop-ui --features ai");

  // The tooltip has the same words as the state.
  const key = button("Copy key");
  await page.locator("main h1").click();
  await page.keyboard.press("Tab");
  for (let i = 0; i < 10 && !(await key.evaluate((b) => b === document.activeElement)); i += 1) await page.keyboard.press("Tab");
  const tip = page.locator("main [role=tooltip]");
  check("tooltip", [await tip.evaluate((t) => t.matches(":popover-open")), (await tip.textContent()).trim()], [true, "Copy"]);
  await page.keyboard.press("Space");
  await page.locator("main [data-tooltip][data-state=copied]").waitFor();
  check("tooltipCopied", (await tip.textContent()).trim(), "Copied");

  // When the browser refuses, the button says so.
  await page.evaluate(() => {
    Object.defineProperty(navigator, "clipboard", { value: { writeText: () => Promise.reject(new Error("no")) }, configurable: true });
    document.execCommand = () => false;
  });
  await page.reload();
  await page.evaluate(() => {
    Object.defineProperty(navigator, "clipboard", { value: { writeText: () => Promise.reject(new Error("no")) }, configurable: true });
    document.execCommand = () => false;
  });
  await button("Copy command").click();
  await page.locator("main [data-state=failed]").waitFor();
  check("failed", [await button("Copy command").count(), await status("Copy command")], [1, "Couldn't copy. Select the text and copy it manually."]);
};
