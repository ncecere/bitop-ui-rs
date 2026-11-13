// Checkbox: `indeterminate` is a property, not an attribute. The server marks such a
// checkbox with `data-indeterminate`, which draws the dash; this sets the property, so
// that assistive technology reads the box as mixed, also on markup added later.
const MIXED = "input[type=checkbox][data-indeterminate]";

function mark(root) {
  if (root.matches?.(MIXED)) root.indeterminate = true;
  for (const box of root.querySelectorAll?.(MIXED) ?? []) box.indeterminate = true;
}

mark(document);
new MutationObserver((records) => {
  for (const record of records) for (const node of record.addedNodes) if (node.nodeType === 1) mark(node);
}).observe(document.documentElement, { childList: true, subtree: true });

// A click clears the mixed state, as in the browser; the dash goes with it.
document.addEventListener("change", (event) => {
  if (event.target instanceof Element && event.target.matches(MIXED)) event.target.removeAttribute("data-indeterminate");
});
