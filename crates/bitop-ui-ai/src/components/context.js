// Context: the card opens on click, Enter and Space through popovertarget, and also on
// hover after a short delay. A card opened by hover closes when the pointer leaves the
// trigger and the card; a click on the trigger then keeps it open. Opened by a click or
// a key, the card takes focus, as upstream's popover does; Escape gives it back.

let contextHoverTimer;
let contextLeaveTimer;
// The card the pointer opened, while it has not been clicked.
let contextHovered;

const contextCard = (trigger) => document.getElementById(trigger.getAttribute("popovertarget"));

function contextOwner(target) {
  if (!(target instanceof Element)) return undefined;
  const trigger = target.closest("[data-context-trigger]");
  if (trigger) return trigger;
  const card = target.closest("[data-context-card]");
  return card ? document.querySelector(`[data-context-trigger][popovertarget="${CSS.escape(card.id)}"]`) : undefined;
}

document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch") return;
  const trigger = contextOwner(event.target);
  if (!trigger) return;
  clearTimeout(contextLeaveTimer);
  const card = contextCard(trigger);
  if (!card || card.matches(":popover-open") || trigger.disabled) return;
  clearTimeout(contextHoverTimer);
  contextHoverTimer = setTimeout(() => {
    if (card.matches(":popover-open")) return;
    contextHovered = card;
    // A click anchors the card to its trigger, as a press would.
    trigger.click();
  }, Number(trigger.dataset.delay ?? 200));
});

document.addEventListener("pointerout", (event) => {
  const trigger = contextOwner(event.target);
  if (!trigger || contextOwner(event.relatedTarget) === trigger) return;
  clearTimeout(contextHoverTimer);
  const card = contextCard(trigger);
  if (contextHovered !== card) return;
  contextLeaveTimer = setTimeout(() => {
    if (contextHovered === card && card.matches(":popover-open")) card.hidePopover();
  }, 200);
});

document.addEventListener("click", (event) => {
  const trigger = event.target instanceof Element && event.target.closest("[data-context-trigger]");
  if (!trigger || !event.isTrusted) return;
  const card = contextCard(trigger);
  // The pointer opened it: the click keeps it open rather than closing it.
  if (contextHovered === card && card?.matches(":popover-open")) event.preventDefault();
  contextHovered = undefined;
  clearTimeout(contextHoverTimer);
});

document.addEventListener(
  "beforetoggle",
  (event) => {
    const card = event.target;
    if (!(card instanceof Element) || !card.hasAttribute("data-context-card")) return;
    const open = event.newState === "open";
    for (const trigger of document.querySelectorAll(`[data-context-trigger][popovertarget="${CSS.escape(card.id)}"]`)) {
      trigger.setAttribute("aria-expanded", String(open));
    }
    if (!open && contextHovered === card) contextHovered = undefined;
  },
  true,
);

// The toggle event is queued: focus moves only if it is not already inside the card.
document.addEventListener(
  "toggle",
  (event) => {
    const card = event.target;
    if (!(card instanceof Element) || !card.hasAttribute("data-context-card")) return;
    if (event.newState !== "open" || !card.matches(":popover-open") || contextHovered === card) return;
    if (!card.contains(document.activeElement)) card.focus();
  },
  true,
);

// Escape returns focus to the trigger; Safari would return it to the page.
document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape" || !(event.target instanceof Element)) return;
  const card = event.target.closest("[data-context-card]");
  if (!card || !card.matches(":popover-open")) return;
  event.preventDefault();
  card.hidePopover();
  contextOwner(card)?.focus();
});
