// Conversation: the log follows new content while the reader is at the bottom, and
// lets go when they scroll up. A MutationObserver and a ResizeObserver on each log
// see content however it arrives. The announcer speaks when its data-status changes.

const logs = new WeakMap();
const resizes = new ResizeObserver((entries) => {
  for (const entry of entries) {
    const log = entry.target.closest("[data-conversation-viewport]");
    if (log) logChanged(log);
  }
});
const reducedMotion = () => matchMedia("(prefers-reduced-motion: reduce)").matches;

// Reduces a scroll to the new state. Reaching the bottom sticks. Moving up while the
// content kept its height, or just after a wheel, touch or key gesture, is the
// reader's doing and lets go. Moving up because content shrank keeps the state.
function nextStickState(prev, metrics, threshold, intent) {
  const distance = Math.max(0, metrics.scrollHeight - metrics.clientHeight - metrics.scrollTop);
  const atBottom = distance <= threshold;
  let stuck = prev.stuck;
  const movedUp = metrics.scrollTop < prev.lastScrollTop - 1;
  if (atBottom) stuck = true;
  else if (movedUp && (intent || metrics.scrollHeight === prev.lastScrollHeight)) stuck = false;
  return { stuck, atBottom, lastScrollTop: metrics.scrollTop, lastScrollHeight: metrics.scrollHeight };
}

function stateOf(log) {
  let state = logs.get(log);
  if (!state) {
    state = { stuck: true, atBottom: true, lastScrollTop: log.scrollTop, lastScrollHeight: log.scrollHeight, intentUntil: 0, holdUntil: 0, smoothUntil: 0, observer: undefined };
    logs.set(log, state);
  }
  return state;
}

const enabled = (log) => log.dataset.stick !== "false";

function pin(log, behavior = "instant") {
  const smooth = behavior === "smooth" && !reducedMotion();
  // "instant": a page that sets scroll-behavior: smooth would otherwise make every
  // pin a slow glide.
  log.scrollTo({ top: log.scrollHeight, behavior: smooth ? "smooth" : "instant" });
  const state = stateOf(log);
  state.stuck = true;
  state.holdUntil = 0;
}

function sync(log) {
  const state = stateOf(log);
  const next = nextStickState(
    state,
    { scrollTop: log.scrollTop, scrollHeight: log.scrollHeight, clientHeight: log.clientHeight },
    Number(log.dataset.threshold ?? 64),
    performance.now() < state.intentUntil,
  );
  Object.assign(state, next);
  if (performance.now() < state.holdUntil) state.stuck = false;
  const button = log.closest("[data-conversation]")?.querySelector("[data-conversation-scroll]");
  if (button && button.hidden !== state.atBottom) {
    // The button is about to go while it has focus: keep focus in the log.
    if (state.atBottom && button.contains(document.activeElement)) log.focus({ preventScroll: true });
    button.hidden = state.atBottom;
  }
}

function logChanged(log) {
  const state = stateOf(log);
  if (enabled(log) && state.stuck) pin(log, performance.now() < state.smoothUntil ? "smooth" : "instant");
  sync(log);
}

function watch(log) {
  const state = stateOf(log);
  if (state.observer) return;
  state.observer = new MutationObserver(() => logChanged(log));
  state.observer.observe(log, { childList: true, subtree: true, characterData: true });
  resizes.observe(log);
  for (const child of log.children) resizes.observe(child);
  if (enabled(log)) pin(log);
  else log.scrollTo({ top: 0, behavior: "instant" });
  sync(log);
}

function scrollToBottom(log, behavior = "smooth") {
  const state = stateOf(log);
  pin(log, behavior);
  state.smoothUntil = behavior === "smooth" ? performance.now() + 600 : 0;
  sync(log);
}

// Stops following and brings `element` to `offset` pixels below the top of the log,
// to read an answer that arrived whole from its start. Following resumes when the
// reader reaches the bottom again.
function scrollToElement(log, element, { offset = 0, behavior = "smooth" } = {}) {
  const state = stateOf(log);
  state.stuck = false;
  state.intentUntil = performance.now() + 600;
  state.holdUntil = performance.now() + 800;
  const top = Math.max(0, log.scrollTop + element.getBoundingClientRect().top - log.getBoundingClientRect().top - offset);
  log.scrollTo({ top, behavior: behavior === "smooth" && !reducedMotion() ? "smooth" : "instant" });
}

window.bitopUi.conversation = { scrollToBottom, scrollToElement };

// A gesture towards the top: the next upward scroll is the reader's. If the log can
// already move up, let go at once, so that growing content cannot pull it back down
// in the middle of the gesture.
function release(log) {
  const state = stateOf(log);
  state.intentUntil = performance.now() + 300;
  if (log.scrollTop > 0) state.stuck = false;
}

const logOf = (target) => (target instanceof Element ? target.closest("[data-conversation-viewport]") : null);

document.addEventListener("scroll", (event) => {
  if (event.target instanceof Element && event.target.matches("[data-conversation-viewport]")) sync(event.target);
}, { capture: true, passive: true });

document.addEventListener("wheel", (event) => {
  const log = logOf(event.target);
  if (log && event.deltaY < 0) release(log);
}, { passive: true });

let touchY = 0;
document.addEventListener("touchstart", (event) => (touchY = event.touches[0]?.clientY ?? 0), { passive: true });
document.addEventListener("touchmove", (event) => {
  const log = logOf(event.target);
  if (log && (event.touches[0]?.clientY ?? 0) > touchY) release(log);
}, { passive: true });

document.addEventListener("keydown", (event) => {
  const log = event.target instanceof Element && event.target.matches("[data-conversation-viewport]") ? event.target : null;
  if (log && ["ArrowUp", "PageUp", "Home"].includes(event.key)) release(log);
});

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-conversation-scroll] > button");
  if (!button) return;
  const log = button.closest("[data-conversation]")?.querySelector("[data-conversation-viewport]");
  if (!log) return;
  // The button goes once at the bottom: keep focus in the log.
  log.focus({ preventScroll: true });
  scrollToBottom(log, "smooth");
});

// ---------- the announcer ----------

const said = new WeakMap();

function announce(announcer, was) {
  const status = announcer.dataset.status;
  if (was === status) return;
  let text;
  if (status === "submitted") text = announcer.dataset.saySubmitted;
  else if (status === "streaming") text = announcer.dataset.sayStreaming;
  else if (status === "error") text = announcer.dataset.sayError;
  else if (status === "ready" && (was === "streaming" || was === "submitted")) text = announcer.dataset.sayComplete;
  if (text === undefined) return;
  // The same words twice in a row: a trailing no-break space changes the region, so
  // that it is announced again.
  const last = said.get(announcer) ?? { text: "", alt: false };
  const alt = text === last.text ? !last.alt : last.alt;
  said.set(announcer, { text, alt });
  announcer.textContent = text && alt ? `${text} ` : text;
}

// ---------- logs and announcers that arrive later ----------

document.querySelectorAll("[data-conversation-viewport]").forEach(watch);

new MutationObserver((records) => {
  for (const record of records) {
    if (record.type === "attributes") {
      const element = record.target;
      if (element.matches("[data-conversation-announcer]") && record.attributeName === "data-status") {
        announce(element, record.oldValue);
      } else if (element.matches("[data-conversation-viewport]") && record.attributeName === "data-stick") {
        // Following turned on: stick to the bottom once.
        if (enabled(element)) pin(element);
        sync(element);
      }
      continue;
    }
    for (const node of record.addedNodes) {
      if (!(node instanceof Element)) continue;
      const added = node.matches("[data-conversation-viewport]") ? [node] : [...node.querySelectorAll("[data-conversation-viewport]")];
      added.forEach(watch);
      // New children of a log are measured too.
      const log = logOf(node);
      if (log && node.parentElement === log) resizes.observe(node);
    }
  }
}).observe(document.documentElement, {
  subtree: true,
  childList: true,
  attributes: true,
  attributeFilter: ["data-status", "data-stick"],
  attributeOldValue: true,
});
