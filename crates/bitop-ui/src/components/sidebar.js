// Sidebar: an icon rail when collapsed on a wide window, a modal drawer on a narrow one.

const narrowQuery = matchMedia("(max-width: 37.5rem)");
const LAYOUT = "[data-sidebar-layout]";
const TABBABLE = 'a[href], button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

const drawerOf = (layout) => layout.querySelector(":scope > [data-sidebar-drawer]");
const togglesOf = (layout) => {
  const drawer = drawerOf(layout);
  return drawer ? document.querySelectorAll(`[data-sidebar-toggle][popovertarget="${CSS.escape(drawer.id)}"]`) : [];
};

// The toggles' labels and aria-expanded, and the drawer's dialog role, follow the
// window's width and the state.
function syncSidebar(layout) {
  const drawer = drawerOf(layout);
  if (!drawer) return;
  const narrow = narrowQuery.matches;
  const open = drawer.matches(":popover-open");
  if (narrow) {
    drawer.setAttribute("role", "dialog");
    drawer.setAttribute("aria-modal", "true");
    drawer.setAttribute("aria-label", drawer.dataset.label ?? "Navigation");
  } else {
    drawer.removeAttribute("role");
    drawer.removeAttribute("aria-modal");
    drawer.removeAttribute("aria-label");
  }
  const collapsed = layout.hasAttribute("data-collapsed");
  for (const toggle of togglesOf(layout)) {
    const { labelOpen, labelClose, labelExpand, labelCollapse } = toggle.dataset;
    const label = narrow ? (open ? labelClose : labelOpen) : collapsed ? labelExpand : labelCollapse;
    if (label) toggle.setAttribute("aria-label", label);
    toggle.setAttribute("aria-expanded", String(narrow ? open : !collapsed));
  }
}

const syncAll = () => document.querySelectorAll(LAYOUT).forEach(syncSidebar);

// Wide: the toggle collapses the sidebar to the rail, and the choice is remembered.
document.addEventListener("click", (event) => {
  const toggle = event.target instanceof Element && event.target.closest("[data-sidebar-toggle]");
  if (!toggle || narrowQuery.matches) return;
  const drawer = document.getElementById(toggle.getAttribute("popovertarget") ?? "");
  const layout = drawer?.closest(LAYOUT);
  if (!layout) return;
  // Stop the button from opening the sidebar as a popover.
  event.preventDefault();
  hideRailTip();
  const collapsed = layout.toggleAttribute("data-collapsed");
  const cookie = layout.dataset.sidebarCookie;
  if (cookie) document.cookie = `${cookie}=${collapsed ? "collapsed" : "expanded"}; path=/; max-age=31536000; samesite=lax`;
  syncSidebar(layout);
  revealCurrent(layout);
});

// ---------------- Drawer ----------------

// While the drawer is open the rest of the page is inert, as under a modal dialog.
const inerted = new Map();

function trap(drawer) {
  const made = [];
  for (let node = drawer; node && node !== document.body; node = node.parentElement) {
    for (const sibling of node.parentElement?.children ?? []) {
      if (sibling === node || sibling.inert || sibling.matches("script, template, [popover]:popover-open")) continue;
      sibling.inert = true;
      made.push(sibling);
    }
  }
  inerted.set(drawer, made);
}

function release(drawer) {
  for (const element of inerted.get(drawer) ?? []) element.inert = false;
  inerted.delete(drawer);
}

const closings = new WeakMap();

const tabbables = (root) => [...root.querySelectorAll(TABBABLE)].filter((element) => element.checkVisibility() && !element.closest("[inert]"));

document.addEventListener("beforetoggle", (event) => {
  const drawer = event.target;
  if (!(drawer instanceof Element) || !drawer.matches("[data-sidebar-drawer]")) return;
  if (event.newState === "open") {
    delete drawer.dataset.followed;
    trap(drawer);
  } else {
    release(drawer);
    // Keeps the closing transition, which the stylesheet only has while this is set.
    drawer.dataset.closing = "";
    clearTimeout(closings.get(drawer));
    closings.set(drawer, setTimeout(() => delete drawer.dataset.closing, 400));
  }
  // State the coming state for the toggles now; the popover reports it once done.
  const layout = drawer.closest(LAYOUT);
  for (const toggle of layout ? togglesOf(layout) : []) {
    toggle.setAttribute("aria-expanded", String(event.newState === "open"));
    const label = event.newState === "open" ? toggle.dataset.labelClose : toggle.dataset.labelOpen;
    if (label) toggle.setAttribute("aria-label", label);
  }
}, true);

document.addEventListener("toggle", (event) => {
  const drawer = event.target;
  if (!(drawer instanceof Element) || !drawer.matches("[data-sidebar-drawer]")) return;
  const layout = drawer.closest(LAYOUT);
  if (event.newState === "open") {
    // The event is queued: leave focus alone if it is inside already.
    if (drawer.matches(":popover-open") && !drawer.contains(document.activeElement)) {
      const current = drawer.querySelector('[aria-current="page"]');
      (current && current.checkVisibility() ? current : tabbables(drawer)[0] ?? drawer).focus();
    }
    if (layout) revealCurrent(layout);
  } else if (drawer.hasAttribute("data-followed")) {
    // After a link to this page, focus goes to the content, not back to the toggle.
    delete drawer.dataset.followed;
    const content = layout && document.getElementById(layout.dataset.sidebarContent ?? "");
    content?.focus({ preventScroll: true });
  }
}, true);

// Tab stays in the open drawer.
document.addEventListener("keydown", (event) => {
  if (event.key !== "Tab" || event.defaultPrevented) return;
  const drawer = document.querySelector("[data-sidebar-drawer]:popover-open");
  if (!drawer) return;
  const items = tabbables(drawer);
  if (!items.length) return;
  const first = items[0];
  const last = items[items.length - 1];
  const active = document.activeElement;
  // A menu opened from the drawer is inside it and handles its own keys.
  if (!drawer.contains(active)) {
    event.preventDefault();
    first.focus();
  } else if (event.shiftKey && (active === first || active === drawer)) {
    event.preventDefault();
    last.focus();
  } else if (!event.shiftKey && active === last) {
    event.preventDefault();
    first.focus();
  }
});

// Following a link in the drawer closes it. A modified click opens a new tab and leaves
// it open.
document.addEventListener("click", (event) => {
  const link = event.target instanceof Element && event.target.closest("a[href]");
  const drawer = link && link.closest("[data-sidebar-drawer]");
  if (!drawer || !drawer.matches(":popover-open") || event.metaKey || event.ctrlKey || event.shiftKey || event.button !== 0) return;
  drawer.dataset.followed = "";
  drawer.hidePopover();
});

narrowQuery.addEventListener("change", () => {
  // Widening the window closes the drawer; the rail takes over.
  for (const drawer of document.querySelectorAll("[data-sidebar-drawer]")) {
    if (!narrowQuery.matches && drawer.matches(":popover-open")) drawer.hidePopover();
  }
  hideRailTip();
  syncAll();
});

// ---------------- Collapsible sections ----------------

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-sidebar-section]");
  if (!button) return;
  const list = document.getElementById(button.getAttribute("aria-controls") ?? "");
  if (!list) return;
  const open = button.getAttribute("aria-expanded") !== "true";
  button.setAttribute("aria-expanded", String(open));
  list.hidden = !open;
  button.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { open } }));
  if (open) {
    const layout = button.closest(LAYOUT);
    if (layout) revealCurrent(layout);
    else revealIn(button.closest("[data-sidebar-scroll]"));
  }
});

// ---------------- Rail tooltips ----------------

const isRail = (item) => {
  const layout = item.closest(LAYOUT);
  return Boolean(layout?.hasAttribute("data-collapsed")) && !narrowQuery.matches && !item.closest(":popover-open");
};

let railTimer;
let railItem;

function showRailTip(item) {
  const tip = item.closest("[data-sidebar]")?.querySelector(":scope > [data-sidebar-tip]");
  const label = item.querySelector("[data-sidebar-label]")?.textContent.trim();
  if (!tip || !label) return;
  hideRailTip();
  railItem = item;
  item.style.anchorName = "--bitop-sidebar-tip";
  tip.textContent = label;
  tip.showPopover();
}

function hideRailTip() {
  clearTimeout(railTimer);
  if (!railItem) return;
  const tip = railItem.closest("[data-sidebar]")?.querySelector(":scope > [data-sidebar-tip]");
  if (tip?.matches(":popover-open")) tip.hidePopover();
  railItem.style.anchorName = "";
  railItem = undefined;
}

const railItemOf = (target) => {
  const item = target instanceof Element && target.closest("[data-sidebar-item]");
  return item && isRail(item) ? item : null;
};

document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch") return;
  const item = railItemOf(event.target);
  if (!item || item === railItem) return;
  clearTimeout(railTimer);
  // Moving from one item to the next shows the next at once.
  if (railItem) showRailTip(item);
  else railTimer = setTimeout(() => showRailTip(item), 600);
});

document.addEventListener("pointerout", (event) => {
  const item = event.target instanceof Element && event.target.closest("[data-sidebar-item]");
  if (!item || item.contains(event.relatedTarget)) return;
  const next = railItemOf(event.relatedTarget);
  if (next) return;
  hideRailTip();
});

document.addEventListener("focusin", (event) => {
  const item = railItemOf(event.target);
  if (item && item.matches(":focus-visible")) showRailTip(item);
});
document.addEventListener("focusout", hideRailTip);
document.addEventListener("pointerdown", hideRailTip);
document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") hideRailTip();
});

// ---------------- Scrolling content ----------------

// When the content overflows it becomes a focusable region, so the keyboard can scroll it.
function syncScroll(scroller) {
  const overflows = scroller.scrollHeight > scroller.clientHeight + 1;
  if (overflows) {
    scroller.setAttribute("role", "region");
    scroller.setAttribute("aria-label", scroller.dataset.label ?? "Sidebar navigation");
    scroller.tabIndex = 0;
  } else {
    scroller.removeAttribute("role");
    scroller.removeAttribute("aria-label");
    scroller.removeAttribute("tabindex");
  }
}

// Keeps the current page's item in view. It scrolls the content itself rather than
// calling scrollIntoView(): Chrome moves the sequential focus starting point to an
// element scrolled into view, so the first Tab would not reach the skip link.
function revealIn(scroller) {
  const item = scroller?.querySelector('[aria-current="page"]');
  if (!item || !item.checkVisibility()) return;
  const view = scroller.getBoundingClientRect();
  const box = item.getBoundingClientRect();
  const above = view.top - box.top;
  const below = box.bottom - view.bottom;
  if (above > 0.5) scroller.scrollTop -= above;
  // Down far enough to show its bottom edge, but never past its top.
  else if (below > 0.5) scroller.scrollTop += Math.min(below, -above);
}

function revealCurrent(layout) {
  // After the rail's width transition the items have their final size.
  requestAnimationFrame(() => {
    for (const scroller of layout.querySelectorAll("[data-sidebar-scroll]")) revealIn(scroller);
  });
}

const resizes = new ResizeObserver((entries) => {
  for (const { target } of entries) {
    const scroller = target.matches("[data-sidebar-scroll]") ? target : target.parentElement;
    if (scroller) syncScroll(scroller);
  }
});

// Another item becoming the current page (a script that marks it) is revealed too.
const marks = new MutationObserver((records) => {
  for (const record of records) {
    if (record.target instanceof Element && record.target.getAttribute("aria-current") === "page") {
      revealIn(record.target.closest("[data-sidebar-scroll]"));
    }
  }
});

for (const scroller of document.querySelectorAll("[data-sidebar-scroll]")) {
  resizes.observe(scroller);
  marks.observe(scroller, { subtree: true, attributes: true, attributeFilter: ["aria-current"] });
  if (scroller.firstElementChild) resizes.observe(scroller.firstElementChild);
  syncScroll(scroller);
  revealIn(scroller);
}

syncAll();
