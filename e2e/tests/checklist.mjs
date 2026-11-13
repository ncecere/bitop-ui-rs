export default async ({ page, check, focused, setDir }) => {
  const list = (name) => page.getByRole("region", { name, exact: true });
  const started = list("Get started");
  check("heading", await started.getByRole("heading", { name: "Get started", level: 3 }).count(), 1);

  // The summary names a progress bar; its value is the share of steps done.
  const bar = started.getByRole("progressbar", { name: "1 of 4 done" });
  check("progress", [await bar.getAttribute("aria-valuenow"), await bar.getAttribute("aria-valuetext")], ["25", "25%"]);

  // An ordered list; each step says in words whether it is done.
  const steps = started.locator("ol > li");
  check("steps", await steps.evaluateAll((all) => all.map((li) => li.querySelector("p").textContent.replace(/\s+/g, " ").trim())), [
    "Done: Add a data source", "To do: Create a knowledge base", "To do: Invite your team", "To do: Read the guide",
  ]);
  check("state", await steps.evaluateAll((all) => all.map((li) => [li.hasAttribute("data-done"), li.hasAttribute("data-current")])), [[true, false], [false, true], [false, false], [false, false]]);
  // The marker is a check when done, else the number; it is hidden from assistive technology.
  check("markers", await steps.evaluateAll((all) => all.map((li) => { const m = li.querySelector("span[aria-hidden=true]"); return m.querySelector("[data-icon=check]") ? "check" : m.textContent.trim(); })), ["check", "2", "3", "4"]);
  const marker = await steps.first().locator("span[aria-hidden=true]").first().boundingBox();
  check("markerSize", [Math.round(marker.width), Math.round(marker.height)], [24, 24]);

  // Actions are links; the current step's is the solid one. A done step has none.
  check("actions", await started.locator("ol a").evaluateAll((all) => all.map((a) => [a.textContent.trim(), a.getAttribute("href"), a.dataset.variant])), [["Create", "#create", "solid"], ["Invite", "#invite", "outline"]]);
  check("doneHasNoAction", await list("Account setup").locator("ol a").count(), 0);
  const title = await steps.nth(1).getByText("Create a knowledge base").boundingBox();
  const action = await steps.nth(1).getByRole("link").boundingBox();
  check("actionAtEnd", action.x > title.x + title.width);

  // Keyboard: Tab goes from the dismiss button over the actions, Enter follows one.
  await started.getByRole("button", { name: "Dismiss Get started" }).focus();
  await page.keyboard.press("Tab");
  check("tabToAction", await focused(), "Create");
  await page.keyboard.press("Tab");
  check("tabToNext", await focused(), "Invite");
  await page.keyboard.press("Enter");
  check("enterFollows", await page.evaluate(() => location.hash), "#invite");

  // Every step done: the completion message replaces the progress bar.
  const done = list("Account setup");
  check("complete", [await done.getAttribute("data-complete"), await done.getByText("All set. You can close this list.").count(), await done.getByRole("progressbar").count()], ["", 1, 0]);
  check("noDismiss", await done.getByRole("button").count(), 0);

  // Without the bar the summary is plain text.
  const plain = list("Before you deploy");
  check("summaryText", [await plain.getByText("1 of 2 done").count(), await plain.getByRole("progressbar").count()], [1, 0]);

  // Strings.
  const german = list("Erste Schritte");
  check("strings", [
    await german.getByRole("progressbar", { name: "1 von 2 erledigt" }).count(),
    await german.locator("ol > li p").evaluateAll((all) => all.map((p) => p.textContent.replace(/\s+/g, " ").trim())),
    await german.getByRole("button").getAttribute("aria-label"),
  ], [1, ["Erledigt: Konto anlegen", "Offen: Profil ausfüllen"], "Ausblenden: Erste Schritte"]);

  // Right to left: the marker is at the right, the dismiss button at the left.
  await setDir("rtl");
  const m = await steps.nth(1).locator("span[aria-hidden=true]").first().boundingBox();
  const t = await steps.nth(1).getByText("Create a knowledge base").boundingBox();
  const head = await started.getByRole("heading").boundingBox();
  const dismissBox = await started.getByRole("button", { name: "Dismiss Get started" }).boundingBox();
  check("rtl", [m.x > t.x, dismissBox.x < head.x], [true, true]);
  await setDir("ltr");

  // Dismiss: an event the application can cancel, then the checklist is removed.
  await page.evaluate(() => {
    window.dismissed = [];
    document.addEventListener("bitop-dismiss", (event) => {
      window.dismissed.push(event.target.matches("section[data-checklist]"));
      if (event.target.textContent.includes("Erste Schritte")) event.preventDefault();
    });
  });
  await german.getByRole("button").click();
  check("cancelledStays", [await german.count(), await page.evaluate(() => window.dismissed)], [1, [true]]);
  await started.getByRole("button", { name: "Dismiss Get started" }).focus();
  await page.keyboard.press("Enter");
  check("dismissed", [await started.count(), await page.evaluate(() => window.dismissed.length)], [0, 2]);
};
