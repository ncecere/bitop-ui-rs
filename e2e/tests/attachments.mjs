export default async ({ page, check }) => {
  const chips = (example) => page.locator(`[data-example=${example}] [data-attachment]`);
  const meta = (example) => chips(example).locator("[data-attachment-meta]").allTextContents();

  // A named list; each chip shows the type and the size, and an icon for the type.
  const sent = page.getByRole("list", { name: "Attachments" }).first();
  check("list", await sent.locator(":scope > li").count(), 6);
  check("meta", await meta("sent"), ["PDF · 471 KB", "PNG · 94 KB", "MD · 812 B", "ZIP · 12 MB", "XLSX · 20 KB", ""]);
  check("icons", await chips("sent").locator("[data-attachment-preview]").evaluateAll((all) => all.map((p) => p.querySelector("[data-icon]")?.dataset.icon ?? p.querySelector("img")?.getAttribute("alt"))), [
    "file-text", "", "file-text", "file", "file", "globe",
  ]);
  check("previewHidden", await chips("sent").locator("[data-attachment-preview]").evaluateAll((all) => all.every((p) => p.getAttribute("aria-hidden") === "true")), true);
  // A long name is cut, with the whole name on hover.
  const long = chips("sent").nth(4).locator("[data-attachment-name]");
  check("longName", [await long.evaluate((n) => n.scrollWidth > n.clientWidth), await long.getAttribute("title")], [true, "a very long file name that does not fit in its chip.xlsx"]);

  // The upload state is spelled out.
  check("states", [await chips("states").evaluateAll((all) => all.map((c) => c.dataset.status)), await meta("states")], [["uploading", "error"], ["Uploading…", "Upload failed"]]);
  check("spinner", await chips("states").first().locator("[data-attachment-preview] svg").count(), 1);

  // Removing in the browser: the chip goes, the page hears of it, focus stays in the list.
  await page.evaluate(() => {
    window.removed = [];
    document.addEventListener("bitop:attachment-remove", (event) => window.removed.push(event.detail.id));
  });
  const list = page.getByRole("list", { name: "Attached files" });
  const remove = list.getByRole("button", { name: "Remove photo.png" });
  await remove.focus();
  await page.keyboard.press("Enter");
  check("removed", await list.locator("[data-attachment-name]").allTextContents(), ["brief.docx", "data.csv"]);
  check("event", await page.evaluate(() => window.removed), ["f2"]);
  check("focusNext", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Remove data.csv");
  await page.keyboard.press("Space");
  check("focusPrevious", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Remove brief.docx");
  // Cancelling the event keeps the chip.
  await page.evaluate(() => document.addEventListener("bitop:attachment-remove", (event) => event.preventDefault(), { once: true }));
  await page.keyboard.press("Enter");
  check("cancelled", await list.locator("li").count(), 1);
  await page.keyboard.press("Enter");
  check("empty", [await list.locator("li").count(), await list.isVisible()], [0, false]);
  check("focusAfterEmpty", await page.evaluate(() => document.activeElement.id), "after-removing");

  // Removing through a form: the button submits its value, and the server leaves the file out.
  const formChips = page.locator("[data-example=form] [data-attachment-name]");
  check("disabledRemove", await page.getByRole("button", { name: "Remove plan.pdf" }).isDisabled(), true);
  await Promise.all([page.waitForURL("**/c/attachments?remove=budget"), page.getByRole("button", { name: "Remove budget.xlsx" }).click()]);
  check("serverRemoved", await formChips.allTextContents(), ["plan.pdf"]);

  // Other strings.
  const german = page.getByRole("list", { name: "Anhänge" });
  check("strings", [await german.locator("[data-attachment-meta]").allTextContents(), await german.getByRole("button").getAttribute("aria-label"), await german.getByRole("button").getAttribute("title")], [
    ["Wird hochgeladen…", "Hochladen fehlgeschlagen"], "bild.png entfernen", "Entfernen",
  ]);
};
