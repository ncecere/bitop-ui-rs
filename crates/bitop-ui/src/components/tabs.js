// Tabs: click selects; arrow keys, Home and End move between tabs and select them.

function selectTab(tab) {
  const list = tab.closest('[role="tablist"]');
  for (const other of list.querySelectorAll('[role="tab"]')) {
    const selected = other === tab;
    other.setAttribute("aria-selected", String(selected));
    other.tabIndex = selected ? 0 : -1;
    const panel = document.getElementById(other.getAttribute("aria-controls"));
    if (panel) panel.hidden = !selected;
  }
}

document.addEventListener("click", (event) => {
  const tab = event.target instanceof Element && event.target.closest('[role="tab"]');
  if (tab) selectTab(tab);
});

document.addEventListener("keydown", (event) => {
  const tab = event.target instanceof Element && event.target.closest('[role="tab"]');
  if (!tab) return;
  const tabs = [...tab.closest('[role="tablist"]').querySelectorAll('[role="tab"]:not(:disabled)')];
  const index = tabs.indexOf(tab);
  // In a right-to-left page the arrow keys follow the visual order.
  const forward = getComputedStyle(tab).direction === "rtl" ? "ArrowLeft" : "ArrowRight";
  const backward = forward === "ArrowLeft" ? "ArrowRight" : "ArrowLeft";
  let next;
  if (event.key === forward) next = tabs[(index + 1) % tabs.length];
  else if (event.key === backward) next = tabs[(index - 1 + tabs.length) % tabs.length];
  else if (event.key === "Home") next = tabs[0];
  else if (event.key === "End") next = tabs.at(-1);
  else return;
  event.preventDefault();
  next.focus();
  selectTab(next);
});
