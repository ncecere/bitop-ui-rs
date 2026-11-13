// Hover card: shown after a delay on hover and at once on keyboard focus. Unlike a
// tooltip it stays open while the pointer or the focus is inside it.

let cardTrigger;
let cardOpenTimer;
let cardCloseTimer;
// The trigger whose card Escape closed. Focus is still on it, or back on it, and must
// not open the card again until it has left.
let cardDismissed;

function cardOf(trigger) {
  return document.getElementById(trigger.dataset.hoverCard);
}

function hideCard() {
  clearTimeout(cardOpenTimer);
  clearTimeout(cardCloseTimer);
  if (!cardTrigger) return;
  const card = cardOf(cardTrigger);
  if (card?.matches(":popover-open")) card.hidePopover();
  cardTrigger.style.anchorName = "";
  cardTrigger.setAttribute("aria-expanded", "false");
  cardTrigger = undefined;
}

function showCard(trigger) {
  clearTimeout(cardOpenTimer);
  clearTimeout(cardCloseTimer);
  if (cardTrigger === trigger) return;
  hideCard();
  const card = cardOf(trigger);
  if (!card) return;
  cardTrigger = trigger;
  trigger.style.anchorName = "--bitop-hover-card";
  trigger.setAttribute("aria-expanded", "true");
  card.showPopover();
}

// The open card's trigger, when `target` is in the trigger or in the card.
function cardOwner(target) {
  if (!(target instanceof Element)) return undefined;
  const trigger = target.closest("[data-hover-card]");
  if (trigger) return trigger;
  const card = target.closest("[data-hover-card-popup]");
  return card && cardTrigger && cardOf(cardTrigger) === card ? cardTrigger : undefined;
}

document.addEventListener("pointerover", (event) => {
  // Touch has no hover.
  if (event.pointerType === "touch") return;
  const trigger = cardOwner(event.target);
  if (!trigger) return;
  clearTimeout(cardCloseTimer);
  if (trigger === cardTrigger) return;
  clearTimeout(cardOpenTimer);
  cardOpenTimer = setTimeout(() => showCard(trigger), Number(trigger.dataset.delay ?? 600));
});

document.addEventListener("pointerout", (event) => {
  const trigger = cardOwner(event.target);
  if (!trigger || cardOwner(event.relatedTarget) === trigger) return;
  clearTimeout(cardOpenTimer);
  if (trigger !== cardTrigger) return;
  clearTimeout(cardCloseTimer);
  cardCloseTimer = setTimeout(() => {
    // The keyboard is still using it.
    if (cardOwner(document.activeElement) === trigger && document.activeElement.matches(":focus-visible")) return;
    hideCard();
  }, Number(trigger.dataset.closeDelay ?? 300));
});

document.addEventListener("focusin", (event) => {
  const trigger = event.target instanceof Element && event.target.closest("[data-hover-card]");
  if (trigger && trigger !== cardDismissed && trigger.matches(":focus-visible")) showCard(trigger);
});

document.addEventListener("focusout", (event) => {
  if (event.target === cardDismissed && event.relatedTarget) cardDismissed = undefined;
  if (!cardTrigger || cardOwner(event.target) !== cardTrigger) return;
  // Focus moved between the trigger and the card, or the window lost focus.
  if (!event.relatedTarget || cardOwner(event.relatedTarget) === cardTrigger) return;
  hideCard();
});

document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape" || !cardTrigger) return;
  const trigger = cardTrigger;
  const inside = cardOf(trigger)?.contains(document.activeElement);
  hideCard();
  cardDismissed = trigger;
  if (inside) trigger.focus();
});
