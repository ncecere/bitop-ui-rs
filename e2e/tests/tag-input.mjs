export default async ({ page, check, button, focused }) => {
  const root = page.locator("#tag-form [data-tag-input]");
  const input = root.getByRole("textbox", { name: "Tags" });
  const tags = () => root.locator("li > span").allTextContents();
  const formValue = (id = "tag-form", name = "tags") => page.evaluate(([i, n]) => new FormData(document.getElementById(i)).getAll(n), [id, name]);
  const status = () => root.locator("[role=status]").textContent();

  // The server's tags are in the form as they stand.
  check("initial", [await tags(), await formValue()], [["rust", "web"], ["rust", "web"]]);
  check("list", await root.getByRole("list", { name: "Tags" }).count(), 1);
  check("described", await input.evaluate((i) => document.getElementById(i.getAttribute("aria-describedby")).textContent.trim()), "Press Enter after each tag.");
  check("placeholder", await input.getAttribute("placeholder"), "Add tags…");

  // Enter and comma add a tag: trimmed, lower-cased, and not twice.
  await page.getByText("Tags", { exact: true }).click();
  check("labelFocuses", await page.evaluate(() => document.activeElement.tagName), "INPUT");
  await page.keyboard.type("  Topcoat ");
  await page.keyboard.press("Enter");
  check("enterAdds", [await tags(), await formValue(), await input.inputValue()], [["rust", "web", "topcoat"], ["rust", "web", "topcoat"], ""]);
  check("announcesAdd", await status(), "Added tag topcoat");
  await page.keyboard.type("css,");
  check("commaAdds", [await tags(), await input.inputValue()], [["rust", "web", "topcoat", "css"], ""]);
  await page.keyboard.type("RUST");
  await page.keyboard.press("Enter");
  check("noDuplicates", [await tags(), await input.inputValue()], [["rust", "web", "topcoat", "css"], ""]);

  // An empty Enter does not submit the form.
  const submitted = await page.evaluate(() => {
    window.tagSubmits = 0;
    document.getElementById("tag-form").addEventListener("submit", (event) => {
      event.preventDefault();
      window.tagSubmits += 1;
    });
  });
  await page.keyboard.press("Enter");
  check("emptyEnterDoesNotSubmit", [submitted, await page.evaluate(() => window.tagSubmits)], [undefined, 0]);

  // Backspace in the empty input removes the last tag.
  await page.keyboard.press("Backspace");
  check("backspaceRemoves", [await tags(), await formValue()], [["rust", "web", "topcoat"], ["rust", "web", "topcoat"]]);
  check("announcesRemove", await status(), "Removed tag css");
  await page.keyboard.type("ab");
  await page.keyboard.press("Backspace");
  check("backspaceEditsText", [await tags(), await input.inputValue()], [["rust", "web", "topcoat"], "a"]);
  await input.fill("");

  // Each tag has its own Remove button, and focus goes back to the input.
  await button("Remove tag web").click();
  check("removeButton", [await tags(), await page.evaluate(() => document.activeElement.tagName)], [["rust", "topcoat"], "INPUT"]);

  // Text left in the input becomes a tag on Tab and on blur.
  await page.keyboard.type("forms");
  await page.keyboard.press("Tab");
  check("tabAdds", await tags(), ["rust", "topcoat", "forms"]);
  check("tabStillMoves", await page.evaluate(() => !document.activeElement.closest("#tag-form")));
  await input.fill("blur");
  await page.locator("h1").click();
  check("blurAdds", await tags(), ["rust", "topcoat", "forms", "blur"]);

  // A paste with commas or new lines adds several, up to the limit of five.
  await button("Remove tag blur").click();
  await button("Remove tag forms").click();
  await page.evaluate(() => {
    const area = document.createElement("textarea");
    area.id = "copy-source";
    area.setAttribute("aria-label", "Copy source");
    area.value = "a11y, HTML\nrust\nextra, more";
    document.querySelector("main").append(area);
    area.select();
  });
  await page.keyboard.press("ControlOrMeta+C");
  await page.evaluate(() => document.getElementById("copy-source").remove());
  await input.focus();
  await page.keyboard.press("ControlOrMeta+V");
  check("pasteAddsUpToLimit", [await tags(), await formValue()], [["rust", "topcoat", "a11y", "html", "extra"], ["rust", "topcoat", "a11y", "html", "extra"]]);
  check("announcesMany", await status(), "Added 3 tags");
  check("limit", [await input.isDisabled(), await input.getAttribute("placeholder")], [true, "Limit of 5 tags reached"]);
  await button("Remove tag extra").click();
  check("belowLimitAgain", [await input.isDisabled(), await input.getAttribute("placeholder"), await focused()], [false, "Add tags…", ""]);

  // A change event tells the page.
  await page.evaluate(() => {
    window.tagChanges = [];
    document.getElementById("tag-form").addEventListener("bitop:change", (event) => window.tagChanges.push(event.detail.value));
  });
  await page.keyboard.type("last");
  await page.keyboard.press("Enter");
  check("changeEvent", await page.evaluate(() => window.tagChanges), [["rust", "topcoat", "a11y", "html", "last"]]);
  // A click on the padding of the control goes to the input.
  await page.locator("h1").click();
  await button("Remove tag last").click();
  await page.locator("h1").click();
  const box = await root.boundingBox();
  await page.mouse.click(box.x + box.width - 4, box.y + box.height - 3);
  check("clickFocusesInput", await page.evaluate(() => document.activeElement.tagName), "INPUT");

  // Another noun, the case kept, and a length limit.
  const origins = page.getByRole("textbox", { name: "Allowed origins" });
  check("noun", [await page.getByRole("list", { name: "Origins" }).count(), await button("Remove origin https://Example.edu").count(), await origins.getAttribute("placeholder")], [1, 1, "Add origins…"]);
  const small = await page.getByRole("list", { name: "Origins" }).locator("xpath=..").boundingBox();
  const normal = await root.boundingBox();
  check("sizes", [Math.round(small.height), Math.round(normal.height)], [32, 38]);
  await origins.fill("https://Docs.Example.org/a/very/long/path");
  await page.keyboard.press("Enter");
  const originTags = await page.getByRole("list", { name: "Origins" }).locator("li > span").allTextContents();
  // Twenty-four characters at most.
  check("keepsCaseAndTruncates", originTags, ["https://Example.edu", "https://Docs.Example.org"]);

  check("invalid", await page.getByRole("textbox", { name: "Reviewers" }).getAttribute("aria-invalid"), "true");
  const locked = page.getByRole("textbox", { name: "Labels" });
  check("disabled", [await locked.isDisabled(), await page.locator("[data-tag-input][data-disabled] li button").count(), await page.locator("input[name=labels]").evaluateAll((all) => all.every((i) => i.disabled))], [true, 0, true]);
};
