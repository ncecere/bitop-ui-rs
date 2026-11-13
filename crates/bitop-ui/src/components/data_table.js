// Data table: row selection with a select-all checkbox and a bulk bar, a columns menu
// that shows and hides columns, and rows that open their record on a click.

const dataTableRows = (root) => [...root.querySelectorAll("input[data-row-select]:not(:disabled)")];

// Keeps the rows' tint, the header checkbox, the bulk bar and the summary in step
// with the checkboxes.
function dataTableSyncSelection(root) {
  const boxes = dataTableRows(root);
  const count = boxes.filter((box) => box.checked).length;
  for (const box of root.querySelectorAll("input[data-row-select]")) {
    box.closest("tr")?.toggleAttribute("data-selected", box.checked);
  }
  const all = root.querySelector("input[data-select-all]");
  if (all) {
    all.checked = boxes.length > 0 && count === boxes.length;
    all.indeterminate = count > 0 && count < boxes.length;
  }
  const fill = (element) => {
    if (!element) return;
    const total = Number(root.dataset.rowCount) || boxes.length;
    const text = element.dataset.template.replaceAll("{count}", String(count)).replaceAll("{total}", String(total));
    if (element.textContent.trim() !== text) element.textContent = text;
  };
  fill(root.querySelector("[data-selected-count]"));
  fill(root.querySelector("[data-selection-summary]"));
}

document.addEventListener("change", (event) => {
  const target = event.target;
  if (!(target instanceof Element)) return;
  const root = target.closest("[data-data-table]");
  if (!root) return;
  if (target.matches("input[data-select-all]")) {
    for (const box of dataTableRows(root)) box.checked = target.checked;
    dataTableSyncSelection(root);
  } else if (target.matches("input[data-row-select]")) {
    dataTableSyncSelection(root);
  } else if (target.matches("[data-column-toggle]")) {
    const hidden = dataTableHidden(root);
    if (target.getAttribute("aria-checked") === "true") hidden.delete(target.dataset.columnToggle);
    else hidden.add(target.dataset.columnToggle);
    dataTableApplyColumns(root, hidden);
    const key = root.dataset.columnsKey;
    if (key) {
      try {
        localStorage.setItem(key, JSON.stringify([...hidden]));
      } catch {
        // Storage can be turned off; the choice then lasts for this page.
      }
    }
  }
});

document.addEventListener("click", (event) => {
  const target = event.target;
  if (!(target instanceof Element)) return;
  const clear = target.closest("[data-clear-selection]");
  if (clear) {
    const root = clear.closest("[data-data-table]");
    if (!root) return;
    for (const box of dataTableRows(root)) box.checked = false;
    dataTableSyncSelection(root);
    // The bar, and the button in it, are gone: focus goes to the header checkbox.
    root.querySelector("input[data-select-all]")?.focus();
    return;
  }
  // A click on a row that opens, but not on a control in it, follows the row's link.
  const row = target.closest("[data-data-table] tr[data-clickable]");
  if (!row || event.defaultPrevented) return;
  // Only a control inside the row counts: the page around it may be focusable too (a
  // main landmark with tabindex="-1").
  const control = target.closest("a, button, input, select, textarea, label, summary, [role=menu], [popover], [tabindex]");
  if (control && row.contains(control)) return;
  if (window.getSelection()?.toString()) return;
  row.querySelector("a[data-row-open]")?.click();
});

// A form reset unchecks the boxes after the event, so look again a moment later.
document.addEventListener("reset", (event) => {
  const form = event.target;
  setTimeout(() => {
    for (const root of document.querySelectorAll("[data-data-table]")) {
      if (form.contains(root) || root.contains(form)) dataTableSyncSelection(root);
    }
  });
});

function dataTableHidden(root) {
  return new Set([...root.querySelectorAll("th[data-column][hidden]")].map((th) => th.dataset.column));
}

function dataTableApplyColumns(root, hidden) {
  const table = root.querySelector("table");
  const heads = [...(table?.tHead?.rows[0]?.cells ?? [])];
  heads.forEach((th, index) => {
    if (!th.dataset.column) return;
    const hide = hidden.has(th.dataset.column);
    th.hidden = hide;
    for (const body of table.tBodies) {
      for (const row of body.rows) {
        const cell = row.cells[index];
        if (cell && cell.colSpan === 1 && row.cells.length === heads.length) cell.hidden = hide;
      }
    }
  });
  // A row that spans the table, such as an empty state, spans what is left.
  const shown = heads.filter((th) => !th.hidden).length;
  for (const cell of table?.querySelectorAll("tbody td[colspan]") ?? []) cell.colSpan = Math.max(1, shown);
  const columns = heads.filter((th) => th.dataset.column);
  const left = columns.filter((th) => !th.hidden).length;
  for (const item of root.querySelectorAll("[data-column-toggle]")) {
    const visible = !hidden.has(item.dataset.columnToggle);
    item.setAttribute("aria-checked", String(visible));
    // The last column left cannot be hidden.
    if (visible && left === 1) item.setAttribute("aria-disabled", "true");
    else item.removeAttribute("aria-disabled");
  }
}

function dataTableSetup(root) {
  let hidden = dataTableHidden(root);
  const key = root.dataset.columnsKey;
  if (key) {
    try {
      const stored = JSON.parse(localStorage.getItem(key) ?? "null");
      if (Array.isArray(stored)) hidden = new Set(stored.filter((id) => typeof id === "string"));
    } catch {
      // Nothing stored, or nothing readable: the server's choice stands.
    }
  }
  dataTableApplyColumns(root, hidden);
  dataTableSyncSelection(root);
}

function dataTableScan(node) {
  const roots = new Set(node.querySelectorAll?.("[data-data-table]") ?? []);
  const around = node.closest?.("[data-data-table]");
  if (around) roots.add(around);
  for (const root of roots) dataTableSetup(root);
}

dataTableScan(document);
// The browser restores checkboxes when a page comes back from history.
window.addEventListener("pageshow", () => dataTableScan(document));
// Tables and rows that the server adds later are set up as they arrive.
new MutationObserver((records) => {
  for (const record of records) {
    for (const node of record.addedNodes) {
      if (node.nodeType === 1 && (node.matches("tr, tbody, [data-data-table]") || node.querySelector("[data-data-table]"))) dataTableScan(node);
    }
  }
}).observe(document.documentElement, { childList: true, subtree: true });
