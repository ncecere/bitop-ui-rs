export default async ({ page, check }) => {
  const queued = page.locator("[data-example=queued] details");
  const summary = queued.locator("summary");
  check("summary", await summary.evaluate((s) => s.textContent.replace(/\s+/g, " ").trim()), "Queued 2 items");
  const list = page.getByRole("list", { name: "Queued messages" });
  check("items", await list.locator("li").count(), 2);
  check("description", await list.locator("li").nth(1).getByText("Sent after the current answer").count(), 1);

  // Row actions are hidden until the row is hovered or focused, and the keyboard
  // reaches them.
  const actions = list.locator("li").first().locator("button");
  const opacity = () => actions.first().evaluate((b) => getComputedStyle(b.parentElement).opacity);
  await page.mouse.move(0, 0);
  const hover = await page.evaluate(() => matchMedia("(hover: hover)").matches);
  check("hiddenAtRest", await opacity(), hover ? "0" : "1");
  await summary.focus();
  await page.keyboard.press("Tab");
  check("focusAction", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Edit");
  check("shownOnFocus", await opacity(), "1");
  // Its tooltip describes it.
  await page.locator("#queue-edit-1").waitFor();
  check("tooltip", await page.locator("#queue-edit-1").textContent(), "Edit");

  // Remove submits the form with the item's value.
  await page.keyboard.press("Tab");
  check("focusRemove", await page.evaluate(() => [document.activeElement.getAttribute("aria-label"), document.activeElement.value]), ["Remove", "q-1"]);
  await Promise.all([page.waitForURL(/remove=q-1/), page.keyboard.press("Enter")]);
  check("submitted", await page.locator("[data-removed]").textContent(), "Removed q-1");

  // A to-do list: a completed item is struck through and says so.
  const todo = page.locator("#todo-queue");
  check("closed", await todo.evaluate((d) => d.open), false);
  check("chevronClosed", await todo.locator("summary [data-icon]").evaluate((i) => getComputedStyle(i).rotate), "-90deg");
  await todo.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("todo-queue").open);
  const done = todo.locator("li").first();
  await done.waitFor();
  check("done", await done.evaluate((li) => [li.hasAttribute("data-completed"), li.textContent.replace(/\s+/g, " ").trim(), getComputedStyle(li.querySelector("span:not([aria-hidden]) > span")).textDecorationLine]), [true, "Read the policy (done)", "line-through"]);
  check("checked", await done.locator("[data-icon]").getAttribute("data-icon"), "check");
  check("oneItem", await todo.locator("summary").evaluate((s) => s.textContent.replace(/\s+/g, " ").trim()), "To do 1 item");

  // Right to left: a closed queue's chevron points the other way.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  await todo.locator("summary").click();
  await page.waitForFunction(() => !document.getElementById("todo-queue").open);
  await page.waitForFunction(() => getComputedStyle(document.querySelector("#todo-queue summary [data-icon]")).rotate === "90deg");
  check("rtlChevron", await todo.locator("summary [data-icon]").evaluate((i) => getComputedStyle(i).rotate), "90deg");

  check("german", await page.locator("[data-example=strings] summary").evaluate((s) => s.textContent.replace(/\s+/g, " ").trim()), "Warteschlange 1 Eintrag");
};
