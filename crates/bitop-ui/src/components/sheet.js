// Sheet: a body that scrolls with nothing focusable in it could not be scrolled from the
// keyboard, so it takes focus itself, as a region named by the sheet's title.

const FOCUSABLE = "a[href], button:not([disabled]), input, select, textarea, [tabindex]:not([tabindex='-1'])";

function markScrollable(body) {
  const scrolls = body.scrollHeight > body.clientHeight + 1 && !body.querySelector(FOCUSABLE);
  if (scrolls === body.hasAttribute("data-sheet-scrolls")) return;
  body.toggleAttribute("data-sheet-scrolls", scrolls);
  if (scrolls) {
    body.tabIndex = 0;
    body.setAttribute("role", "region");
    body.setAttribute("aria-labelledby", body.dataset.sheetTitle);
  } else {
    body.removeAttribute("tabindex");
    body.removeAttribute("role");
    body.removeAttribute("aria-labelledby");
  }
}

// Opening a modal dialog always moves focus into it, in every engine, so this is where
// an opened sheet is first seen.
document.addEventListener("focusin", (event) => {
  const sheet = event.target instanceof Element && event.target.closest("dialog[data-sheet]");
  if (!sheet) return;
  for (const body of sheet.querySelectorAll(":scope > [data-sheet-body]")) markScrollable(body);
});

addEventListener("resize", () => {
  for (const body of document.querySelectorAll("dialog[data-sheet][open] > [data-sheet-body]")) markScrollable(body);
});
