// Context menu: opens a menu popover at the pointer on right-click or long-press, and
// at the focused element on Shift+F10 or the context-menu key. The menu itself (focus,
// arrow keys, typeahead, closing on choice) is menu.js.

const CONTEXT_EDGE = 8;
const contextTriggers = new WeakMap();
let contextPointerDown = false;
let contextKeyAt = 0;

function openContextMenu(trigger, x, y) {
  const popup = document.getElementById(trigger.dataset.contextMenu);
  if (!popup) return;
  if (popup.matches(":popover-open")) popup.hidePopover();
  contextTriggers.set(popup, trigger);
  popup.showPopover();
  // Start-aligned at the pointer; on the other side of it where there is no room.
  const { width, height } = popup.getBoundingClientRect();
  const rtl = getComputedStyle(trigger).direction === "rtl";
  let left = rtl ? x - width : x;
  if (left + width > innerWidth - CONTEXT_EDGE) left = x - width;
  if (left < CONTEXT_EDGE) left = Math.min(x, innerWidth - width - CONTEXT_EDGE);
  let top = y;
  if (top + height > innerHeight - CONTEXT_EDGE) top = y - height;
  popup.style.left = `${Math.max(CONTEXT_EDGE, left)}px`;
  popup.style.top = `${Math.max(CONTEXT_EDGE, top)}px`;
  trigger.setAttribute("data-popup-open", "");
}

document.addEventListener(
  "toggle",
  (event) => {
    const trigger = contextTriggers.get(event.target);
    if (trigger && event.newState === "closed") trigger.removeAttribute("data-popup-open");
  },
  true,
);

for (const type of ["pointerdown", "pointerup", "pointercancel"]) {
  document.addEventListener(type, () => (contextPointerDown = type === "pointerdown"), true);
}

document.addEventListener("contextmenu", (event) => {
  if (!(event.target instanceof Element) || event.target.closest("[data-context-menu-popup]")) return;
  const trigger = event.target.closest("[data-context-menu]");
  if (!trigger) return;
  event.preventDefault();
  // The browser's own event for a key press that was handled below. A mouse sends
  // button 2.
  if (event.button !== 2 && Date.now() - contextKeyAt < 500) return;
  const show = () => openContextMenu(trigger, event.clientX, event.clientY);
  if (!contextPointerDown && event.buttons === 0) {
    show();
    return;
  }
  // Some platforms send contextmenu when the button goes down. A popover shown then
  // would be closed as a click outside it when the button comes up, so wait for that.
  const release = () => {
    document.removeEventListener("pointerup", release, true);
    document.removeEventListener("pointercancel", release, true);
    setTimeout(show);
  };
  document.addEventListener("pointerup", release, true);
  document.addEventListener("pointercancel", release, true);
});

document.addEventListener("keydown", (event) => {
  if (event.key !== "ContextMenu" && !(event.key === "F10" && event.shiftKey)) return;
  if (!(event.target instanceof Element) || event.target.closest("[data-context-menu-popup]")) return;
  const trigger = event.target.closest("[data-context-menu]");
  if (!trigger) return;
  event.preventDefault();
  contextKeyAt = Date.now();
  const box = event.target.getBoundingClientRect();
  openContextMenu(trigger, box.left + box.width / 2, box.top + box.height / 2);
});
