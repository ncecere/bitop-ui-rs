export default async ({ page, check, base }) => {
  const turns = page.locator("[data-example=turns]");

  // Each turn is an article named for its sender; the user's is a bubble on the end side.
  const user = turns.getByRole("article", { name: "You said" });
  const assistant = turns.getByRole("article", { name: "Assistant said" });
  check("articles", [await user.getAttribute("data-from"), await assistant.getAttribute("data-from")], ["user", "assistant"]);
  const bubble = user.locator("div").first();
  check("bubble", await bubble.evaluate((b) => [getComputedStyle(b).whiteSpace, getComputedStyle(b).backgroundColor !== "rgba(0, 0, 0, 0)"]), ["pre-wrap", true]);
  const ends = await turns.evaluate((t) => {
    const box = t.getBoundingClientRect();
    const b = t.querySelector("[data-from=user] > div").getBoundingClientRect();
    return Math.abs(b.right - box.right) <= 1;
  });
  check("bubbleAtEnd", ends, true);
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  check("bubbleAtEndRtl", await turns.evaluate((t) => Math.abs(t.querySelector("[data-from=user] > div").getBoundingClientRect().left - t.getBoundingClientRect().left) <= 1), true);
  await page.evaluate(() => (document.documentElement.dir = "ltr"));

  // Actions: a named group of icon buttons with tooltips.
  const actions = assistant.getByRole("group", { name: "Message actions" });
  check("actions", await actions.getByRole("button").evaluateAll((all) => all.map((b) => b.getAttribute("aria-label"))), ["Copy message", "Regenerate", "Good response", "Bad response"]);
  await actions.getByRole("button", { name: "Regenerate" }).focus();
  await page.waitForFunction(() => document.getElementById("regenerate-tip").matches(":popover-open"));
  check("tooltip", (await page.locator("#regenerate-tip").textContent()).trim(), "Regenerate");

  // Feedback: toggles that release each other.
  const good = actions.getByRole("button", { name: "Good response" });
  const bad = actions.getByRole("button", { name: "Bad response" });
  const pressed = () => Promise.all([good.getAttribute("aria-pressed"), bad.getAttribute("aria-pressed")]);
  check("unpressed", await pressed(), ["false", "false"]);
  await good.focus();
  await page.keyboard.press("Enter");
  check("good", await pressed(), ["true", "false"]);
  await page.keyboard.press("Tab");
  await page.keyboard.press("Space");
  check("exclusive", await pressed(), ["false", "true"]);
  await page.keyboard.press("Space");
  check("release", await pressed(), ["false", "false"]);

  // Copy: the message's text, confirmed and announced.
  await actions.getByRole("button", { name: "Copy message" }).click();
  await actions.locator("[data-state=copied]").waitFor();
  check("copied", await actions.locator("[role=status]").textContent(), "Copied message to clipboard");

  // Avatars are decorative; actions on hover appear on hover and on focus.
  const avatars = page.locator("[data-example=avatars]");
  check("avatarHidden", await avatars.locator("[data-from] > span[aria-hidden=true]").count(), 2);
  check("ownLabel", await avatars.getByRole("article", { name: "Handbook assistant said" }).count(), 1);
  // The avatar inside is decorative too; a custom colour fills the tile, square for the assistant.
  check("avatarDecorative", await avatars.locator("[data-hue]").evaluateAll((all) => all.map((a) => [a.getAttribute("aria-hidden"), a.getAttribute("role")])), [["true", null], ["true", null]]);
  const tile = avatars.locator("[data-color] [data-hue]");
  check("avatarColour", await tile.evaluate((a) => {
    const probe = document.createElement("div");
    probe.style.background = "var(--color-primary)";
    document.body.append(probe);
    const primary = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return getComputedStyle(a).backgroundColor === primary;
  }));
  check("avatarSquare", await tile.evaluate((a) => [a.dataset.shape, a.dataset.size, getComputedStyle(a).borderTopLeftRadius]), ["square", "sm", "6px"]);
  const hoverActions = avatars.locator("[data-visibility=hover]");
  await page.mouse.move(1, 1);
  check("hiddenUntilHover", await hoverActions.evaluate((a) => getComputedStyle(a).opacity), "0");
  await avatars.getByRole("article", { name: "Handbook assistant said" }).hover();
  await page.waitForFunction(() => getComputedStyle(document.querySelector("[data-visibility=hover]")).opacity === "1");
  check("hoverShows", true);
  await page.mouse.move(1, 1);
  await hoverActions.getByRole("button").focus();
  await page.waitForFunction(() => getComputedStyle(document.querySelector("[data-visibility=hover]")).opacity === "1");
  check("focusShows", true);
  check("bubbleVariant", await avatars.locator("[data-variant=bubble]").evaluate((b) => getComputedStyle(b).borderRadius), "20px");

  // Feedback sent with a form: only the pressed vote is submitted.
  const form = page.locator("[data-example=feedback]");
  const votes = () => form.evaluate((f) => new FormData(f).getAll("vote"));
  check("formVote", await votes(), ["up"]);
  await form.getByRole("button", { name: "Not helpful" }).click();
  check("formVoteChanged", await votes(), ["down"]);
  check("disabledAction", await form.getByRole("button", { name: "Regenerate" }).isDisabled(), true);
  await Promise.all([page.waitForURL("**/c/message?vote=down"), form.evaluate((f) => f.requestSubmit())]);
  check("submitted", await page.locator("[data-query]").textContent(), "vote=down");
  await page.goto(`${base}/c/message`);

  // Versions: "2 / 3", previous and next, the status announced politely.
  const branch = page.locator("#branch");
  const read = () => branch.evaluate((b) => [
    [...b.querySelector("[data-message-branch-content]").children].map((v) => v.hidden),
    b.querySelector("[data-branch-page]").textContent,
    b.querySelector("[data-branch-status]").textContent,
    [...b.querySelectorAll("[data-branch-step]")].map((s) => s.disabled),
  ]);
  check("branchStart", await read(), [[true, false, true], "2 / 3", "Response 2 of 3", [false, false]]);
  check("branchGroup", await page.getByRole("group", { name: "Response versions" }).first().getAttribute("data-message-branch-selector"), "");
  check("live", await branch.locator("[data-branch-status]").getAttribute("aria-live"), "polite");
  await page.evaluate(() => {
    window.branchEvents = [];
    document.addEventListener("bitop:branch-change", (e) => window.branchEvents.push(e.detail.index));
  });
  const next = branch.getByRole("button", { name: "Next response" });
  await next.focus();
  await page.keyboard.press("Enter");
  check("branchNext", await read(), [[true, true, false], "3 / 3", "Response 3 of 3", [false, true]]);
  check("focusMoved", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Previous response");
  await page.keyboard.press("Enter");
  await page.keyboard.press("Enter");
  check("branchFirst", await read(), [[false, true, true], "1 / 3", "Response 1 of 3", [true, false]]);
  check("events", await page.evaluate(() => window.branchEvents), [2, 1, 0]);
  check("focusAtStart", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Next response");

  // A version removed by the server while the last one is shown: the index is clamped
  // and reported.
  await next.click();
  await next.click();
  await page.evaluate(() => {
    window.branchEvents = [];
    document.querySelector("#branch [data-message-branch-content]").lastElementChild.remove();
  });
  await page.waitForFunction(() => window.branchEvents.length === 1);
  check("shrinkClamps", [await read(), await page.evaluate(() => window.branchEvents)], [[[true, false], "2 / 2", "Response 2 of 2", [false, true]], [1]]);

  // One version: no selector. An index past the end shows the last version.
  const clamp = page.locator("[data-example=clamp]");
  check("single", await clamp.locator("[data-message-branch-selector]").first().isHidden(), true);
  check("pastEnd", await page.locator("#clamped-branch").evaluate((b) => [b.dataset.index, [...b.querySelector("[data-message-branch-content]").children].map((v) => v.hidden), b.querySelector("[data-branch-page]").textContent]), ["1", [true, false], "2 / 2"]);

  // Other strings.
  const german = page.locator("[data-example=strings]");
  check("germanArticles", await german.getByRole("article").evaluateAll((all) => all.filter((a) => !a.hidden).map((a) => a.getAttribute("aria-label"))), ["Sie sagten", "Der Assistent sagte"]);
  const germanGroup = german.getByRole("group", { name: "Antwortversionen" });
  await germanGroup.getByRole("button", { name: "Nächste Antwort" }).click();
  check("germanStatus", await germanGroup.locator("[data-branch-status]").textContent(), "Antwort 2 von 2");
  check("germanCopy", await german.getByRole("group", { name: "Aktionen" }).getByRole("button").first().getAttribute("aria-label"), "Nachricht kopieren");
};
