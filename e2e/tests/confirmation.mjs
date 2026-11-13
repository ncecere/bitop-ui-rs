export default async ({ page, check, base }) => {
  const shown = (card) => card.evaluate((c) => [...c.querySelectorAll("[data-when]")].filter((e) => e.checkVisibility()).map((e) => e.textContent.trim() || e.dataset.icon));

  // Requested: the question, the request and a form with the actions.
  const card = page.locator("#form-confirmation");
  check("group", [await card.getAttribute("role"), await card.getAttribute("aria-label"), await card.getAttribute("data-state")], ["group", "Send the email?", "requested"]);
  check("requested", await shown(card), ["shield-question", "The agent wants to call send_email.", "RejectApprove"]);
  check("form", await card.locator("form").evaluate((f) => [f.method, new URL(f.action).pathname]), ["get", "/c/confirmation"]);

  // The keyboard approves; the server renders the new state.
  await card.getByRole("button", { name: "Approve" }).focus();
  await Promise.all([page.waitForURL(/decision=approve/), page.keyboard.press("Enter")]);
  check("sent", new URL(page.url()).searchParams.get("call"), "call-7");
  check("accepted", [await card.getAttribute("data-state"), await shown(card)], ["accepted", ["circle-check", "Approved. Sending…"]]);
  check("noActions", await card.getByRole("button").count(), 0);
  await page.goto(`${base}/c/confirmation`);
  await card.getByRole("button", { name: "Reject" }).click();
  await page.waitForURL(/decision=reject/);
  check("rejected", await shown(card), ["circle-x", "Rejected. Nothing was sent."]);
  await page.goto(`${base}/c/confirmation`);

  // In script: the page cancels the event and sets the state. The outcome is
  // announced, and focus moves from the button that has gone to the card.
  const scripted = page.locator("#script-confirmation");
  await page.evaluate(() => {
    window.answers = [];
    document.addEventListener("bitop:confirmation", (e) => window.answers.push([e.detail.name, e.detail.value]));
  });
  await scripted.getByRole("button", { name: "Keep it" }).focus();
  await page.keyboard.press("Tab");
  check("tabToDelete", await page.evaluate(() => document.activeElement.textContent.trim()), "Delete");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("script-confirmation").dataset.state === "accepted");
  check("event", await page.evaluate(() => window.answers), [["decision", "approve"]]);
  check("stayed", page.url(), `${base}/c/confirmation`);
  check("announced", await scripted.locator("[role=status]").textContent(), "Approved. The branch was deleted.");
  check("focusToCard", await page.evaluate(() => document.activeElement.id), "script-confirmation");
  check("scriptShown", await shown(scripted), ["circle-check", "Approved. The branch was deleted."]);

  // A card that is already answered: no actions, and the outcome shows.
  const answered = page.locator("[data-example=answered] [data-confirmation]");
  check("answered", [await shown(answered.first()), await shown(answered.nth(1))], [["circle-check", "Approved."], ["circle-x", "Rejected. Nothing was posted."]]);

  // Right to left: the icon is at the start.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const iconRight = await answered.first().evaluate((c) => c.firstElementChild.getBoundingClientRect().right > c.lastElementChild.getBoundingClientRect().right);
  check("rtlIcon", iconRight, true);
};
