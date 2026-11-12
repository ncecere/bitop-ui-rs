export default async ({ page, check }) => {
  await page.getByText("Display name", { exact: true }).click();
  check("labelFocusesInput", await page.evaluate(() => document.activeElement.name), "name");
  check(
    "described",
    await page.getByLabel("Display name").evaluate((i) => document.getElementById(i.getAttribute("aria-describedby")).textContent),
    "Shown next to your messages.",
  );
  check(
    "errorDescribes",
    await page.getByLabel("Email").evaluate((i) => [i.getAttribute("aria-invalid"), document.getElementById(i.getAttribute("aria-describedby")).textContent]),
    ["true", "Enter a full email address."],
  );
  check("height", await page.getByLabel("Display name").evaluate((i) => i.getBoundingClientRect().height), 36);
  const notes = page.getByLabel("Custom instructions");
  const before = await notes.evaluate((t) => t.getBoundingClientRect().height);
  await notes.fill("one\ntwo\nthree\nfour\nfive\nsix");
  // field-sizing is not in every engine yet; where it is, the textarea grows.
  const grows = await page.evaluate(() => CSS.supports("field-sizing: content"));
  check("textareaGrows", (await notes.evaluate((t) => t.getBoundingClientRect().height)) > before, grows);
};
