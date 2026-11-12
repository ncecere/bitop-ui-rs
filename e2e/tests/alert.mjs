export default async ({ page, check, focused }) => {
  const alerts = page.locator("main [data-alert]:not([data-example=errors] *)");
  check("roles", await alerts.evaluateAll((all) => all.map((a) => `${a.dataset.tone}:${a.getAttribute("role")}`)), [
    "info:status", "success:status", "warning:status", "danger:alert", "danger:alert", "info:status",
  ]);
  check("icons", await alerts.nth(3).locator("[data-icon]").getAttribute("data-icon"), "circle-alert");
  check("iconHidden", await alerts.last().locator("[data-icon=info]").count(), 0);
  check("action", await page.getByRole("button", { name: "Try again" }).isVisible());
  await page.getByRole("button", { name: "Dismiss" }).click();
  check("dismissed", await alerts.count(), 5);

  // Error alerts: the title, the message and the tone come from the HTTP status.
  const errors = page.locator("[data-example=errors] [data-alert]");
  const read = (n) => errors.nth(n).evaluate((a) => [a.dataset.errorStatus ?? null, a.dataset.tone, a.getAttribute("role"), [...a.querySelectorAll("p, p + div, div > div:only-child")].map((e) => e.textContent.trim())]);
  check("notFound", await read(0), ["404", "danger", "alert", ["Not found", "It may have been moved or deleted."]]);
  check("tooManyIsAWarning", await read(1), ["429", "warning", "status", ["Too many requests", "Wait a moment and try again."]]);
  check("ownMessageKept", await read(2), ["503", "danger", "alert", ["Something went wrong on our side", "The index is rebuilding."]]);
  check("noStatus", await read(3), [null, "danger", "alert", ["The upload was interrupted."]]);
  check("ownTitle", await read(4), ["401", "danger", "alert", ["Sitzung abgelaufen", "Bitte melden Sie sich erneut an."]]);
  check("ownTone", [await errors.nth(5).getAttribute("data-tone"), await errors.nth(5).getAttribute("role"), await errors.nth(5).locator("[data-icon]").count()], ["info", "status", 0]);
  check("retryStrings", await errors.nth(4).getByRole("link").textContent().then((t) => t.trim()), "Erneut versuchen");
  check("actionsAfterRetry", await errors.nth(4).locator("a, button").evaluateAll((all) => all.map((e) => e.textContent.trim())), ["Erneut versuchen", "Details"]);
  check("noRetryWithoutHref", await errors.nth(1).getByRole("link").count(), 0);
  // Retry is a link: Tab reaches it and Enter requests the page again.
  const retry = errors.first().getByRole("link", { name: "Try again" });
  await retry.focus();
  check("retryFocus", await focused(), "Try again");
  await page.keyboard.press("Enter");
  await page.waitForURL("**/c/alert?retried=1");
  check("retryFollows", await page.locator("[data-example=errors] [data-alert]").count(), 6);
};
