export default async ({ page, check }) => {
  const details = (name) => page.locator(`[data-example=${name}] details`);
  const isOpen = (name) => details(name).evaluate((d) => d.open);

  // A <details> whose <summary> counts the sources; closed, the cards are not shown.
  const links = details("links");
  const trigger = links.locator("summary");
  check("label", (await trigger.textContent()).trim(), "Used 3 sources");
  check("closed", [await isOpen("links"), await links.getByRole("list", { name: "Sources" }).isVisible()], [false, false]);
  check("decorativeIcons", await trigger.locator("[data-icon]").evaluateAll((all) => all.map((i) => i.getAttribute("aria-hidden"))), ["true", "true"]);

  // The keyboard opens and closes it.
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-example=links] details").open);
  const list = links.getByRole("list", { name: "Sources" });
  await list.waitFor();
  check("opens", await list.isVisible(), true);
  check("cards", await list.locator(":scope > li").count(), 3);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => !document.querySelector("[data-example=links] details").open);
  check("closes", await isOpen("links"), false);
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-example=links] details").open);

  // A linked source opens in a new tab and says so; its name starts with its number.
  const first = list.getByRole("link").first();
  check("link", [await first.getAttribute("href"), await first.getAttribute("target"), await first.getAttribute("rel")], [
    "https://www.example.com/policies/parental-leave", "_blank", "noreferrer noopener",
  ]);
  check("linkName", await list.getByRole("link", { name: "Source 1: Parental leave policy 2025 (opens in a new tab) example.com" }).count(), 1);
  check("announced", (await first.textContent()).replace(/\s+/g, " ").includes("Source 1: Parental leave policy 2025 (opens in a new tab)"), true);
  // The host is the meta text, without www; a source without a link is plain text.
  check("meta", await list.locator("li span[class*=meta]").allTextContents(), ["example.com", "help.example.org", "p. 4"]);
  check("plain", [await list.locator("li").nth(2).getByRole("link").count(), await list.locator("li").nth(2).locator("[data-icon]").first().getAttribute("data-icon")], [0, "file"]);
  // Tab moves through the cards.
  await page.keyboard.press("Tab");
  check("tabToCard", await page.evaluate(() => document.activeElement.getAttribute("href")), "https://www.example.com/policies/parental-leave");

  // One source.
  check("one", [(await details("one").locator("summary").textContent()).trim(), await isOpen("one")], ["Used 1 source", true]);

  // Selectable cards: a button named by its text, with the action read first, and a separate link.
  const select = details("select");
  const show = select.getByRole("button", { name: "Show Source 1: Leave policy" });
  check("selectName", await show.count(), 1);
  check("selectLink", await select.getByRole("link", { name: "Open the link (opens in a new tab)" }).getAttribute("href"), "https://www.example.com/leave");
  await page.evaluate(() => {
    window.selected = [];
    document.addEventListener("bitop:source-select", (event) => window.selected.push(event.detail.index));
  });
  await show.focus();
  await page.keyboard.press("Enter");
  const own = select.getByRole("button", { name: "Open Travel policy in the side panel" });
  await own.click();
  check("selectEvents", await page.evaluate(() => window.selected), [1, null]);
  check("ownLabel", await own.getAttribute("aria-describedby"), null);
  // A focusable source shows a ring when focused from a script.
  const target = page.locator("#select-source-1");
  await target.focus();
  check("focusTarget", await target.evaluate((li) => document.activeElement === li), true);
  // The ring arrives through a transition.
  await page.waitForFunction(() => getComputedStyle(document.querySelector("#select-source-1 > button")).boxShadow.includes("0px 0px 0px 2px"));
  check("focusRing", true);

  // Other strings.
  const german = details("strings");
  check("strings", [(await german.locator("summary").textContent()).trim(), await german.locator("ol").getAttribute("aria-label")], ["2 Quellen verwendet", "Quellen"]);
  check("stringsSr", (await german.locator("li").first().textContent()).replace(/\s+/g, " ").trim(), "1Quelle 1: Urlaubsregelung (öffnet in einem neuen Tab)intranet.example.de");
};
