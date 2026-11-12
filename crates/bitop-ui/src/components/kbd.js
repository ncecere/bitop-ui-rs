// The server renders modifier keys the Windows and Linux way. On Apple devices, show
// the symbols people know from their keyboards.
const APPLE = { mod: "⌘", shift: "⇧", alt: "⌥", enter: "↵" };

if (/Mac|iPhone|iPad|iPod/i.test(navigator.platform || navigator.userAgent)) {
  const swap = (root) => {
    for (const key of root.querySelectorAll("[data-kbd-key]")) {
      const symbol = APPLE[key.dataset.kbdKey];
      if (symbol) key.textContent = symbol;
    }
  };
  swap(document);
  // Shortcuts in markup the server adds later.
  new MutationObserver((records) => {
    for (const record of records) {
      for (const node of record.addedNodes) if (node instanceof Element) swap(node);
    }
  }).observe(document.documentElement, { childList: true, subtree: true });
}
