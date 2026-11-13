export default async ({ page, check }) => {
  // A section named by its title, with an ordered list of steps.
  const card = page.getByRole("region", { name: "Migrate the billing service" });
  check("region", await card.count(), 1);
  const steps = card.getByRole("list", { name: "Steps" });
  check("ordered", await steps.evaluate((l) => l.tagName), "OL");
  check("spoken", await steps.locator("li").evaluateAll((all) => all.map((li) => li.textContent.replace(/\s+/g, " ").trim())), [
    "Snapshot the database (done)",
    "Run the migration (in progress)",
    "Switch traffic to the new service (to do)",
    "Remove the old tables (to do)",
  ]);
  check("current", await steps.locator("li").evaluateAll((all) => all.map((li) => li.getAttribute("aria-current"))), [null, "step", null, null]);
  check("markers", await steps.locator("li").evaluateAll((all) => all.map((li) => li.querySelector("[data-icon]")?.dataset.icon ?? "dot")), ["check", "dot", "circle", "circle"]);
  check("footer", await card.getByRole("button", { name: "Approve plan" }).count(), 1);
  check("titleColour", await card.locator("p").first().evaluate((p) => getComputedStyle(p.firstElementChild).color === getComputedStyle(p).color), true);

  // The toggle hides and shows the steps from the keyboard, and says so.
  const toggle = card.getByRole("button", { name: "Show steps" });
  check("expanded", [await toggle.getAttribute("aria-expanded"), await toggle.getAttribute("aria-controls")], ["true", "migration-steps"]);
  await page.evaluate(() => {
    window.toggles = [];
    document.addEventListener("bitop:plan-toggle", (event) => window.toggles.push(event.detail.open));
  });
  await toggle.focus();
  await page.keyboard.press("Enter");
  // It closes through a transition, so wait for it to be gone.
  await steps.waitFor({ state: "hidden" });
  check("collapsed", [await toggle.getAttribute("aria-expanded"), await steps.isVisible()], ["false", false]);
  await page.keyboard.press("Space");
  await steps.waitFor();
  check("reopened", [await toggle.getAttribute("aria-expanded"), await steps.isVisible(), await page.evaluate(() => window.toggles)], ["true", true, [false, true]]);
  check("focusKept", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Show steps");

  // Rendered closed: the steps are hidden until the toggle opens them.
  const closed = page.locator("#collapsed-plan");
  check("startsClosed", [await closed.locator("ol").isVisible(), await closed.locator("[data-plan-toggle]").getAttribute("aria-expanded")], [false, "false"]);
  await closed.locator("[data-plan-toggle]").click();
  await closed.locator("ol").waitFor();
  check("opens", await closed.locator("ol").isVisible(), true);

  // Streaming: busy and shimmering until the app removes data-streaming.
  const live = page.locator("#live-plan");
  check("busy", [await live.getAttribute("aria-busy"), await live.locator("[data-shimmer][data-active]").count()], ["true", 2]);
  await page.getByRole("button", { name: "Finish streaming" }).click();
  await page.waitForFunction(() => !document.getElementById("live-plan").hasAttribute("aria-busy"));
  check("done", await live.locator("[data-shimmer][data-active]").count(), 0);

  // Right to left: the toggle is at the start edge of the header's end.
  const x = () => card.evaluate((c) => {
    const t = c.querySelector("[data-plan-toggle]").getBoundingClientRect();
    const title = c.querySelector("p").getBoundingClientRect();
    return t.left > title.left;
  });
  check("ltrToggleRight", await x(), true);
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  check("rtlToggleLeft", await x(), false);

  check("german", [await page.locator("#german-plan [data-plan-toggle]").getAttribute("aria-label"), await page.locator("#german-plan li").evaluate((li) => li.textContent.replace(/\s+/g, " ").trim())], ["Schritte zeigen", "Zahlen sammeln (erledigt)"]);
};
