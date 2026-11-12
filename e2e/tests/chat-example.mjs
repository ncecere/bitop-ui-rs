export default async ({ page, check, axe }) => {
  const log = page.locator("#chat-log");
  const form = page.locator("#chat-composer");
  const announcer = page.locator("#chat-announcer");
  const ready = () => page.waitForFunction(() => document.getElementById("chat-composer").dataset.status === "ready" && document.querySelector("#chat-turns [data-reply-actions]:not([hidden]), #chat-turns [data-reply-stopped]:not([hidden])"));
  const atBottom = () => page.waitForFunction(() => {
    const l = document.getElementById("chat-log");
    return l.scrollHeight - l.clientHeight - l.scrollTop <= 1;
  });

  // Empty: a greeting with suggestions, and the composer.
  check("empty", [await page.locator("#chat-empty h2").textContent(), await page.locator("#chat-turns").isHidden()], ["Ask the handbook", true]);
  check("parts", [
    await page.getByRole("button", { name: "Model: Claude Sonnet" }).count(),
    await page.getByRole("link", { name: "New chat" }).count(),
    await page.getByRole("log", { name: "Chat with the handbook assistant" }).count(),
    await form.getByRole("textbox", { name: "Message" }).count(),
  ], [1, 1, 1, 1]);

  // A suggestion sends itself: the person's turn, then the reply, announced.
  await page.evaluate(() => {
    window.said = [];
    new MutationObserver(() => window.said.push(document.getElementById("chat-announcer").textContent)).observe(document.getElementById("chat-announcer"), { childList: true, characterData: true, subtree: true });
  });
  await page.getByRole("button", { name: "What is our parental leave policy?" }).click();
  await page.locator("#chat-turns [data-from=user]").waitFor();
  check("userTurn", [await page.locator("#chat-empty").isHidden(), (await page.locator("#chat-turns [data-from=user]").textContent()).trim()], [true, "What is our parental leave policy?"]);
  check("stopWhileWaiting", await form.locator("[data-prompt-submit]").getAttribute("aria-label"), "Stop generating");
  await page.locator("#chat-turns [data-from=assistant]").waitFor();
  check("actionsHiddenWhileStreaming", await page.locator("#chat-turns [data-reply-actions]").first().isHidden(), true);
  await ready();
  check("announced", await page.evaluate(() => window.said), ["Message sent", "Assistant is responding", "Response complete"]);
  const reply = page.locator("#chat-turns [data-from=assistant]").first();
  check("reasoningClosed", await reply.locator("details[data-reply-reasoning] summary").textContent().then((t) => /Thought for \d+ seconds?/.test(t)), true);
  const answer = reply.locator("[data-response]").nth(1);
  check("answer", [
    await answer.locator("table").count(),
    await answer.locator("[data-code-block]").count(),
    await answer.locator("[data-citation-trigger]").count(),
    await answer.getAttribute("data-streaming"),
  ], [1, 1, 4, null]);
  check("sources", [await reply.locator("[data-reply-sources]").isVisible(), (await reply.locator("[data-reply-sources] summary").textContent()).trim()], [true, "Used 3 sources"]);
  check("actions", await reply.getByRole("group", { name: "Message actions" }).getByRole("button").evaluateAll((all) => all.map((b) => b.getAttribute("aria-label"))), ["Copy message", "Regenerate", "Good response", "Bad response"]);
  await atBottom();
  check("followed", true);
  check("sendAgain", await form.locator("[data-prompt-submit]").getAttribute("aria-label"), "Send message");

  // A citation chip opens its card; its source action is not used here, the chip is.
  await answer.locator("[data-citation-trigger]").first().click();
  await page.waitForFunction(() => document.querySelector("#chat-turns [data-citation-card]:popover-open"));
  check("citationCard", (await page.locator("#chat-turns [data-citation-card]:popover-open a").first().textContent()).includes("Parental leave policy 2025"), true);
  await page.keyboard.press("Escape");

  // Typing with a file: Enter sends both, and the text box is cleared.
  const box = form.getByRole("textbox", { name: "Message" });
  const [chooser] = await Promise.all([page.waitForEvent("filechooser"), form.getByRole("button", { name: "Attach files" }).click()]);
  await chooser.setFiles([{ name: "contract.pdf", mimeType: "application/pdf", buffer: Buffer.alloc(20, 1) }]);
  await box.fill("Does this contract change anything?");
  await box.press("Enter");
  await page.locator("#chat-turns [data-from=user]").nth(1).waitFor();
  check("secondTurn", [(await page.locator("#chat-turns [data-from=user]").nth(1).innerText()).replace(/\s+/g, " ").trim(), await box.inputValue(), await form.locator("[data-attachment]").count()], [
    "contract.pdf Does this contract change anything?", "", 0,
  ]);
  // Stop while it streams: the reply says so and the composer is ready again.
  await page.waitForFunction(() => document.getElementById("chat-composer").dataset.status === "streaming");
  await form.locator("[data-prompt-submit]").click();
  await ready();
  const second = page.locator("#chat-turns [data-from=assistant]").nth(1);
  check("stopped", [await second.locator("[data-reply-stopped]").isVisible(), await second.locator("[data-reply-sources]").isHidden(), await form.getAttribute("data-status")], [true, true, "ready"]);
  check("uniqueIds", await page.evaluate(() => {
    const ids = [...document.querySelectorAll("[id]")].map((e) => e.id);
    return ids.length === new Set(ids).size;
  }), true);

  // Regenerate replaces the reply.
  await second.getByRole("button", { name: "Regenerate" }).click();
  await page.waitForFunction(() => document.getElementById("chat-composer").dataset.status !== "ready");
  await ready();
  check("regenerated", [await page.locator("#chat-turns [data-from=assistant]").count(), await page.locator("#chat-turns [data-from=assistant]").nth(1).locator("[data-reply-sources]").isVisible()], [2, true]);
  await atBottom();
  check("axeChat", await axe(), []);
};
