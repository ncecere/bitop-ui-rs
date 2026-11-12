const ITEM = '[role="menuitem"]:not([aria-disabled="true"])';

function menuItems(menu) {
  return [...menu.querySelectorAll(ITEM)];
}

// Opening a menu moves focus to its first item, as the ARIA menu pattern requires.
document.addEventListener(
  "toggle",
  (event) => {
    const popup = event.target;
    if (!(popup instanceof HTMLElement) || event.newState !== "open") return;
    const menu = popup.matches('[role="menu"]') ? popup : popup.querySelector('[role="menu"]');
    if (menu) menuItems(menu)[0]?.focus();
  },
  true,
);

let typed = "";
let typedAt = 0;

document.addEventListener("keydown", (event) => {
  const target = event.target;
  if (!(target instanceof HTMLElement)) return;
  const menu = target.closest('[role="menu"]');
  if (!menu) return;

  const items = menuItems(menu);
  const index = items.indexOf(target.closest(ITEM));
  let next;

  switch (event.key) {
    case "ArrowDown":
      next = items[(index + 1) % items.length];
      break;
    case "ArrowUp":
      next = items[(index - 1 + items.length) % items.length];
      break;
    case "Home":
      next = items[0];
      break;
    case "End":
      next = items.at(-1);
      break;
    case "Tab":
      // A menu is one tab stop: Tab leaves it and closes it.
      menu.closest("[popover]")?.hidePopover();
      return;
    default: {
      if (event.key.length !== 1 || event.ctrlKey || event.metaKey || event.altKey) return;
      if (event.key === " " && !typed) return;
      // Typeahead: keys typed within half a second build one search string.
      const now = Date.now();
      typed = now - typedAt > 500 ? event.key : typed + event.key;
      typedAt = now;
      // Repeating one letter cycles through the items that start with it.
      const repeated = [...typed].every((char) => char === typed[0]);
      const search = (repeated ? typed[0] : typed).toLowerCase();
      const from = repeated ? index + 1 : Math.max(index, 0);
      const ordered = [...items.slice(from), ...items.slice(0, from)];
      next = ordered.find((item) => item.textContent.trim().toLowerCase().startsWith(search));
      if (!next) return;
    }
  }

  event.preventDefault();
  next?.focus();
});

// Choosing an item closes the menu. Items that open something else, such as a dialog,
// still run their own command first because this listener is on the document.
document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const item = event.target.closest(ITEM);
  if (!item || item.hasAttribute("data-keep-open")) return;
  item.closest('[role="menu"]')?.closest("[popover]")?.hidePopover();
});
