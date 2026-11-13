export default async ({ page, check }) => {
  const style = (locator, ...props) => locator.evaluate((e, ps) => ps.map((p) => getComputedStyle(e).getPropertyValue(p)), props);

  // A label, a description and an error, connected by IDs.
  await page.getByText("Project name", { exact: true }).click();
  check("labelFocusesInput", await page.evaluate(() => document.activeElement.id), "project-name");
  const describedBy = (locator) => locator.evaluate((i) => i.getAttribute("aria-describedby").split(" ").map((id) => document.getElementById(id).textContent.trim()));
  check("described", await describedBy(page.getByLabel("Project name")), ["Shown to everyone in the workspace."]);
  const webhook = page.getByLabel("Webhook URL");
  check("errorDescribes", [await webhook.getAttribute("aria-invalid"), await describedBy(webhook)], ["true", ["Must use HTTPS.", "Enter a URL that starts with https://."]]);
  check("errorIcon", await page.locator("#webhook-url-error [data-icon]").getAttribute("data-icon"), "circle-alert");

  // Upstream's type: label 13px medium, description 13px, error 13px medium.
  check("labelType", await style(page.locator("label[for=project-name]"), "font-size", "font-weight"), ["13px", "500"]);
  check("descriptionType", await style(page.locator("#project-name-description"), "font-size", "font-weight"), ["13px", "400"]);
  check("errorType", await style(page.locator("#webhook-url-error"), "font-size", "font-weight"), ["13px", "500"]);
  // The control: 36px, with a faint shadow; sm is 32px.
  check("height", await page.getByLabel("Project name").evaluate((i) => i.getBoundingClientRect().height), 36);
  check("shadow", (await style(page.getByLabel("Project name"), "box-shadow"))[0] !== "none", true);
  check("small", await page.getByLabel("Small").evaluate((i) => [i.getBoundingClientRect().height, i.dataset.size]), [32, "sm"]);

  // A start icon inside the input, before the text; the label is for assistive technology.
  const search = page.getByRole("searchbox", { name: "Search" });
  const [iconBox, inputBox, padding] = await search.evaluate((i) => [
    i.parentElement.querySelector("[data-icon]").getBoundingClientRect(),
    i.getBoundingClientRect(),
    parseFloat(getComputedStyle(i).paddingInlineStart),
  ].map((b) => (typeof b === "number" ? b : { left: b.left, right: b.right, width: b.width })));
  check("startIcon", [iconBox.width, iconBox.left > inputBox.left, iconBox.right < inputBox.left + padding], [16, true, true]);
  check("hiddenLabel", await page.locator("label[for=search-projects]").evaluate((l) => l.parentElement.classList.contains("sr-only")), true);
  check("labelHint", await page.locator("label[for=review-notes] + span").textContent(), "Optional");

  // The textarea is three rows high and does not grow by itself, as upstream's.
  const notes = page.getByLabel("Notes");
  check("textareaRows", await notes.getAttribute("rows"), "3");
  const before = await notes.evaluate((t) => t.getBoundingClientRect().height);
  await notes.fill("one\ntwo\nthree\nfour\nfive\nsix");
  check("textareaKeepsHeight", await notes.evaluate((t) => t.getBoundingClientRect().height), before);

  // A disabled control dims its field's label.
  const label = (id) => page.locator(`label[for=${id}]`).evaluate((l) => getComputedStyle(l).color);
  check("disabledLabel", (await label("disabled-input")) !== (await label("small-input")), true);

  // The fieldset is named by its legend; the form stacks fields; actions sit at the end.
  const webhookGroup = page.getByRole("group", { name: "Webhook" });
  check("fieldset", [await webhookGroup.count(), await webhookGroup.getByRole("textbox").count()], [1, 2]);
  check("formGap", (await style(page.locator("#webhook-form"), "row-gap"))[0], "20px");
  const save = page.getByRole("button", { name: "Save webhook" });
  const [saveRight, formRight] = await save.evaluate((b) => [b.getBoundingClientRect().right, b.closest("form").getBoundingClientRect().right]);
  check("actionsAtEnd", Math.round(formRight - saveRight), 0);
  // The browser's own validation stops an empty submission.
  await save.click();
  check("nativeValidation", [new URL(page.url()).search, await page.evaluate(() => document.activeElement.id)], ["", "webhook-name"]);
};
