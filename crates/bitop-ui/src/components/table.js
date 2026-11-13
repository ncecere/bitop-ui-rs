// Table: a wrapper that scrolls becomes a focusable region named by the caption, an
// edge that hides columns gets a shadow, and the cells of a stacked table are named
// after their columns.

const tableWatcher = typeof ResizeObserver === "undefined" ? null : new ResizeObserver((entries) => {
  for (const entry of entries) {
    const wrap = entry.target.closest("[data-table-wrap]");
    if (wrap) tableMeasure(wrap);
  }
});

function tableMeasure(wrap) {
  const across = wrap.scrollWidth - wrap.clientWidth;
  const overflowing = across > 1 || wrap.scrollHeight > wrap.clientHeight + 1;
  wrap.toggleAttribute("data-overflowing", overflowing);
  if (overflowing) {
    // A scroll container must be reachable from the keyboard (WCAG 2.1.1).
    wrap.tabIndex = 0;
    wrap.setAttribute("role", "region");
    wrap.setAttribute("aria-label", wrap.dataset.label ?? "");
  } else {
    wrap.removeAttribute("tabindex");
    wrap.removeAttribute("role");
    wrap.removeAttribute("aria-label");
  }
  // scrollLeft is negative in right-to-left.
  const position = Math.abs(wrap.scrollLeft);
  const frame = wrap.closest("[data-table-frame]");
  frame?.toggleAttribute("data-overflow-start", across > 1 && position > 1);
  frame?.toggleAttribute("data-overflow-end", across > 1 && position < across - 1);
}

// The header texts name a stacked table's cells; the first column and headers that
// are hidden visually have none.
function tableLabelCells(table) {
  const heads = [...(table.tHead?.rows[0]?.cells ?? [])].map((th, i) => (i === 0 || th.querySelector(".sr-only") ? "" : th.textContent.trim()));
  for (const body of table.tBodies) {
    for (const row of body.rows) {
      [...row.cells].forEach((cell, i) => {
        // A name the server gave is kept.
        if (cell.dataset.label !== undefined && cell.dataset.labelAuto === undefined) return;
        const label = cell.colSpan > 1 ? "" : (heads[i] ?? "");
        if (label) {
          cell.dataset.label = label;
          cell.dataset.labelAuto = "";
        } else {
          delete cell.dataset.label;
          delete cell.dataset.labelAuto;
        }
      });
    }
  }
}

function tableScan(root) {
  const wraps = [...(root.querySelectorAll?.("[data-table-wrap]") ?? [])];
  if (root.matches?.("[data-table-wrap]")) wraps.push(root);
  for (const wrap of wraps) {
    tableWatcher?.observe(wrap);
    if (wrap.firstElementChild) tableWatcher?.observe(wrap.firstElementChild);
    tableMeasure(wrap);
  }
  const stacked = new Set(root.querySelectorAll?.("table[data-stack]") ?? []);
  const inside = root.closest?.("table[data-stack]");
  if (inside) stacked.add(inside);
  for (const table of stacked) tableLabelCells(table);
}

tableScan(document);
// Tables and rows that the server adds later are picked up as they arrive.
new MutationObserver((records) => {
  for (const record of records) for (const node of record.addedNodes) if (node.nodeType === 1) tableScan(node);
}).observe(document.documentElement, { childList: true, subtree: true });

// `scroll` does not bubble: hear it from every wrapper.
document.addEventListener(
  "scroll",
  (event) => {
    if (event.target instanceof Element && event.target.matches("[data-table-wrap]")) tableMeasure(event.target);
  },
  { capture: true, passive: true },
);
