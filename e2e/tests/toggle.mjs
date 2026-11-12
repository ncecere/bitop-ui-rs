export default async ({ page, check, button }) => {
  const pressed = (name) => button(name).getAttribute("aria-pressed");
  const formValue = () => page.evaluate(() => [...new FormData(document.getElementById("toggle-form"))].map((pair) => pair.join("=")));

  check("initial", [await pressed("Ghost"), await pressed("Ghost, pressed")], ["false", "true"]);
  await button("Ghost").click();
  check("clickPresses", await pressed("Ghost"), "true");
  await button("Ghost").click();
  check("clickReleases", await pressed("Ghost"), "false");

  await button("Outline").focus();
  await page.keyboard.press("Space");
  check("space", await pressed("Outline"), "true");
  await page.keyboard.press("Enter");
  check("enter", await pressed("Outline"), "false");

  await button("Disabled").click({ force: true });
  check("disabled", [await button("Disabled").isDisabled(), await pressed("Disabled")], [true, "false"]);

  check("iconOnlyNamed", await button("Pin").count(), 1);
  const pin = await button("Pin").boundingBox();
  const small = await button("Search in files").boundingBox();
  check("sizes", [pin.width, pin.height, small.width, small.height], [36, 36, 32, 32]);

  // Pressed is shown by more than colour: the ring changes too.
  const ring = (name) => button(name).evaluate((b) => getComputedStyle(b).boxShadow);
  check("pressedRing", (await ring("Outline")) !== (await ring("Outline, pressed")));

  // A pressed toggle with a name is submitted; one that is not pressed is left out.
  check("formInitial", await formValue(), ["wrap=yes"]);
  await button("Preview").click();
  check("formPressed", await formValue(), ["preview=on", "wrap=yes"]);
  await button("Wrap lines").click();
  check("formReleased", await formValue(), ["preview=on"]);
  check("notSubmit", await button("Preview").getAttribute("type"), "button");
};
