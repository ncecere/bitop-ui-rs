export default async ({ page, check, settled }) => {
  const trigger = page.getByRole("button", { name: "Voice: Leo" });
  const dialog = page.locator("dialog#voice");
  const list = dialog.getByRole("radiogroup", { name: "Voice" });
  const rows = () => dialog.locator("[data-voice-row]").evaluateAll((all) => all.filter((r) => !r.hidden).map((r) => r.dataset.voiceRow));

  // The trigger shows the chosen voice and opens the dialog from the keyboard.
  check("trigger", [await trigger.count(), await trigger.getAttribute("aria-haspopup")], [1, "dialog"]);
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("voice").open);
  await settled(dialog);
  check("title", await page.getByRole("dialog", { name: "Choose a voice" }).count(), 1);
  check("voices", await rows(), ["ava", "leo", "sam", "mia"]);
  check("meta", await list.locator("[data-voice-row=leo] label > span > span").allTextContents(), ["Leo", "Male · American · Middle-aged", "Calm, for long reads"]);
  check("checked", await list.getByRole("radio", { name: /^Leo/ }).isChecked(), true);

  // The search filters by every word, and says how many match.
  const search = dialog.getByRole("searchbox", { name: "Search voices" });
  await search.fill("female");
  check("filtered", [await rows(), await dialog.locator("[data-voice-status]").textContent()], [["ava"], "1 voice"]);
  await search.fill("calm american");
  check("words", await rows(), ["leo"]);
  await search.fill("nobody");
  check("empty", [await rows(), await dialog.locator("[data-voice-status]").textContent()], [[], "No voices match your search."]);
  await search.fill("a");
  check("several", await dialog.locator("[data-voice-status]").textContent(), "4 voices");
  await search.fill("english");
  check("english", await rows(), ["ava", "sam"]);

  // The arrow keys move among the voices shown, and choose; the trigger follows.
  await list.getByRole("radio", { name: /^Ava/ }).focus();
  await page.keyboard.press("ArrowDown");
  check("arrowSkipsHidden", await page.evaluate(() => document.activeElement.value), "sam");
  check("chosen", [await list.getByRole("radio", { name: /^Sam/ }).isChecked(), await page.locator("[data-example=form] [data-voice-trigger]").getAttribute("aria-label")], [true, "Voice: Sam"]);
  await page.keyboard.press("ArrowDown");
  check("wraps", await page.evaluate(() => document.activeElement.value), "ava");

  // Tab reaches the voice's preview button; the page plays it and the button follows.
  await page.keyboard.press("Tab");
  check("previewFocused", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Preview Ava");
  await page.evaluate(() => {
    window.previews = [];
    document.addEventListener("bitop:voice-preview", (e) => window.previews.push(e.detail));
  });
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-voice-preview=ava]").getAttribute("aria-pressed") === "true");
  check("pressed", [await page.evaluate(() => window.previews), await dialog.locator("[data-voice-preview=ava] [data-icon]").evaluateAll((all) => all.filter((i) => i.checkVisibility()).map((i) => i.dataset.icon))], [[{ id: "ava", playing: false }], ["pause"]]);
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-voice-preview=ava]").getAttribute("aria-pressed") === "false");

  // Done closes it; the search starts again, and the choice is sent with the form.
  await dialog.getByRole("button", { name: "Done" }).click();
  await page.waitForFunction(() => !document.getElementById("voice").open);
  // The close event comes a moment after.
  await page.waitForFunction(() => document.querySelector("#voice [data-voice-search]").value === "");
  // A role query does not find the search box in the closed dialog.
  check("cleared", [await dialog.locator("[data-voice-search]").inputValue(), (await rows()).length], ["", 4]);
  check("focusBack", await page.evaluate(() => document.activeElement.dataset.voiceTrigger), "");
  check("form", await page.evaluate(() => new FormData(document.querySelector("[data-example=form]")).get("voice")), "ava");
  check("triggerText", await page.locator("[data-example=form] [data-voice-value]").textContent(), "Ava");

  // Escape closes it too.
  await page.locator("[data-example=form] [data-voice-trigger]").click();
  await page.waitForFunction(() => document.getElementById("voice").open);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.getElementById("voice").open);

  // Samples the script plays: the button is pressed while it plays, and the sample
  // ends on its own.
  const sampled = page.locator("[data-example=samples] [data-voice-trigger]");
  check("placeholder", [await sampled.getAttribute("aria-label"), await sampled.locator("[data-voice-value]").getAttribute("data-placeholder")], ["Narrator: Select a voice", ""]);
  await sampled.click();
  await page.waitForFunction(() => document.getElementById("sampled").open);
  await page.locator("#sampled [data-voice-preview=leo]").click();
  await page.waitForFunction(() => document.querySelector("#sampled [data-voice-preview=leo]").getAttribute("aria-pressed") === "true");
  check("playing", await page.locator("#sampled [data-voice-preview=leo]").getAttribute("aria-busy"), null);
  await page.waitForFunction(() => document.querySelector("#sampled [data-voice-preview=leo]").getAttribute("aria-pressed") === "false");
  check("ended", await page.locator("#sampled [data-voice-preview=ava]").getAttribute("aria-pressed"), "false");
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.getElementById("sampled").open);

  // Disabled, and an empty list.
  check("disabled", await page.locator("[data-example=other] [data-voice-trigger]").first().isDisabled(), true);
  check("noVoices", await page.locator("#none [data-voice-status]").textContent(), "No voices match your search.");

  // Other strings.
  await page.locator("[data-example=strings] [data-voice-trigger]").click();
  await page.waitForFunction(() => document.getElementById("stimme").open);
  await page.locator("#stimme [data-voice-search]").fill("leo");
  check("german", [await page.locator("#stimme h2").textContent(), await page.locator("#stimme [data-voice-status]").textContent()], ["Stimme wählen", "1 Stimme"]);
  // In a search box with text, the first Escape empties it in Chromium and Firefox;
  // the next one closes the dialog.
  await page.keyboard.press("Escape");
  if (await page.evaluate(() => document.getElementById("stimme").open)) {
    check("escapeClears", await page.locator("#stimme [data-voice-search]").inputValue(), "");
    await page.keyboard.press("Escape");
  }
  await page.waitForFunction(() => !document.getElementById("stimme").open);

  // Right to left: the preview buttons are at the left.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  await page.locator("[data-example=form] [data-voice-trigger]").click();
  await page.waitForFunction(() => document.getElementById("voice").open);
  await settled(dialog);
  check("rtlPreview", await dialog.locator("[data-voice-row=ava]").evaluate((r) => r.querySelector("[data-voice-preview]").getBoundingClientRect().left < r.querySelector("label").getBoundingClientRect().left), true);
};
