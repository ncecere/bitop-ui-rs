// Breadcrumbs: each crumb's full width, as --crumb-width. It is the floor the crumb may
// be cut to when it is shorter than the floor in the stylesheet: CSS cannot say "5.5rem
// or my own width, whichever is smaller" for text on one line.

function measureCrumbs(list) {
  for (const item of list.children) {
    const text = item.firstElementChild;
    if (!text) continue;
    // The text clips what does not fit, so scrollWidth is its width uncut.
    const full = item.getBoundingClientRect().width - text.clientWidth + text.scrollWidth;
    if (full > 0) item.style.setProperty("--crumb-width", `${Math.ceil(full)}px`);
  }
}

// Measured once now and again whenever a trail's size changes.
const crumbObserver = new ResizeObserver((entries) => {
  for (const entry of entries) measureCrumbs(entry.target);
});
for (const list of document.querySelectorAll("[data-breadcrumbs] > ol")) {
  measureCrumbs(list);
  crumbObserver.observe(list);
}
