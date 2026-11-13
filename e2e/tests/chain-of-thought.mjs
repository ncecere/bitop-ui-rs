export default async ({ page, check }) => {
  const open = page.locator("[data-example=open] details");
  const steps = open.locator("ol > li");
  check("open", await open.evaluate((d) => d.open), true);
  check("summary", await open.locator("summary").evaluate((s) => [s.textContent.trim(), [...s.querySelectorAll("[data-icon]")].map((i) => i.dataset.icon)]), ["Chain of thought", ["list-tree", "chevron-down"]]);
  check("listName", await open.locator("ol").getAttribute("aria-label"), "Steps");

  // Each step says its status; the active one is current and shimmers.
  check("spoken", await steps.evaluateAll((all) => all.map((li) => li.querySelector(":scope > div > span").textContent.replace(/\s+/g, " ").trim())), [
    "Searched the handbook (complete)",
    "Checked the web (complete)",
    "Drafting the answer (in progress)",
    "Checking citations (not started)",
  ]);
  check("current", await steps.evaluateAll((all) => all.map((li) => li.getAttribute("aria-current"))), [null, null, "step", null]);
  check("shimmer", await steps.evaluateAll((all) => all.map((li) => li.querySelector("[data-shimmer][data-active]") !== null)), [false, false, true, false]);
  check("markers", await steps.evaluateAll((all) => all.map((li) => li.querySelector("span[aria-hidden] [data-icon]")?.dataset.icon ?? "dot")), ["check", "globe", "dot", "circle"]);
  check("results", await steps.first().locator("div > div > span").allTextContents(), ["handbook.pdf", "leave-policy-2025.md", "faq.md"]);
  check("description", await steps.first().getByText("Found 3 documents").count(), 1);

  // A line joins the markers, except after the last.
  const line = (n) => steps.nth(n).evaluate((li) => getComputedStyle(li, "::before").content);
  check("lines", [await line(0), await line(3)], ['""', "none"]);

  // The keyboard opens it.
  const closed = page.locator("#closed-chain");
  check("startsClosed", await closed.evaluate((d) => d.open), false);
  await closed.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("closed-chain").open);
  await closed.getByText("Answered it").waitFor();
  check("opened", await closed.getByText("Answered it").isVisible(), true);
  check("ownHeader", await closed.locator("summary").evaluate((s) => [s.textContent.trim(), s.querySelector("[data-icon]").dataset.icon]), ["How I got here", "brain"]);

  // Right to left: the line sits under the markers on the right.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const aligned = await steps.first().evaluate((li) => {
    const marker = li.querySelector("span[aria-hidden]").getBoundingClientRect();
    const x = li.getBoundingClientRect().right - parseFloat(getComputedStyle(li, "::before").insetInlineStart) - 0.5;
    return Math.abs(marker.left + marker.width / 2 - x) <= 1;
  });
  check("rtlLine", aligned, true);

  check("german", await page.locator("[data-example=strings] li .sr-only").textContent(), "(erledigt)");
};
