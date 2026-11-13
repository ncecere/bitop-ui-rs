// A SpeechRecognition stand-in that the check drives, and a MediaRecorder one: speech
// and permissions cannot be faked in every engine, so these run the component's state
// machine the same way everywhere. The real APIs are checked where the engine has them.
const fakeSpeech = () => {
  window.recognitions = [];
  class FakeRecognition extends EventTarget {
    constructor() {
      super();
      window.recognitions.push(this);
    }
    start() {
      this.started = true;
      setTimeout(() => this.dispatchEvent(new Event("start")));
    }
    stop() {
      this.stopped = true;
      setTimeout(() => this.dispatchEvent(new Event("end")));
    }
    abort() {
      this.stop();
    }
    // Sends a result: [text, final] pairs.
    say(...parts) {
      const results = parts.map(([text, isFinal]) => Object.assign([{ transcript: text }], { isFinal }));
      this.dispatchEvent(Object.assign(new Event("result"), { resultIndex: 0, results }));
    }
    fail(error) {
      this.dispatchEvent(Object.assign(new Event("error"), { error }));
      this.dispatchEvent(new Event("end"));
    }
  }
  window.SpeechRecognition = FakeRecognition;
  delete window.webkitSpeechRecognition;
};

const fakeRecorder = () => {
  delete window.SpeechRecognition;
  delete window.webkitSpeechRecognition;
  window.MediaRecorder = class extends EventTarget {
    constructor(stream) {
      super();
      this.stream = stream;
      this.state = "inactive";
      this.mimeType = "audio/webm";
    }
    start() {
      this.state = "recording";
    }
    stop() {
      this.state = "inactive";
      this.dispatchEvent(Object.assign(new Event("dataavailable"), { data: new Blob(["sound"], { type: "audio/webm" }) }));
      this.dispatchEvent(new Event("stop"));
    }
  };
  const media = navigator.mediaDevices ?? new EventTarget();
  if (!navigator.mediaDevices) Object.defineProperty(Navigator.prototype, "mediaDevices", { get: () => media });
  media.getUserMedia = async () => ({ getTracks: () => [{ stop: () => (window.trackStopped = true) }] });
};

const nothing = () => {
  delete window.SpeechRecognition;
  delete window.webkitSpeechRecognition;
  delete window.MediaRecorder;
};

export default async ({ page, check, base }) => {
  const state = (p, id) =>
    p.locator(`#${id}`).evaluate((root) => {
      const button = root.querySelector("[data-speech-toggle]");
      const status = root.querySelector("[data-speech-status]");
      return {
        mode: root.dataset.mode,
        status: root.dataset.status,
        pressed: button.getAttribute("aria-pressed"),
        disabled: button.disabled,
        text: status.textContent,
        shown: !status.classList.contains("sr-only"),
      };
    });

  // The real page: the mode follows what the engine has.
  const has = await page.evaluate(() => ({
    recognition: Boolean(window.SpeechRecognition ?? window.webkitSpeechRecognition),
    recorder: "MediaRecorder" in window && typeof navigator.mediaDevices?.getUserMedia === "function",
  }));
  const dictate = await state(page, "dictate");
  check("realMode", dictate.mode, has.recognition ? "speech-recognition" : "none");
  check("realRecordMode", (await state(page, "record")).mode, has.recognition ? "speech-recognition" : has.recorder ? "media-recorder" : "none");
  if (!has.recognition) {
    // Firefox: no SpeechRecognition. The unsupported text shows, and nothing throws.
    check("realUnsupported", [dictate.disabled, dictate.text, dictate.shown], [true, "Voice input isn't supported in this browser.", true]);
  }
  if (!has.recognition && has.recorder) {
    // Firefox records from its fake microphone for real; the server is stood in for.
    await page.route("**/c/speech-input/transcribe", (route) => route.fulfill({ body: "Recorded for real", contentType: "text/plain" }));
    await page.locator("#record [data-speech-toggle]").click();
    await page.waitForFunction(() => document.querySelector("#record [data-speech-toggle]").getAttribute("aria-pressed") === "true");
    await page.waitForTimeout(300);
    await page.locator("#record [data-speech-toggle]").click();
    await page.waitForFunction(() => document.getElementById("message").value === "Recorded for real");
    check("realRecorded", (await state(page, "record")).status, "idle");
  }
  check("disabled", (await state(page, "off")).disabled, true);

  // Speech recognition, driven by the check.
  const speech = await page.context().newPage();
  await speech.addInitScript(fakeSpeech);
  await speech.goto(`${base}/c/speech-input`);
  check("fakeMode", [await state(speech, "dictate")], [{ mode: "speech-recognition", status: "idle", pressed: "false", disabled: false, text: "", shown: true }]);
  const events = speech.evaluate(() => {
    window.speechEvents = [];
    for (const type of ["bitop:speech-listening", "bitop:speech-result", "bitop:speech-interim", "bitop:speech-error"]) {
      document.addEventListener(type, (e) => window.speechEvents.push([type.slice(13), e.detail]));
    }
  });
  await events;
  const toggle = speech.getByRole("button", { name: "Voice input", exact: true });
  await toggle.focus();
  await speech.keyboard.press("Enter");
  await speech.waitForFunction(() => document.querySelector("#dictate").dataset.status === "listening");
  const listening = await state(speech, "dictate");
  check("listening", [listening.pressed, listening.text, await toggle.getAttribute("data-variant"), await toggle.getAttribute("aria-describedby")], ["true", "Listening…", "danger", "dictate-status"]);
  check("recognition", await speech.evaluate(() => [window.recognitions.length, window.recognitions[0].continuous, window.recognitions[0].interimResults, window.recognitions[0].lang]), [1, true, true, "en"]);
  check("pulse", await speech.locator("#dictate [aria-hidden=true]").first().evaluate((p) => getComputedStyle(p).display), "block");
  // The phrase being spoken shows; a final one goes into the text box.
  await speech.evaluate(() => window.recognitions[0].say(["hello wor", false]));
  check("interim", await speech.locator("#dictate [data-speech-interim]").textContent(), "hello wor");
  await speech.evaluate(() => {
    document.getElementById("notes").value = "Note:";
    window.recognitions[0].say(["hello world", true], ["and", false]);
  });
  check("final", [await speech.locator("#notes").inputValue(), await speech.locator("#dictate [data-speech-interim]").textContent()], ["Note: hello world", "and"]);
  // Enter stops it.
  await speech.keyboard.press("Enter");
  await speech.waitForFunction(() => document.querySelector("#dictate").dataset.status === "idle");
  check("stopped", [await speech.evaluate(() => window.recognitions[0].stopped), (await state(speech, "dictate")).pressed, await speech.locator("#dictate [data-speech-interim]").textContent()], [true, "false", ""]);
  check("events", await speech.evaluate(() => window.speechEvents.map(([type, d]) => [type, d.listening ?? d.text])), [
    ["listening", true],
    ["interim", "hello wor"],
    ["interim", "and"],
    ["result", "hello world"],
    ["listening", false],
  ]);
  // A page that takes the text itself cancels the result.
  await toggle.click();
  await speech.evaluate(() => document.addEventListener("bitop:speech-result", (e) => e.preventDefault(), { once: true }));
  await speech.evaluate(() => window.recognitions[1].say(["kept out", true]));
  check("canceled", await speech.locator("#notes").inputValue(), "Note: hello world");
  // An error is said, visibly, in the danger colour.
  await speech.evaluate(() => window.recognitions[1].fail("not-allowed"));
  await speech.waitForFunction(() => document.querySelector("#dictate").dataset.status === "error");
  const failed = await state(speech, "dictate");
  check("error", [failed.text, failed.pressed, await speech.locator("#dictate [data-speech-status]").getAttribute("data-tone")], ["Microphone access is blocked.", "false", "danger"]);
  // Hidden status, other language.
  await speech.locator("#small [data-speech-toggle]").click();
  await speech.waitForFunction(() => document.querySelector("#small").dataset.status === "listening");
  check("srOnlyStatus", [(await state(speech, "small")).text, (await state(speech, "small")).shown], ["Listening…", false]);
  await speech.locator("#german [data-speech-toggle]").click();
  await speech.waitForFunction(() => document.querySelector("#german").dataset.status === "listening");
  check("german", [(await state(speech, "german")).text, await speech.evaluate(() => window.recognitions.at(-1).lang)], ["Hört zu…", "de-DE"]);
  await speech.close();

  // Recording, driven by the check: the page transcribes, or the server does.
  const recording = await page.context().newPage();
  await recording.addInitScript(fakeRecorder);
  await recording.route("**/c/speech-input/transcribe", (route) => route.fulfill({ body: "From the server", contentType: "text/plain" }));
  await recording.goto(`${base}/c/speech-input`);
  check("recordModes", [(await state(recording, "record")).mode, (await state(recording, "dictate")).mode], ["media-recorder", "none"]);
  const record = recording.getByRole("button", { name: "Dictate a message" });
  await record.click();
  await recording.waitForFunction(() => document.querySelector("#record").dataset.status === "listening");
  await recording.evaluate(() => {
    window.release = undefined;
    document.addEventListener("bitop:speech-audio", (e) => {
      window.audioType = e.detail.audio.type;
      e.detail.respondWith(new Promise((resolve) => (window.release = resolve)));
    }, { once: true });
  });
  await record.click();
  await recording.waitForFunction(() => document.querySelector("#record").dataset.status === "processing");
  const processing = await state(recording, "record");
  check("processing", [processing.disabled, processing.text, await record.getAttribute("aria-busy"), await recording.evaluate(() => [window.audioType, window.trackStopped])], [true, "Transcribing…", "true", ["audio/webm", true]]);
  await recording.evaluate(() => window.release("From the page"));
  await recording.waitForFunction(() => document.querySelector("#record").dataset.status === "idle");
  check("pageText", await recording.locator("#message").inputValue(), "From the page");
  await record.click();
  await recording.waitForFunction(() => document.querySelector("#record").dataset.status === "listening");
  await record.click();
  await recording.waitForFunction(() => document.getElementById("message").value === "From the page From the server");
  check("serverText", (await state(recording, "record")).status, "idle");
  await recording.close();

  // Neither API: disabled, saying why, in every engine.
  const none = await page.context().newPage();
  await none.addInitScript(nothing);
  await none.goto(`${base}/c/speech-input`);
  const off = await state(none, "record");
  check("unsupported", [off.mode, off.disabled, off.text, off.shown], ["none", true, "Voice input isn't supported in this browser.", true]);
  check("germanUnsupported", (await state(none, "german")).text, "Spracheingabe gibt es in diesem Browser nicht.");
  await none.close();

  // Right to left: the button moves to the right end.
  const edge = () => page.locator("#dictate").evaluate((root) => {
    const box = root.getBoundingClientRect();
    const button = root.querySelector("[data-speech-toggle]").getBoundingClientRect();
    return [Math.abs(button.left - box.left) <= 1, Math.abs(button.right - box.right) <= 1];
  });
  check("ltr", await edge(), [true, false]);
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  check("rtl", await edge(), [false, true]);
};
