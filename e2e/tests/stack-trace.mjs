export default async ({ page, check, button, setDir }) => {
  const target = page.getByLabel("Paste target");
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };
  const text = (locator) => locator.evaluate((e) => e.textContent.replace(/\s+/g, " ").trim());
  const frames = (root) => root.locator("li[class*=frame]:not([class*=internal])");

  // Closed: the trigger shows the parsed type and message; the frames are hidden.
  const closed = page.locator("#node-trace");
  const trigger = closed.locator("button[aria-controls]");
  check("trigger", [await trigger.getAttribute("aria-expanded"), await text(trigger.locator("[dir=ltr] > span").first()), await text(trigger.locator("[dir=ltr] > span").last())], ["false", "TypeError:", "Cannot read properties of undefined (reading 'id')"]);
  check("hidden", await page.locator("#node-trace-frames").isHidden(), true);
  check("alert", await trigger.locator("[data-icon]").first().getAttribute("data-icon"), "triangle-alert");

  // The keyboard opens it; the frames are a region named by the error.
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.locator("#node-trace-frames").waitFor({ state: "visible" });
  check("opened", await trigger.getAttribute("aria-expanded"), "true");
  const region = page.getByRole("region", { name: "TypeError: Cannot read properties of undefined (reading 'id')" }).first();
  check("region", [await region.getAttribute("tabindex"), await region.getAttribute("dir")], ["0", "ltr"]);
  // Parsed frames: function and location; the run of four internal frames is folded.
  check("frames", await frames(closed).evaluateAll((all) => all.filter((li) => li.checkVisibility()).map((li) => li.textContent.replace(/\s+/g, " ").trim())), [
    "at getUser (/app/src/users.ts:42:18)",
    "at handler (/app/src/routes/profile.ts:17:22)",
    "at async main (/app/src/server.ts:8:3)",
  ]);
  const fold = closed.locator("details");
  check("fold", [await fold.count(), await text(fold.locator("summary")), await fold.evaluate((d) => d.open)], [1, "4 internal frames", false]);
  // The fold opens from the keyboard and shows dimmed frames.
  await fold.locator("summary").focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#node-trace details").open);
  const internal = fold.locator("li");
  await internal.first().waitFor();
  check("internal", [await internal.count(), await internal.evaluateAll((all) => all.every((li) => li.hasAttribute("data-internal"))), await text(internal.last())], [4, true, "at processTicksAndRejections (node:internal/process/task_queues:95:5)"]);
  const dim = await closed.evaluate((root) => {
    const plain = root.querySelector("li:not([data-internal]) [class*=fn]").parentElement;
    const quiet = root.querySelector("li[data-internal]");
    return getComputedStyle(plain).color !== getComputedStyle(quiet).color;
  });
  check("dimmed", dim, true);
  // Space closes the panel from its trigger.
  await trigger.focus();
  await page.keyboard.press("Space");
  await page.locator("#node-trace-frames").waitFor({ state: "hidden" });
  check("closes", await trigger.getAttribute("aria-expanded"), "false");

  // Copy copies the raw trace.
  await closed.getByRole("button", { name: "Copy stack trace" }).click();
  await closed.locator("[data-copy][data-state=copied]").waitFor();
  const copied = await pasted();
  check("copy", [copied.split("\n").length, copied.startsWith("TypeError: Cannot read"), copied.endsWith("(/app/src/server.ts:8:3)")], [8, true, true]);

  // Locations as buttons send the frame to the page.
  const open = page.locator("#open-trace");
  const location = open.getByRole("button", { name: "/app/src/routes/profile.ts:17:22" });
  await location.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("[data-frame-clicked]").textContent !== "");
  check("frameEvent", await page.locator("[data-frame-clicked]").textContent(), "/app/src/routes/profile.ts:17:22");

  // Firefox's format; the message keeps its second line; internal frames are listed.
  const firefox = page.locator("#firefox-trace");
  check("firefoxHeader", [await text(firefox.locator("button[aria-controls] [dir=ltr] > span").first()), (await firefox.locator("button[aria-controls] [dir=ltr] > span").last().textContent()).includes("Received 12")], ["RangeError [ERR_OUT_OF_RANGE]:", true]);
  check("firefoxFrames", await frames(firefox).evaluateAll((all) => all.map((li) => li.textContent.replace(/\s+/g, " ").trim())), [
    "at readUInt32 (http://localhost:3000/assets/buffer.js:210:11)",
    "at parseHeader (http://localhost:3000/assets/parser.js:33:20)",
    "at (http://localhost:3000/assets/main.js:5:1)",
  ]);
  check("noFold", await firefox.locator("details").count(), 0);

  // Hidden internal frames; a lower height that scrolls from the keyboard.
  const hidden = page.locator("#hidden-trace");
  check("hideInternal", [await frames(hidden).count(), await hidden.locator("details").count()], [3, 0]);
  const scroller = hidden.getByRole("region");
  check("scrolls", await scroller.evaluate((r) => r.scrollHeight > r.clientHeight), true);
  await scroller.focus();
  await page.keyboard.press("End");
  await page.waitForFunction(() => document.querySelector("#hidden-trace [role=region]").scrollTop > 0);
  check("scrolled", await scroller.evaluate((r) => r.scrollTop > 0), true);

  // No frames: the whole text is the message.
  const empty = page.locator("#empty-trace");
  check("empty", [await text(empty.locator("button[aria-controls] [dir=ltr]")), await text(empty.getByRole("region"))], ["Something went wrong", "No stack frames"]);

  const german = page.locator("#german-trace");
  check("german", [await german.getByRole("button", { name: "Stacktrace kopieren" }).count(), await text(german.locator("details summary")), await text(frames(german).first())], [1, "2 interne Frames", "bei getUser (/app/src/users.ts:42:18)"]);

  // Right to left: the copy button goes to the left; the frames stay left to right.
  await setDir("rtl");
  const rtl = await open.evaluate((root) => {
    const copy = root.querySelector("[data-copy]").getBoundingClientRect();
    const triggerBox = root.querySelector("button[aria-controls]").getBoundingClientRect();
    const frame = root.querySelector("li");
    return [copy.right <= triggerBox.left + 1, getComputedStyle(frame).direction];
  });
  check("rtl", rtl, [true, "ltr"]);
};
