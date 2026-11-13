export default async ({ page, check, focused, visible, setDir, settled }) => {
  const views = page.getByRole("tablist", { name: "Project views" });
  const selected = (list) => visible(list.locator("[role=tab][aria-selected=true]"));
  const panels = () => visible(page.locator("[role=tabpanel]:not([hidden])"));
  const focusedId = () => page.evaluate(() => document.activeElement.id);

  // A count after the label, read as part of the name.
  check("countInName", await views.getByRole("tab", { name: "Deployments 42" }).count(), 1);

  // Manual activation: the arrow keys move focus only; Enter or Space selects.
  await views.getByRole("tab", { name: /Deployments/ }).focus();
  await page.keyboard.press("ArrowRight");
  check("arrowMovesFocus", [await focused(), await selected(views)], ["Domains", ["Deployments 42"]]);
  check("rovingFollowsFocus", await visible(views.locator("[role=tab][tabindex='0']")), ["Domains"]);
  await page.keyboard.press("Enter");
  check("enterSelects", [await selected(views), (await panels()).includes("Domains panel.")], [["Domains"], true]);
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowRight");
  check("wraps", await focused(), "Deployments 42");
  await page.keyboard.press("End");
  check("end", await focused(), "Settings");
  await page.keyboard.press(" ");
  check("spaceSelects", await selected(views), ["Settings"]);
  await page.keyboard.press("Home");
  check("home", [await focused(), await selected(views)], ["Deployments 42", ["Settings"]]);
  await page.keyboard.press("Tab");
  check("tabToPanel", await focused(), "Settings panel.");
  await page.keyboard.press("Shift+Tab");
  check("tabReturnsToFocusedTab", await focused(), "Deployments 42");
  await views.getByRole("tab", { name: "Domains" }).click();
  check("click", [await selected(views), await focused()], [["Domains"], "Domains"]);

  // The indicator slides under the selected tab.
  const indicator = views.locator("> span:last-child");
  await page.waitForFunction(() => document.querySelector('[aria-label="Project views"]').dataset.indicator === "ready");
  await indicator.evaluate((i) => Promise.all(i.getAnimations().map((a) => a.finished)));
  const under = async (list, name) => {
    const tab = await list.getByRole("tab", { name }).boundingBox();
    const line = await list.locator("> span:last-child").boundingBox();
    return [Math.round(line.x - tab.x), Math.round(line.width - tab.width)];
  };
  check("indicatorUnderSelected", await under(views, "Domains"), [0, 0]);
  check("indicatorUnderline", await indicator.evaluate((i) => [getComputedStyle(i).height, Math.round(i.getBoundingClientRect().bottom - i.parentElement.getBoundingClientRect().bottom)]), ["2px", 0]);

  // Pills: a segmented control, the selected pill raised.
  const range = page.getByRole("tablist", { name: "Range" });
  check("pills", [await range.getAttribute("data-variant"), await range.getAttribute("data-overflow-mode")], ["pills", "wrap"]);
  await range.getByRole("tab", { name: "7d" }).click();
  const pill = range.locator("> span:last-child");
  await page.waitForFunction(() => document.querySelector('[aria-label="Range"]').dataset.indicator === "ready");
  await pill.evaluate((i) => Promise.all(i.getAnimations().map((a) => a.finished)));
  const pillBox = await range.getByRole("tab", { name: "7d" }).boundingBox();
  const raised = await pill.boundingBox();
  check("pillUnderSelected", [raised.x, raised.y, raised.width, raised.height].map((v, i) => Math.round(v - [pillBox.x, pillBox.y, pillBox.width, pillBox.height][i])), [0, 0, 0, 0]);
  check("pillHeight", Math.round(pillBox.height), 30);
  check("pillPanel", (await panels()).includes("Last 7 days."), true);
  check("change", await range.evaluate((list) => new Promise((resolve) => {
    list.addEventListener("bitop:change", (e) => resolve(e.detail), { once: true });
    list.querySelector('[aria-controls="range-month"]').click();
  })), "range-month");

  // Nav tabs: a <nav> of links, the current one marked.
  const nav = page.getByRole("navigation", { name: "Project sections (example)" });
  check("navLinks", await nav.getByRole("link").evaluateAll((all) => all.map((a) => [a.textContent.trim(), a.getAttribute("aria-current")])), [
    ["Overview", "page"], ["Domains 3", null], ["Analytics", null], ["Settings", null],
  ]);
  check("navList", await nav.locator("ul > li > a").count(), 4);
  check("navUnderline", await nav.getByRole("link", { name: "Overview" }).evaluate((a) => getComputedStyle(a, "::after").height), "2px");

  // Icons, a disabled tab that the arrows skip, a grouped count, and a list that scrolls.
  const workspace = page.getByRole("tablist", { name: "Workspace" });
  check("icons", await workspace.locator("[role=tab] > [data-icon]").evaluateAll((all) => all.map((i) => i.dataset.icon)), ["folder", "bell", "settings"]);
  check("groupedCount", await workspace.getByRole("tab", { name: "Activity 1,250" }).count(), 1);
  await workspace.getByRole("tab", { name: "Activity 1,250" }).focus();
  await page.keyboard.press("ArrowRight");
  check("skipsDisabled", await focusedId(), "ws-settings-tab");
  const billing = workspace.getByRole("tab", { name: "Billing" });
  await billing.click({ force: true });
  check("disabledNotSelected", await billing.getAttribute("aria-selected"), "false");
  check("scrolls", await workspace.evaluate((l) => [l.hasAttribute("data-overflowing"), getComputedStyle(l).overflowX, l.scrollWidth > l.clientWidth]), [true, "auto", true]);
  await workspace.getByRole("tab", { name: "Files" }).focus();
  await page.keyboard.press("ArrowLeft");
  await page.waitForFunction(() => document.querySelector('[aria-label="Workspace"]').hasAttribute("data-overflow-start"));
  check("fadeEdges", await workspace.evaluate((l) => [l.hasAttribute("data-overflow-start"), l.scrollLeft > 0]), [true, true]);

  await setDir("rtl");
  await views.getByRole("tab", { name: /Deployments/ }).click();
  await page.keyboard.press("ArrowLeft");
  check("rtlArrows", await focused(), "Domains");
};
