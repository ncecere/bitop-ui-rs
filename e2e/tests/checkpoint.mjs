export default async ({ page, check }) => {
  const plain = page.locator("[data-example=plain] [data-checkpoint]");
  check("group", [await plain.getAttribute("role"), await plain.getAttribute("aria-label")], ["group", "Checkpoint 10:42"]);
  check("time", await plain.locator("time").getAttribute("datetime"), "2026-11-12T10:42");
  check("noButton", await plain.locator("button").count(), 0);
  check("icon", await plain.locator("[data-icon]").getAttribute("data-icon"), "bookmark");

  // Each restore button names its checkpoint.
  const form = page.locator("[data-example=form]");
  const buttons = form.getByRole("button");
  check("names", await buttons.evaluateAll((all) => all.map((b) => b.getAttribute("aria-label"))), ["Restore checkpoint 10:42", "Restore checkpoint 11:05"]);
  check("types", await buttons.evaluateAll((all) => all.map((b) => [b.type, b.name, b.value])), [["submit", "checkpoint", "c-41"], ["submit", "checkpoint", "c-42"]]);

  // The keyboard submits the form with the checkpoint's value.
  await page.getByRole("button", { name: "Restore checkpoint 11:05" }).focus();
  await Promise.all([page.waitForURL(/checkpoint=c-42/), page.keyboard.press("Enter")]);
  check("submitted", await page.locator("[data-restored]").textContent(), "Restored c-42");

  // Without a name it sends an event with the checkpoint's id.
  await page.evaluate(() => {
    window.restored = [];
    document.addEventListener("bitop:checkpoint-restore", (event) => window.restored.push([event.target.id, event.detail.id]));
  });
  const scripted = page.getByRole("button", { name: "Restore before the refactor" });
  check("scriptType", await scripted.getAttribute("type"), "button");
  await scripted.focus();
  await page.keyboard.press("Space");
  check("event", await page.evaluate(() => window.restored), [["script-checkpoint", "before-refactor"]]);
  check("staysOnPage", page.url().includes("before-refactor"), false);
  check("disabled", await page.getByRole("button", { name: "Restore checkpoint 09:15" }).isDisabled(), true);

  // Strings and the order of the parts in the name.
  check("german", await page.locator("[data-example=strings] button").getAttribute("aria-label"), "prüfpunkt 10:42 Wiederherstellen");

  // Lines on both sides, in either direction.
  const lines = async () => plain.evaluate((root) => [...root.children].map((c) => Math.round(c.getBoundingClientRect().left)));
  const ltr = await lines();
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const rtl = await lines();
  check("rtlMirrors", ltr[0] < ltr[1] && rtl[0] > rtl[1], true);
};
