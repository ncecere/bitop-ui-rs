// File tree: the WAI-ARIA tree view keyboard model, selection, and folders that open and
// close, on server-rendered role="tree" markup. One visible item is in the tab order.

const TREE_ITEM = '[role="treeitem"]';
const treeTypeahead = new WeakMap();

function treeOf(item) {
  return item.closest('[role="tree"]');
}

function parentItem(item) {
  const parent = item.parentElement?.closest(TREE_ITEM);
  return parent && treeOf(parent) === treeOf(item) ? parent : null;
}

// Items whose folders are all open, in document order.
function visibleTreeItems(tree) {
  return [...tree.querySelectorAll(TREE_ITEM)].filter((item) => {
    for (let parent = parentItem(item); parent; parent = parentItem(parent)) {
      if (parent.getAttribute("aria-expanded") !== "true") return false;
    }
    return true;
  });
}

// Keeps one visible item in the tab order: the one given, or the one that was, or the
// selected one, or the first.
function setTabStop(tree, item) {
  const items = visibleTreeItems(tree);
  const stop =
    (item && items.includes(item) && item) ||
    items.find((i) => i.tabIndex === 0) ||
    items.find((i) => i.getAttribute("aria-selected") === "true") ||
    items[0];
  for (const other of tree.querySelectorAll(TREE_ITEM)) other.tabIndex = other === stop ? 0 : -1;
}

function focusTreeItem(item) {
  if (!item) return;
  setTabStop(treeOf(item), item);
  item.focus();
}

function setTreeOpen(item, open) {
  if (item.dataset.kind !== "folder" || (item.getAttribute("aria-expanded") === "true") === open) return;
  item.setAttribute("aria-expanded", String(open));
  const group = item.querySelector(':scope > [role="group"]');
  if (group) group.hidden = !open;
  const icon = item.querySelector(":scope > [data-tree-row] [data-folder-icon] [data-icon]");
  icon?.setAttribute("data-icon", open ? "folder-open" : "folder");
  const tree = treeOf(item);
  // An item that was the tab stop may now be hidden inside the closed folder.
  if (!open && item.querySelector('[role="treeitem"][tabindex="0"]')) setTabStop(tree, item);
  tree.dispatchEvent(new CustomEvent("bitop:file-tree-toggle", { bubbles: true, detail: { path: item.dataset.path, open } }));
}

function selectTreeItem(item) {
  const tree = treeOf(item);
  for (const other of tree.querySelectorAll(TREE_ITEM)) {
    const selected = other === item;
    other.setAttribute("aria-selected", String(selected));
    other.toggleAttribute("data-selected", selected);
  }
  tree.dispatchEvent(new CustomEvent("bitop:file-tree-select", { bubbles: true, detail: { path: item.dataset.path, kind: item.dataset.kind } }));
}

function activateTreeItem(item) {
  selectTreeItem(item);
  if (item.dataset.kind === "folder") setTreeOpen(item, item.getAttribute("aria-expanded") !== "true");
}

// Typing moves to the next item whose name starts with what was typed in the last
// half second. One letter typed again and again cycles through the items that start
// with it.
function treeTypeAhead(tree, item, key, items) {
  const state = treeTypeahead.get(tree) ?? { text: "", timer: undefined };
  treeTypeahead.set(tree, state);
  clearTimeout(state.timer);
  state.text += key.toLowerCase();
  state.timer = setTimeout(() => (state.text = ""), 500);
  const repeated = [...state.text].every((c) => c === state.text[0]);
  const query = repeated ? state.text[0] : state.text;
  const index = items.indexOf(item);
  const start = repeated || state.text.length === 1 ? index + 1 : index;
  for (let i = 0; i < items.length; i += 1) {
    const candidate = items[(start + i) % items.length];
    if ((candidate.dataset.name ?? "").toLowerCase().startsWith(query)) {
      focusTreeItem(candidate);
      return;
    }
  }
}

document.addEventListener("keydown", (event) => {
  const item = event.target instanceof Element && event.target.matches(TREE_ITEM) ? event.target : null;
  const tree = item && treeOf(item);
  if (!tree || !tree.hasAttribute("data-file-tree") || event.altKey || event.ctrlKey || event.metaKey) return;
  const items = visibleTreeItems(tree);
  const index = items.indexOf(item);
  const folder = item.dataset.kind === "folder";
  const open = item.getAttribute("aria-expanded") === "true";
  const rtl = getComputedStyle(tree).direction === "rtl";
  const inward = rtl ? "ArrowLeft" : "ArrowRight";
  const outward = rtl ? "ArrowRight" : "ArrowLeft";
  const state = treeTypeahead.get(tree);
  if (state && (event.key.length !== 1 || event.key === " ")) state.text = "";
  if (event.key === "ArrowDown") focusTreeItem(items[index + 1]);
  else if (event.key === "ArrowUp") focusTreeItem(items[index - 1]);
  else if (event.key === "Home") focusTreeItem(items[0]);
  else if (event.key === "End") focusTreeItem(items.at(-1));
  else if (event.key === inward) {
    if (folder && !open) setTreeOpen(item, true);
    else if (folder && items[index + 1] && parentItem(items[index + 1]) === item) focusTreeItem(items[index + 1]);
  } else if (event.key === outward) {
    if (folder && open) setTreeOpen(item, false);
    else focusTreeItem(parentItem(item) ?? undefined);
  } else if (event.key === "Enter" || event.key === " ") activateTreeItem(item);
  else if (event.key === "*") {
    for (const sibling of item.parentElement.children) {
      if (sibling.matches(TREE_ITEM)) setTreeOpen(sibling, true);
    }
  } else if (event.key.length === 1 && event.key !== " ") treeTypeAhead(tree, item, event.key, items);
  else return;
  event.preventDefault();
});

document.addEventListener("click", (event) => {
  const row = event.target instanceof Element && event.target.closest("[data-tree-row]");
  const item = row?.parentElement;
  if (!item?.matches(TREE_ITEM) || !treeOf(item)?.hasAttribute("data-file-tree")) return;
  activateTreeItem(item);
  focusTreeItem(item);
});

document.addEventListener("focusin", (event) => {
  const item = event.target instanceof Element && event.target.matches(TREE_ITEM) ? event.target : null;
  if (item && treeOf(item)?.hasAttribute("data-file-tree")) setTabStop(treeOf(item), item);
});

// Fills in what the server left to the script: each item's level, and the tab stop.
function setUpTree(tree) {
  for (const item of tree.querySelectorAll(TREE_ITEM)) {
    if (item.hasAttribute("aria-level")) continue;
    let level = 1;
    for (let parent = parentItem(item); parent; parent = parentItem(parent)) level += 1;
    item.setAttribute("aria-level", String(level));
  }
  setTabStop(tree);
}

document.querySelectorAll("[data-file-tree]").forEach(setUpTree);
new MutationObserver((records) => {
  for (const record of records) {
    for (const node of record.addedNodes) {
      if (!(node instanceof Element)) continue;
      const tree = node.closest("[data-file-tree]");
      if (tree) setUpTree(tree);
      else node.querySelectorAll("[data-file-tree]").forEach(setUpTree);
    }
  }
}).observe(document.documentElement, { subtree: true, childList: true });
