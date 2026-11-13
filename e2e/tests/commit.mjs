export default async ({ page, check, button, setDir }) => {
  const target = page.getByLabel("Paste target");
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };
  // The text as it is read: the parts that are not hidden, with a space between them.
  const text = (locator) =>
    locator.evaluate((root) => {
      const parts = [];
      const walk = (node) => {
        if (node.nodeType === Node.TEXT_NODE) parts.push(node.textContent);
        else if (node.nodeType === Node.ELEMENT_NODE && node.getAttribute("aria-hidden") !== "true") node.childNodes.forEach(walk);
        if (node.nodeType === Node.ELEMENT_NODE) parts.push(" ");
      };
      walk(root);
      return parts.join("").replace(/\s+/g, " ").replace(/ ,/g, ",").trim();
    });
  // The rotation of an icon once its transition has ended.
  const rotation = (locator) =>
    locator.evaluate(async (icon) => {
      await Promise.all(icon.getAnimations().map((a) => a.finished));
      return getComputedStyle(icon).rotate;
    });
  const card = page.locator("#token-commit");

  // The header: message, author, a relative time with the full date as its title, the
  // short hash read as "Commit …" with the full hash as its title.
  check("message", await card.locator("p").first().textContent(), "Fix the token refresh race");
  const when = card.locator("time");
  check("time", [await when.textContent(), (await when.getAttribute("title")).length > 10, /^\d{4}-\d\d-\d\dT/.test(await when.getAttribute("datetime"))], ["3 hours ago", true, true]);
  const hash = card.locator("[data-commit-hash]");
  check("hash", [await text(hash), await hash.getAttribute("title"), await hash.locator("[data-icon]").getAttribute("data-icon")], ["Commit 8f3c2a1", "8f3c2a1d9e7b6f5a4c3b2a1908f7e6d5c4b3a291", "git-commit-horizontal"]);
  check("separatorHidden", await card.locator("span[aria-hidden=true]", { hasText: "•" }).count(), 1);

  // Copy copies the full hash.
  await card.getByRole("button", { name: "Copy commit hash" }).click();
  await card.locator("[data-copy][data-state=copied]").waitFor();
  check("copied", await pasted(), "8f3c2a1d9e7b6f5a4c3b2a1908f7e6d5c4b3a291");

  // The trigger: a disclosure button, closed; its name has the counts in words.
  const trigger = card.locator("button[aria-controls]");
  const panel = page.locator("#token-files");
  check("trigger", [await trigger.getAttribute("aria-expanded"), await text(trigger), await panel.isHidden()], ["false", "4 files changed, 14 lines added, 3 lines removed", true]);
  check("shownCounts", await trigger.locator("[aria-hidden=true]:not([data-icon])").allTextContents(), ["+14", "−3"]);
  await trigger.focus();
  await page.keyboard.press("Enter");
  await panel.waitFor({ state: "visible" });
  check("opened", [await trigger.getAttribute("aria-expanded"), await trigger.getAttribute("data-open")], ["true", ""]);
  check("chevronDown", await rotation(trigger.locator("[data-icon]")), "none");

  // The files: a named list; each status is a letter and a word, renames read "to".
  const files = page.getByRole("list", { name: "Changed files" }).first();
  const items = files.locator("li");
  check("files", [await items.evaluateAll((all) => all.map((li) => li.dataset.status)), await text(items.nth(0)), await text(items.nth(1)), await text(items.nth(2)), await text(items.nth(3))], [
    ["modified", "added", "deleted", "renamed"],
    "Modified: src/auth.rs, 12 lines added, 3 lines removed",
    "Added: src/auth_test.rs, 2 lines added",
    "Deleted: src/legacy_token.rs",
    "Renamed: src/refresh.rs to src/session/refresh.rs, 1 line added, 1 line removed",
  ]);
  check("letters", await files.locator("li > span:first-child > [aria-hidden=true]").allTextContents(), ["M", "A", "D", "R"]);
  check("statusColours", await files.locator("li > span:first-child").evaluateAll((all) => new Set(all.map((s) => getComputedStyle(s).color)).size), 4);
  check("deletedStruck", await files.locator("li[data-status=deleted] [dir=ltr]").evaluate((p) => getComputedStyle(p).textDecorationLine), "line-through");

  // Space closes it again.
  await page.keyboard.press("Space");
  await panel.waitFor({ state: "hidden" });
  check("closed", await trigger.getAttribute("aria-expanded"), "false");

  // Open from the start; a hash of another length; a long message is cut.
  const open = page.locator("[data-example=open]");
  check("startOpen", [await open.locator("button[aria-controls]").getAttribute("aria-expanded"), await page.locator("#docs-files").isVisible(), await text(open.locator("[data-commit-hash]"))], ["true", true, "Commit c0ffee1234"]);
  check("singular", await text(open.locator("button[aria-controls]")), "1 file changed, 1 line added");
  check("ellipsis", await open.locator("p").first().evaluate((p) => [getComputedStyle(p).textOverflow, p.scrollWidth > p.clientWidth]), ["ellipsis", true]);
  check("days", await open.locator("time").textContent(), "2 days ago");

  const german = page.locator("[data-example=strings]");
  check("german", [await german.getByRole("button", { name: "Commit-Hash kopieren" }).count(), await text(german.locator("button[aria-controls]")), await page.locator('ul[aria-label="Geänderte Dateien"]').count()], [1, "2 Dateien geändert, 5 Zeilen hinzugefügt, 1 Zeile entfernt", 1]);

  // Right to left: the actions go to the left and the closed chevron points left.
  await setDir("rtl");
  const rtl = await card.evaluate((root) => {
    const message = root.querySelector("p").getBoundingClientRect();
    const hash = root.querySelector("[data-commit-hash]").getBoundingClientRect();
    return hash.right <= message.left;
  });
  check("rtl", [rtl, await rotation(trigger.locator("[data-icon]"))], [true, "90deg"]);
};
