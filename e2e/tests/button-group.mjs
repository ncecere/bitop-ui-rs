export default async ({ page, check, button }) => {
  const groups = page.locator("main [role=group]");
  check("named", await groups.evaluateAll((all) => all.map((g) => g.getAttribute("aria-label"))), [
    "Pagination", "Save options", "Invite by email", "Reorder", "Editor toolbar", "History", "Clipboard",
  ]);

  // Rounded, because Firefox lays out on fractions of a pixel.
  const box = (locator) =>
    locator.evaluate((e) => Object.fromEntries(Object.entries(e.getBoundingClientRect().toJSON()).map(([k, v]) => [k, Math.round(v)])));
  const previous = await box(button("Previous"));
  const text = await box(page.getByText("Page 2 of 9"));
  const next = await box(button("Next"));
  // Neighbours overlap by one pixel, so that their borders are one line.
  check("joined", [text.left - previous.right, next.left - text.right], [-1, -1]);
  check("sameHeight", [text.height, next.height], [previous.height, previous.height]);
  const radii = (locator) =>
    locator.evaluate((e) => {
      const s = getComputedStyle(e);
      return [s.borderStartStartRadius !== "0px", s.borderStartEndRadius !== "0px"];
    });
  check("outerCornersOnly", [await radii(button("Previous")), await radii(page.getByText("Page 2 of 9")), await radii(button("Next"))], [
    [true, false], [false, false], [false, true],
  ]);

  // The buttons keep their own keyboard behaviour: Tab moves between them.
  await button("Previous").focus();
  await page.keyboard.press("Tab");
  check("chevrons", [await button("Previous").locator("[data-icon]").getAttribute("data-icon"), await button("Next").locator("[data-icon]").getAttribute("data-icon")], ["chevron-left", "chevron-right"]);
  check("tabMoves", await page.evaluate(() => document.activeElement.textContent.trim()), "Next");

  const separator = page.locator("main [role=separator]");
  check("separator", [await separator.getAttribute("aria-orientation"), (await box(separator)).width], ["vertical", 1]);
  const save = await box(button("Save"));
  check("separatorFillsHeight", (await box(separator)).height, save.height);

  const email = await box(page.getByLabel("Email address"));
  const invite = await box(button("Invite"));
  check("inputGrows", email.width > 150 && invite.left - email.right === -1);

  const moveUp = await box(button("Move up"));
  const moveDown = await box(button("Move down"));
  check("vertical", [moveDown.left, moveDown.top - moveUp.bottom], [moveUp.left, -1]);

  const redo = await box(button("Redo"));
  const cut = await box(button("Cut"));
  check("spaced", cut.left - redo.right, 8);
  check("disabled", await button("Paste").isDisabled());
};
