export default async ({ page, check, base }) => {
  const composer = page.locator("#composer");
  const box = composer.getByRole("textbox", { name: "Message" });
  const send = composer.locator("[data-prompt-submit]");
  const file = (name, mimeType, bytes = 10) => ({ name, mimeType, buffer: Buffer.alloc(bytes, 1) });
  const chips = () => composer.locator("[data-attachment-name]").allTextContents();
  const formFiles = () => composer.evaluate((f) => new FormData(f).getAll("files").map((x) => x.name).filter(Boolean));
  const error = composer.locator("[data-prompt-file-error]");

  // A real form with a named text box; nothing to send yet.
  check("form", await composer.evaluate((f) => [f.tagName, f.method, f.getAttribute("action"), f.querySelector("textarea").name]), ["FORM", "get", "/c/prompt-input", "message"]);
  check("emptyDisabled", [await send.isDisabled(), await send.getAttribute("aria-label"), await send.getAttribute("type")], [true, "Send message", "submit"]);
  await box.focus();
  await page.keyboard.type("Hello");
  check("textEnables", await send.isDisabled(), false);

  // Shift+Enter is a new line; the box grows with its content, up to a limit.
  const height = () => box.evaluate((b) => Math.round(b.getBoundingClientRect().height));
  const one = await height();
  await page.keyboard.press("Shift+Enter");
  await page.keyboard.type("Second line");
  check("newLine", await box.inputValue(), "Hello\nSecond line");
  check("grows", (await height()) > one, true);
  for (let i = 0; i < 12; i += 1) await page.keyboard.press("Shift+Enter");
  check("capped", Math.abs((await height()) - 224) <= 1, true);
  check("scrolls", await box.evaluate((b) => b.scrollHeight > b.clientHeight), true);

  // The Enter that confirms an input method's composition does not send.
  await box.fill("にほんご");
  await box.dispatchEvent("compositionstart");
  await page.keyboard.press("Enter");
  check("imeDoesNotSend", [page.url().includes("message="), (await box.inputValue()).startsWith("にほんご")], [false, true]);
  await box.dispatchEvent("compositionend");

  // A page that sends the message itself cancels bitop:prompt-submit: the box is cleared.
  await page.evaluate(() => {
    window.sent = [];
    document.getElementById("composer").addEventListener("bitop:prompt-submit", (event) => {
      window.sent.push([event.detail.text, event.detail.files.map((f) => f.name)]);
      event.preventDefault();
    }, { once: true });
  });
  await box.fill("Handled by the page");
  await page.keyboard.press("Enter");
  check("handled", [await page.evaluate(() => window.sent), await box.inputValue(), page.url().includes("message="), await page.evaluate(() => document.activeElement.name)], [
    [["Handled by the page", []]], "", false, "message",
  ]);
  check("disabledAgain", await send.isDisabled(), true);

  // Preview URLs that are released. (Fetching a revoked URL would tell too, but logs an
  // error in the console.)
  await page.evaluate(() => {
    window.revoked = [];
    const revoke = URL.revokeObjectURL.bind(URL);
    URL.revokeObjectURL = (url) => {
      window.revoked.push(url);
      revoke(url);
    };
  });

  // Files: the attach button opens the picker; chips with previews; sent with the form.
  const [chooser] = await Promise.all([page.waitForEvent("filechooser"), composer.getByRole("button", { name: "Attach files" }).click()]);
  await chooser.setFiles([file("photo.png", "image/png", 2048), file("notes.md", "text/markdown", 300)]);
  await composer.locator("[data-attachment]").nth(1).waitFor();
  check("chips", [await chips(), await composer.locator("[data-attachment-meta]").allTextContents(), await formFiles()], [["photo.png", "notes.md"], ["PNG · 2 KB", "MD · 300 B"], ["photo.png", "notes.md"]]);
  const preview = await composer.locator("[data-attachment] img").getAttribute("src");
  check("preview", [preview.startsWith("blob:"), await composer.locator("[data-attachment]").nth(1).locator("[data-icon]").first().getAttribute("data-icon")], [true, "file-text"]);
  check("filesEnable", await send.isDisabled(), false);
  check("listNamed", await composer.getByRole("list", { name: "Attachments" }).count(), 1);
  // Removing one revokes its preview and returns focus to the text box.
  await composer.getByRole("button", { name: "Remove photo.png" }).click();
  check("removed", [await chips(), await formFiles(), await page.evaluate(() => document.activeElement.name)], [["notes.md"], ["notes.md"], "message"]);
  check("revoked", await page.evaluate((url) => window.revoked.includes(url), preview), true);

  // Dropped and pasted files go through accept, the size limit and the count limit.
  const drop = async (target, files) => {
    const data = await page.evaluateHandle((list) => {
      const d = new DataTransfer();
      for (const [name, type, size] of list) d.items.add(new File(["x".repeat(size)], name, { type }));
      return d;
    }, files);
    await target.dispatchEvent("dragenter", { dataTransfer: data });
    await target.dispatchEvent("drop", { dataTransfer: data });
  };
  await drop(composer, [["setup.exe", "application/x-msdownload", 10], ["huge.pdf", "application/pdf", 1024 * 1024 + 1], ["report.pdf", "application/pdf", 50]]);
  check("dropChecked", [await chips(), await error.textContent()], [["notes.md", "report.pdf"], "setup.exe isn't a supported file type. huge.pdf is too large."]);
  check("dragEnds", await composer.evaluate((f) => f.hasAttribute("data-dragging")), false);
  await drop(composer, [["a.txt", "text/plain", 5], ["b.txt", "text/plain", 5]]);
  check("countLimit", [await chips(), await error.textContent()], [["notes.md", "report.pdf", "a.txt"], "You can attach up to 3 files."]);
  // Paste into the text box.
  await composer.getByRole("button", { name: "Remove a.txt" }).click();
  // A ClipboardEvent made in script carries no data in Firefox, and files cannot be put
  // on the clipboard from a check: where the event comes without them, there is no paste
  // to check.
  const pastes = await box.evaluate((b) => {
    const data = new DataTransfer();
    data.items.add(new File(["png"], "pasted.png", { type: "image/png" }));
    const event = new ClipboardEvent("paste", { clipboardData: data, bubbles: true, cancelable: true });
    if (!event.clipboardData?.files.length) return false;
    b.dispatchEvent(event);
    return true;
  });
  const expected = pastes ? ["notes.md", "report.pdf", "pasted.png"] : ["notes.md", "report.pdf"];
  check("pasted", await chips(), expected);

  // A pressed tool is sent with the form; Enter sends it all as a normal form.
  const web = composer.getByRole("button", { name: "Search the web" });
  await web.click();
  check("toolPressed", await web.getAttribute("aria-pressed"), "true");
  await box.fill("What is our leave policy?");
  await box.focus();
  await Promise.all([page.waitForURL("**/c/prompt-input?*"), page.keyboard.press("Enter")]);
  const sentQuery = new URL(page.url()).searchParams;
  check("submitted", [sentQuery.get("message"), sentQuery.getAll("files"), sentQuery.get("web")], ["What is our leave policy?", expected, "on"]);
  await page.goto(`${base}/c/prompt-input`);

  // Streaming: the button stops, Enter does not send, and Stop submits action=stop.
  const streaming = page.locator("#streaming-composer");
  const stop = streaming.locator("[data-prompt-submit]");
  check("stop", [await stop.getAttribute("aria-label"), await stop.getAttribute("data-mode"), await stop.getAttribute("name"), await stop.getAttribute("value"), await stop.isDisabled()], ["Stop generating", "stop", "action", "stop", false]);
  check("stopIcon", await stop.locator("[data-icon]:visible").getAttribute("data-icon"), "square");
  await streaming.locator("textarea").focus();
  await page.keyboard.press("Enter");
  await page.waitForTimeout(100);
  check("busyIgnoresEnter", page.url().includes("?"), false);
  // A page that stops the answer itself cancels bitop:prompt-stop.
  await page.evaluate(() => document.addEventListener("bitop:prompt-stop", (event) => { window.stopped = true; event.preventDefault(); }, { once: true }));
  await stop.click();
  check("stopEvent", [await page.evaluate(() => window.stopped), page.url().includes("?")], [true, false]);
  await Promise.all([page.waitForURL("**/c/prompt-input?*"), stop.click()]);
  check("stopSubmits", new URL(page.url()).searchParams.get("action"), "stop");
  await page.goto(`${base}/c/prompt-input`);

  // The button follows data-status when an app changes it.
  const live = page.locator("#streaming-composer [data-prompt-submit]");
  await page.locator("#streaming-composer").evaluate((f) => (f.dataset.status = "ready"));
  await page.waitForFunction(() => document.querySelector("#streaming-composer [data-prompt-submit]").dataset.mode === "send");
  check("toReady", [await live.getAttribute("aria-label"), await live.getAttribute("name"), await live.isDisabled()], ["Send message", null, false]);
  await page.locator("#streaming-composer").evaluate((f) => (f.dataset.status = "submitted"));
  await page.waitForFunction(() => document.querySelector("#streaming-composer [data-prompt-submit]").dataset.mode === "sending");
  check("toSubmitted", [await live.getAttribute("aria-label"), await live.isDisabled(), await live.locator("svg:visible").count()], ["Sending message", true, 1]);

  // Waiting: a disabled spinner, or Stop already.
  const sending = page.locator("#sending-composer [data-prompt-submit]");
  check("sending", [await sending.getAttribute("aria-label"), await sending.isDisabled(), await sending.locator("[data-icon]:visible").count(), await sending.locator("svg:visible").count()], ["Sending message", true, 0, 1]);
  const early = page.locator("#early-stop-composer [data-prompt-submit]");
  check("earlyStop", [await early.getAttribute("aria-label"), await early.isDisabled(), (await early.textContent()).trim(), await early.locator("svg:visible").count()], ["Stop generating", false, "Stop", 1]);

  // A character limit: a counter near it, and an announcement at it.
  const limited = page.getByRole("textbox", { name: "Short message" });
  const counter = page.locator("#limit-composer [data-prompt-count]");
  await limited.fill("x".repeat(20));
  check("noCounterYet", await counter.isHidden(), true);
  await limited.fill("x".repeat(32));
  check("counter", [await counter.isVisible(), (await counter.textContent()).replace(/\s+/g, " ").trim()], [true, "32 / 40 characters"]);
  await limited.fill("x".repeat(40));
  check("atLimit", [await counter.getAttribute("data-limit"), await page.locator("#limit-composer [data-prompt-limit]").textContent()], ["", "Character limit of 40 reached"]);
  check("alwaysCounter", (await page.locator("[data-example=limit] [data-prompt-count]").nth(1).textContent()).replace(/\s+/g, " ").trim(), "0 / 2,000 characters");
  // Enter is a new line when it does not send; Ctrl+Enter still sends.
  const long = page.getByRole("textbox", { name: "Long message" });
  await long.fill("one");
  await long.press("Enter");
  check("enterNewLine", await long.inputValue(), "one\n");

  // Disabled: nothing can be typed, sent, attached or dropped.
  const disabled = page.locator("#disabled-composer");
  check("disabled", await disabled.evaluate((f) => [...f.querySelectorAll("textarea, button")].map((c) => c.disabled)), [true, true, true, true]);
  await drop(disabled, [["ok.png", "image/png", 5]]);
  check("dropIgnored", [await disabled.locator("[data-attachment]").count(), await disabled.evaluate((f) => f.querySelector("[data-prompt-files]").files.length)], [0, 0]);

  // One file at a time: the next replaces it, and the replaced preview is revoked.
  await page.evaluate(() => {
    window.revoked = [];
    const revoke = URL.revokeObjectURL.bind(URL);
    URL.revokeObjectURL = (url) => {
      window.revoked.push(url);
      revoke(url);
    };
  });
  const single = page.locator("#single-composer");
  await drop(single, [["one.png", "image/png", 5], ["two.png", "image/png", 5]]);
  check("singleKeepsFirst", await single.locator("[data-attachment-name]").allTextContents(), ["one.png"]);
  const first = await single.locator("[data-attachment] img").getAttribute("src");
  await drop(single, [["three.png", "image/png", 5]]);
  check("singleReplaced", await single.locator("[data-attachment-name]").allTextContents(), ["three.png"]);
  check("replacedRevoked", await page.evaluate((url) => window.revoked.includes(url), first), true);

  // Other strings.
  const german = page.locator("#german-composer");
  check("germanParts", [
    await german.getByRole("textbox", { name: "Nachricht" }).getAttribute("placeholder"),
    await german.getByRole("button", { name: "Dateien anhängen" }).count(),
    await german.locator("[data-prompt-submit]").getAttribute("aria-label"),
    (await german.locator("[data-prompt-submit]").textContent()).trim(),
  ], ["Frag etwas…", 1, "Nachricht senden", "Senden"]);
  await drop(german, [["bild.png", "image/png", 5], ["brief.pdf", "application/pdf", 5]]);
  check("germanFiles", [await german.getByRole("list", { name: "Anhänge" }).locator("[data-attachment-name]").allTextContents(), await german.locator("[data-prompt-file-error]").textContent(), await german.getByRole("button", { name: "brief.pdf entfernen" }).count()], [["brief.pdf"], "bild.png wird nicht unterstützt.", 1]);
  await german.locator("textarea").fill("0123456789");
  check("germanLimit", await german.locator("[data-prompt-limit]").textContent(), "Höchstens 10 Zeichen");
};
