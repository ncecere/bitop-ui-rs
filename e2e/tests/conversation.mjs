export default async ({ page, check }) => {
  const log = page.locator("#log");
  const button = page.locator("[data-example=log] [data-conversation-scroll]");
  const distance = () => log.evaluate((l) => Math.round(l.scrollHeight - l.clientHeight - l.scrollTop));
  const atBottom = () => page.waitForFunction(() => {
    const l = document.getElementById("log");
    return l.scrollHeight - l.clientHeight - l.scrollTop <= 1;
  });
  // Adds a paragraph to the last message, as a server streaming HTML would.
  const grow = (text = "More of the answer arrives and makes the log taller than before.") =>
    page.evaluate((t) => {
      const p = document.createElement("p");
      p.textContent = t;
      document.getElementById("streamed").after(p);
    }, text);

  // A focusable log, silent for screen readers, scrolled to the latest message.
  check("log", [await log.getAttribute("role"), await log.getAttribute("aria-live"), await log.getAttribute("aria-label"), await log.getAttribute("tabindex")], [
    "log", "off", "Chat with the handbook assistant", "0",
  ]);
  await atBottom();
  check("startsAtBottom", await distance() <= 1, true);
  check("buttonHidden", await button.isHidden(), true);

  // Content that grows keeps the view at the bottom, by node and by text.
  for (let i = 0; i < 4; i += 1) await grow();
  await atBottom();
  await page.evaluate(() => document.getElementById("streamed").append("Streamed words. ".repeat(40)));
  await atBottom();
  check("follows", await distance() <= 1, true);

  // Scrolling up with the wheel lets go: growth no longer moves the view, and the
  // button appears.
  const box = await log.boundingBox();
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.wheel(0, -300);
  await page.waitForFunction(() => {
    const l = document.getElementById("log");
    return l.scrollHeight - l.clientHeight - l.scrollTop > 100;
  });
  await button.waitFor();
  // WebKit scrolls by the wheel smoothly: wait until the log has stopped moving.
  await log.evaluate((l) => new Promise((resolve, reject) => {
    const started = performance.now();
    let last = -1;
    let still = 0;
    const poll = () => {
      still = l.scrollTop === last ? still + 1 : 0;
      last = l.scrollTop;
      if (still >= 6) resolve();
      else if (performance.now() - started > 30_000) reject(new Error("the log did not stop scrolling"));
      else requestAnimationFrame(poll);
    };
    poll();
  }));
  const before = await log.evaluate((l) => l.scrollTop);
  await grow();
  await grow();
  await page.waitForTimeout(150);
  check("staysWhenScrolledUp", Math.abs((await log.evaluate((l) => l.scrollTop)) - before) <= 1, true);
  check("buttonShown", [await button.isVisible(), await button.getByRole("button").getAttribute("aria-label")], [true, "Scroll to latest message"]);

  // The button brings the view back and keeps focus in the log; following resumes.
  await button.getByRole("button").focus();
  await page.keyboard.press("Enter");
  await atBottom();
  await button.waitFor({ state: "hidden" });
  check("backToBottom", await page.evaluate(() => document.activeElement.id), "log");
  await grow();
  await atBottom();
  check("followsAgain", await distance() <= 1, true);

  // The keyboard: Home in the log lets go, End returns, and following resumes.
  await log.focus();
  await page.keyboard.press("Home");
  await page.waitForFunction(() => document.getElementById("log").scrollTop < 5);
  await grow();
  await page.waitForTimeout(150);
  check("homeLetsGo", await log.evaluate((l) => l.scrollTop) < 5, true);
  await button.waitFor();
  await page.keyboard.press("End");
  await atBottom();
  await grow();
  await atBottom();
  check("endSticks", await distance() <= 1, true);

  // Content that shrinks does not let go.
  await page.evaluate(() => document.querySelectorAll("#log-content p:not([id])").forEach((p, i) => i < 3 && p.remove()));
  await grow();
  await atBottom();
  check("shrinkKeeps", await distance() <= 1, true);

  // The script can show an answer from its start; that stops following.
  await page.evaluate(() => window.bitopUi.conversation.scrollToElement(document.getElementById("log"), document.querySelector("#log article"), { behavior: "instant" }));
  await grow();
  await page.waitForTimeout(150);
  check("scrollToElement", await log.evaluate((l) => Math.abs(l.querySelector("article").getBoundingClientRect().top - l.getBoundingClientRect().top) <= 1), true);
  await page.evaluate(() => window.bitopUi.conversation.scrollToBottom(document.getElementById("log"), "instant"));
  await atBottom();

  // The announcer speaks the transitions, and the same words again.
  const announcer = page.locator("#announcer");
  const say = async (status) => {
    await announcer.evaluate((a, s) => (a.dataset.status = s), status);
    await page.waitForTimeout(50);
    return announcer.textContent();
  };
  check("announcerRole", [await announcer.getAttribute("role"), await announcer.textContent()], ["status", ""]);
  check("submitted", await say("submitted"), "Message sent");
  check("streaming", await say("streaming"), "Assistant is responding");
  check("complete", await say("ready"), "Response complete");
  check("error", await say("error"), "The response failed");
  check("readyAfterError", await say("ready"), "The response failed");
  check("again", await say("error"), "The response failed\u00a0");
  check("thirdTime", await say("submitted"), "Message sent\u00a0");

  // The gallery's streamed reply: the log follows it.
  await page.getByRole("button", { name: "Stream a reply" }).click();
  await page.waitForFunction(() => document.getElementById("announcer").dataset.status === "ready" && document.getElementById("streamed").textContent.length > 100);
  await atBottom();
  check("streamedReply", [await distance() <= 1, (await announcer.textContent()).startsWith("Response complete")], [true, true]);

  // Empty: following is off, so a tall welcome starts at its top.
  const empty = page.locator("[data-example=empty] [data-conversation-viewport]");
  check("emptyNotFollowing", [await empty.getAttribute("data-stick"), await empty.evaluate((l) => l.scrollTop)], ["false", 0]);
  check("emptyState", [await page.locator("[data-example=empty] h3").textContent(), await page.locator("[data-example=empty] [data-icon=book-open-text]").count()], ["Ask the handbook", 1]);
  check("emptySuggestions", await page.locator("[data-example=empty]").getByRole("group", { name: "Suggestions" }).getByRole("button").count(), 2);
  // An avatar replaces the icon; the default title.
  const media = page.locator("[data-example=media]");
  check("media", [await media.locator("[data-icon]").isVisible(), await media.locator("[role=img]").count(), (await media.locator("p").first().textContent()).trim()], [false, 1, "How can I help?"]);
  // Turning following on sticks once.
  await empty.evaluate((l) => {
    l.firstElementChild.style.minBlockSize = "60rem";
    l.dataset.stick = "true";
  });
  await page.waitForFunction(() => {
    const l = document.querySelector("[data-example=empty] [data-conversation-viewport]");
    return l.scrollTop > 0 && l.scrollHeight - l.clientHeight - l.scrollTop <= 1;
  });
  check("stickTurnedOn", true);
  await empty.evaluate((l) => {
    l.firstElementChild.style.minBlockSize = "";
    l.dataset.stick = "false";
  });

  // Right to left: the button stays centred.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  // The new direction resizes the log, which pins it while it follows: let that pass first.
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  await page.evaluate(() => document.getElementById("log").scrollTo({ top: 0, behavior: "instant" }));
  await button.waitFor();
  const centred = await page.evaluate(() => {
    const b = document.querySelector("[data-example=log] [data-conversation-scroll]").getBoundingClientRect();
    const r = document.querySelector("[data-example=log] [data-conversation]").getBoundingClientRect();
    return Math.abs(b.left + b.width / 2 - (r.left + r.width / 2)) <= 1;
  });
  check("rtlCentred", centred, true);
  await page.evaluate(() => (document.documentElement.dir = "ltr"));

  // Other strings.
  const german = page.getByRole("log", { name: "Unterhaltung" });
  check("germanLog", await german.count(), 1);
  await german.evaluate((l) => l.scrollTo({ top: 0, behavior: "instant" }));
  await page.locator("[data-example=strings] [data-conversation-scroll]").waitFor();
  check("germanButton", await page.locator("[data-example=strings] [data-conversation-scroll] button").getAttribute("aria-label"), "Zur neuesten Nachricht");
  const gAnnouncer = page.locator("#german-announcer");
  await gAnnouncer.evaluate((a) => (a.dataset.status = "ready"));
  await page.waitForFunction(() => document.getElementById("german-announcer").textContent === "Antwort fertig");
  check("germanAnnouncer", await gAnnouncer.textContent(), "Antwort fertig");
};
