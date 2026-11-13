export default async ({ page, base, check }) => {
  const field = page.locator("#code-form [data-otp]");
  const slots = field.locator("[data-otp-slot]");
  const values = () => slots.evaluateAll((all) => all.map((s) => s.value).join(""));
  const formValue = () => page.evaluate(() => new FormData(document.getElementById("code-form")).get("code"));
  const index = () => slots.evaluateAll((all) => all.indexOf(document.activeElement));

  check("slots", [await slots.count(), await field.locator("> [aria-hidden=true]").count()], [6, 1]);
  check("names", await slots.evaluateAll((all) => all.map((s) => s.getAttribute("aria-label"))), [
    null, "Character 2 of 6", "Character 3 of 6", "Character 4 of 6", "Character 5 of 6", "Character 6 of 6",
  ]);
  check("firstLabelled", await page.getByLabel("Verification code").count(), 1);
  check("autocomplete", [await slots.first().getAttribute("autocomplete"), await slots.first().getAttribute("inputmode")], ["one-time-code", "numeric"]);

  // Typing fills a slot and moves to the next.
  await page.getByText("Verification code", { exact: true }).click();
  check("labelFocusesFirst", await index(), 0);
  await page.keyboard.type("12");
  check("typingAdvances", [await values(), await index(), await formValue()], ["12", 2, "12"]);
  await page.keyboard.type("x");
  check("rejectsLetters", [await values(), await index()], ["12", 2]);
  await page.keyboard.type("3456");
  check("filled", [await values(), await index(), await formValue()], ["123456", 5, "123456"]);

  // Backspace clears the slot, then goes back.
  await page.keyboard.press("Backspace");
  check("backspaceClears", [await values(), await index()], ["12345", 5]);
  await page.keyboard.press("Backspace");
  check("backspaceGoesBack", [await values(), await index(), await formValue()], ["1234", 4, "1234"]);

  await page.keyboard.press("ArrowLeft");
  check("arrowLeft", await index(), 3);
  await page.keyboard.type("9");
  check("typingReplaces", [await values(), await index()], ["1239", 4]);
  await page.keyboard.press("Home");
  check("home", await index(), 0);
  await page.keyboard.press("End");
  check("end", await index(), 5);
  await page.keyboard.press("ArrowRight");
  check("arrowStopsAtEnd", await index(), 5);

  // Pasting a whole code fills every slot, wherever the caret is.
  // A real paste: the text is copied from a text box made for the purpose.
  const paste = async (text) => {
    const index = await slots.evaluateAll((all) => all.indexOf(document.activeElement));
    await page.evaluate((t) => {
      const area = document.createElement("textarea");
      area.id = "copy-source";
      area.setAttribute("aria-label", "Copy source");
      area.value = t;
      document.querySelector("main").append(area);
      area.select();
    }, text);
    await page.keyboard.press("ControlOrMeta+C");
    await page.evaluate(() => document.getElementById("copy-source").remove());
    await slots.nth(index).focus();
    await page.keyboard.press("ControlOrMeta+V");
  };
  await paste("987 654");
  check("pasteFills", [await values(), await index(), await formValue()], ["987654", 5, "987654"]);
  await slots.nth(2).focus();
  await paste("11");
  check("pastePartFromCaret", [await values(), await index()], ["981154", 4]);

  // A code filled in at once, as from a text message, spreads over the slots.
  await slots.first().fill("246810");
  check("autofillSpreads", [await values(), await formValue()], ["246810", "246810"]);

  // The change event reports the code.
  await page.evaluate(() => {
    const seen = [];
    document.querySelector("#code-form input[type=hidden]").addEventListener("change", (e) => seen.push(e.target.value));
    window.otpSeen = seen;
  });
  await slots.last().focus();
  await page.keyboard.press("Backspace");
  check("changeEvent", await page.evaluate(() => window.otpSeen), ["24681"]);

  // One tab stop, as Base UI's: the focused slot, and once focus leaves, the first empty one.
  const stops = (locator) => locator.evaluateAll((all) => all.map((s) => s.tabIndex));
  check("rovingFocused", await stops(slots), [-1, -1, -1, -1, -1, 0]);
  await page.keyboard.press("Shift+Tab");
  check("oneTabStop", await page.evaluate(() => document.activeElement.closest("[data-otp]") === null), true);
  check("rovingFirstEmpty", await stops(slots), [-1, -1, -1, -1, -1, 0]);
  await page.keyboard.press("Tab");
  check("tabReturns", await index(), 5);

  // Letters and digits, upper-cased as they are typed or pasted.
  const recovery = page.getByRole("textbox", { name: "Recovery code" });
  const recoveryField = page.locator("[data-otp]").filter({ has: recovery });
  const recoveryCode = () => recoveryField.locator("input[type=hidden]").inputValue();
  check("recoveryServer", await recoveryField.evaluate((r) => [r.dataset.validation, r.dataset.normalize, r.querySelector("[data-otp-slot]").inputMode]), ["alphanumeric", "upper", "text"]);
  await recovery.focus();
  await page.keyboard.type("a7c9-xz42");
  check("upperCase", [await recoveryCode(), await recoveryField.locator("[data-otp-slot]").evaluateAll((all) => all.map((s) => s.value).join(""))], ["A7C9XZ42", "A7C9XZ42"]);
  await page.evaluate(() => {
    const area = document.createElement("textarea");
    area.id = "copy-source";
    area.setAttribute("aria-label", "Copy source");
    area.value = "b8d0ée11";
    document.querySelector("main").append(area);
    area.select();
  });
  await page.keyboard.press("ControlOrMeta+C");
  await page.evaluate(() => document.getElementById("copy-source").remove());
  await recovery.focus();
  await page.keyboard.press("ControlOrMeta+V");
  check("pasteNormalized", await recoveryCode(), "B8D0E112");

  // A PIN is masked: the slots are password fields, in the small size.
  const pin = page.getByRole("group").filter({ has: page.locator("#pin") });
  check("masked", await pin.locator("[data-otp-slot]").evaluateAll((all) => all.map((s) => s.type)), ["password", "password", "password", "password"]);
  await page.locator("#pin").focus();
  await page.keyboard.type("12");
  check("maskedValue", await pin.locator("input[type=hidden]").inputValue(), "12");
  const small = await page.locator("#pin").boundingBox();
  const normal = await slots.first().boundingBox();
  check("sizes", [small.width, small.height, normal.width, normal.height].map(Math.round), [32, 32, 40, 40]);

  const invite = page.getByRole("group", { name: "Invite code" });
  check("initialValue", [await invite.locator("[data-otp-slot]").evaluateAll((all) => all.map((s) => s.value)), await invite.locator("input[type=hidden]").inputValue()], [["1", "2", "", "", ""], "12"]);
  check("firstNamedByLabel", await invite.getByRole("textbox", { name: "Invite code" }).count(), 1);
  check("initialTabStop", await stops(invite.locator("[data-otp-slot]")), [-1, -1, 0, -1, -1]);
  await invite.locator("[data-otp-slot]").first().focus();
  await page.keyboard.type("x");
  check("numericRejectsLetters", await invite.locator("input[type=hidden]").inputValue(), "12");

  check("disabled", await page.locator("[data-otp]").filter({ has: page.locator("#locked") }).locator("input").evaluateAll((all) => all.every((s) => s.disabled)));

  // The form sends the code; the page answers with an error, then with success.
  await page.goto(`${base}/c/input-otp?code=111111`);
  const wrong = page.locator("#code-form [data-otp-slot]");
  check("wrongCode", [
    await wrong.evaluateAll((all) => all.every((s) => s.getAttribute("aria-invalid") === "true")),
    (await page.locator("#code-error").textContent()).trim(),
    await page.locator("#code-error [data-icon]").getAttribute("data-icon"),
  ], [true, "That code is incorrect or has expired.", "circle-alert"]);
  await page.locator("#code").focus();
  await page.keyboard.type("424242");
  await Promise.all([page.waitForURL(/code=424242/), page.getByRole("button", { name: "Verify" }).click()]);
  check("verified", [await page.locator("#code-error").count(), (await page.locator("#code-description").textContent()).trim()], [0, "Verified. You can close this window."]);
};
