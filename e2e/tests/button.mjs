export default async ({ page, check, button }) => {
  check("variants", await page.locator("main button[data-variant]").evaluateAll((all) => [...new Set(all.map((b) => b.dataset.variant))]), ["primary", "secondary", "ghost", "danger", "link"]);
  check("typeIsButton", await button("Create project").getAttribute("type"), "button");
  check("disabled", await button("Disabled").isDisabled());
  const heights = await page.locator("main button[data-size]:not([data-icon-only])").evaluateAll((all) => Object.fromEntries(all.map((b) => [b.dataset.size, b.getBoundingClientRect().height])));
  check("sizes", heights, { md: 36, sm: 32 });

  // Loading: busy, disabled, and a spinner drawn before the text.
  const uploading = button("Uploading");
  check("loading", [await uploading.getAttribute("aria-busy"), await uploading.isDisabled(), await uploading.evaluate((b) => getComputedStyle(b, "::before").animationName.includes("spin"))], ["true", true, true]);

  // Icon buttons are square and named by their label.
  const edit = button("Edit");
  const box = await edit.boundingBox();
  check("iconOnly", [await edit.getAttribute("aria-label"), Math.round(box.width), Math.round(box.height)], ["Edit", 36, 36]);
  const add = button("Add");
  const small = await add.boundingBox();
  check("iconOnlySmall", [Math.round(small.width), Math.round(small.height)], [32, 32]);

  // A script turns loading on and off.
  const save = button("Save changes");
  await save.click();
  const saving = page.locator("main [data-gallery-loading]");
  check("setLoading", [await saving.getAttribute("aria-busy"), await saving.isDisabled(), (await saving.textContent()).trim()], ["true", true, "Saving…"]);
  await page.waitForFunction(() => !document.querySelector("[data-gallery-loading]").hasAttribute("aria-busy"), null, { timeout: 4000 });
  check("loadingEnds", await saving.isDisabled(), false);

  const link = page.getByRole("link", { name: "Button rendered as a link" });
  check("link", Math.round((await link.boundingBox()).height), 36);
  const block = button("Continue").last();
  check("block", await block.evaluate((b) => b.getBoundingClientRect().width === b.parentElement.getBoundingClientRect().width), true);
};
