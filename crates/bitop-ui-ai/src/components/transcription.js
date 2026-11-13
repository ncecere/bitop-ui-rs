// Transcription: the segment playing is highlighted as its media element plays, or as
// data-current-time on the transcript changes; a segment that is a button seeks.

function syncTranscript(root, time) {
  for (const segment of root.querySelectorAll("[data-transcription-segment]")) {
    const start = Number(segment.dataset.start);
    const end = Number(segment.dataset.end);
    const active = time >= start && time < end;
    const past = time >= end;
    const item = segment.closest("li");
    for (const element of [segment, item]) element?.toggleAttribute("data-active", active);
    segment.toggleAttribute("data-past", past);
    if (active) segment.setAttribute("aria-current", "true");
    else segment.removeAttribute("aria-current");
    if (active && root.hasAttribute("data-auto-scroll") && segment.dataset.wasActive === undefined) {
      const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;
      item?.scrollIntoView({ block: "nearest", behavior: reduce ? "instant" : "smooth" });
    }
    if (active) segment.dataset.wasActive = "";
    else delete segment.dataset.wasActive;
  }
}

const transcriptsOf = (media) => (media.id ? document.querySelectorAll(`[data-transcription][data-media="${CSS.escape(media.id)}"]`) : []);

// Media events do not bubble: listen in the capture phase.
for (const type of ["timeupdate", "seeked", "loadedmetadata", "emptied"]) {
  document.addEventListener(
    type,
    (event) => {
      if (!(event.target instanceof HTMLMediaElement)) return;
      for (const root of transcriptsOf(event.target)) syncTranscript(root, event.target.currentTime);
    },
    true,
  );
}

document.addEventListener("click", (event) => {
  const segment = event.target instanceof Element && event.target.closest("button[data-transcription-segment]");
  if (!segment) return;
  const root = segment.closest("[data-transcription]");
  const time = Number(segment.dataset.start);
  const proceed = root.dispatchEvent(new CustomEvent("bitop:transcription-seek", { bubbles: true, cancelable: true, detail: { time } }));
  if (!proceed) return;
  const media = root.dataset.media ? document.getElementById(root.dataset.media) : null;
  if (media instanceof HTMLMediaElement) media.currentTime = time;
  root.dataset.currentTime = String(time);
});

new MutationObserver((records) => {
  for (const record of records) {
    const root = record.target;
    if (root instanceof Element && root.matches("[data-transcription]")) syncTranscript(root, Number(root.dataset.currentTime) || 0);
  }
}).observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-current-time"] });
