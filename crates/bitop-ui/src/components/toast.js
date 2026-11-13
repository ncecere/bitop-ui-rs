// Toast: the stack, timers, keyboard, and bitopUi.toast for scripts. Follows Base UI's
// toast manager, which upstream uses: the newest toast is in front, the stack spreads
// out while the pointer or the keyboard is in it, and timers pause meanwhile.

const toastTimers = new Map(); // toast element → { timer, remaining, started }
let toastSeq = 0;
// Timers also pause while the window is in the background.
let windowFocused = true;
// Per region: { hovering, focused, prevFocus }.
const regionState = new WeakMap();

function stateOf(region) {
  let state = regionState.get(region);
  if (!state) {
    state = { hovering: false, focused: false, prevFocus: null };
    regionState.set(region, state);
  }
  return state;
}

const liveToasts = (region) =>
  [...region.querySelectorAll(":scope > [data-toast]")].filter((t) => !t.hasAttribute("data-closing"));

function expanded(region) {
  const state = stateOf(region);
  return state.hovering || state.focused;
}

function paused(region) {
  return expanded(region) || !windowFocused;
}

// The height the toast takes when it is not squeezed to the front toast's height.
function measure(toast) {
  const height = toast.style.height;
  toast.style.height = "auto";
  const natural = toast.offsetHeight;
  toast.style.height = height;
  toast.dataset.height = String(natural);
}

function layout(region) {
  const toasts = [...region.querySelectorAll(":scope > [data-toast]")];
  const limit = Number(region.dataset.limit ?? 3);
  let visible = 0;
  let offset = 0;
  let front;
  toasts.forEach((toast, index) => {
    const ending = toast.hasAttribute("data-closing");
    if (!ending && !toast.dataset.height) measure(toast);
    const limited = !ending && visible >= limit;
    toast.toggleAttribute("data-limited", limited);
    toast.inert = limited || ending;
    toast.style.setProperty("--toast-index", String(ending ? index : visible));
    toast.style.setProperty("--toast-offset-y", `${offset}px`);
    if (!ending) toast.style.setProperty("--toast-height", `${toast.dataset.height}px`);
    toast.toggleAttribute("data-behind", !ending && visible > 0);
    if (!ending) {
      front ??= toast;
      offset += Number(toast.dataset.height);
      visible += 1;
    }
  });
  if (front) region.style.setProperty("--toast-frontmost-height", `${front.dataset.height}px`);
  region.toggleAttribute("data-expanded", expanded(region));
}

function startTimer(toast, ms) {
  const entry = toastTimers.get(toast);
  clearTimeout(entry?.timer);
  if (!(ms > 0)) {
    toastTimers.delete(toast);
    return;
  }
  const record = { remaining: ms, started: 0, timer: undefined };
  toastTimers.set(toast, record);
  const region = toast.closest("[data-toast-region]");
  if (!region || !paused(region)) resumeTimer(toast);
}

function resumeTimer(toast) {
  const record = toastTimers.get(toast);
  if (!record || record.timer !== undefined) return;
  record.started = performance.now();
  record.timer = setTimeout(() => closeToast(toast), record.remaining);
}

function pauseTimer(toast) {
  const record = toastTimers.get(toast);
  if (!record || record.timer === undefined) return;
  clearTimeout(record.timer);
  record.timer = undefined;
  record.remaining = Math.max(0, record.remaining - (performance.now() - record.started));
}

function setPaused(region) {
  const pause = paused(region);
  for (const toast of liveToasts(region)) (pause ? pauseTimer : resumeTimer)(toast);
  region.toggleAttribute("data-expanded", expanded(region));
}

function restoreFocus(region) {
  const state = stateOf(region);
  const previous = state.prevFocus;
  state.prevFocus = null;
  if (previous?.isConnected) previous.focus({ preventScroll: true });
}

function closeToast(toast) {
  if (!toast || toast.hasAttribute("data-closing")) return;
  const region = toast.closest("[data-toast-region]");
  clearTimeout(toastTimers.get(toast)?.timer);
  toastTimers.delete(toast);
  // Focus that was in the toast moves to the next one, or the one before, or back
  // where it was before the keyboard came to the toasts.
  const focusInside = toast.contains(document.activeElement) && document.activeElement.matches(":focus-visible");
  let next;
  if (focusInside && region) {
    const toasts = liveToasts(region);
    const index = toasts.indexOf(toast);
    next = toasts[index + 1] ?? toasts[index - 1];
  }
  toast.toggleAttribute("data-closing", true);
  if (region) layout(region);
  if (focusInside) {
    if (next) next.focus({ preventScroll: true });
    else if (region) restoreFocus(region);
  }
  // Remove after the exit transition, or at once when there is none.
  const duration = Math.max(...getComputedStyle(toast).transitionDuration.split(",").map((d) => parseFloat(d) * 1000 || 0));
  setTimeout(() => {
    toast.remove();
    if (region) {
      layout(region);
      if (!liveToasts(region).length) {
        const state = stateOf(region);
        state.hovering = false;
        state.focused = false;
        region.toggleAttribute("data-expanded", false);
      }
    }
  }, duration);
}

function fill(toast, { title = "", description = "", tone = "neutral", action, priority }) {
  toast.dataset.tone = tone;
  toast.setAttribute("role", tone === "danger" || priority === "high" ? "alert" : "status");
  // The template holds an icon for each tone; the toast gets a copy of its own.
  for (const icon of toast.querySelectorAll("[data-toast-icon]")) icon.remove();
  const template = toast.closest("[data-toast-region]")?.querySelector(":scope > template");
  const icon = template?.content.querySelector(`[data-toast-icon="${CSS.escape(tone)}"]`);
  if (icon) toast.querySelector("[data-toast-content]").prepend(icon.cloneNode(true));
  toast.querySelector("[data-toast-title]").textContent = title;
  toast.querySelector("[data-toast-description]").textContent = description ?? "";
  const button = toast.querySelector("[data-toast-action]");
  button.textContent = action?.label ?? "";
  button.hidden = !action;
  button.onclick = action ? (event) => action.onClick?.(event) : null;
  delete toast.dataset.height;
}

function regionOf() {
  const region = document.querySelector("[data-toast-region]");
  if (!region) throw new Error("bitopUi.toast: the page has no toaster");
  return region;
}

function findToast(id) {
  return id === undefined ? null : document.querySelector(`[data-toast-region] > [data-toast][data-toast-id="${CSS.escape(id)}"]`);
}

function add(options) {
  const { id, timeout = 5000, tone = "neutral" } = options;
  const existing = findToast(id);
  if (existing && !existing.hasAttribute("data-closing")) {
    update(id, options);
    return id;
  }
  existing?.remove();
  const region = regionOf();
  const template = region.querySelector(":scope > template");
  const toast = template.content.firstElementChild.cloneNode(true);
  const toastId = id ?? `toast-${++toastSeq}`;
  toast.dataset.toastId = toastId;
  region.prepend(toast);
  fill(toast, options);
  layout(region);
  startTimer(toast, tone === "loading" ? 0 : timeout);
  return toastId;
}

// Changes an open toast: its text, tone, action or timeout.
function update(id, options) {
  const toast = findToast(id);
  if (!toast || toast.hasAttribute("data-closing")) return;
  const merged = {
    title: toast.querySelector("[data-toast-title]").textContent,
    description: toast.querySelector("[data-toast-description]").textContent,
    tone: toast.dataset.tone,
    ...options,
  };
  fill(toast, merged);
  const region = toast.closest("[data-toast-region]");
  if (region) layout(region);
  startTimer(toast, merged.tone === "loading" ? 0 : (options.timeout ?? 5000));
}

function close(id) {
  if (id === undefined) {
    for (const toast of document.querySelectorAll("[data-toast-region] > [data-toast]")) closeToast(toast);
  } else {
    closeToast(findToast(id));
  }
}

// `loading`, `success` and `error` are each a title, options for the toast, or a
// function of the result (or the error) that returns either.
function resolveOptions(value, result) {
  const resolved = typeof value === "function" ? value(result) : value;
  return typeof resolved === "string" ? { title: resolved } : (resolved ?? {});
}

function promise(value, { loading, success, error }) {
  const id = add({ ...resolveOptions(loading), tone: "loading" });
  return Promise.resolve(value).then(
    (result) => {
      update(id, { description: "", action: undefined, ...resolveOptions(success, result), tone: "success" });
      return result;
    },
    (reason) => {
      update(id, { description: "", action: undefined, ...resolveOptions(error, reason), tone: "danger" });
      throw reason;
    },
  );
}

const toast = (options) => add(options);
Object.assign(toast, {
  add,
  update,
  close,
  promise,
  success: (title, description) => add({ title, description, tone: "success" }),
  error: (title, description) => add({ title, description, tone: "danger" }),
  info: (title, description) => add({ title, description, tone: "info" }),
  warning: (title, description) => add({ title, description, tone: "warning" }),
});
window.bitopUi.toast = toast;

const regionFrom = (target) => (target instanceof Element ? target.closest("[data-toast-region]") : null);

document.addEventListener("pointerover", (event) => {
  const region = regionFrom(event.target);
  if (!region || stateOf(region).hovering) return;
  stateOf(region).hovering = true;
  setPaused(region);
});

document.addEventListener("pointerout", (event) => {
  const region = regionFrom(event.target);
  if (!region || regionFrom(event.relatedTarget) === region) return;
  stateOf(region).hovering = false;
  setPaused(region);
});

document.addEventListener("focusin", (event) => {
  const region = regionFrom(event.target);
  if (!region || stateOf(region).focused) return;
  // A click inside does not spread the stack; the keyboard does.
  if (event.target !== region && !event.target.matches(":focus-visible")) return;
  stateOf(region).focused = true;
  setPaused(region);
});

document.addEventListener("focusout", (event) => {
  const region = regionFrom(event.target);
  if (!region || regionFrom(event.relatedTarget) === region) return;
  // The window lost focus: the toasts stay as they are until it comes back.
  if (!event.relatedTarget && !document.hasFocus()) return;
  const state = stateOf(region);
  if (!state.focused) return;
  state.focused = false;
  setPaused(region);
});

document.addEventListener("keydown", (event) => {
  const region = document.querySelector("[data-toast-region]");
  if (!region) return;
  if (event.key === "F6" && event.target !== region) {
    if (!liveToasts(region).length) return;
    event.preventDefault();
    const state = stateOf(region);
    if (!region.contains(document.activeElement)) state.prevFocus = document.activeElement;
    region.focus({ preventScroll: true });
    state.focused = true;
    setPaused(region);
    return;
  }
  if (!region.contains(event.target)) return;
  if (event.key === "Escape") {
    const toast = event.target.closest("[data-toast]");
    if (toast) {
      event.preventDefault();
      closeToast(toast);
    }
    return;
  }
  if (event.key === "Tab" && stateOf(region).prevFocus) {
    // Shift+Tab from the region, or Tab from its last control, goes back where the
    // keyboard was before F6.
    const focusable = [...region.querySelectorAll("[data-toast], [data-toast] button")].filter(
      (el) => !el.closest("[inert]") && !el.hidden && el.checkVisibility(),
    );
    const leaving = event.shiftKey ? event.target === region : event.target === focusable.at(-1);
    if (leaving) {
      event.preventDefault();
      restoreFocus(region);
    }
  }
});

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-toast-close]");
  if (button) closeToast(button.closest("[data-toast]"));
});

addEventListener("blur", (event) => {
  if (event.target !== window) return;
  windowFocused = false;
  for (const region of document.querySelectorAll("[data-toast-region]")) setPaused(region);
});
addEventListener("focus", (event) => {
  if (event.target !== window) return;
  windowFocused = true;
  for (const region of document.querySelectorAll("[data-toast-region]")) setPaused(region);
});
addEventListener("resize", () => {
  for (const region of document.querySelectorAll("[data-toast-region]")) {
    for (const toast of liveToasts(region)) delete toast.dataset.height;
    layout(region);
  }
});

// Toasts the server rendered.
for (const region of document.querySelectorAll("[data-toast-region]")) {
  const toasts = liveToasts(region);
  if (!toasts.length) continue;
  for (const toast of toasts) toast.dataset.toastId ??= `toast-${++toastSeq}`;
  layout(region);
  for (const toast of toasts) startTimer(toast, Number(toast.dataset.timeout ?? 5000));
}
