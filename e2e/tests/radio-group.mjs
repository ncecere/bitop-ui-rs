export default async ({ page, check, setDir }) => {
  const radio = (name) => page.getByRole("radio", { name, exact: true });
  const formValue = () => page.evaluate(() => new FormData(document.getElementById("plan-form")).get("plan"));

  const plan = page.getByRole("radiogroup", { name: "Plan" });
  check("group", await plan.count(), 1);
  check(
    "groupDescribed",
    await plan.evaluate((g) => document.getElementById(g.getAttribute("aria-describedby")).textContent.trim()),
    "You can change this later.",
  );
  check("initialValue", await formValue(), "free");

  // The radio pattern: arrow keys move and select, skip a disabled radio and wrap.
  await page.getByRole("radio", { name: "Free" }).focus();
  await page.keyboard.press("ArrowDown");
  check("arrowSelects", [await page.evaluate(() => document.activeElement.value), await formValue()], ["team", "team"]);
  await page.keyboard.press("ArrowDown");
  check("skipsDisabled", await formValue(), "custom");
  await page.keyboard.press("ArrowDown");
  check("wraps", await formValue(), "free");
  await page.keyboard.press("ArrowUp");
  check("arrowUp", await formValue(), "custom");

  // One tab stop for the group: Tab leaves it.
  await page.keyboard.press("Tab");
  check("tabLeaves", await page.evaluate(() => document.activeElement.textContent.trim()), "Save");

  await page.getByText("For up to 20 people.").click();
  check("descriptionSelects", await formValue(), "team");
  check("disabledRadio", await page.getByRole("radio", { name: "Enterprise" }).isDisabled());
  await page.getByText("Enterprise").click({ force: true });
  check("disabledStays", await formValue(), "team");

  await page.getByText("Roomy").click();
  check("labelSelects", await radio("Roomy").isChecked());
  check("oneChecked", await radio("Normal").isChecked(), false);
  await page.keyboard.press("ArrowRight");
  check("arrowRightWraps", await radio("Compact").isChecked());
  await setDir("rtl");
  await page.keyboard.press("ArrowRight");
  check("rtlArrows", await radio("Roomy").isChecked());
  await setDir("ltr");

  // A card is selected anywhere on its surface.
  const card = page.locator("label", { hasText: "Best answers." });
  const cardBox = await card.boundingBox();
  await page.mouse.click(cardBox.x + cardBox.width - 4, cardBox.y + cardBox.height - 4);
  check("cardSelects", await page.getByRole("radio", { name: "Large" }).isChecked());
  check("cardIsWide", cardBox.width > 150);

  const region = page.getByRole("radiogroup", { name: "Region" });
  check(
    "error",
    await region.evaluate((g) => [g.getAttribute("aria-invalid"), document.getElementById(g.getAttribute("aria-describedby")).textContent.trim()]),
    ["true", "Choose a region."],
  );
  check(
    "disabledGroup",
    await page.getByRole("radiogroup", { name: "Billing" }).locator("input").evaluateAll((all) => all.every((r) => r.matches(":disabled"))),
  );
  const size = await page.getByRole("radio", { name: "Free" }).boundingBox();
  check("size", [size.width, size.height], [16, 16]);
};
