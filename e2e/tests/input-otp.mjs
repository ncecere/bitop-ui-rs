export default async ({ page, check }) => {
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
  await page.getByText("Verification code").click();
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

  // One tab stop per slot is the native behaviour; the change event reports the code.
  const changed = await page.evaluate(() => {
    const seen = [];
    document.querySelector("#code-form input[type=hidden]").addEventListener("change", (e) => seen.push(e.target.value));
    window.otpSeen = seen;
  });
  await slots.last().focus();
  await page.keyboard.press("Backspace");
  check("changeEvent", [changed, await page.evaluate(() => window.otpSeen)], [undefined, ["24681"]]);

  const pin = page.getByRole("group", { name: "PIN" });
  check("initialValue", [await pin.locator("[data-otp-slot]").evaluateAll((all) => all.map((s) => s.value)), await pin.locator("input[type=hidden]").inputValue()], [["1", "2", "", ""], "12"]);
  check("firstNamedByLabel", await pin.getByRole("textbox", { name: "PIN" }).count(), 1);
  const small = await pin.locator("[data-otp-slot]").first().boundingBox();
  const normal = await slots.first().boundingBox();
  check("sizes", [small.width, small.height, normal.width, normal.height].map(Math.round), [32, 32, 40, 40]);

  const invite = page.getByRole("group", { name: "Invite code" });
  await invite.locator("[data-otp-slot]").first().focus();
  await page.keyboard.type("a1-b2");
  check("alphanumeric", await invite.locator("input[type=hidden]").inputValue(), "a1b2");

  check("invalid", await page.getByRole("group", { name: "Wrong code" }).locator("[data-otp-slot]").evaluateAll((all) => all.every((s) => s.getAttribute("aria-invalid") === "true")));
  check("disabled", await page.getByRole("group", { name: "Disabled code" }).locator("input").evaluateAll((all) => all.every((s) => s.disabled)));
};
