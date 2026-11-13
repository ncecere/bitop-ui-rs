export default async ({ page, check }) => {
  const avatar = (name) => page.getByRole("img", { name });
  check("keepsGoodImage", await avatar("With Picture").locator("img").count(), 1);
  check("dropsBrokenImage", await avatar("Broken Picture").locator("img").count(), 0);
  check("initials", (await avatar("Broken Picture").textContent()).trim(), "BP");
  const sizes = await page.locator("main [role=img][data-size]").evaluateAll((all) => all.slice(0, 5).map((a) => Math.round(a.getBoundingClientRect().width)));
  check("sizes", sizes, [20, 24, 32, 40, 56]);

  // The square shape rounds its corners by a quarter of the size.
  const acme = avatar("Acme Inc");
  check("square", await acme.evaluate((a) => [a.dataset.shape, getComputedStyle(a).borderTopLeftRadius]), ["square", "10px"]);

  // The tint follows bitop-ui's hash of the name: the same name, the same colour.
  const hues = await page.locator("main [role=img][data-hue]").evaluateAll((all) => Object.fromEntries(all.map((a) => [a.getAttribute("aria-label"), a.dataset.hue])));
  check("hues", [hues["Ada Lovelace"], hues["Grace Hopper"], hues["Alan Turing"], hues["Acme Inc"], hues["Cher"]], ["6", "3", "4", "3", "3"]);
  const background = (name) => avatar(name).evaluate((a) => getComputedStyle(a).backgroundColor);
  check("sameNameSameTint", (await background("Grace Hopper")) === (await background("Acme Inc")));
  check("otherTint", (await background("Grace Hopper")) !== (await background("Alan Turing")));

  check("initialsSet", await page.locator("main [data-hue] > span[aria-hidden]").allTextContents().then((all) => all.slice(5, 9)), ["IH", "GD", "LB", "C"]);

  // Decorative: hidden from assistive technology, no image role.
  const decorative = page.locator("main [data-hue][aria-hidden=true]");
  check("decorative", await decorative.evaluate((a) => [a.getAttribute("role"), a.getAttribute("aria-label")]), [null, null]);
};
