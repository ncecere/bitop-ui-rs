export default async ({ page, check, focused, setDir }) => {
  const markers = page.locator("main [data-variant]");
  check("variants", await markers.evaluateAll((all) => all.map((m) => `${m.tagName}:${m.dataset.variant}`)), ["DIV:default", "DIV:border", "DIV:separator", "A:default", "DIV:default"]);
  check("iconsHidden", await page.locator("main span[aria-hidden=true] > [data-icon]").count(), 4);
  const icon = await markers.first().locator("[data-icon]").boundingBox();
  check("iconSize", [Math.round(icon.width), Math.round(icon.height)], [16, 16]);

  // Border: a hairline under the row.
  check("border", await markers.nth(1).evaluate((m) => getComputedStyle(m).borderBottomWidth), "1px");

  // Separator: the text is centred between two lines.
  const separator = markers.nth(2);
  const sep = await separator.boundingBox();
  const text = await separator.getByText("Today").boundingBox();
  check("separatorCentred", Math.abs(text.x + text.width / 2 - (sep.x + sep.width / 2)) <= 1);
  const lines = await separator.evaluate((m) => ["::before", "::after"].map((p) => [getComputedStyle(m, p).height, parseFloat(getComputedStyle(m, p).width) > 100]));
  check("separatorLines", lines, [["1px", true], ["1px", true]]);

  // A link marker is reached by Tab and followed by Enter.
  const link = page.getByRole("link", { name: "Open run 42" });
  await link.focus();
  check("linkFocus", await focused(), "Open run 42");
  check("linkUnderlined", await link.evaluate((a) => getComputedStyle(a).textDecorationLine), "underline");
  await page.keyboard.press("Enter");
  check("enterFollows", await page.evaluate(() => location.hash), "#run-42");
  await page.keyboard.press("Tab");
  check("tabToInnerLink", await focused(), "Details");
  check("status", await page.getByRole("status").filter({ hasText: "Compacting" }).count(), 1);

  // Right to left: the icon is on the right of the text.
  const first = markers.first();
  const side = async () => (await first.locator("[data-icon]").boundingBox()).x < (await first.getByText("Explored 4 files").boundingBox()).x;
  check("ltr", await side());
  await setDir("rtl");
  check("rtl", await side(), false);
};
