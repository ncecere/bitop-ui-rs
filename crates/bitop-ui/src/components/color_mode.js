// Colour mode controls. The mode itself is applied by color-mode.js, which is loaded in
// <head> before the page paints; this keeps the toggle and the select in step with it.

function syncColorMode() {
  const dark = document.documentElement.dataset.theme === "dark";
  for (const toggle of document.querySelectorAll("[data-color-mode-toggle]")) {
    toggle.setAttribute("aria-pressed", String(dark));
  }
  for (const select of document.querySelectorAll("[data-color-mode-select]")) {
    select.value = window.bitopUi.colorMode?.() ?? "system";
  }
}

syncColorMode();
document.addEventListener("bitop:color-mode", syncColorMode);

document.addEventListener("click", (event) => {
  const toggle = event.target instanceof Element && event.target.closest("[data-color-mode-toggle]");
  if (!toggle) return;
  window.bitopUi.setColorMode(document.documentElement.dataset.theme === "dark" ? "light" : "dark");
  syncColorMode();
});

document.addEventListener("change", (event) => {
  const select = event.target instanceof Element && event.target.closest("[data-color-mode-select]");
  if (!select) return;
  window.bitopUi.setColorMode(select.value);
  syncColorMode();
});
