export default async ({ page, check, base, setDir }) => {
  const root = page.locator("#full-preview");
  const frame = root.locator("iframe");
  const address = root.getByRole("textbox", { name: "Address" });
  const back = root.getByRole("button", { name: "Back" });
  const forward = root.getByRole("button", { name: "Forward" });
  const reload = root.getByRole("button", { name: "Reload" });
  const heading = () => page.frameLocator("#full-preview iframe").locator("h1").textContent();
  // Waits for the frame to show a page, and for the spinner to go.
  const loaded = async (title) => {
    await page.frameLocator("#full-preview iframe").locator("h1", { hasText: title }).waitFor();
    await root.locator("[data-web-preview-loading]").waitFor({ state: "hidden" });
  };

  // The frame is sandboxed, titled, and shows the local page.
  check("frame", [await frame.getAttribute("sandbox"), await frame.getAttribute("title"), await frame.getAttribute("referrerpolicy"), await frame.getAttribute("src")], ["allow-scripts allow-forms", "Preview of the generated site", "no-referrer", "/preview?page=home"]);
  await loaded("Home");
  check("home", await heading(), "Home");
  check("bar", [await root.getByRole("group", { name: "Preview navigation" }).count(), await address.inputValue(), await back.isDisabled(), await forward.isDisabled(), await reload.isDisabled()], [1, "/preview?page=home", true, true, false]);
  check("open", [await root.getByRole("link", { name: "Open in new tab" }).getAttribute("href"), await root.getByRole("link", { name: "Open in new tab" }).getAttribute("target"), await root.getByRole("link", { name: "Open in new tab" }).getAttribute("rel")], ["/preview?page=home", "_blank", "noopener noreferrer"]);
  // No same-origin access, so the frame cannot reach the page. (Reading the frame's
  // document from the page to show that logs an error in WebKit.)
  check("noSameOrigin", (await frame.getAttribute("sandbox")).includes("allow-same-origin"), false);

  // Enter in the address field goes to the address; the whole address is selected on focus.
  await address.focus();
  check("selected", await address.evaluate((i) => [i.selectionStart, i.selectionEnd]), [0, "/preview?page=home".length]);
  await page.keyboard.type("/preview?page=about");
  await page.keyboard.press("Enter");
  await loaded("About");
  check("went", [await frame.getAttribute("src"), await back.isDisabled(), await forward.isDisabled(), await page.locator("[data-preview-event]").textContent()], ["/preview?page=about", false, true, "Went to /preview?page=about"]);
  // A bare local host gets http://.
  const host = new URL(base).host;
  await address.fill(`${host}/preview?page=pricing`);
  await address.press("Enter");
  await loaded("Pricing");
  check("http", [await address.inputValue(), await frame.getAttribute("src")], [`http://${host}/preview?page=pricing`, `http://${host}/preview?page=pricing`]);
  // javascript: is refused and the field goes back to the address.
  await address.fill("javascript:alert(1)");
  await address.press("Enter");
  check("refused", [await address.inputValue(), await frame.getAttribute("src")], [`http://${host}/preview?page=pricing`, `http://${host}/preview?page=pricing`]);

  // Back and forward walk the addresses visited, from the keyboard.
  await back.focus();
  await page.keyboard.press("Enter");
  await loaded("About");
  check("back", [await address.inputValue(), await forward.isDisabled()], ["/preview?page=about", false]);
  await back.click();
  await loaded("Home");
  check("backAgain", [await address.inputValue(), await back.isDisabled()], ["/preview?page=home", true]);
  await forward.focus();
  await page.keyboard.press("Space");
  await loaded("About");
  check("forward", await address.inputValue(), "/preview?page=about");
  // A new address drops the ones ahead.
  await address.fill("/preview?page=home");
  await address.press("Enter");
  await loaded("Home");
  check("dropsAhead", await forward.isDisabled(), true);

  // Reload loads the address again, with the spinner over the frame meanwhile.
  await page.evaluate(() => {
    window.previewLoads = 0;
    document.querySelector("#full-preview iframe").addEventListener("load", () => (window.previewLoads += 1));
    // Read straight after the library's click handler on document: the frame may load
    // before the check could look.
    window.addEventListener("click", () => {
      const body = document.querySelector("#full-preview [data-web-preview-body]");
      const cover = body.querySelector("[data-web-preview-loading]");
      window.previewBusy = [body.getAttribute("aria-busy"), cover.hidden, cover.querySelector("[role=status]").textContent.trim()];
    }, { once: true });
  });
  await reload.click();
  await page.waitForFunction(() => window.previewBusy !== undefined);
  check("busy", await page.evaluate(() => window.previewBusy), ["true", false, "Loading preview"]);
  await page.waitForFunction(() => window.previewLoads === 1);
  await root.locator("[data-web-preview-loading]").waitFor({ state: "hidden" });
  check("reloaded", await root.locator("[data-web-preview-body]").getAttribute("aria-busy"), null);

  // The console: a disclosure with the counts; each level is spelled out.
  const consoleTrigger = root.locator("details summary");
  check("consoleTrigger", await consoleTrigger.evaluate((s) => s.textContent.replace(/\s+/g, " ").trim()), "Console 1 error, 2 warnings");
  await consoleTrigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#full-preview details").open);
  const logs = root.getByRole("region", { name: "Console output" }).locator("li");
  await logs.first().waitFor();
  check("levels", await logs.evaluateAll((all) => all.map((li) => [li.dataset.level, li.firstElementChild.textContent])), [["log", "Log"], ["info", "Info"], ["warn", "Warning"], ["warn", "Warning"], ["error", "Error"]]);
  check("timestamps", [await logs.locator("time").count(), /^\d{4}-\d\d-\d\dT/.test(await logs.locator("time").first().getAttribute("datetime"))], [4, true]);
  check("logColours", await logs.evaluateAll((all) => new Set(all.map((li) => getComputedStyle(li).backgroundColor)).size), 3);

  // srcdoc in a sandboxed frame; an empty console says so.
  const srcdoc = page.locator("#srcdoc-preview");
  check("srcdoc", [await srcdoc.locator("iframe").getAttribute("sandbox"), await page.frameLocator("#srcdoc-preview iframe").locator("h1").textContent(), await srcdoc.getByText("No console output.").isVisible()], ["allow-scripts allow-forms", "Pro", true]);

  // No address: the empty text, no frame and no open link; an address loads one.
  const empty = page.locator("#empty-preview");
  check("empty", [await empty.getByText("Enter an address to load the preview.").isVisible(), await empty.locator("iframe").isHidden(), await empty.locator("[data-web-preview-open]").isHidden(), await empty.getByRole("button", { name: "Reload" }).isDisabled(), await empty.getByRole("textbox").getAttribute("placeholder")], [true, true, true, true, "Enter a URL"]);
  await empty.getByRole("textbox").fill("/preview?page=pricing");
  await empty.getByRole("textbox").press("Enter");
  await page.frameLocator("#empty-preview iframe").locator("h1", { hasText: "Pricing" }).waitFor();
  check("emptyLoaded", [await empty.locator("iframe").isVisible(), await empty.getByText("Enter an address").isHidden(), await empty.locator("[data-web-preview-open]").getAttribute("href"), await empty.getByRole("button", { name: "Reload" }).isDisabled()], [true, true, "/preview?page=pricing", false]);

  const german = page.locator("#german-preview");
  check("german", [await german.getByRole("group", { name: "Vorschau-Navigation" }).count(), await german.getByRole("button", { name: "Zurück" }).count(), await german.getByRole("textbox", { name: "Adresse" }).getAttribute("placeholder"), await german.locator("summary").evaluate((s) => s.textContent.replace(/\s+/g, " ").trim())], [1, 1, "Adresse eingeben", "Konsole 1 Warnung"]);

  // Right to left: back sits at the right end and points right; the address stays left to right.
  await setDir("rtl");
  const rtl = await root.evaluate((r) => {
    const bar = r.querySelector("[role=group]").getBoundingClientRect();
    const backButton = r.querySelector("[data-web-preview-action=back]");
    return [bar.right - backButton.getBoundingClientRect().right < 20, getComputedStyle(backButton.querySelector("[data-icon]")).scale, getComputedStyle(r.querySelector("[data-web-preview-address]")).direction];
  });
  check("rtl", rtl, [true, "-1 1", "ltr"]);
};
