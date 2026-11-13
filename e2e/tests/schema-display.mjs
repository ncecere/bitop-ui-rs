export default async ({ page, check, setDir }) => {
  const text = (locator) => locator.evaluate((e) => e.innerText.replace(/\s+/g, " ").trim());
  const endpoint = page.locator("#create-key");
  const rotation = (locator) =>
    locator.evaluate(async (icon) => {
      await Promise.all(icon.getAnimations().map((a) => a.finished));
      return getComputedStyle(icon).rotate;
    });

  // The header: the method in words, the path with its parameters set off.
  check("method", [await endpoint.locator("[data-method]").textContent(), await endpoint.locator("code[dir=ltr]").first().locator("span").allTextContents(), await text(endpoint.locator("code[dir=ltr]").first())], ["POST", ["{id}"], "/users/{id}/keys"]);
  check("description", await endpoint.locator("p").first().textContent(), "Create an API key for a user.");

  // Sections are disclosures, open by default; the parameters' count is shown.
  const sections = endpoint.locator("details[data-schema-section]");
  check("sections", await sections.evaluateAll((all) => all.map((d) => [d.querySelector("summary").textContent.replace(/\s+/g, " ").trim(), d.open])), [
    ["Parameters 2", true],
    ["Request body", true],
    ["Response", true],
  ]);
  const parameters = endpoint.locator("li[data-parameter]");
  check("parameters", [await text(parameters.first()), await parameters.first().locator("[data-location]").textContent(), await parameters.first().locator("[data-required]").textContent(), await parameters.nth(1).locator("[data-required]").count()], ["id string path required The user's ID.", "path", "required", 0]);
  // The keyboard closes a section.
  const params = sections.first();
  await params.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => !document.querySelector("#create-key details").open);
  check("closeSection", await params.evaluate((d) => d.open), false);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector("#create-key details").open);

  // Properties: types, formats, enum values; nested objects are buttons that start open
  // down to the second level.
  const prop = (name) => endpoint.locator(`li[data-property="${name}"]`);
  check("scopes", await text(prop("scopes")), 'scopes string[] One of: "read", "write"');
  check("key", await text(prop("key")), "key string<password> required");
  const limits = prop("limits").locator("> button");
  check("limits", [await limits.getAttribute("aria-expanded"), await text(limits), await text(prop("limits").locator("> span"))], ["true", "limits object", "Rate limits for the key."]);
  const window = prop("window").locator("> button");
  check("depth", [await prop("burst").locator("> button").getAttribute("aria-expanded"), await window.getAttribute("aria-expanded")], ["true", "false"]);
  check("depthList", await endpoint.locator("ul[data-depth]").evaluateAll((all) => all.map((u) => u.dataset.depth)), ["0", "1", "2", "3", "0"]);
  // The keyboard opens the closed one, and its chevron turns.
  const windowPanel = page.locator(`#${await window.getAttribute("aria-controls")}`);
  check("windowHidden", await windowPanel.isHidden(), true);
  await window.focus();
  await page.keyboard.press("Enter");
  await windowPanel.waitFor({ state: "visible" });
  check("windowOpen", [await window.getAttribute("aria-expanded"), await text(windowPanel), await rotation(window.locator("[data-icon]"))], ["true", "seconds integer", "90deg"]);
  await limits.focus();
  await page.keyboard.press("Space");
  await page.locator(`#${await limits.getAttribute("aria-controls")}`).waitFor({ state: "hidden" });
  check("limitsClosed", [await limits.getAttribute("aria-expanded"), await prop("limits").locator("> span").isVisible()], ["false", true]);

  // A JSON Schema turned into rows: union types, enums, an array of objects.
  const schema = page.locator("#invoice");
  check("title", await schema.locator("p").first().textContent(), "Invoice");
  check("schemaRows", await schema.locator("ul[data-depth='0'] > li").evaluateAll((all) => all.map((li) => li.dataset.property)), ["id", "status", "total", "lines", "paid_at"]);
  const sp = (name) => schema.locator(`li[data-property="${name}"]`);
  check("schemaText", [await text(sp("id")), await text(sp("status")), await text(sp("paid_at")), await text(sp("lines").locator("> button"))], [
    "id string<uuid> required The invoice's ID.",
    'status enum Where the invoice is. One of: "draft", "open", "paid"',
    "paid_at string | null<date-time>",
    "lines object[]",
  ]);
  check("items", await sp("lines").locator("li").evaluateAll((all) => all.map((li) => li.textContent.replace(/\s+/g, " ").trim())), ["sku string required", "quantity integer"]);
  check("example", [await schema.locator("pre code").getAttribute("class"), await schema.locator("[data-code-block]").getAttribute("data-language")], ["language-json", "json"]);
  check("propertiesLabel", await text(schema.locator("summary")), "Properties");

  // Every method has its own colour and its name.
  const methods = page.locator("[data-example=methods] [data-method]");
  check("methods", await methods.allTextContents(), ["GET", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]);
  check("methodColours", await page.locator("main [data-method]").evaluateAll((all) => new Set(all.map((m) => getComputedStyle(m).backgroundColor)).size), 5);

  // A closed section composed from the parts opens from the keyboard.
  const custom = page.locator("#webhook details");
  check("customClosed", await custom.evaluate((d) => d.open), false);
  await custom.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#webhook details").open);
  check("customRows", await custom.locator("li").count(), 2);

  check("german", [await text(page.locator("#german summary")), await page.locator("#german [data-required]").textContent()], ["Parameter 1", "Pflicht"]);

  // Right to left: the chevron of a closed property points left, the count goes left.
  await setDir("rtl");
  check("rtlChevron", await rotation(limits.locator("[data-icon]")), "none");
  check("rtlScale", await limits.locator("[data-icon]").evaluate((i) => getComputedStyle(i).scale), "-1 1");
  const counted = await params.locator("summary").evaluate((s) => s.querySelector("span:last-child").getBoundingClientRect().right < s.querySelector("span").getBoundingClientRect().left);
  check("rtlCount", counted, true);
};
