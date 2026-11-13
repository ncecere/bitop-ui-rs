// Navigation menu: panels that open on hover as well as on click, aria-expanded on
// their buttons, and arrow keys. Each panel is a popover opened by its button, so the
// browser handles Escape, a click outside and the anchoring.

const NAV = "[data-navigation-menu]";
const NAV_TRIGGER = "[data-navigation-trigger]";
const NAV_PANEL = "[data-navigation-content]";
const NAV_LINK = "[data-navigation-link]";
const NAV_OPEN_DELAY = 50;
const NAV_CLOSE_DELAY = 150;
let navOpenTimer;
let navCloseTimer;
let navOpening = false;
// The triggers whose panels the pointer opened, and no click has pinned yet.
const navHoverOpened = new WeakSet();

function navPanel(trigger) {
  return document.getElementById(trigger.getAttribute("popovertarget"));
}

function navTrigger(panel) {
  return panel.closest(NAV)?.querySelector(`${NAV_TRIGGER}[popovertarget="${CSS.escape(panel.id)}"]`);
}

// The trigger of the panel that `target` is in, or the trigger that `target` is in.
function navOwner(target) {
  if (!(target instanceof Element)) return undefined;
  const panel = target.closest(NAV_PANEL);
  return panel ? navTrigger(panel) : (target.closest(NAV_TRIGGER) ?? undefined);
}

// Opens through the button, so that the browser anchors the panel to it.
function openNav(trigger) {
  const panel = navPanel(trigger);
  if (!panel || panel.matches(":popover-open") || trigger.disabled) return;
  navOpening = true;
  trigger.click();
  navOpening = false;
}

function closeNav(trigger) {
  const panel = navPanel(trigger);
  if (panel?.matches(":popover-open")) panel.hidePopover();
}

// beforetoggle is sent as the popover opens or closes, so aria-expanded is never behind.
document.addEventListener(
  "beforetoggle",
  (event) => {
    const panel = event.target;
    if (!(panel instanceof HTMLElement) || !panel.matches(NAV_PANEL)) return;
    const open = event.newState === "open";
    const trigger = navTrigger(panel);
    trigger?.setAttribute("aria-expanded", String(open));
    if (!open && trigger) navHoverOpened.delete(trigger);
  },
  true,
);

document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch") return;
  const trigger = navOwner(event.target);
  if (!trigger) return;
  clearTimeout(navCloseTimer);
  if (navPanel(trigger)?.matches(":popover-open")) return;
  clearTimeout(navOpenTimer);
  navOpenTimer = setTimeout(() => {
    openNav(trigger);
    navHoverOpened.add(trigger);
  }, NAV_OPEN_DELAY);
});

document.addEventListener("pointerout", (event) => {
  const trigger = navOwner(event.target);
  if (!trigger || navOwner(event.relatedTarget) === trigger) return;
  clearTimeout(navOpenTimer);
  // A panel opened by a click or a key stays until it is dismissed.
  if (!navHoverOpened.has(trigger)) return;
  clearTimeout(navCloseTimer);
  navCloseTimer = setTimeout(() => {
    if (navHoverOpened.has(trigger)) closeNav(trigger);
  }, NAV_CLOSE_DELAY);
});

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const trigger = event.target.closest(NAV_TRIGGER);
  if (trigger) {
    if (navOpening) return;
    clearTimeout(navOpenTimer);
    // The pointer opened it: the first click keeps it open, so that it stays when the
    // pointer leaves. The next click closes it.
    if (navHoverOpened.delete(trigger)) event.preventDefault();
    return;
  }
  // Following a link in a panel closes the panel.
  const link = event.target.closest(`${NAV_PANEL} ${NAV_LINK}`);
  if (link) link.closest(NAV_PANEL).hidePopover();
});

// A panel closes when focus leaves it and its button.
document.addEventListener("focusout", (event) => {
  const trigger = navOwner(event.target);
  if (!trigger || !event.relatedTarget || navOwner(event.relatedTarget) === trigger) return;
  closeNav(trigger);
});

document.addEventListener("keydown", (event) => {
  if (!(event.target instanceof Element) || event.ctrlKey || event.metaKey || event.altKey) return;
  const nav = event.target.closest(NAV);
  if (!nav) return;
  const panel = event.target.closest(NAV_PANEL);

  if (panel) {
    const trigger = navTrigger(panel);
    const links = [...panel.querySelectorAll(NAV_LINK)];
    const index = links.indexOf(event.target.closest(NAV_LINK));
    let next;
    if (event.key === "Escape") {
      panel.hidePopover();
      next = trigger;
    } else if (event.key === "ArrowDown") next = links[index + 1];
    // Up from the first link goes back to the button.
    else if (event.key === "ArrowUp") next = links[index - 1] ?? trigger;
    else if (event.key === "Home") next = links[0];
    else if (event.key === "End") next = links.at(-1);
    else return;
    event.preventDefault();
    next?.focus();
    return;
  }

  const entry = event.target.closest(`${NAV_TRIGGER}, ${NAV_LINK}`);
  if (!entry) return;
  if (event.key === "ArrowDown" && entry.matches(NAV_TRIGGER)) {
    event.preventDefault();
    openNav(entry);
    navPanel(entry)?.querySelector(NAV_LINK)?.focus();
    return;
  }
  const list = entry.closest("ul");
  if (!list || list.dataset.orientation === "vertical") return;
  const entries = [...list.querySelectorAll(`:scope > li > ${NAV_TRIGGER}:not(:disabled), :scope > li > ${NAV_LINK}`)];
  const index = entries.indexOf(entry);
  const rtl = getComputedStyle(list).direction === "rtl";
  let next;
  if (event.key === (rtl ? "ArrowLeft" : "ArrowRight")) next = entries[index + 1];
  else if (event.key === (rtl ? "ArrowRight" : "ArrowLeft")) next = entries[index - 1];
  else if (event.key === "Home") next = entries[0];
  else if (event.key === "End") next = entries.at(-1);
  else return;
  event.preventDefault();
  next?.focus();
});
