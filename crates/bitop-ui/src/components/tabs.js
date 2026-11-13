// Tabs: manual activation, as Base UI's tabs. The arrow keys, Home and End move focus
// between tabs; Enter, Space (a button's click) or a click selects the focused tab.
// An indicator slides under the selected tab, and a list that scrolls fades at the
// edge where tabs are hidden.

const TAB = '[role="tab"]';
const LIST = '[role="tablist"]';
const tabsOf = (list) => [...list.querySelectorAll(TAB)].filter((tab) => tab.closest(LIST) === list);
const usable = (tab) => tab.getAttribute("aria-disabled") !== "true" && !tab.disabled;

// Places the indicator under the selected tab, through the list's --active-tab-*
// properties. The first placement is not animated.
function tabsPlace(list) {
  const tab = tabsOf(list).find((t) => t.getAttribute("aria-selected") === "true");
  if (!tab || !list.checkVisibility()) return;
  list.style.setProperty("--active-tab-left", `${tab.offsetLeft}px`);
  list.style.setProperty("--active-tab-top", `${tab.offsetTop}px`);
  list.style.setProperty("--active-tab-width", `${tab.offsetWidth}px`);
  list.style.setProperty("--active-tab-height", `${tab.offsetHeight}px`);
  if (!list.hasAttribute("data-indicator")) {
    list.setAttribute("data-indicator", "");
    requestAnimationFrame(() => requestAnimationFrame(() => list.setAttribute("data-indicator", "ready")));
  }
}

// Marks a list wider than its box, and the edges where tabs are hidden.
function tabsEdges(list) {
  const max = list.scrollWidth - list.clientWidth;
  // scrollLeft is negative in right-to-left.
  const position = Math.abs(list.scrollLeft);
  const overflowing = max > 1;
  list.toggleAttribute("data-overflowing", overflowing);
  list.toggleAttribute("data-overflow-start", overflowing && position > 1);
  list.toggleAttribute("data-overflow-end", overflowing && position < max - 1);
}

const tabsWatcher =
  typeof ResizeObserver === "undefined"
    ? null
    : new ResizeObserver((entries) => {
        for (const entry of entries) {
          const list = entry.target.closest(LIST);
          if (!list) continue;
          tabsEdges(list);
          tabsPlace(list);
        }
      });

function tabsScan(root) {
  const lists = [...(root.querySelectorAll?.(LIST) ?? [])];
  if (root.matches?.(LIST)) lists.push(root);
  for (const list of lists) {
    if (!list.hasAttribute("data-variant")) continue;
    tabsWatcher?.observe(list);
    for (const tab of tabsOf(list)) tabsWatcher?.observe(tab);
    tabsEdges(list);
    tabsPlace(list);
  }
}

tabsScan(document);
new MutationObserver((records) => {
  for (const record of records) for (const node of record.addedNodes) if (node.nodeType === 1) tabsScan(node);
}).observe(document.documentElement, { childList: true, subtree: true });
// Web fonts change the tabs' widths once they load.
document.fonts?.ready.then(() => tabsScan(document));

document.addEventListener("scroll", (event) => {
  if (event.target instanceof Element && event.target.matches(LIST)) tabsEdges(event.target);
}, { capture: true, passive: true });

function selectTab(tab) {
  const list = tab.closest(LIST);
  if (tab.getAttribute("aria-selected") === "true") return;
  for (const other of tabsOf(list)) {
    const selected = other === tab;
    other.setAttribute("aria-selected", String(selected));
    other.tabIndex = selected ? 0 : -1;
    const panel = document.getElementById(other.getAttribute("aria-controls"));
    if (panel) panel.hidden = !selected;
  }
  tabsPlace(list);
  list.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: tab.getAttribute("aria-controls") }));
}

document.addEventListener("click", (event) => {
  const tab = event.target instanceof Element && event.target.closest(TAB);
  if (tab && usable(tab)) selectTab(tab);
});

// The focused tab is the one in the tab order, so that Tab returns to it, as in
// Base UI, also when it is not the selected one.
document.addEventListener("focusin", (event) => {
  const tab = event.target instanceof Element && event.target.closest(TAB);
  if (!tab) return;
  for (const other of tabsOf(tab.closest(LIST))) other.tabIndex = other === tab ? 0 : -1;
});

document.addEventListener("keydown", (event) => {
  const tab = event.target instanceof Element && event.target.closest(TAB);
  if (!tab || event.altKey || event.ctrlKey || event.metaKey) return;
  const list = tab.closest(LIST);
  const tabs = tabsOf(list).filter(usable);
  const index = tabs.indexOf(tab);
  // A vertical list uses the up and down keys; a right-to-left page reverses left and right.
  const vertical = list.getAttribute("aria-orientation") === "vertical";
  const rtl = getComputedStyle(list).direction === "rtl";
  const forward = vertical ? "ArrowDown" : rtl ? "ArrowLeft" : "ArrowRight";
  const backward = vertical ? "ArrowUp" : rtl ? "ArrowRight" : "ArrowLeft";
  let next;
  if (event.key === forward) next = tabs[(index + 1) % tabs.length];
  else if (event.key === backward) next = tabs[(index - 1 + tabs.length) % tabs.length];
  else if (event.key === "Home") next = tabs[0];
  else if (event.key === "End") next = tabs.at(-1);
  else if ((event.key === "Enter" || event.key === " ") && !usable(tab)) {
    event.preventDefault();
    return;
  } else return;
  event.preventDefault();
  if (!next) return;
  next.focus();
  // Focus leaves a tab that is partly in view where it is: bring it in, as Base UI does.
  next.scrollIntoView({ block: "nearest", inline: "nearest" });
});
