// Terminal: the output follows new output to the bottom while the reader is there,
// aria-busy follows the root's data-streaming, and the clear button empties the output.

// Whether each output was at the bottom when it last scrolled.
const terminalPinned = new WeakMap();

function terminalPin(content) {
  if (content.hasAttribute("data-auto-scroll") && terminalPinned.get(content) !== false) {
    content.scrollTop = content.scrollHeight;
  }
}

document.addEventListener(
  "scroll",
  (event) => {
    const content = event.target;
    if (!(content instanceof Element) || !content.hasAttribute("data-terminal-content")) return;
    terminalPinned.set(content, content.scrollHeight - content.scrollTop - content.clientHeight < 24);
  },
  { capture: true },
);

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-terminal-clear]");
  if (!button) return;
  const root = button.closest("[data-terminal]");
  if (!root) return;
  const proceed = root.dispatchEvent(new CustomEvent("bitop:terminal-clear", { bubbles: true, cancelable: true }));
  if (!proceed) return;
  root.querySelector("[data-terminal-output]")?.replaceChildren();
  const content = root.querySelector("[data-terminal-content]");
  if (content) terminalPinned.set(content, true);
});

function followTerminalStreaming(root) {
  const content = root.querySelector("[data-terminal-content]");
  if (!content) return;
  if (root.hasAttribute("data-streaming")) content.setAttribute("aria-busy", "true");
  else content.removeAttribute("aria-busy");
  terminalPin(content);
}

document.querySelectorAll("[data-terminal-content]").forEach(terminalPin);

new MutationObserver((records) => {
  const touched = new Set();
  for (const record of records) {
    const target = record.target instanceof Element ? record.target : record.target.parentElement;
    if (record.type === "attributes") {
      if (target?.matches("[data-terminal]")) followTerminalStreaming(target);
      continue;
    }
    const content = target?.closest("[data-terminal-content]");
    if (content) touched.add(content);
    for (const node of record.addedNodes) {
      if (node instanceof Element) node.querySelectorAll("[data-terminal-content]").forEach((c) => touched.add(c));
    }
  }
  touched.forEach(terminalPin);
}).observe(document.documentElement, {
  subtree: true,
  childList: true,
  characterData: true,
  attributes: true,
  attributeFilter: ["data-streaming"],
});
