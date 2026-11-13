export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const view = page.viewportSize();
  const sheet = (text) => page.locator("dialog[data-sheet]", { hasText: text });
  const isOpen = (dialog) => dialog.evaluate((d) => d.open && d.matches(":modal"));
  const box = async (dialog) => {
    const b = await dialog.boundingBox();
    return { x: Math.round(b.x), y: Math.round(b.y), width: Math.round(b.width), height: Math.round(b.height) };
  };

  // From the right: a modal dialog on the edge, named and described.
  const filters = sheet("Narrow the list of deployments.");
  await button("Filters").click();
  check("open", await isOpen(filters));
  await settled(filters);
  check("right", await box(filters), { x: view.width - 384, y: 0, width: 384, height: view.height });
  check("name", await page.getByRole("dialog", { name: "Filters" }).count(), 1);
  check("described", await filters.evaluate((d) => document.getElementById(d.getAttribute("aria-describedby")).textContent), "Narrow the list of deployments.");
  check("focusFirstField", await page.evaluate(() => document.activeElement.name), "name");
  check("axeOpen", await axe(), []);
  for (let i = 0; i < 6; i++) await page.keyboard.press("Tab");
  check("focusTrapped", await page.evaluate(() => !!document.activeElement.closest("dialog[data-sheet]")));
  await page.keyboard.press("Escape");
  check("closedByEsc", await filters.evaluate((d) => !d.open));
  check("focusReturned", await focused(), "Filters");

  await button("Filters").click();
  await page.mouse.click(5, 5);
  check("closedByBackdrop", await filters.evaluate((d) => !d.open));
  await button("Filters").click();
  // The × is last in the sheet, so that focus starts on the first field.
  check("closeIsLast", await filters.evaluate((d) => d.lastElementChild.getAttribute("aria-label")), "Close");
  await filters.getByRole("button", { name: "Close" }).click();
  check("closedByX", await filters.evaluate((d) => !d.open));
  await button("Filters").click();
  await filters.getByRole("button", { name: "Cancel" }).click();
  check("closedByFooter", await filters.evaluate((d) => !d.open));

  // The other edges.
  const left = sheet("A sheet on the left edge.");
  await button("Open left").click();
  await settled(left);
  check("left", await box(left), { x: 0, y: 0, width: 384, height: view.height });
  await page.keyboard.press("Escape");
  const top = sheet("A sheet on the top edge");
  await button("Open top").click();
  await settled(top);
  const topBox = await box(top);
  check("top", [topBox.x, topBox.y, topBox.width, topBox.height < view.height], [0, 0, view.width, true]);
  await page.keyboard.press("Escape");
  const bottom = sheet("A sheet on the bottom edge");
  await button("Open bottom").click();
  await settled(bottom);
  const bottomBox = await box(bottom);
  check("bottom", [bottomBox.x, bottomBox.y + bottomBox.height, bottomBox.width], [0, view.height, view.width]);
  check("hideClose", await bottom.getByRole("button", { name: "Close" }).count(), 0);
  check("focusFooterButton", await focused(), "Done");
  await page.keyboard.press("Escape");

  // Sizes.
  await button("Open small").click();
  await settled(sheet("Twenty rems wide."));
  check("small", (await box(sheet("Twenty rems wide."))).width, 320);
  check("emptyPartsHidden", await sheet("Twenty rems wide.").locator("> div").evaluateAll((all) => all.map((d) => getComputedStyle(d).display)), ["flex", "none", "none"]);
  await page.keyboard.press("Escape");
  await button("Open extra large").click();
  await settled(sheet("Forty-two rems wide"));
  check("extraLarge", (await box(sheet("Forty-two rems wide"))).width, 672);
  await page.keyboard.press("Escape");
  const full = sheet("A full-screen sheet");
  await button("Open full screen").click();
  await settled(full);
  check("full", await box(full), { x: 0, y: 0, width: view.width, height: view.height });
  check("fullCloseFirst", [await full.evaluate((d) => d.firstElementChild.getAttribute("aria-label")), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], ["Back", "Back"]);
  const back = await full.getByRole("button", { name: "Back" }).boundingBox();
  check("fullCloseAtStart", back.x < 40);
  await page.keyboard.press("Enter");
  check("fullClosed", await full.evaluate((d) => !d.open));
  check("fullFocusReturned", await focused(), "Open full screen");

  // Text to read: focus starts on the title, and the body can be scrolled from the keyboard.
  const terms = sheet("Last updated in November.");
  await button("Read the terms").click();
  await settled(terms);
  check("focusTitle", await page.evaluate(() => [document.activeElement.tagName, document.activeElement.textContent.trim()]), ["H2", "Terms of service"]);
  const body = terms.locator("[data-sheet-body]");
  check("scrollableBody", await body.evaluate((b) => [b.scrollHeight > b.clientHeight, b.tabIndex, b.getAttribute("role"), document.getElementById(b.getAttribute("aria-labelledby")).textContent.trim()]), [true, 0, "region", "Terms of service"]);
  await page.keyboard.press("Tab");
  check("bodyTakesFocus", await page.evaluate(() => document.activeElement.hasAttribute("data-sheet-body")));
  await page.keyboard.press("PageDown");
  await page.waitForFunction(() => document.activeElement.scrollTop > 0);
  check("keyboardScrolls", await body.evaluate((b) => b.scrollTop > 0));
  check("axeScrollable", await axe(), []);
  await page.keyboard.press("Tab");
  check("thenFooter", await focused(), "Accept");
  await page.keyboard.press("Enter");
  check("acceptCloses", await terms.evaluate((d) => !d.open));
  // A body with fields in it stays out of the tab order.
  check("bodyWithFieldsNotFocusable", await filters.locator("[data-sheet-body]").evaluate((b) => b.hasAttribute("tabindex")), false);

  // Right to left: the side stays, the contents mirror.
  await setDir("rtl");
  await button("Filters").click();
  await settled(filters);
  check("rtlSameSide", (await box(filters)).x, view.width - 384);
  const x = await filters.getByRole("button", { name: "Close" }).boundingBox();
  const title = await filters.locator("h2").boundingBox();
  check("rtlMirrors", x.x < title.x + title.width && x.x - (view.width - 384) < 40);
  await page.keyboard.press("Escape");
};
