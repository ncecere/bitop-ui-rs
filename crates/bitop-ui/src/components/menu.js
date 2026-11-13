// Menu: the ARIA menu keyboard pattern, checkbox and radio items, and submenus. The
// same items are used by the context menu and the menubar.

const ITEM =
  ':is([role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"]):not([aria-disabled="true"])';
const SUB_TRIGGER = "[data-submenu-trigger]";

// The items of this menu, without those of a submenu inside it.
function menuItems(menu) {
  return [...menu.querySelectorAll(ITEM)].filter((item) => item.closest('[role="menu"]') === menu);
}

function menuOf(popup) {
  return popup.matches('[role="menu"]') ? popup : popup.querySelector('[role="menu"]');
}

// The outermost popover around a menu. Closing it closes its submenus too.
function rootPopup(element) {
  let popup = element.closest("[popover]");
  while (popup?.matches("[data-submenu]")) {
    const parent = popup.parentElement?.closest("[popover]");
    if (!parent) break;
    popup = parent;
  }
  return popup;
}

function submenuOf(trigger) {
  return document.getElementById(trigger.getAttribute("popovertarget"));
}

function submenuTrigger(popup) {
  const trigger = popup.previousElementSibling;
  return trigger?.matches(SUB_TRIGGER) ? trigger : undefined;
}

// Submenus that the pointer opened: focus stays on their item.
const openedByPointer = new WeakSet();

// Opens a submenu, with focus on its first item when the keyboard asked for it. The
// click goes through the item, so that the browser anchors the popup to it.
function openSubmenu(trigger, focusFirst) {
  const popup = submenuOf(trigger);
  if (!popup) return;
  if (popup.matches(":popover-open")) {
    if (focusFirst) menuItems(menuOf(popup))[0]?.focus();
    return;
  }
  if (!focusFirst) openedByPointer.add(popup);
  trigger.focus();
  trigger.click();
}

function setChecked(item, checked) {
  item.setAttribute("aria-checked", String(checked));
  const input = item.querySelector('input[type="hidden"]');
  if (input) input.disabled = !checked;
}

// `focusMenu` marks a menu that is about to open from the pointer: focus goes to the
// menu itself, not to its first item.
window.bitopUi.menu = { items: menuItems, setChecked, focusMenu: (popup) => openedByClick.add(popup) };

// The buttons that open a popup: a menu's trigger, a submenu's item, a menubar's menu.
function triggersOf(popup) {
  return popup.id ? document.querySelectorAll(`[popovertarget="${CSS.escape(popup.id)}"]`) : [];
}

// A menu's trigger that is not itself an item of a menu or a menubar.
const MENU_TRIGGER = '[aria-haspopup="menu"][popovertarget]:not([role="menuitem"])';

// Menus opened by a click of the pointer on their trigger: focus goes to the menu
// itself, not its first item, as upstream. Menus opened by ArrowUp: their last item.
const openedByClick = new WeakSet();
const openOnLast = new WeakSet();

// beforetoggle is sent as the popover opens or closes, so aria-expanded is never behind.
document.addEventListener(
  "beforetoggle",
  (event) => {
    const popup = event.target;
    if (!(popup instanceof HTMLElement) || !menuOf(popup)) return;
    for (const trigger of triggersOf(popup)) {
      if (trigger.hasAttribute("aria-expanded")) {
        trigger.setAttribute("aria-expanded", String(event.newState === "open"));
      }
    }
  },
  true,
);

// Opening a menu moves focus to its first item, as the ARIA menu pattern requires.
// The toggle event is queued, so it can arrive after a script or the keyboard has
// already moved focus into the menu; leave focus alone then, or keys typed in that
// moment are undone.
document.addEventListener(
  "toggle",
  (event) => {
    const popup = event.target;
    if (!(popup instanceof HTMLElement)) return;
    if (event.newState !== "open") {
      openedByClick.delete(popup);
      openOnLast.delete(popup);
      return;
    }
    if (openedByPointer.delete(popup)) return;
    const menu = menuOf(popup);
    if (!menu || menu.contains(document.activeElement)) return;
    if (openedByClick.delete(popup)) menu.focus({ preventScroll: true });
    else if (openOnLast.delete(popup)) menuItems(menu).at(-1)?.focus();
    else menuItems(menu)[0]?.focus();
  },
  true,
);

// A click of the pointer on a menu's trigger (Enter and Space send one with no detail).
document.addEventListener(
  "click",
  (event) => {
    const trigger =
      event.target instanceof Element && event.target.closest(`${MENU_TRIGGER}, [data-menubar-trigger]`);
    if (!trigger || event.detail === 0) return;
    const popup = document.getElementById(trigger.getAttribute("popovertarget"));
    if (popup && menuOf(popup) && !popup.matches(":popover-open")) openedByClick.add(popup);
  },
  true,
);

// ArrowDown on a menu's trigger opens the menu on its first item, ArrowUp on its last.
document.addEventListener("keydown", (event) => {
  if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
  if (event.ctrlKey || event.metaKey || event.altKey || event.defaultPrevented) return;
  const trigger = event.target instanceof Element && event.target.closest(MENU_TRIGGER);
  if (!trigger || trigger.closest('[role="menu"], [role="menubar"]')) return;
  const popup = document.getElementById(trigger.getAttribute("popovertarget"));
  const menu = popup && menuOf(popup);
  if (!menu || trigger.disabled) return;
  event.preventDefault();
  if (popup.matches(":popover-open")) {
    const items = menuItems(menu);
    (event.key === "ArrowUp" ? items.at(-1) : items[0])?.focus();
    return;
  }
  if (event.key === "ArrowUp") openOnLast.add(popup);
  trigger.click();
});

let typed = "";
let typedAt = 0;

document.addEventListener("keydown", (event) => {
  const target = event.target;
  if (!(target instanceof HTMLElement)) return;
  const menu = target.closest('[role="menu"]');
  if (!menu) return;

  const items = menuItems(menu);
  const item = target.closest(ITEM);
  const index = items.indexOf(item);
  const popup = menu.closest("[popover]");
  // The arrow key that points at a submenu opens it and the other one closes it.
  const rtl = getComputedStyle(menu).direction === "rtl";
  const openKey = rtl ? "ArrowLeft" : "ArrowRight";
  const closeKey = rtl ? "ArrowRight" : "ArrowLeft";
  let next;

  if (event.key === openKey && item?.matches(SUB_TRIGGER)) {
    event.preventDefault();
    openSubmenu(item, true);
    return;
  }
  if ((event.key === closeKey || event.key === "Escape") && popup?.matches("[data-submenu]")) {
    // Escape closes one level; the browser would return focus to what had it before
    // the submenu opened, which is not its item when the pointer opened it.
    event.preventDefault();
    popup.hidePopover();
    submenuTrigger(popup)?.focus();
    return;
  }

  if (
    event.key === "Escape" &&
    popup &&
    !popup.matches("[data-menubar-popup], [data-context-menu-popup]")
  ) {
    // Safari does not focus a button when it is clicked, so the browser would return
    // focus to the page: close here and focus the trigger.
    const trigger = [...triggersOf(popup)].find((other) => other.matches(MENU_TRIGGER));
    if (trigger) {
      event.preventDefault();
      popup.hidePopover();
      trigger.focus();
      return;
    }
  }

  switch (event.key) {
    case "ArrowDown":
      next = items[(index + 1) % items.length];
      break;
    case "ArrowUp":
      next = index === -1 ? items.at(-1) : items[(index - 1 + items.length) % items.length];
      break;
    case "Home":
      next = items[0];
      break;
    case "End":
      next = items.at(-1);
      break;
    case "Tab":
      // A menu is one tab stop: Tab leaves it and closes it.
      rootPopup(menu)?.hidePopover();
      return;
    default: {
      if (event.key.length !== 1 || event.ctrlKey || event.metaKey || event.altKey) return;
      // Typeahead: keys typed within half a second build one search string. A space
      // is part of it only when one is being typed; otherwise it chooses the item.
      const now = Date.now();
      if (now - typedAt > 500) typed = "";
      if (event.key === " " && !typed) return;
      typed += event.key;
      typedAt = now;
      // Repeating one letter cycles through the items that start with it.
      const repeated = [...typed].every((char) => char === typed[0]);
      const search = (repeated ? typed[0] : typed).toLowerCase();
      const from = repeated ? index + 1 : Math.max(index, 0);
      const ordered = [...items.slice(from), ...items.slice(0, from)];
      next = ordered.find((other) => other.textContent.trim().toLowerCase().startsWith(search));
      if (!next) return;
    }
  }

  event.preventDefault();
  next?.focus();
});

// Choosing an action closes the menu, submenus and all. Items that open something
// else, such as a dialog, still run their own command first because this listener is
// on the document. Checkbox and radio items change and keep the menu open.
document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  // A disabled item does nothing: no link is followed and no submenu opens.
  if (event.target.closest('[role^="menuitem"][aria-disabled="true"]')) {
    event.preventDefault();
    return;
  }
  const item = event.target.closest(ITEM);
  const menu = item?.closest('[role="menu"]');
  if (!menu) return;

  if (item.matches(SUB_TRIGGER)) {
    // The button would close a submenu that the pointer has already opened.
    const popup = submenuOf(item);
    if (popup?.matches(":popover-open")) {
      event.preventDefault();
      // Enter and Space send a click with no detail.
      if (event.detail === 0) menuItems(menuOf(popup))[0]?.focus();
    }
    return;
  }

  const role = item.getAttribute("role");
  if (role === "menuitemcheckbox") {
    setChecked(item, item.getAttribute("aria-checked") !== "true");
    item.dispatchEvent(new Event("change", { bubbles: true }));
    return;
  }
  if (role === "menuitemradio") {
    if (item.getAttribute("aria-checked") === "true") return;
    const group = item.closest("[data-menu-radio-group]") ?? menu;
    for (const other of group.querySelectorAll('[role="menuitemradio"]')) setChecked(other, other === item);
    item.dispatchEvent(new Event("change", { bubbles: true }));
    return;
  }

  if (item.hasAttribute("data-keep-open")) return;
  rootPopup(menu)?.hidePopover();
});

// Pointing at a submenu's item opens it; pointing at another item of the same menu
// closes it. Both wait a moment, so that the pointer can cross other items on its way
// into the submenu.
let hoverTimer;

document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const item = event.target.closest(ITEM);
  const menu = item?.closest('[role="menu"]');
  clearTimeout(hoverTimer);
  if (!menu) return;
  // Only the submenus that are open now: one that the keyboard opens while the timer
  // runs stays open.
  const siblings = menuItems(menu).filter(
    (other) => other !== item && other.matches(`${SUB_TRIGGER}[aria-expanded="true"]`),
  );
  if (!siblings.length && !item.matches(SUB_TRIGGER)) return;
  hoverTimer = setTimeout(() => {
    if (!item.isConnected || !menu.closest("[popover]")?.matches(":popover-open")) return;
    for (const other of siblings) {
      const popup = submenuOf(other);
      if (popup?.matches(":popover-open")) popup.hidePopover();
    }
    if (item.matches(SUB_TRIGGER)) openSubmenu(item, false);
  }, 120);
});

// The pointer moves the highlight, which is focus: pointing at an item focuses it, and
// pointing at a disabled item or leaving the items focuses the menu, so that one item
// at most is highlighted.
const ANY_ITEM = '[role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"]';

document.addEventListener("pointermove", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const item = event.target.closest(ANY_ITEM);
  const menu = item?.closest('[role="menu"]');
  if (!menu) return;
  const next = item.matches(ITEM) ? item : menu;
  if (next !== document.activeElement) next.focus({ preventScroll: true });
});

document.addEventListener("pointerout", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const item = event.target.closest(ANY_ITEM);
  if (!item || item !== document.activeElement) return;
  const to = event.relatedTarget;
  if (to instanceof Element && (item.contains(to) || to.closest(ANY_ITEM))) return;
  item.closest('[role="menu"]')?.focus({ preventScroll: true });
});
