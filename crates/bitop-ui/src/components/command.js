// The command palette is a combobox in a dialog; combobox.js filters and moves through
// its list. This file adds what is specific to the palette.

// A palette starts each time with an empty search.
document.addEventListener(
  "close",
  (event) => {
    if (!(event.target instanceof HTMLDialogElement)) return;
    if (!event.target.matches("[data-command-palette]")) return;
    const { parts, filter, activate } = window.bitopUi.combobox;
    const found = parts(event.target);
    found.input.value = "";
    filter(found, "");
    activate(found, undefined);
  },
  true,
);

// Ctrl+K or Command+K, or whichever key data-shortcut names, opens the palette.
document.addEventListener("keydown", (event) => {
  if (!(event.ctrlKey || event.metaKey) || event.altKey || event.shiftKey) return;
  for (const palette of document.querySelectorAll("dialog[data-shortcut]")) {
    if (palette.dataset.shortcut.toLowerCase() !== event.key.toLowerCase()) continue;
    event.preventDefault();
    if (palette.open) palette.close();
    else palette.showModal();
    return;
  }
});
