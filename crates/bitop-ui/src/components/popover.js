// Popover: focus moves into the popup when it opens, and `open_on_hover` opens it when
// the pointer rests on its trigger.

const POPOVER_FOCUSABLE =
  "a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";

// Popups opened by hover. They do not take focus, and close when the pointer leaves.
const hoverOpened = new WeakSet();
let hoverTimer;
let leaveTimer;
let hoverOpening;

function popoverOf(trigger) {
  const popup = document.getElementById(trigger.getAttribute("popovertarget"));
  return popup?.hasAttribute("data-popover") ? popup : null;
}

function hoverTrigger(target) {
  const trigger = target instanceof Element && target.closest("[popovertarget]");
  const popup = trigger && popoverOf(trigger);
  return popup?.hasAttribute("data-open-on-hover") ? [trigger, popup] : [];
}

// Opened by its button, which anchors the popup to it and returns focus to it.
document.addEventListener(
  "toggle",
  (event) => {
    const popup = event.target;
    if (!(popup instanceof HTMLElement) || !popup.hasAttribute("data-popover")) return;
    if (event.newState !== "open") {
      hoverOpened.delete(popup);
      return;
    }
    if (hoverOpened.has(popup) || popup.contains(document.activeElement)) return;
    const first = [...popup.querySelectorAll(POPOVER_FOCUSABLE)].find((el) => el.checkVisibility());
    (first ?? popup).focus({ preventScroll: true });
  },
  true,
);

document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch") return;
  const [trigger, popup] = hoverTrigger(event.target);
  if (trigger) {
    clearTimeout(leaveTimer);
    if (popup.matches(":popover-open")) return;
    clearTimeout(hoverTimer);
    hoverTimer = setTimeout(() => {
      if (popup.matches(":popover-open")) return;
      hoverOpened.add(popup);
      hoverOpening = trigger;
      trigger.click();
      hoverOpening = undefined;
    }, Number(popup.dataset.hoverDelay ?? 300));
    return;
  }
  // Inside a popup that hover opened: keep it.
  const inside = event.target instanceof Element && event.target.closest("[data-popover]");
  if (inside && hoverOpened.has(inside)) clearTimeout(leaveTimer);
});

document.addEventListener("pointerout", (event) => {
  const from = event.target instanceof Element ? event.target : null;
  const [trigger, triggerPopup] = hoverTrigger(from);
  const popup = triggerPopup ?? from?.closest("[data-popover]");
  if (!popup) return;
  const owner = trigger ?? document.querySelector(`[popovertarget="${CSS.escape(popup.id)}"]`);
  const to = event.relatedTarget;
  if (to instanceof Node && owner?.contains(to)) return;
  if (trigger) clearTimeout(hoverTimer);
  if (!hoverOpened.has(popup) || (to instanceof Node && popup.contains(to))) return;
  clearTimeout(leaveTimer);
  // A moment to cross the gap between the trigger and the popup.
  leaveTimer = setTimeout(() => {
    if (hoverOpened.has(popup) && popup.matches(":popover-open")) popup.hidePopover();
  }, 150);
});

// A click on the trigger of a popup that hover opened keeps it open, as upstream.
document.addEventListener("click", (event) => {
  const [trigger, popup] = hoverTrigger(event.target);
  if (!trigger || trigger === hoverOpening) return;
  if (hoverOpened.has(popup) && popup.matches(":popover-open")) {
    event.preventDefault();
    hoverOpened.delete(popup);
    clearTimeout(leaveTimer);
    const first = [...popup.querySelectorAll(POPOVER_FOCUSABLE)].find((el) => el.checkVisibility());
    (first ?? popup).focus({ preventScroll: true });
  }
  clearTimeout(hoverTimer);
});
