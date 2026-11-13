export default async ({ page, check, axe, button, focused, settled }) => {
  const view = page.viewportSize();
  const box = async (locator) => {
    const b = await locator.boundingBox();
    return { x: Math.round(b.x), y: Math.round(b.y), width: Math.round(b.width), height: Math.round(b.height) };
  };
  const isOpen = (d) => d.evaluate((el) => el.open);

  const dialog = page.locator("dialog", { hasText: "Projects group deployments" });
  await button("New project").click();
  check("open", await dialog.evaluate((d) => d.open && d.matches(":modal")));
  await settled(dialog);
  const b = await box(dialog);
  // Near the top, as upstream: max(4rem, 10vh) down, centred across, 32rem wide.
  check("placement", [b.y, Math.abs(b.x * 2 + b.width - view.width) < 2, b.width], [Math.round(Math.max(64, view.height / 10)), true, 512]);
  check("name", await page.getByRole("dialog", { name: "New project" }).count(), 1);
  check("described", await dialog.evaluate((d) => document.getElementById(d.getAttribute("aria-describedby"))?.textContent), "Projects group deployments, domains and environment variables.");
  check("focusFirstField", await page.evaluate(() => document.activeElement.placeholder), "marketing-site");
  check("backdropBlur", await dialog.evaluate((d) => getComputedStyle(d, "::backdrop").backdropFilter), "blur(2px)");
  // The footer is a tinted band with a border on top.
  const footer = dialog.locator("button", { hasText: "Cancel" }).locator("..");
  check(
    "footerBand",
    await footer.evaluate((f) => {
      const probe = document.createElement("div");
      probe.style.background = "var(--color-surface-sunken)";
      document.body.append(probe);
      const sunken = getComputedStyle(probe).backgroundColor;
      probe.remove();
      const style = getComputedStyle(f);
      return [style.backgroundColor === sunken, style.borderTopWidth];
    }),
    [true, "1px"],
  );
  // The × is in the corner, and last, so that focus did not start on it.
  const close = dialog.getByRole("button", { name: "Close", exact: true });
  const c = await box(close);
  check("closeInCorner", [c.x + c.width, c.y], [b.x + b.width - 16, b.y + 16]);
  check("closeIsLast", await dialog.evaluate((d) => [...d.querySelectorAll("button")].at(-1).getAttribute("aria-label")), "Close");
  check("axeOpen", await axe(), []);
  for (let i = 0; i < 6; i++) await page.keyboard.press("Tab");
  check("focusTrapped", await page.evaluate(() => !!document.activeElement.closest("dialog")));
  await page.keyboard.press("Escape");
  await page.waitForTimeout(300);
  check("closedByEsc", !(await isOpen(dialog)));
  check("focusReturned", await focused(), "New project");
  await button("New project").click();
  await close.click();
  check("closedByX", !(await isOpen(dialog)));
  await button("New project").click();
  await page.mouse.click(5, view.height - 5);
  await page.waitForTimeout(300);
  check("closedByBackdrop", !(await isOpen(dialog)));
  await button("New project").click();
  await dialog.getByRole("button", { name: "Create" }).click();
  check("createCloses", [await isOpen(dialog), await page.locator("[data-toast]", { hasText: "Project created" }).count()], [false, 1]);

  // Sizes.
  const small = page.locator("dialog", { hasText: "Small dialog" });
  await button("Small").click();
  await settled(small);
  check("small", [(await box(small)).width, await small.evaluate((d) => d.getAttribute("data-size"))], [416, "sm"]);
  check("emptyFooterHidden", await small.evaluate((d) => [...d.children].filter((c) => getComputedStyle(c).display === "none").length), 1);
  await page.keyboard.press("Escape");
  const large = page.locator("dialog", { hasText: "Large dialog" });
  await button("Large").click();
  await settled(large);
  check("large", (await box(large)).width, 672);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(300);

  // final_focus: focus moves there when the dialog closes, not back to the trigger.
  await page.evaluate(() => {
    document.querySelector("button[commandfor][aria-haspopup]").id = "final-target";
    [...document.querySelectorAll("dialog")].find((d) => d.textContent.includes("Small dialog")).dataset.finalFocus = "final-target";
  });
  await button("Small").click();
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => document.activeElement.id === "final-target");
  check("finalFocus", await page.evaluate(() => document.activeElement.textContent.trim()), "New project");
  await page.evaluate(() => delete [...document.querySelectorAll("dialog")].find((d) => d.textContent.includes("Small dialog")).dataset.finalFocus);

  // Alert dialog.
  const alert = page.locator("dialog", { hasText: "Delete “marketing-site”?" });
  const error = alert.locator("[data-dialog-error]");
  const confirm = alert.getByRole("button", { name: "Delete project", exact: true });
  await button("Delete project…").click();
  await settled(alert);
  check("alertRole", await page.getByRole("alertdialog", { name: "Delete “marketing-site”?" }).count(), 1);
  check("alertDescribed", await alert.evaluate((d) => document.getElementById(d.getAttribute("aria-describedby"))?.tagName), "P");
  check("alertFocusCancel", await focused(), "Cancel");
  check("alertNoX", await alert.getByRole("button", { name: "Close" }).count(), 0);
  check("alertSmall", (await box(alert)).width, 416);
  check("alertNoBody", await error.evaluate((e) => getComputedStyle(e.parentElement).display), "none");
  check("confirmDanger", await confirm.getAttribute("data-variant"), "danger");
  await page.mouse.click(5, view.height - 5);
  await page.waitForTimeout(300);
  check("alertIgnoresBackdrop", await isOpen(alert));
  await confirm.click();
  check("busy", [await confirm.getAttribute("aria-busy"), await confirm.isDisabled()], ["true", true]);
  await error.waitFor({ state: "visible" });
  check("error", [await error.getAttribute("role"), (await error.textContent()).trim(), await confirm.getAttribute("aria-busy")], ["alert", "The project still has an active production domain. Remove it first.", null]);
  await settled(alert);
  check("axeAlert", await axe(), []);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(300);
  check("alertClosedByEsc", !(await isOpen(alert)));
  check("alertFocusReturned", await focused(), "Delete project…");
  await button("Delete project…").click();
  check("errorCleared", await error.isHidden());
  await alert.getByRole("button", { name: "Cancel" }).click();
  check("closedByCancel", !(await isOpen(alert)));
};
