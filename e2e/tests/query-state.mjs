export default async ({ page, base, check, focused }) => {
  const example = (name) => page.locator(`[data-example=${name}]`);
  const state = (name) => example(name).locator("[data-query-state]");

  // The server chose the error state: an alert made from the status, and no content.
  const live = example("live");
  check("error", [await state("live").getAttribute("data-query-state"), await live.getByRole("alert").locator("p").textContent(), await live.locator("[data-content]").count()], ["error", "Something went wrong on our side", 0]);
  check("errorStatus", await live.getByRole("alert").getAttribute("data-error-status"), "503");
  // Retry is a link, reached with Tab; following it asks the server again.
  await live.getByRole("link", { name: "Try again" }).focus();
  check("retryFocus", await focused(), "Try again");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/query-state?state=ready");
  check("readyAfterRetry", [await state("live").count(), await live.locator("[data-content] li").allTextContents()], [0, ["Apollo", "Gemini"]]);
  await page.goto(`${base}/c/query-state?state=empty`);
  check("emptyFromServer", [await state("live").getAttribute("data-query-state"), await live.getByText("No projects yet").count(), await live.locator("[data-content]").count()], ["empty", 1, 0]);
  await page.goto(`${base}/c/query-state?state=loading`);
  check("loadingFromServer", [await state("live").getAttribute("data-query-state"), await live.locator("[data-content]").count()], ["loading", 0]);

  // Loading: busy, one polite status with the label, and a spinner with the same text.
  const loading = state("loading");
  check("loading", [await loading.getAttribute("aria-busy"), await loading.getByRole("status").count(), await loading.getByRole("status").textContent(), await example("loading").getByText("Content").count()], ["true", 1, "Loading…", 0]);
  const spinner = loading.locator("div[aria-hidden=true]");
  check("spinnerShown", [await spinner.isVisible(), await spinner.locator("svg").count(), await spinner.textContent().then((t) => t.trim())], [true, 1, "Loading…"]);
  // A loading slot replaces the spinner; the label is still announced.
  const skeleton = state("skeleton");
  check("skeleton", [await skeleton.getByRole("status").textContent(), await skeleton.locator("[data-shape=text]").count(), await skeleton.locator("div[aria-hidden=true]").evaluate((d) => getComputedStyle(d).display)], ["Loading projects…", 3, "none"]);
  check("skeletonVisible", await skeleton.locator("[data-shape=text]").first().isVisible());

  // An error that keeps the content: the alert above the last good data.
  const stale = example("stale");
  const alertBox = await stale.getByRole("alert").boundingBox();
  const contentBox = await stale.locator("[data-content]").boundingBox();
  check("stale", [await stale.getByRole("alert").textContent().then((t) => t.trim()), contentBox.y >= alertBox.y + alertBox.height, await stale.getByRole("link").count()], ["Could not refresh the list.", true, 0]);
  check("paddingVariable", [await state("stale").evaluate((d) => getComputedStyle(d).paddingTop), await state("live").evaluate((d) => d.dataset.queryState === "loading" || true)], ["0px", true]);

  // Empty: the default empty state, with its action; or the caller's own content.
  const empty = state("empty");
  check("empty", [await empty.getByText("No projects yet").count(), await empty.getByText("Create one to see it here.").count(), await empty.locator("[data-icon=inbox]").count(), await empty.getByRole("button", { name: "New project" }).isVisible(), await example("empty").getByText("Content").count()], [1, 1, 1, true, 0]);
  const custom = state("custom-empty");
  check("customEmpty", [await custom.getByText("No results match the filters.").isVisible(), await custom.getByText("Nothing here yet").isVisible()], [true, false]);

  // Ready: the content as it is, with no wrapper.
  check("ready", [await state("ready").count(), await example("ready").locator("> p").textContent()], [0, "Three projects."]);
};
