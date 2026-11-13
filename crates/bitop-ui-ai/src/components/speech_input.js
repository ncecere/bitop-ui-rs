// Speech input: a microphone toggle. It uses the Web Speech API (SpeechRecognition)
// where the browser has it; otherwise, with data-record, it records with
// MediaRecorder and hands the audio to the page or to data-transcribe-url; with
// neither it is disabled and says why. Text goes into the element whose ID is in
// data-target, after a cancelable bitop:speech-result event.

const speechStates = new WeakMap();

const recognitionCtor = () => window.SpeechRecognition ?? window.webkitSpeechRecognition;

function speechMode(root) {
  if (recognitionCtor()) return "speech-recognition";
  if (root.hasAttribute("data-record") && "MediaRecorder" in window && typeof navigator.mediaDevices?.getUserMedia === "function") {
    return "media-recorder";
  }
  return "none";
}

function speechOf(root) {
  let state = speechStates.get(root);
  if (!state) {
    state = { mode: speechMode(root), status: "idle", error: "", interim: "" };
    speechStates.set(root, state);
  }
  return state;
}

function speechMessage(root, code) {
  const d = root.dataset;
  if (["not-allowed", "service-not-allowed", "NotAllowedError", "SecurityError"].includes(code)) return d.blockedText;
  if (code === "no-speech") return d.noSpeechText;
  if (code === "audio-capture" || code === "NotFoundError") return d.notFoundText;
  if (code === "network") return d.networkText;
  return d.errorText;
}

function renderSpeech(root) {
  const state = speechOf(root);
  const d = root.dataset;
  const button = root.querySelector("[data-speech-toggle]");
  const status = root.querySelector("[data-speech-status]");
  const interim = root.querySelector("[data-speech-interim]");
  const listening = state.status === "listening";
  const processing = state.status === "processing";
  const unsupported = state.mode === "none";
  root.dataset.mode = state.mode;
  root.dataset.status = state.status;
  const text = unsupported
    ? d.unsupportedText
    : listening
      ? d.listeningText
      : processing
        ? d.processingText
        : state.status === "error"
          ? state.error
          : "";
  if (status) {
    if (status.textContent !== text) status.textContent = text;
    status.classList.toggle("sr-only", !root.hasAttribute("data-show-status") && !unsupported && state.status !== "error");
    if (state.status === "error") status.dataset.tone = "danger";
    else delete status.dataset.tone;
  }
  if (button) {
    button.setAttribute("aria-pressed", String(listening));
    button.disabled = root.hasAttribute("data-disabled") || unsupported || processing;
    button.dataset.variant = listening ? "danger" : "outline";
    if (processing) button.setAttribute("aria-busy", "true");
    else button.removeAttribute("aria-busy");
    if (text && status) button.setAttribute("aria-describedby", status.id);
    else button.removeAttribute("aria-describedby");
  }
  if (interim) interim.textContent = root.hasAttribute("data-show-interim") ? state.interim : "";
}

function setSpeechStatus(root, next) {
  const state = speechOf(root);
  const was = state.status === "listening";
  state.status = next;
  renderSpeech(root);
  if (was !== (next === "listening")) {
    root.dispatchEvent(new CustomEvent("bitop:speech-listening", { bubbles: true, detail: { listening: next === "listening" } }));
  }
}

function speechFail(root, code) {
  const state = speechOf(root);
  state.error = speechMessage(root, code);
  state.interim = "";
  setSpeechStatus(root, "error");
  root.dispatchEvent(new CustomEvent("bitop:speech-error", { bubbles: true, detail: { code } }));
}

// A final phrase: the page hears of it, then it goes into the target.
function speechDeliver(root, text) {
  text = text.trim();
  if (!text) return;
  const proceed = root.dispatchEvent(new CustomEvent("bitop:speech-result", { bubbles: true, cancelable: true, detail: { text } }));
  const target = root.dataset.target ? document.getElementById(root.dataset.target) : null;
  if (!proceed || !target || !("value" in target)) return;
  target.value = target.value && !/\s$/.test(target.value) ? `${target.value} ${text}` : `${target.value}${text}`;
  target.dispatchEvent(new Event("input", { bubbles: true }));
}

function startRecognition(root) {
  const state = speechOf(root);
  const Recognition = recognitionCtor();
  const recognition = new Recognition();
  recognition.continuous = true;
  recognition.interimResults = true;
  recognition.lang = root.dataset.lang || document.documentElement.lang || "en-US";
  recognition.addEventListener("start", () => state.recognition === recognition && setSpeechStatus(root, "listening"));
  recognition.addEventListener("result", (event) => {
    let final = "";
    let partial = "";
    for (let i = event.resultIndex; i < event.results.length; i += 1) {
      const result = event.results[i];
      const text = result?.[0]?.transcript ?? "";
      if (result?.isFinal) final += text;
      else partial += text;
    }
    state.interim = partial;
    renderSpeech(root);
    root.dispatchEvent(new CustomEvent("bitop:speech-interim", { bubbles: true, detail: { text: partial } }));
    speechDeliver(root, final);
  });
  recognition.addEventListener("error", (event) => {
    // "aborted" is our own stop, not an error worth reporting.
    if (event.error !== "aborted") speechFail(root, event.error);
  });
  recognition.addEventListener("end", () => {
    if (state.recognition !== recognition) return;
    state.recognition = undefined;
    state.interim = "";
    if (state.status !== "error") setSpeechStatus(root, "idle");
    else renderSpeech(root);
  });
  state.recognition = recognition;
  state.error = "";
  try {
    recognition.start();
    // Some implementations send "start" late; show the press at once.
    setSpeechStatus(root, "listening");
  } catch {
    state.recognition = undefined;
    speechFail(root, "start-failed");
  }
}

// The recorded audio becomes text: through the page (bitop:speech-audio, whose
// detail.respondWith takes a promise of the text), or data-transcribe-url.
async function transcribe(root, audio) {
  let answer;
  root.dispatchEvent(
    new CustomEvent("bitop:speech-audio", { bubbles: true, detail: { audio, respondWith: (promise) => (answer = promise) } }),
  );
  if (answer) return answer;
  const url = root.dataset.transcribeUrl;
  if (!url) return "";
  const response = await fetch(url, { method: "POST", body: audio, headers: { "Content-Type": audio.type } });
  if (!response.ok) throw new Error(String(response.status));
  return response.text();
}

async function startRecorder(root) {
  const state = speechOf(root);
  state.error = "";
  setSpeechStatus(root, "listening");
  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    if (state.status !== "listening" || !root.isConnected) {
      for (const track of stream.getTracks()) track.stop();
      return;
    }
    state.stream = stream;
    const recorder = new MediaRecorder(stream);
    const chunks = [];
    recorder.addEventListener("dataavailable", (event) => event.data.size > 0 && chunks.push(event.data));
    recorder.addEventListener("stop", async () => {
      for (const track of stream.getTracks()) track.stop();
      state.stream = undefined;
      state.recorder = undefined;
      const audio = new Blob(chunks, { type: recorder.mimeType || "audio/webm" });
      if (audio.size === 0) {
        setSpeechStatus(root, "idle");
        return;
      }
      setSpeechStatus(root, "processing");
      try {
        const text = await transcribe(root, audio);
        if (typeof text === "string") speechDeliver(root, text);
        setSpeechStatus(root, "idle");
      } catch {
        speechFail(root, "transcription-failed");
      }
    });
    state.recorder = recorder;
    recorder.start();
  } catch (error) {
    speechFail(root, error?.name || "start-failed");
  }
}

function stopSpeech(root) {
  const state = speechOf(root);
  if (state.recognition) state.recognition.stop();
  else if (state.recorder?.state === "recording") state.recorder.stop();
  else setSpeechStatus(root, "idle");
}

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-speech-toggle]");
  if (!button || button.disabled) return;
  const root = button.closest("[data-speech-input]");
  const state = speechOf(root);
  if (button.getAttribute("aria-pressed") === "true") stopSpeech(root);
  else if (state.mode === "speech-recognition") startRecognition(root);
  else if (state.mode === "media-recorder") startRecorder(root);
});

addEventListener("pagehide", () => {
  for (const root of document.querySelectorAll("[data-speech-input]")) {
    const state = speechStates.get(root);
    state?.recognition?.abort();
    if (state?.recorder?.state === "recording") state.recorder.stop();
    for (const track of state?.stream?.getTracks() ?? []) track.stop();
  }
});

for (const root of document.querySelectorAll("[data-speech-input]")) renderSpeech(root);
