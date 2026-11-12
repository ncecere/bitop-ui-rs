// Resizable panels: lays a group out, and moves a handle with the pointer or the keys.
// A panel's size is its share of the group in percent, kept in --panel-size.

const RESIZE_GROUP = "[data-resizable-group]";
const RESIZE_HANDLE = "[data-resizable-handle]";
const RESIZE_PANEL = "[data-resizable-panel]";

const clampSize = (value, low, high) => Math.min(Math.max(value, low), high);

function resizePanels(group) {
  return [...group.children].filter((child) => child.matches(RESIZE_PANEL));
}

function panelLimits(panel) {
  const number = (text, fallback) => (text === undefined || Number.isNaN(Number(text)) ? fallback : Number(text));
  return {
    min: number(panel.dataset.minSize, 0),
    max: number(panel.dataset.maxSize, 100),
    initial: number(panel.dataset.defaultSize, undefined),
  };
}

function panelSize(panel) {
  const value = panel.style.getPropertyValue("--panel-size");
  return value === "" ? undefined : Number(value);
}

function storedLayout(group) {
  if (!group.dataset.storage) return undefined;
  try {
    return JSON.parse(localStorage.getItem(group.dataset.storage));
  } catch {
    return undefined;
  }
}

// First sizes: a stored layout if it fits; otherwise the default sizes, with the rest
// shared among the panels that have none, each kept within its limits. Either way
// they are scaled to add up to 100.
function firstLayout(limits, stored) {
  if (Array.isArray(stored) && stored.length === limits.length && stored.every((n) => Number.isFinite(n) && n >= 0)) {
    const sum = stored.reduce((a, b) => a + b, 0);
    if (sum > 0) return stored.map((n) => (n / sum) * 100);
  }
  const fixed = limits.reduce((sum, limit) => sum + (limit.initial ?? 0), 0);
  const flexible = limits.filter((limit) => limit.initial === undefined).length;
  const share = flexible > 0 ? Math.max(0, 100 - fixed) / flexible : 0;
  const raw = limits.map((limit) => clampSize(limit.initial ?? share, limit.min, limit.max));
  const sum = raw.reduce((a, b) => a + b, 0) || 1;
  return raw.map((n) => (n / sum) * 100);
}

// The two panels a handle separates, their sizes, and how far the first can go.
function handlePair(handle) {
  const before = handle.previousElementSibling;
  const after = handle.nextElementSibling;
  if (!before?.matches(RESIZE_PANEL) || !after?.matches(RESIZE_PANEL)) return undefined;
  const beforeSize = panelSize(before);
  const afterSize = panelSize(after);
  if (beforeSize === undefined || afterSize === undefined) return undefined;
  const b = panelLimits(before);
  const a = panelLimits(after);
  const total = beforeSize + afterSize;
  const min = Math.max(b.min, total - a.max);
  const max = Math.max(min, Math.min(b.max, total - a.min));
  return { before, after, beforeSize, total, min, max };
}

function updateHandles(group) {
  const horizontal = group.dataset.orientation !== "vertical";
  for (const handle of group.children) {
    if (!handle.matches(RESIZE_HANDLE)) continue;
    // A horizontal group has a vertical splitter between its panels.
    handle.setAttribute("aria-orientation", horizontal ? "vertical" : "horizontal");
    const pair = handlePair(handle);
    if (!pair) continue;
    if (pair.before.id) handle.setAttribute("aria-controls", pair.before.id);
    handle.setAttribute("aria-valuenow", String(Math.round(pair.beforeSize)));
    handle.setAttribute("aria-valuemin", String(Math.round(pair.min)));
    handle.setAttribute("aria-valuemax", String(Math.round(pair.max)));
  }
}

// Lays a group out once. Markup that the server adds later is laid out when one of
// its handles is first used.
function layoutGroup(group) {
  const panels = resizePanels(group);
  if (panels.some((panel) => panelSize(panel) === undefined)) {
    const layout = firstLayout(panels.map(panelLimits), storedLayout(group));
    panels.forEach((panel, index) => {
      panel.style.setProperty("--panel-size", String(layout[index]));
      panel.removeAttribute("data-size-pending");
    });
  }
  updateHandles(group);
}

function resizeTo(handle, size) {
  const pair = handlePair(handle);
  if (!pair) return;
  const next = clampSize(size, pair.min, pair.max);
  if (next === pair.beforeSize) return;
  pair.before.style.setProperty("--panel-size", String(next));
  pair.after.style.setProperty("--panel-size", String(pair.total - next));
  updateHandles(handle.parentElement);
}

// A resize has finished: keep the layout if asked to, and tell the page.
function commitLayout(group) {
  const layout = resizePanels(group).map((panel) => Math.round((panelSize(panel) ?? 0) * 100) / 100);
  if (group.dataset.storage) {
    try {
      localStorage.setItem(group.dataset.storage, JSON.stringify(layout));
    } catch {
      // Storage can be full or turned off; the layout still holds for this page.
    }
  }
  group.dispatchEvent(new CustomEvent("bitop:layoutchange", { bubbles: true, detail: { layout } }));
}

function usableHandle(target) {
  const handle = target instanceof Element && target.closest(RESIZE_HANDLE);
  if (!handle || handle.getAttribute("aria-disabled") === "true") return undefined;
  const group = handle.parentElement;
  if (!group?.matches(RESIZE_GROUP)) return undefined;
  layoutGroup(group);
  return handle;
}

function isRtl(handle, horizontal) {
  return horizontal && getComputedStyle(handle).direction === "rtl";
}

for (const group of document.querySelectorAll(RESIZE_GROUP)) layoutGroup(group);

document.addEventListener("keydown", (event) => {
  if (event.ctrlKey || event.metaKey || event.altKey) return;
  const handle = usableHandle(event.target);
  const pair = handle && handlePair(handle);
  if (!pair) return;
  const group = handle.parentElement;
  const horizontal = group.dataset.orientation !== "vertical";
  const step = Number(group.dataset.keyboardStep) || 5;
  // In a right-to-left page the first panel is on the right, so ArrowLeft grows it.
  const sign = isRtl(handle, horizontal) ? -1 : 1;
  let next;
  if (horizontal && event.key === "ArrowLeft") next = pair.beforeSize - step * sign;
  else if (horizontal && event.key === "ArrowRight") next = pair.beforeSize + step * sign;
  else if (!horizontal && event.key === "ArrowUp") next = pair.beforeSize - step;
  else if (!horizontal && event.key === "ArrowDown") next = pair.beforeSize + step;
  else if (event.key === "Home") next = pair.min;
  else if (event.key === "End") next = pair.max;
  else return;
  event.preventDefault();
  resizeTo(handle, next);
  commitLayout(group);
});

let resizeDrag;

document.addEventListener("pointerdown", (event) => {
  if (event.button !== 0) return;
  const handle = usableHandle(event.target);
  const pair = handle && handlePair(handle);
  if (!pair) return;
  const horizontal = handle.parentElement.dataset.orientation !== "vertical";
  const a = pair.before.getBoundingClientRect();
  const b = pair.after.getBoundingClientRect();
  const pixels = horizontal ? a.width + b.width : a.height + b.height;
  // No text selection while dragging, and the keys work from here on.
  event.preventDefault();
  handle.focus();
  handle.setPointerCapture?.(event.pointerId);
  handle.setAttribute("data-dragging", "");
  resizeDrag = {
    handle,
    pointer: event.pointerId,
    horizontal,
    start: horizontal ? event.clientX : event.clientY,
    startSize: pair.beforeSize,
    pixelsPerPercent: pixels > 0 ? pixels / pair.total : 0,
    sign: isRtl(handle, horizontal) ? -1 : 1,
  };
});

document.addEventListener("pointermove", (event) => {
  const drag = resizeDrag;
  if (!drag || event.pointerId !== drag.pointer || drag.pixelsPerPercent === 0) return;
  const moved = ((drag.horizontal ? event.clientX : event.clientY) - drag.start) * drag.sign;
  resizeTo(drag.handle, drag.startSize + moved / drag.pixelsPerPercent);
});

for (const type of ["pointerup", "pointercancel"]) {
  document.addEventListener(type, (event) => {
    const drag = resizeDrag;
    if (!drag || event.pointerId !== drag.pointer) return;
    resizeDrag = undefined;
    drag.handle.removeAttribute("data-dragging");
    if (drag.handle.hasPointerCapture?.(event.pointerId)) drag.handle.releasePointerCapture(event.pointerId);
    commitLayout(drag.handle.parentElement);
  });
}
