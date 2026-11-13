export default async ({ page, check }) => {
  // A colour token as the page resolves it, to compare with a computed style.
  const token = (name, property = "background-color") =>
    page.evaluate(([n, p]) => {
      const probe = document.createElement("span");
      probe.style.setProperty(p, `var(${n})`);
      document.body.append(probe);
      const value = getComputedStyle(probe).getPropertyValue(p);
      probe.remove();
      return value;
    }, [name, property]);

  // The group: a fieldset named by its legend, described by its description.
  const scopes = page.getByRole("group", { name: "Scopes" });
  check("groupDescribed", await scopes.evaluate((g) => document.getElementById(g.getAttribute("aria-describedby")).textContent.trim()), "What this token can do.");
  const formValues = () => page.evaluate(() => new FormData(document.getElementById("scopes-form")).getAll("scope"));
  check("initialValues", await formValues(), ["read"]);
  const write = scopes.getByRole("checkbox", { name: /^Write/ });
  await write.focus();
  await page.keyboard.press("Space");
  check("spaceToggles", [await write.isChecked(), await formValues()], [true, ["read", "write"]]);
  // The whole option is the label: a click on the description toggles too.
  await scopes.getByText("Create and update deployments.").click();
  check("descriptionClick", [await write.isChecked(), await formValues()], [false, ["read"]]);
  check("disabled", await scopes.getByRole("checkbox", { name: /^Admin/ }).isDisabled());

  // Upstream's look: a 16px box with a 12px check, a description indented under the text.
  const read = scopes.getByRole("checkbox", { name: /^Read/ });
  check("boxSize", await read.evaluate((b) => [b.getBoundingClientRect().width, b.getBoundingClientRect().height]), [16, 16]);
  check("checkedFill", await read.evaluate((b) => getComputedStyle(b).backgroundColor), await token("--color-primary"));
  check("indentedDescription", await scopes.getByText("List projects and deployments.").evaluate((d) => {
    const text = d.closest("label").querySelector("input + span").getBoundingClientRect().left;
    return Math.round(d.getBoundingClientRect().left + parseFloat(getComputedStyle(d).paddingInlineStart) - text);
  }), 0);

  // Indeterminate: the script sets the property, the dash is drawn; a click clears it.
  const mixed = page.getByRole("checkbox", { name: "Partially selected" });
  check("indeterminate", await mixed.evaluate((b) => [b.indeterminate, b.checked, b.matches(":indeterminate")]), [true, false, true]);
  check("dash", await mixed.evaluate((b) => getComputedStyle(b, "::before").maskImage.includes("M5 12h14")), true);
  await mixed.click();
  check("clickClears", await mixed.evaluate((b) => [b.indeterminate, b.checked, b.hasAttribute("data-indeterminate")]), [false, true, false]);
  check("check", await mixed.evaluate((b) => getComputedStyle(b, "::before").maskImage.includes("M20 6")), true);
  // Markup added later is marked too.
  await page.evaluate(() => {
    const copy = document.querySelector("[data-indeterminate]") ?? document.querySelector("input[type=checkbox]");
    const added = copy.closest("label").cloneNode(true);
    added.querySelector("input").setAttribute("data-indeterminate", "");
    added.querySelector("input").id = "added-mixed";
    added.querySelector("input").setAttribute("aria-label", "Added");
    document.querySelector("main").append(added);
  });
  await page.waitForFunction(() => document.getElementById("added-mixed").indeterminate);
  await page.evaluate(() => document.getElementById("added-mixed").closest("label").remove());

  // Switches: 32 by 18, a 14px thumb, the unchecked track in the strong border colour.
  const previews = page.getByRole("switch", { name: /^Preview deployments/ });
  const row = page.getByRole("switch", { name: "Settings-row layout" });
  check("switchSize", await row.evaluate((s) => [s.getBoundingClientRect().width, s.getBoundingClientRect().height, getComputedStyle(s, "::before").width]), [32, 18, "14px"]);
  check("offTrack", await row.evaluate((s) => getComputedStyle(s).backgroundColor), await token("--color-border-strong"));
  check("onTrack", await previews.evaluate((s) => getComputedStyle(s).backgroundColor), await token("--color-primary"));
  await page.getByText("Settings-row layout").click();
  check("labelToggles", await row.isChecked(), true);
  // label_position start: the text first, the switch at the end of the row.
  const [text, track, end] = await row.evaluate((s) => {
    const label = s.closest("label");
    return [label.querySelector("input + span").getBoundingClientRect().left, s.getBoundingClientRect().right, label.getBoundingClientRect().right];
  });
  check("switchAtEnd", [text < track, Math.round(end - track)], [true, 0]);
  check("switchDescription", await page.getByText("Build every pull request.").count(), 1);
  check("switchDisabled", await page.getByRole("switch", { name: "Disabled" }).isDisabled());
  check("switchCount", await page.getByRole("switch").count(), 3);

  // An error: an icon and the message, the boxes in the danger colour.
  const channels = page.getByRole("group", { name: "Notify me by" });
  check("errorDescribes", await channels.evaluate((g) => document.getElementById(g.getAttribute("aria-describedby")).textContent.trim()), "Choose at least one channel.");
  check("errorIcon", await page.locator("#channels-error [data-icon]").getAttribute("data-icon"), "circle-alert");
  check("invalidBorder", await channels.getByRole("checkbox", { name: "Email" }).evaluate((b) => getComputedStyle(b).borderTopColor), await token("--color-danger", "border-top-color"));
  check("horizontal", await channels.getByRole("checkbox").evaluateAll((all) => new Set(all.map((b) => Math.round(b.getBoundingClientRect().top))).size), 1);
};
