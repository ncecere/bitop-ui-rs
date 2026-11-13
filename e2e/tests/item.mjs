export default async ({ page, check, focused, setDir }) => {
  const list = (name) => page.getByRole("list", { name });
  // A group is a list; each row is in a list item and a separator is hidden.
  check("listItems", [await list("Integrations").getByRole("listitem").count(), await list("Settings").getByRole("listitem").count()], [3, 2]);
  const rows = list("Integrations").locator("[role=listitem] > [data-variant]");
  check("variants", await rows.evaluateAll((all) => all.map((r) => r.dataset.variant)), ["default", "outline", "muted"]);
  const styles = await rows.evaluateAll((all) => all.map((r) => [getComputedStyle(r).borderTopColor, getComputedStyle(r).backgroundColor]));
  check("outlineHasBorder", styles[1][0] !== styles[0][0]);
  check("mutedHasFill", styles[2][1] !== styles[0][1]);

  // Sizes change the padding, and the gap of the group.
  const heights = await list("Sizes").locator("[data-size]").evaluateAll((all) => all.map((r) => Math.round(r.getBoundingClientRect().height)));
  check("sizes", heights[0] > heights[1] && heights[1] > heights[2]);
  check("sizeNames", await list("Sizes").locator("[data-size]").evaluateAll((all) => all.map((r) => r.dataset.size)), ["md", "sm", "xs"]);

  // The media is at the start, the actions at the end, the text between.
  const first = rows.first();
  const order = async () => {
    const media = await first.locator("[data-icon=inbox]").boundingBox();
    const title = await first.getByText("Default", { exact: true }).boundingBox();
    const action = await first.getByRole("button").boundingBox();
    return [media.x < title.x, title.x < action.x];
  };
  check("order", await order(), [true, true]);
  const iconBox = await first.locator("[data-icon=inbox]").boundingBox();
  check("iconSize", [Math.round(iconBox.width), Math.round(iconBox.height)], [16, 16]);

  // A link row is a link in a list item, reached by Tab and followed by Enter.
  const links = list("Settings").getByRole("link");
  check("links", await links.evaluateAll((all) => all.map((a) => [a.tagName, a.getAttribute("href"), a.parentElement.getAttribute("role")])), [["A", "#billing", "listitem"], ["A", "#members", "listitem"]]);
  await links.first().focus();
  check("linkFocus", (await focused()).startsWith("Billing"));
  await page.keyboard.press("Tab");
  check("tabToNext", (await focused()).startsWith("Members"));
  await page.keyboard.press("Enter");
  check("enterFollows", await page.evaluate(() => location.hash), "#members");
  await links.first().hover();
  check("hoverUnderlinesTitle", await links.first().locator("[data-item-title]").evaluate((t) => getComputedStyle(t).textDecorationLine), "underline");
  check("separatorHidden", await list("Settings").locator("[role=listitem][aria-hidden=true]").count(), 1);

  // The description is clamped to two lines; header and footer take a full row.
  const item = page.locator("main [data-variant=outline]").last();
  const description = item.locator("[data-item-description]");
  const clamp = await description.evaluate((p) => [p.scrollHeight > p.clientHeight, Math.round(p.clientHeight / parseFloat(getComputedStyle(p).lineHeight))]);
  check("clamped", clamp, [true, 2]);
  const box = await item.boundingBox();
  const header = await item.getByText("Pinned").locator("..").boundingBox();
  const footer = await item.getByText("Joined in 1843").locator("..").boundingBox();
  const title = await item.getByText("Ada Lovelace", { exact: true }).boundingBox();
  check("headerAndFooter", [header.width > box.width - 40, header.y < title.y, footer.y > title.y, footer.width > box.width - 40], [true, true, true, true]);
  const image = await item.getByRole("img", { name: "Ada Lovelace" }).locator("..").boundingBox();
  check("imageMedia", [Math.round(image.width), Math.round(image.height)], [40, 40]);

  await setDir("rtl");
  check("rtlOrder", await order(), [false, false]);
};
