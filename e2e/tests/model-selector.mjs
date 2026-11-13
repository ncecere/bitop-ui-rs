export default async ({ page, check, settled, axe }) => {
  const trigger = page.getByRole("button", { name: "Model: Claude Sonnet" });
  const popup = page.locator("#model");
  const search = popup.getByRole("combobox", { name: "Search models" });
  const value = () => page.locator("input[type=hidden][name=model]").inputValue();
  const visible = () => popup.locator("[role=option]:not([hidden])").evaluateAll((all) => all.map((o) => o.dataset.name));
  const active = () => popup.locator("[data-active]").evaluate((o) => o.dataset.name);
  const isOpen = () => popup.evaluate((p) => p.matches(":popover-open"));

  check("trigger", [await trigger.getAttribute("aria-haspopup"), await value()], ["dialog", "claude-sonnet"]);

  // Opening from the keyboard: the search box takes focus, the chosen model is active.
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.activeElement?.matches("[data-model-search]"));
  check("opens", await isOpen(), true);
  await settled(popup);
  const t = await trigger.boundingBox();
  const p = await popup.boundingBox();
  check("below", Math.round(p.y) >= Math.round(t.y + t.height), true);
  check("activeIsChosen", await active(), "Claude Sonnet");
  check("activedescendant", await search.evaluate((s) => document.getElementById(s.getAttribute("aria-activedescendant")).dataset.name), "Claude Sonnet");
  check("groups", await popup.locator("[role=group]").evaluateAll((all) => all.map((g) => g.getAttribute("aria-label"))), ["Anthropic", "OpenAI", "Local"]);
  check("meta", await popup.locator("[role=option]").first().evaluate((o) => [o.querySelector("span span:nth-child(2)").textContent, [...o.querySelectorAll("[data-variant]")].map((b) => b.textContent.trim())]), [
    "Balanced speed and quality · 200K context", ["Vision", "Tools", "Reasoning"],
  ]);
  check("tone", await popup.locator("[data-value=local-llama] [data-variant]").getAttribute("data-variant"), "warning");
  check("selectedMark", await popup.locator("[aria-selected=true]").evaluate((o) => o.dataset.value), "claude-sonnet");
  check("axeOpen", await axe(), []);

  // Typing filters by every word; groups without a match are hidden.
  await page.keyboard.type("open vision");
  check("filtered", await visible(), ["GPT Large"]);
  check("groupsHidden", await popup.locator("[role=group]:not([hidden])").count(), 1);
  await search.fill("");
  await page.keyboard.type("anthropic");
  check("byProvider", await visible(), ["Claude Sonnet", "Claude Haiku"]);
  check("firstActive", await active(), "Claude Sonnet");
  await page.keyboard.press("ArrowDown");
  check("arrowDown", await active(), "Claude Haiku");
  await page.keyboard.press("ArrowDown");
  check("wraps", await active(), "Claude Sonnet");
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("Enter");
  check("chosen", [await value(), await isOpen(), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], ["claude-haiku", false, "Model: Claude Haiku"]);
  check("triggerText", (await page.locator("[data-example=form] [data-model-trigger]").textContent()).trim(), "Claude Haiku");

  // Nothing matches: the empty text shows.
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.activeElement?.matches("#model [data-model-search]"));
  check("searchCleared", await search.inputValue(), "");
  await page.keyboard.type("zzz");
  check("empty", [await popup.locator("[data-model-empty]").isVisible(), await visible()], [true, []]);
  // Escape closes and returns focus, without changing the value.
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.getElementById("model").matches(":popover-open"));
  check("escape", [await value(), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], ["claude-haiku", "Model: Claude Haiku"]);

  // The pointer chooses too, and the value is sent with the form.
  await page.locator("[data-example=form] [data-model-trigger]").click();
  await popup.locator("[data-value=gpt-mini]").click();
  check("clicked", await value(), "gpt-mini");
  await Promise.all([page.waitForURL("**/c/model-selector?model=gpt-mini"), page.locator("[data-example=form]").evaluate((f) => f.requestSubmit())]);
  check("submitted", await page.locator("[data-query]").textContent(), "model=gpt-mini");

  // Nothing chosen shows the placeholder; disabled cannot open and submits nothing.
  const small = page.locator("[data-example=small] [data-model-trigger]");
  check("placeholder", [await small.getAttribute("aria-label"), await small.getAttribute("data-size"), await small.locator("[data-placeholder]").count()], ["Model: Select a model", "sm", 1]);
  const locked = page.locator("[data-example=disabled] [data-model-trigger]");
  check("disabled", [await locked.isDisabled(), await page.locator("input[name=locked]").isDisabled()], [true, true]);

  // Other strings.
  const german = page.locator("[data-example=strings]");
  check("germanTrigger", await german.locator("[data-model-trigger]").getAttribute("aria-label"), "Modell: Modell wählen");
  await german.locator("[data-model-trigger]").click();
  await page.waitForFunction(() => document.activeElement?.matches("#german-model [data-model-search]"));
  check("germanSearch", [await page.locator("#german-model [data-model-search]").getAttribute("aria-label"), await page.locator("#german-model [data-model-search]").getAttribute("placeholder")], ["Modelle suchen", "Modelle suchen…"]);
  check("germanMeta", await page.locator("#german-model [data-value=claude-sonnet]").evaluate((o) => [o.querySelector("span span:nth-child(2)").textContent, [...o.querySelectorAll("[data-variant]")].map((b) => b.textContent.trim())]), [
    "Balanced speed and quality · 200K Kontext", ["Bilder", "Werkzeuge", "Reasoning"],
  ]);
  await page.keyboard.type("qqq");
  check("germanEmpty", (await page.locator("#german-model [data-model-empty]").textContent()).trim(), "Keine Modelle gefunden.");
};
