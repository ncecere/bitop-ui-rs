// Reasoning: the panel follows data-streaming on its <details>. It opens when
// streaming starts, and closes once, a moment after it ends, unless the reader has
// opened or closed it. The trigger's label says how long the thinking took.

const reasoningState = new WeakMap();

function reasoningOf(details) {
  let state = reasoningState.get(details);
  if (!state) {
    // `shown` is the state the server rendered or the script last set: a panel in
    // another state was opened or closed by the reader.
    state = { started: undefined, userToggled: false, closedOnce: false, shown: details.open, timer: undefined };
    reasoningState.set(details, state);
  }
  return state;
}

// Opens or closes the panel from the script.
function setReasoningOpen(details, open) {
  reasoningOf(details).shown = open;
  details.open = open;
}

// Whether the reader has opened or closed the panel. Read from the panel itself, not
// only from the toggle event, which is queued and may arrive after streaming changes.
function readerToggled(details) {
  const state = reasoningOf(details);
  if (details.open !== state.shown) state.userToggled = true;
  return state.userToggled;
}

function setReasoningLabel(details, streaming, seconds) {
  const label = details.querySelector(":scope > summary [data-reasoning-label]");
  if (!label || label.dataset.reasoningLabel === "fixed") return;
  const shimmer = label.querySelector("[data-shimmer]") ?? label;
  shimmer.toggleAttribute("data-active", streaming);
  let text;
  if (streaming) text = label.dataset.thinking;
  else if (seconds === undefined) text = label.dataset.doneUnknown;
  else text = (seconds === 1 ? label.dataset.doneOne : label.dataset.doneOther).replace("{n}", String(seconds));
  shimmer.textContent = text;
}

function reasoningStarted(details) {
  const state = reasoningOf(details);
  clearTimeout(state.timer);
  state.started = performance.now();
  setReasoningLabel(details, true);
  if (!readerToggled(details)) setReasoningOpen(details, true);
}

function reasoningEnded(details) {
  const state = reasoningOf(details);
  const given = details.dataset.duration ? Number(details.dataset.duration) : undefined;
  const measured = state.started === undefined ? undefined : Math.max(1, Math.round((performance.now() - state.started) / 1000));
  state.started = undefined;
  setReasoningLabel(details, false, given ?? measured);
  if (details.dataset.autoClose === undefined || readerToggled(details) || state.closedOnce) return;
  state.closedOnce = true;
  state.timer = setTimeout(() => {
    if (readerToggled(details)) return;
    // A keyboard user reading the panel would otherwise lose focus to the page.
    const panel = details.querySelector(":scope > [data-reasoning-panel]");
    if (panel?.contains(document.activeElement)) details.querySelector(":scope > summary")?.focus();
    setReasoningOpen(details, false);
  }, Number(details.dataset.autoClose) || 0);
}

document.addEventListener(
  "toggle",
  (event) => {
    const details = event.target;
    if (!(details instanceof HTMLDetailsElement) || !details.hasAttribute("data-reasoning")) return;
    if (readerToggled(details)) clearTimeout(reasoningOf(details).timer);
  },
  true,
);

// Streaming that is under way when the page loads, or in markup added later, is timed
// from when the script first sees it.
// Every panel's state is noted as soon as it is seen, so that a reader's toggle is told
// apart from the server's.
function seen(details) {
  const state = reasoningOf(details);
  if (details.hasAttribute("data-streaming")) state.started ??= performance.now();
}
document.querySelectorAll("details[data-reasoning]").forEach(seen);

new MutationObserver((records) => {
  for (const record of records) {
    if (record.type === "attributes") {
      const details = record.target;
      if (!details.matches("details[data-reasoning]")) continue;
      const streaming = details.hasAttribute("data-streaming");
      if (streaming === (record.oldValue !== null)) continue;
      if (streaming) reasoningStarted(details);
      else reasoningEnded(details);
    } else {
      for (const node of record.addedNodes) {
        if (!(node instanceof Element)) continue;
        node.querySelectorAll("details[data-reasoning]").forEach(seen);
        if (node.matches("details[data-reasoning]")) seen(node);
      }
    }
  }
}).observe(document.documentElement, {
  subtree: true,
  childList: true,
  attributes: true,
  attributeFilter: ["data-streaming"],
  attributeOldValue: true,
});
