export default async ({ page, check, button }) => {
  const site = page.getByLabel("Website");
  const group = site.locator("xpath=..");
  const style = (locator, property) => locator.evaluate((e, p) => getComputedStyle(e)[p], property);
  const token = (name) =>
    page.evaluate((n) => {
      const probe = document.body.appendChild(document.createElement("i"));
      probe.style.color = `var(${n})`;
      const colour = getComputedStyle(probe).color;
      probe.remove();
      return colour;
    }, name);

  check("controlHasNoBorder", [await style(site, "borderTopWidth"), await style(group, "borderTopWidth")], ["0px", "1px"]);
  const box = await group.boundingBox();
  check("height", Math.round(box.height), 36);
  check("smallHeight", (await page.getByLabel("Search").locator("xpath=..").boundingBox()).height, 32);

  // A click on an addon focuses the control; a click on a button in one does not.
  await page.getByText("https://").click();
  check("addonFocusesControl", await page.evaluate(() => document.activeElement.name), "site");
  // The border colour is a transition; wait for it to end.
  await page.waitForFunction(
    ([element, colour]) => getComputedStyle(element).borderTopColor === colour,
    [await group.elementHandle(), await token("--color-focus-ring")],
  ).catch(() => {});
  check("groupShowsFocus", await style(group, "borderTopColor"), await token("--color-focus-ring"));
  check("controlHasNoOutline", await style(site, "outlineStyle"), "none");
  await page.keyboard.type("example.com");
  check("types", await site.inputValue(), "example.com");
  await page.keyboard.press("Tab");
  check("tabReachesButton", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Copy URL");
  await page.locator("main h1").click();
  await button("Copy URL").click();
  check("buttonKeepsFocus", await page.evaluate(() => document.activeElement.name ?? null) !== "site");

  // The addons sit either side of the control, whatever their order in the markup.
  const prefix = await page.getByText("https://").boundingBox();
  const input = await site.boundingBox();
  const copy = await button("Copy URL").boundingBox();
  check("order", prefix.x < input.x && input.x + input.width <= copy.x);
  check("buttonInside", copy.height < box.height && copy.x + copy.width < box.x + box.width);

  const amount = page.getByLabel("Amount");
  check("invalidBorder", await style(amount.locator("xpath=.."), "borderTopColor"), await token("--color-danger"));

  const message = page.getByLabel("Message");
  const toolbar = await page.getByText("0 / 280").locator("xpath=..").boundingBox();
  const area = await message.boundingBox();
  check("toolbarBelow", toolbar.y >= area.y + area.height && Math.abs(toolbar.width - (await message.locator("xpath=..").boundingBox()).width) <= 2);
  await page.getByText("0 / 280").click();
  check("toolbarFocusesTextarea", await page.evaluate(() => document.activeElement.name), "message");

  check("disabled", await page.getByLabel("Handle, disabled").isDisabled());
  check(
    "readOnlyTint",
    await style(page.getByLabel("Handle, read-only").locator("xpath=.."), "backgroundColor") !== (await style(group, "backgroundColor")),
  );
};
