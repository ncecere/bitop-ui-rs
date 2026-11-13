export default async ({ page, check, base }) => {
  const root = page.locator("[data-example=form] [data-mic-selector]");
  const select = page.locator("#mic");
  const options = () => select.locator("option").evaluateAll((all) => all.map((o) => [o.value, o.textContent]));
  const state = () =>
    root.evaluate((r) => ({
      field: !r.querySelector("[data-mic-field]").hidden,
      permission: !r.querySelector("[data-mic-permission]").hidden,
      unsupported: !r.querySelector("[data-mic-unsupported]").hidden,
      status: r.querySelector("[data-mic-status]").textContent.trim(),
    }));

  // What the browser itself reports, to compare the control with. Engines without
  // mediaDevices must show the unsupported text; the stubbed page below checks that
  // state in every engine.
  await page.waitForFunction(() => document.querySelector("[data-mic-selector]").dataset.status !== "loading");
  const reported = await page.evaluate(async () => {
    if (!navigator.mediaDevices?.enumerateDevices) return null;
    const devices = (await navigator.mediaDevices.enumerateDevices()).filter((d) => d.kind === "audioinput");
    return { ids: devices.filter((d) => d.deviceId).map((d) => d.deviceId), named: devices.some((d) => d.label) };
  });
  if (!reported) {
    check("unsupportedHere", (await state()).unsupported, true);
  } else {
    check("listed", (await options()).slice(1).map(([id]) => id), reported.ids);
    check("fieldShown", (await state()).field, reported.ids.length > 0);
    check("askShown", (await state()).permission, !reported.named);
    check("placeholder", (await options())[0], ["", "Select a microphone"]);

    // Asking for access: Chromium and Firefox run with a fake microphone that is
    // allowed; WebKit has none, so it reports an error, which the selector says.
    if (!reported.named) {
      const ask = root.getByRole("button", { name: "Allow microphone access" });
      const result = page.evaluate(() => new Promise((resolve) => document.addEventListener("bitop:mic-permission", (e) => resolve(e.detail), { once: true })));
      await ask.focus();
      await page.keyboard.press("Enter");
      const detail = await result;
      if (detail.granted) {
        await page.waitForFunction(() => document.querySelector("#mic").options.length > 1);
        const after = await state();
        check("granted", [after.permission, after.field, after.status], [false, true, ""]);
        check("named", (await options()).slice(1).every(([id, label]) => id && label), true);
      } else {
        const messages = [
          "Microphone access is blocked. Allow it in your browser's site settings, then try again.",
          "No microphone was found.",
          "Couldn't access the microphone.",
        ];
        await page.waitForFunction(() => document.querySelector("[data-mic-status]").textContent.trim());
        check("deniedSaid", messages.includes((await state()).status), true);
        check("deniedIcon", await root.locator("[data-mic-status] [data-icon]").getAttribute("data-icon"), "circle-alert");
      }
    }
    // A listed device can be chosen and is sent with the form.
    if ((await options()).length > 1) {
      const [id] = (await options())[1];
      await select.focus();
      await select.selectOption(id);
      check("chosen", await page.evaluate(() => new FormData(document.querySelector("[data-example=form]")).get("microphone")), id);
    }
  }
  check("disabled", await page.locator("#mic-disabled").isDisabled(), true);
  check("hiddenLabel", await page.locator("label[for=mic-small]").evaluate((l) => l.classList.contains("sr-only")), true);

  // The state machine with the API stubbed, the same in every engine.
  const stubbed = await page.context().newPage();
  await stubbed.addInitScript(() => {
    window.devices = [
      { kind: "audioinput", deviceId: "", label: "" },
      { kind: "videoinput", deviceId: "", label: "" },
    ];
    window.denied = true;
    const media = navigator.mediaDevices ?? new EventTarget();
    if (!navigator.mediaDevices) Object.defineProperty(Navigator.prototype, "mediaDevices", { get: () => media });
    media.enumerateDevices = async () => window.devices.map((d) => ({ ...d }));
    media.getUserMedia = async () => {
      if (window.denied) throw new DOMException("Denied", "NotAllowedError");
      window.devices = [
        { kind: "audioinput", deviceId: "usb", label: "USB Mic (046d:0825)" },
        { kind: "audioinput", deviceId: "built-in", label: "" },
      ];
      return { getTracks: () => [{ stop: () => (window.stopped = true) }] };
    };
  });
  await stubbed.goto(`${base}/c/mic-selector`);
  const stub = stubbed.locator("[data-example=form] [data-mic-selector]");
  await stubbed.waitForFunction(() => document.querySelector("[data-mic-selector]").dataset.status === "ready");
  // Before permission: no device can be chosen, so only the label and the button.
  check("stubBefore", await stub.evaluate((r) => [r.querySelector("[data-mic-field]").hidden, r.querySelector("[data-mic-label]").hidden, r.querySelector("[data-mic-permission]").hidden]), [true, false, false]);
  await stub.getByRole("button", { name: "Allow microphone access" }).click();
  await stubbed.waitForFunction(() => document.querySelector("[data-mic-status]").textContent.trim());
  check("blocked", [await stub.locator("[data-mic-status]").textContent(), await stub.locator("[data-mic-status]").getAttribute("role")], ["Microphone access is blocked. Allow it in your browser's site settings, then try again.", "status"]);
  // Allowed: the stream is stopped, the devices are named, the error goes.
  await stubbed.evaluate(() => (window.denied = false));
  await stub.getByRole("button", { name: "Allow microphone access" }).click();
  await stubbed.waitForFunction(() => document.querySelector("#mic").options.length === 3);
  check("allowed", [await stubbed.evaluate(() => window.stopped), await stub.locator("option").allTextContents(), await stub.locator("[data-mic-status]").textContent(), await stub.locator("[data-mic-permission]").isVisible()], [true, ["Select a microphone", "USB Mic", "Microphone 2"], "", false]);
  // A device is plugged in.
  await stubbed.evaluate(() => {
    window.devices.push({ kind: "audioinput", deviceId: "headset", label: "Headset" });
    navigator.mediaDevices.dispatchEvent(new Event("devicechange"));
  });
  await stubbed.waitForFunction(() => document.querySelector("#mic").options.length === 4);
  check("devicechange", await stub.locator("option").last().textContent(), "Headset");
  await stubbed.close();

  // Without mediaDevices: the unsupported text, and no control.
  const old = await page.context().newPage();
  await old.addInitScript(() => Object.defineProperty(Navigator.prototype, "mediaDevices", { get: () => undefined }));
  await old.goto(`${base}/c/mic-selector`);
  await old.waitForFunction(() => document.querySelector("[data-mic-selector]").dataset.status === "unsupported");
  const none = old.locator("[data-example=form] [data-mic-selector]");
  check("unsupported", [await none.locator("[data-mic-unsupported]").isVisible(), await none.locator("select").isVisible(), await none.locator("[data-mic-permission]").isVisible(), await none.locator("[data-mic-label]").textContent()], [true, false, false, "Microphone"]);
  check("germanUnsupported", await old.locator("[data-example=strings] [data-mic-unsupported]").textContent(), "Die Mikrofonauswahl gibt es in diesem Browser nicht.");
  await old.close();
};
