export default async ({ page, check, button }) => {
  const limit = page.getByRole("spinbutton", { name: "Monthly limit" });
  const formValue = (id, name) => page.evaluate(([i, n]) => new FormData(document.getElementById(i)).get(n), [id, name]);
  const now = (locator) => locator.getAttribute("aria-valuenow");

  // Grouped while it is not being edited; the form gets the plain number.
  check("grouped", await limit.inputValue(), "50,000");
  check("formPlain", await formValue("limit-form", "limit"), "50000");
  check("aria", [await now(limit), await limit.getAttribute("aria-valuemin"), await limit.getAttribute("aria-valuemax")], ["50000", "0", "100000"]);
  check("inputMode", await limit.getAttribute("inputmode"), "numeric");

  await limit.focus();
  check("plainOnFocus", await limit.inputValue(), "50000");
  await page.keyboard.press("ArrowUp");
  check("arrowUp", [await limit.inputValue(), await now(limit)], ["51000", "51000"]);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  check("arrowDown", await limit.inputValue(), "49000");
  await page.keyboard.press("PageUp");
  check("pageUp", await limit.inputValue(), "59000");
  await page.keyboard.press("PageDown");
  check("pageDown", await limit.inputValue(), "49000");
  await page.keyboard.press("End");
  check("endIsMax", await limit.inputValue(), "100000");
  await page.keyboard.press("ArrowUp");
  check("clampsAtMax", await limit.inputValue(), "100000");
  await page.keyboard.press("Home");
  check("homeIsMin", [await limit.inputValue(), await now(limit)], ["0", "0"]);

  await limit.fill("1234567");
  check("typing", [await now(limit), await formValue("limit-form", "limit")], ["1234567", "1234567"]);
  await page.keyboard.press("Tab");
  check("groupsOnBlur", await limit.inputValue(), "1,234,567");

  // The step buttons are not tab stops; the arrow keys do the same.
  const steps = page.locator("#limit-form button");
  check("stepButtons", await steps.evaluateAll((all) => all.map((b) => `${b.getAttribute("aria-label")}:${b.tabIndex}`)), ["Decrease:-1", "Increase:-1"]);
  await limit.fill("5000");
  await steps.last().click();
  check("increase", await formValue("limit-form", "limit"), "6000");
  await steps.first().click();
  await steps.first().click();
  check("decrease", [await formValue("limit-form", "limit"), await now(limit)], ["4000", "4000"]);
  await page.locator("main h1").click();
  check("groupedAfterStep", await limit.inputValue(), "4,000");

  await limit.fill("abc");
  await page.keyboard.press("Tab");
  check("notANumber", [await limit.inputValue(), await now(limit), await formValue("limit-form", "limit")], ["abc", null, "abc"]);
  await limit.fill("");
  await page.keyboard.press("ArrowUp");
  check("stepFromEmpty", await limit.inputValue(), "1000");

  // German: a full stop groups and a comma is the decimal mark.
  const price = page.getByRole("spinbutton", { name: "Price in euros" });
  check("german", [await price.inputValue(), await formValue("price-form", "price")], ["1.234,5", "1234.5"]);
  await price.focus();
  check("germanOnFocus", await price.inputValue(), "1234,5");
  await page.keyboard.press("ArrowUp");
  check("germanStep", [await price.inputValue(), await formValue("price-form", "price")], ["1235,0", "1235.0"]);
  await price.fill("0,25");
  check("germanTyped", await formValue("price-form", "price"), "0.25");
  check("decimalInputMode", await price.getAttribute("inputmode"), "decimal");

  const seats = page.getByRole("spinbutton", { name: "Seats" });
  check("noSteppers", await seats.locator("xpath=..").locator("button").count(), 0);
  await seats.focus();
  await page.keyboard.press("End");
  check("endWithoutMaxKeepsValue", await seats.inputValue(), "12");

  const quota = page.getByRole("spinbutton", { name: "Quota, disabled" });
  check("disabled", [await quota.isDisabled(), await button("Increase").last().isDisabled()], [true, true]);
  check("unitIsDecorative", await page.getByText("tokens", { exact: true }).getAttribute("aria-hidden"), "true");
};
