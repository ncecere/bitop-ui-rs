// Save bar: opens when a field of its form changes, and closes when the form is reset
// or every field is back to the value it was rendered with.

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

// Whether a field holds something other than what the page rendered in it.
function saveBarFieldDirty(field) {
  if (field instanceof HTMLInputElement) {
    if (field.type === "checkbox" || field.type === "radio") return field.checked !== field.defaultChecked;
    if (field.type === "file") return field.files.length > 0;
    return field.value !== field.defaultValue;
  }
  if (field instanceof HTMLTextAreaElement) return field.value !== field.defaultValue;
  if (field instanceof HTMLSelectElement) return [...field.options].some((option) => option.selected !== option.defaultSelected);
  return false;
}

function saveBarFormDirty(form) {
  return [...form.elements].some((field) => !field.closest("[data-save-bar]") && saveBarFieldDirty(field));
}

for (const type of ["input", "change"]) {
  document.addEventListener(type, (event) => {
    const field = event.target;
    if (!(field instanceof Element) || field.closest("[data-save-bar]")) return;
    const form = field.closest("form");
    const bars = autoBars(form);
    if (bars.length === 0) return;
    const dirty = saveBarFormDirty(form);
    for (const bar of bars) setSaveBar(bar, dirty);
  });
}

document.addEventListener("reset", (event) => {
  for (const bar of autoBars(event.target)) setSaveBar(bar, false);
});
