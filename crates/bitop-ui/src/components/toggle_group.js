// Toggle group: shared pressed state, and one tab stop with arrow-key navigation.

const GROUP = "[data-toggle-group]";

function groupItems(group) {
  return [...group.querySelectorAll("[data-toggle]")].filter(
    (item) => item.closest(GROUP) === group && !item.disabled,
  );
}

// Makes `stop` the group's only tab stop: by default the first pressed item, or the
// first item when none is pressed.
function setTabStop(group, stop) {
  const items = groupItems(group);
  stop ??= items.find((item) => item.getAttribute("aria-pressed") === "true") ?? items[0];
  for (const item of group.querySelectorAll("[data-toggle]")) item.tabIndex = item === stop ? 0 : -1;
  return stop;
}

for (const group of document.querySelectorAll(GROUP)) setTabStop(group);

document.addEventListener("click", (event) => {
  const item = event.target instanceof Element && event.target.closest("[data-toggle]");
  const group = item && item.closest(GROUP);
  if (!group || item.disabled) return;
  const pressed = item.getAttribute("aria-pressed") !== "true";
  if (pressed && !group.hasAttribute("data-multiple")) {
    for (const other of groupItems(group)) window.bitopUi.setPressed(other, false);
  }
  window.bitopUi.setPressed(item, pressed);
  setTabStop(group, item);
});

document.addEventListener("focusin", (event) => {
  const item = event.target instanceof Element && event.target.closest("[data-toggle]");
  const group = item && item.closest(GROUP);
  if (!group) return;
  // A group the server added after the page loaded has no tab stop yet. Set it, and
  // if the keyboard brought focus in from outside, move focus to it.
  if (!item.hasAttribute("tabindex")) {
    const stop = setTabStop(group);
    if (stop !== item && !group.contains(event.relatedTarget) && item.matches(":focus-visible")) stop.focus();
  }
});

// Leaving the group puts the tab stop back on the pressed item.
document.addEventListener("focusout", (event) => {
  const group = event.target instanceof Element && event.target.closest(GROUP);
  if (group && !group.contains(event.relatedTarget)) setTabStop(group);
});

document.addEventListener("keydown", (event) => {
  const item = event.target instanceof Element && event.target.closest("[data-toggle]");
  const group = item && item.closest(GROUP);
  if (!group) return;
  const items = groupItems(group);
  const index = items.indexOf(item);
  let forward = "ArrowDown";
  let backward = "ArrowUp";
  if (group.dataset.orientation !== "vertical") {
    // In a right-to-left page the arrow keys follow the visual order.
    forward = getComputedStyle(group).direction === "rtl" ? "ArrowLeft" : "ArrowRight";
    backward = forward === "ArrowLeft" ? "ArrowRight" : "ArrowLeft";
  }
  let next;
  if (event.key === forward) next = items[(index + 1) % items.length];
  else if (event.key === backward) next = items[(index - 1 + items.length) % items.length];
  else if (event.key === "Home") next = items[0];
  else if (event.key === "End") next = items.at(-1);
  else return;
  event.preventDefault();
  setTabStop(group, next);
  next.focus();
});
