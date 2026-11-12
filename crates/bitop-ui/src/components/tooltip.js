// Tooltip: shown after a delay on hover and at once on keyboard focus.

const TOOLTIP_DELAY = 500;
let tooltipTimer;
let tooltipTrigger;

function tooltipFor(target) {
  const trigger = target instanceof Element && target.closest("[data-tooltip]");
  return trigger ? [trigger, document.getElementById(trigger.dataset.tooltip)] : [];
}

function showTooltip(trigger, tip) {
  if (!tip || tooltipTrigger === trigger) return;
  hideTooltip();
  tooltipTrigger = trigger;
  trigger.style.anchorName = "--bitop-tooltip";
  tip.showPopover();
}

function hideTooltip() {
  clearTimeout(tooltipTimer);
  if (!tooltipTrigger) return;
  const tip = document.getElementById(tooltipTrigger.dataset.tooltip);
  if (tip?.matches(":popover-open")) tip.hidePopover();
  tooltipTrigger.style.anchorName = "";
  tooltipTrigger = undefined;
}

document.addEventListener("pointerover", (event) => {
  // Touch has no hover; a tooltip there would only appear after the tap it describes.
  if (event.pointerType === "touch") return;
  const [trigger, tip] = tooltipFor(event.target);
  if (!trigger) return;
  if (trigger === tooltipTrigger) return;
  clearTimeout(tooltipTimer);
  tooltipTimer = setTimeout(() => showTooltip(trigger, tip), TOOLTIP_DELAY);
});

document.addEventListener("pointerout", (event) => {
  const [trigger] = tooltipFor(event.target);
  if (!trigger || trigger.contains(event.relatedTarget)) return;
  hideTooltip();
});

document.addEventListener("focusin", (event) => {
  const [trigger, tip] = tooltipFor(event.target);
  if (trigger && trigger.matches(":focus-visible")) showTooltip(trigger, tip);
});

document.addEventListener("focusout", hideTooltip);
// Pressing the trigger means the hint is no longer needed.
document.addEventListener("pointerdown", hideTooltip);
document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") hideTooltip();
});
