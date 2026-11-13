// Voice selector: a search over the voices in its dialog, the trigger that shows the
// chosen voice, and preview buttons that play a voice's sample or tell the page.

const voiceRoot = (target) => (target instanceof Element ? target.closest("[data-voice-selector]") : null);

function filterVoices(root) {
  const search = root.querySelector("[data-voice-search]");
  const words = (search?.value ?? "").trim().toLowerCase().split(/\s+/).filter(Boolean);
  let shown = 0;
  for (const row of root.querySelectorAll("[data-voice-row]")) {
    const match = words.every((word) => row.dataset.search.includes(word));
    row.hidden = !match;
    if (match) shown += 1;
  }
  const status = root.querySelector("[data-voice-status]");
  if (!status) return;
  const d = root.dataset;
  if (shown === 0) {
    const empty = root.querySelector("template[data-voice-empty]").content.firstElementChild.cloneNode(true);
    status.replaceChildren(empty);
  } else if (words.length) {
    const count = document.createElement("span");
    count.className = "sr-only";
    count.textContent = (shown === 1 ? d.countOne : d.countOther).replace("{n}", String(shown));
    status.replaceChildren(count);
  } else {
    status.replaceChildren();
  }
}

document.addEventListener("input", (event) => {
  const root = voiceRoot(event.target);
  if (root && event.target.matches("[data-voice-search]")) filterVoices(root);
});

// The search only filters the list. Enter in it would submit the form around the
// selector and leave the page.
document.addEventListener("keydown", (event) => {
  if (event.key === "Enter" && event.target instanceof Element && event.target.matches("[data-voice-search]")) {
    event.preventDefault();
  }
});

document.addEventListener("change", (event) => {
  const root = voiceRoot(event.target);
  if (!root || !event.target.matches("[data-voice-row] input[type=radio]")) return;
  const name = event.target.closest("[data-voice-row]").dataset.name;
  const trigger = root.querySelector("[data-voice-trigger]");
  trigger?.setAttribute("aria-label", `${root.dataset.label}: ${name}`);
  const value = trigger?.querySelector("[data-voice-value]");
  if (value) {
    value.textContent = name;
    value.removeAttribute("data-placeholder");
  }
});

// The dialog closes: the search starts again next time, and a sample stops.
document.addEventListener(
  "close",
  (event) => {
    const root = voiceRoot(event.target);
    if (!root || !(event.target instanceof HTMLDialogElement)) return;
    const search = root.querySelector("[data-voice-search]");
    if (search) search.value = "";
    filterVoices(root);
    if (voicePreview && root.contains(voicePreview.button)) stopVoicePreview();
  },
  true,
);

// ---------- previews ----------

let voicePreview;

function setPreviewState(button, playing, loading) {
  button.setAttribute("aria-pressed", String(playing));
  if (loading) button.setAttribute("aria-busy", "true");
  else button.removeAttribute("aria-busy");
}

function stopVoicePreview() {
  if (!voicePreview) return;
  voicePreview.audio.pause();
  setPreviewState(voicePreview.button, false, false);
  voicePreview = undefined;
}

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-voice-preview]");
  if (!button) return;
  const root = voiceRoot(button);
  const playing = button.getAttribute("aria-pressed") === "true";
  const proceed = root.dispatchEvent(
    new CustomEvent("bitop:voice-preview", { bubbles: true, cancelable: true, detail: { id: button.dataset.voicePreview, playing } }),
  );
  const url = button.dataset.previewUrl;
  if (!proceed || !url) return;
  const same = voicePreview?.button === button;
  stopVoicePreview();
  if (same && playing) return;
  const audio = new Audio(url);
  voicePreview = { audio, button };
  setPreviewState(button, false, true);
  // Pressed once the sound starts; the promise of play() can settle later than that.
  audio.addEventListener("playing", () => voicePreview?.audio === audio && setPreviewState(button, true, false));
  audio.addEventListener("ended", () => voicePreview?.audio === audio && stopVoicePreview());
  audio.play()?.catch(() => voicePreview?.audio === audio && stopVoicePreview());
});

// A page that plays previews itself sets data-previewing (and data-preview-loading)
// on the selector to a voice's ID.
function syncPreviews(root) {
  for (const button of root.querySelectorAll("[data-voice-preview]")) {
    if (voicePreview?.button === button) continue;
    const id = button.dataset.voicePreview;
    setPreviewState(button, root.dataset.previewing === id, root.dataset.previewLoading === id);
  }
}

new MutationObserver((records) => {
  for (const record of records) {
    if (record.target instanceof Element && record.target.matches("[data-voice-selector]")) syncPreviews(record.target);
  }
}).observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-previewing", "data-preview-loading"] });

// A list rendered empty says so at once.
for (const root of document.querySelectorAll("[data-voice-selector]")) {
  if (!root.querySelector("[data-voice-row]")) filterVoices(root);
}
