// Save bar: opens when a field of its form changes and closes when the form is reset.

function setSaveBar(bar, open) {
  if (bar.hasAttribute("data-open") === open) return;
  // "now" marks a bar that opened on the page, which rises into place.
  if (open) bar.setAttribute("data-open", "now");
  else bar.removeAttribute("data-open");
  // The message is a live region: putting the text in announces it.
  const [message, actions] = bar.children;
  message.textContent = open ? bar.dataset.message : "";
  actions.hidden = !open;
  bar.dispatchEvent(new CustomEvent("bitop:toggle", { bubbles: true, detail: { open } }));
}

window.bitopUi.setSaveBar = setSaveBar;

function autoBars(form) {
  return form instanceof HTMLFormElement ? form.querySelectorAll('[data-save-bar="auto"]') : [];
}

for (const type of ["input", "change"]) {
  document.addEventListener(type, (event) => {
    const field = event.target;
    if (!(field instanceof Element) || field.closest("[data-save-bar]")) return;
    for (const bar of autoBars(field.closest("form"))) setSaveBar(bar, true);
  });
}

document.addEventListener("reset", (event) => {
  for (const bar of autoBars(event.target)) setSaveBar(bar, false);
});
