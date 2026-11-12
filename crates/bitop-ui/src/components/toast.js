// Toast: timers, the dismiss button, and bitopUi.toast() for scripts.

const toastTimers = new Map();

function closeToast(toast) {
  clearTimeout(toastTimers.get(toast));
  toastTimers.delete(toast);
  toast.toggleAttribute("data-closing", true);
  // Remove after the exit transition, or at once when there is none.
  const duration = parseFloat(getComputedStyle(toast).transitionDuration) * 1000 || 0;
  setTimeout(() => toast.remove(), duration);
}

function startToast(toast) {
  const duration = Number(toast.dataset.duration ?? 5000);
  if (duration > 0) toastTimers.set(toast, setTimeout(() => closeToast(toast), duration));
}

function pauseToasts(region) {
  for (const toast of region.querySelectorAll("[data-toast]")) {
    clearTimeout(toastTimers.get(toast));
  }
}

function resumeToasts(region) {
  if (region.matches(":hover, :focus-within")) return;
  for (const toast of region.querySelectorAll("[data-toast]:not([data-closing])")) {
    clearTimeout(toastTimers.get(toast));
    startToast(toast);
  }
}

for (const type of ["pointerenter", "focusin"]) {
  document.addEventListener(type, (event) => {
    const region = event.target instanceof Element && event.target.closest("[data-toast-region]");
    if (region) pauseToasts(region);
  }, true);
}

for (const type of ["pointerleave", "focusout"]) {
  document.addEventListener(type, (event) => {
    const region = event.target instanceof Element && event.target.closest("[data-toast-region]");
    // Wait a tick so :focus-within reflects where focus went.
    if (region) setTimeout(() => resumeToasts(region));
  }, true);
}

document.addEventListener("click", (event) => {
  const close = event.target instanceof Element && event.target.closest("[data-toast-close]");
  if (close) closeToast(close.closest("[data-toast]"));
});

function toast({ title, description = "", variant = "neutral", duration = 5000 }) {
  const region = document.querySelector("[data-toast-region]");
  const template = region?.querySelector(":scope > template");
  if (!template) throw new Error("bitopUi.toast: the page has no toast region");
  const element = template.content.firstElementChild.cloneNode(true);
  element.dataset.variant = variant;
  element.dataset.duration = String(duration);
  if (variant === "danger") element.setAttribute("role", "alert");
  element.querySelector("[data-toast-title]").textContent = title;
  element.querySelector("[data-toast-description]").textContent = description;
  region.insertBefore(element, template);
  startToast(element);
  return element;
}

for (const element of document.querySelectorAll("[data-toast]")) startToast(element);

window.bitopUi.toast = toast;
