// Menubar: one tab stop, arrow keys between the top-level menus, and opening the next
// menu from inside an open one. What happens inside a menu is menu.js.

const BAR = "[data-menubar]";
const BAR_TRIGGER = "[data-menubar-trigger]";
const BAR_ITEM = '[role="menuitem"]:not([aria-disabled="true"])';

function barTriggers(bar) {
  return [...bar.querySelectorAll(BAR_TRIGGER)].filter((trigger) => !trigger.disabled);
}

function barPopup(trigger) {
  return document.getElementById(trigger.getAttribute("popovertarget"));
}

function barTriggerOf(popup) {
  return popup.previousElementSibling?.matches(BAR_TRIGGER) ? popup.previousElementSibling : undefined;
}

// Makes `stop` the bar's only tab stop; by default its first menu.
function setBarStop(bar, stop) {
  stop ??= barTriggers(bar)[0];
  for (const trigger of bar.querySelectorAll(BAR_TRIGGER)) trigger.tabIndex = trigger === stop ? 0 : -1;
}

// The keys that move forward and back along the bar, and the one that opens a menu
// in a vertical bar. In a right-to-left page they follow the visual order.
function barKeys(bar) {
  const rtl = getComputedStyle(bar).direction === "rtl";
  const forward = rtl ? "ArrowLeft" : "ArrowRight";
  const backward = rtl ? "ArrowRight" : "ArrowLeft";
  return bar.dataset.orientation === "vertical"
    ? { next: "ArrowDown", previous: "ArrowUp", open: forward, close: backward }
    : { next: forward, previous: backward };
}

// Opens the trigger's menu with focus on its first item, or its last. The click goes
// through the button, so that the browser anchors the popup to it and returns focus to
// it when the menu closes.
function openBarMenu(trigger, last = false) {
  const popup = barPopup(trigger);
  if (!popup) return;
  trigger.focus();
  if (!popup.matches(":popover-open")) {
    // menu.js moves focus to the first item when the menu has opened; the listener
    // below moves it on to the last.
    popup.toggleAttribute("data-menubar-last", last);
    trigger.click();
  }
  const items = popup.querySelectorAll(BAR_ITEM);
  (last ? items[items.length - 1] : items[0])?.focus();
}

for (const bar of document.querySelectorAll(BAR)) setBarStop(bar);

// beforetoggle is sent as the popover opens or closes, so aria-expanded is never behind.
document.addEventListener(
  "beforetoggle",
  (event) => {
    const popup = event.target;
    if (!(popup instanceof HTMLElement) || !popup.matches("[data-menubar-popup]")) return;
    barTriggerOf(popup)?.setAttribute("aria-expanded", String(event.newState === "open"));
  },
  true,
);

document.addEventListener(
  "toggle",
  (event) => {
    const popup = event.target;
    if (!(popup instanceof HTMLElement) || !popup.matches("[data-menubar-popup]")) return;
    if (event.newState === "open" && popup.hasAttribute("data-menubar-last")) {
      popup.removeAttribute("data-menubar-last");
      [...popup.querySelectorAll(BAR_ITEM)].at(-1)?.focus();
    }
  },
  true,
);

document.addEventListener("focusin", (event) => {
  const trigger = event.target instanceof Element && event.target.closest(BAR_TRIGGER);
  const bar = trigger && trigger.closest(BAR);
  if (bar) setBarStop(bar, trigger);
});

document.addEventListener("keydown", (event) => {
  if (!(event.target instanceof Element) || event.ctrlKey || event.metaKey || event.altKey) return;
  const bar = event.target.closest(BAR);
  if (!bar) return;
  const keys = barKeys(bar);
  const triggers = barTriggers(bar);
  const popup = event.target.closest("[data-menubar-popup]");
  const trigger = popup ? barTriggerOf(popup) : event.target.closest(BAR_TRIGGER);
  const index = triggers.indexOf(trigger);
  if (index === -1) return;
  const next = triggers[(index + 1) % triggers.length];
  const previous = triggers[(index - 1 + triggers.length) % triggers.length];

  if (popup) {
    if (event.key === "Escape") {
      // The browser would return focus to what had it before the menu opened, and in
      // Safari a click does not focus a button.
      event.preventDefault();
      popup.hidePopover();
      trigger.focus();
      return;
    }
    // Inside an open menu: the bar's keys open the neighbouring menu.
    if (bar.dataset.orientation === "vertical") {
      if (event.key !== keys.close) return;
      event.preventDefault();
      popup.hidePopover();
      trigger.focus();
    } else if (event.key === keys.next || event.key === keys.previous) {
      event.preventDefault();
      openBarMenu(event.key === keys.next ? next : previous);
    }
    return;
  }

  let target;
  if (event.key === keys.next) target = next;
  else if (event.key === keys.previous) target = previous;
  else if (event.key === "Home") target = triggers[0];
  else if (event.key === "End") target = triggers.at(-1);
  else if (event.key === (keys.open ?? "ArrowDown")) {
    event.preventDefault();
    openBarMenu(trigger);
    return;
  } else if (event.key === "ArrowUp" && !keys.open) {
    event.preventDefault();
    openBarMenu(trigger, true);
    return;
  } else if (event.key.length === 1 && event.key !== " ") {
    // A letter moves to the next menu whose name starts with it.
    const ordered = [...triggers.slice(index + 1), ...triggers.slice(0, index + 1)];
    target = ordered.find((other) => other.textContent.trim().toLowerCase().startsWith(event.key.toLowerCase()));
    if (!target) return;
  } else return;
  event.preventDefault();
  target.focus();
});

// With one menu open, pointing at another top-level menu opens it.
document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const trigger = event.target.closest(BAR_TRIGGER);
  const bar = trigger && trigger.closest(BAR);
  if (!bar || trigger.disabled || trigger.contains(event.relatedTarget)) return;
  const open = bar.querySelector("[data-menubar-popup]:popover-open");
  if (open && open !== barPopup(trigger)) openBarMenu(trigger);
});
