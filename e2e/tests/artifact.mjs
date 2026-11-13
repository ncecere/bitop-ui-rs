export default async ({ page, check }) => {
  // A region named by its title.
  const doc = page.getByRole("region", { name: "Q3 summary" });
  check("region", await doc.count(), 1);
  check("title", [await doc.locator("p").first().textContent(), await doc.locator("p").nth(1).textContent()], ["Q3 summary", "Markdown · 2 min ago"]);

  // The actions, each named, with a tooltip.
  const actions = doc.locator("[data-artifact-action]");
  check("actions", await actions.evaluateAll((all) => all.map((b) => [b.getAttribute("aria-label"), b.type])), [["Open in a new tab", "button"], ["Save a copy", "submit"], ["Close", "button"]]);
  check("closeIcon", await actions.last().locator("[data-icon]").getAttribute("data-icon"), "x");

  // The content scrolls and takes focus, after the actions.
  await page.evaluate(() => {
    window.events = [];
    for (const type of ["bitop:artifact-action", "bitop:artifact-close"]) document.addEventListener(type, (e) => window.events.push([type, e.target.id, e.detail.action]));
  });
  await actions.first().focus();
  await page.keyboard.press("Enter");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Space");
  check("events", await page.evaluate(() => window.events), [["bitop:artifact-action", "document-artifact", "open"], ["bitop:artifact-close", "document-artifact", "close"]]);
  await page.keyboard.press("Tab");
  const content = doc.locator("[tabindex='0']");
  check("contentFocused", await content.evaluate((c) => c === document.activeElement), true);
  check("scrolls", await content.evaluate((c) => c.scrollHeight > c.clientHeight), true);
  await page.keyboard.press("End");
  await page.waitForFunction(() => {
    const c = document.querySelector("#document-artifact [tabindex='0']");
    return c.scrollTop > 0;
  });
  check("keyboardScrolls", await content.evaluate((c) => c.scrollTop > 0), true);

  // A tooltip shows on focus.
  await actions.nth(1).focus();
  await page.locator("#artifact-save-tip").waitFor();
  check("tooltip", await page.locator("#artifact-save-tip").textContent(), "Save a copy");

  // Save submits the form around it.
  await Promise.all([page.waitForURL(/artifact=save/), page.keyboard.press("Enter")]);
  check("submitted", await page.locator("[data-saved]").textContent(), "Action: save");

  // Flush content, and a heading title.
  const code = page.getByRole("region", { name: "main.rs" });
  check("heading", await code.locator("h3").textContent(), "main.rs");
  check("flush", await code.locator("[data-flush]").evaluate((c) => getComputedStyle(c).paddingTop), "0px");

  // Right to left: the actions go to the left.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const left = await page.locator("#document-artifact").evaluate((a) => {
    const actions = a.querySelector("[data-artifact-action]").parentElement.getBoundingClientRect();
    return actions.left - a.getBoundingClientRect().left < 20;
  });
  check("rtlActions", left, true);
};
