// Audio player: custom controls for the <audio> inside it. The element's own events
// keep the controls in step, so they also follow a script that drives the element.

function formatTime(seconds) {
  if (!Number.isFinite(seconds) || seconds < 0) return "0:00";
  const s = Math.floor(seconds);
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = String(s % 60).padStart(2, "0");
  return h > 0 ? `${h}:${String(m).padStart(2, "0")}:${sec}` : `${m}:${sec}`;
}

function playerParts(root) {
  const find = (name) => root.querySelector(`[data-player-${name}]`);
  return {
    root,
    audio: root.querySelector("audio"),
    play: find("play"),
    back: find("back"),
    forward: find("forward"),
    current: find("current"),
    duration: find("duration"),
    seek: find("seek"),
    mute: find("mute"),
    volume: find("volume"),
    rate: find("rate"),
  };
}

const setFill = (input) => {
  const max = Number(input.max) || 1;
  input.style.setProperty("--slider-fill", String(Math.min(1, Math.max(0, Number(input.value) / max))));
};

// While the reader drags the seek slider, playback does not move the thumb.
const seeking = new WeakSet();

function updatePlayer(root) {
  const p = playerParts(root);
  const { audio } = p;
  if (!audio) return;
  const d = root.dataset;
  const playing = !audio.paused && !audio.ended;
  root.toggleAttribute("data-playing", playing);
  if (p.play) {
    p.play.setAttribute("aria-label", playing ? d.pauseLabel : d.playLabel);
    p.play.disabled = !audio.currentSrc && !audio.getAttribute("src");
  }
  const known = Number.isFinite(audio.duration) && audio.duration > 0;
  for (const button of [p.back, p.forward]) if (button) button.disabled = !known;
  if (p.current) p.current.textContent = formatTime(audio.currentTime);
  if (p.duration) p.duration.textContent = known ? formatTime(audio.duration) : d.unknownTime;
  if (p.seek) {
    p.seek.disabled = !known;
    p.seek.max = known ? String(audio.duration) : "1";
    if (!seeking.has(p.seek)) p.seek.value = String(known ? Math.min(audio.currentTime, audio.duration) : 0);
    const total = known ? formatTime(audio.duration) : d.unknownDuration;
    p.seek.setAttribute("aria-valuetext", d.seekText.replace("{current}", formatTime(Number(p.seek.value))).replace("{total}", total));
    setFill(p.seek);
  }
  const level = audio.muted ? 0 : audio.volume;
  if (p.mute) {
    p.mute.setAttribute("aria-pressed", String(audio.muted));
    p.mute.querySelector("[data-icon]")?.setAttribute("data-icon", level === 0 ? "volume-x" : level < 0.5 ? "volume-1" : "volume-2");
  }
  if (p.volume) {
    p.volume.value = String(level);
    p.volume.setAttribute("aria-valuetext", `${Math.round(level * 100)}%`);
    setFill(p.volume);
  }
  if (p.rate && document.activeElement !== p.rate) p.rate.value = String(audio.playbackRate);
}

const playerOf = (target) => (target instanceof Element ? target.closest("[data-audio-player]") : null);

// Media events do not bubble: listen in the capture phase.
for (const type of ["loadedmetadata", "durationchange", "timeupdate", "play", "pause", "ended", "volumechange", "ratechange", "emptied", "seeked"]) {
  document.addEventListener(
    type,
    (event) => {
      const root = event.target instanceof HTMLMediaElement && playerOf(event.target);
      if (root) updatePlayer(root);
    },
    true,
  );
}

document.addEventListener("click", (event) => {
  const root = playerOf(event.target);
  if (!root) return;
  const { audio, play, back, forward, mute } = playerParts(root);
  const button = event.target.closest("button");
  if (!audio || !button || button.disabled) return;
  if (button === play) {
    // play() rejects when playback is blocked or the source fails; the element's own
    // events keep the controls in step, so the rejection is ignored.
    if (audio.paused || audio.ended) audio.play()?.catch(() => {});
    else audio.pause();
  } else if (button === back || button === forward) {
    const step = Number(root.dataset.skip) * (button === back ? -1 : 1);
    audio.currentTime = Math.min(Math.max(0, audio.currentTime + step), audio.duration);
  } else if (button === mute) {
    audio.muted = !audio.muted;
    if (!audio.muted && audio.volume === 0) audio.volume = 1;
  }
});

document.addEventListener("input", (event) => {
  const root = playerOf(event.target);
  if (!root) return;
  const { audio, seek, volume } = playerParts(root);
  if (!audio) return;
  if (event.target === seek) {
    audio.currentTime = Number(seek.value);
    updatePlayer(root);
  } else if (event.target === volume) {
    audio.volume = Number(volume.value);
    if (audio.volume > 0 && audio.muted) audio.muted = false;
  }
});

document.addEventListener("change", (event) => {
  const root = playerOf(event.target);
  if (!root) return;
  const { audio, rate, seek } = playerParts(root);
  if (event.target === rate && audio) audio.playbackRate = Number(rate.value);
  if (event.target === seek) seeking.delete(seek);
});

document.addEventListener("pointerdown", (event) => {
  const root = playerOf(event.target);
  const seek = root && playerParts(root).seek;
  if (seek && event.target === seek) seeking.add(seek);
});
document.addEventListener("pointerup", () => {
  for (const seek of document.querySelectorAll("[data-player-seek]")) seeking.delete(seek);
});

for (const root of document.querySelectorAll("[data-audio-player]")) {
  const audio = root.querySelector("audio");
  if (audio && root.dataset.rate) audio.defaultPlaybackRate = audio.playbackRate = Number(root.dataset.rate);
  updatePlayer(root);
}
