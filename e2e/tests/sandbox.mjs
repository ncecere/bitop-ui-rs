export default async ({ page, check, button, setDir }) => {
  const name = (id) => page.locator(`#${id} > summary`).evaluate((s) => s.textContent.replace(/\s+/g, " ").trim());
  const done = page.locator("#completed-sandbox");

  // The header: the title, the state as a word, the summary; open by default.
  check("header", [await name("completed-sandbox"), await done.evaluate((d) => d.open)], ["analyze.py Completed 1.2 s", true]);
  check("states", await page.locator("details[data-sandbox] > summary").evaluateAll((all) => all.map((s) => s.dataset.state)), ["completed", "error", "pending", "running", "running"]);

  // Code and Output are tabs; the code is a code block with a copy button.
  const tabs = done.getByRole("tab");
  check("tabs", [await tabs.allTextContents(), await done.getByRole("tablist", { name: "Run of analyze.py" }).count()], [["Code", "Output"], 1]);
  check("code", [await done.locator("#analyze-code pre code").getAttribute("class"), (await done.locator("#analyze-code code").textContent()).startsWith("import pandas"), await done.locator("#analyze-code [data-code-block]").locator("span").first().textContent()], ["language-python", true, "analyze.py"]);
  check("copy", await done.getByRole("button", { name: "Copy Python code" }).count(), 1);
  check("action", await button("Run again").count(), 1);

  // The arrow keys move between tabs, and Enter shows the focused tab's panel.
  await tabs.first().focus();
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("analyze-output-tab").getAttribute("aria-selected") === "true");
  check("output", [await page.locator("#analyze-output").isVisible(), await page.locator("#analyze-code").isHidden(), await page.evaluate(() => document.activeElement.id)], [true, true, "analyze-output-tab"]);
  const output = page.locator("#analyze-output");
  check("outputBlock", [(await output.locator("code").textContent()).includes("West     1411.25"), await output.locator("[data-code-block] > div > span").first().textContent(), await output.getByRole("button", { name: "Copy output" }).count()], [true, "Output", 1]);
  await page.keyboard.press("Home");
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.getElementById("analyze-code-tab").getAttribute("aria-selected") === "true");

  // The header closes and opens the card from the keyboard.
  await done.locator("> summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => !document.getElementById("completed-sandbox").open);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.getElementById("completed-sandbox").open);
  check("toggles", await done.evaluate((d) => d.open), true);

  // An error is shown as an error, read with a word.
  const error = page.locator("#config-output [data-sandbox-output] > div");
  check("error", [await error.locator(".sr-only").textContent(), (await error.textContent()).includes("FileNotFoundError"), await error.locator("[data-icon]").getAttribute("data-icon"), await error.getAttribute("dir")], ["Error: ", true, "circle-alert", "ltr"]);
  check("errorName", await name("error-sandbox"), "load_config.py Error exit code 1");

  // Closed cards; the running badge pulses; an empty output says so.
  check("closed", await page.locator("[data-example=states] details").evaluateAll((all) => all.map((d) => d.open)), [false, false]);
  check("pulse", await page.locator("#running-sandbox summary span[data-state] > span").evaluate((d) => getComputedStyle(d, "::after").animationName !== "none"), true);
  check("badgeColours", await page.locator("main summary span[data-state]").evaluateAll((all) => new Set(all.map((b) => getComputedStyle(b).backgroundColor)).size), 4);
  await page.locator("#pending-sandbox > summary").click();
  await page.waitForFunction(() => document.getElementById("pending-sandbox").open);
  check("empty", await page.locator("#pending-sandbox p").textContent(), "No output");

  check("german", [await page.locator("[data-example=strings] summary span[data-state]").textContent(), await page.locator("#german-output p").textContent(), await page.getByRole("tablist", { name: "Ausführung" }).count()], ["Läuft", "Noch keine Ausgabe", 1]);

  // Right to left: the chevron goes to the left end; the action to the left of the tabs.
  await setDir("rtl");
  const rtl = await done.evaluate((d) => {
    const summary = d.querySelector("summary");
    const chevron = summary.querySelector("[data-icon=chevron-down]").getBoundingClientRect();
    const list = d.querySelector("[role=tablist]").getBoundingClientRect();
    const action = [...d.querySelectorAll("button")].find((b) => b.textContent.trim() === "Run again").getBoundingClientRect();
    return [chevron.left - summary.getBoundingClientRect().left < 40, action.right <= list.left + 1];
  });
  check("rtl", rtl, [true, true]);
};
