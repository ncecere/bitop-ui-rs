// Copy controls: a button with data-copy writes text to the clipboard, shows a copied
// state for a moment and announces the result. Copy field uses it too.

// Writes text to the clipboard. Without the Clipboard API (an insecure context), or
// when it refuses, a temporary textarea and execCommand do it.
async function copy(text) {
  try {
    await navigator.clipboard.writeText(text);
    return;
  } catch {
    // Fall through.
  }
  const area = document.createElement("textarea");
  area.value = text;
  area.setAttribute("readonly", "");
  area.style.position = "fixed";
  area.style.opacity = "0";
  document.body.append(area);
  area.select();
  const ok = document.execCommand("copy");
  area.remove();
  if (!ok) throw new Error("copy failed");
}
window.bitopUi.copy = copy;

const timers = new WeakMap();

// The state is "idle", "copied" or "failed".
function setState(button, state) {
  const root = button.closest("[data-copy-root]") ?? button;
  const copied = state === "copied";
  button.dataset.state = state;
  const label = copied ? button.dataset.copiedLabel : button.dataset.label;
  if (label) button.setAttribute("aria-label", label);
  for (const text of root.querySelectorAll("[data-copy-text]")) {
    text.textContent = copied ? text.dataset.copied : text.dataset.idle;
  }
  const status = root.querySelector("[role=status]");
  if (status) status.textContent = (copied ? status.dataset.copied : state === "failed" ? status.dataset.failed : "") ?? "";
}

document.addEventListener("click", async (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-copy]");
  if (!button) return;
  const target = document.getElementById(button.dataset.copyTarget);
  const text = target ? (target.value ?? target.textContent) : (button.dataset.copyValue ?? "");
  clearTimeout(timers.get(button));
  try {
    await copy(text);
    setState(button, "copied");
  } catch {
    setState(button, "failed");
  }
  timers.set(
    button,
    setTimeout(() => setState(button, "idle"), Number(button.dataset.copyTimeout) || 2000),
  );
});
