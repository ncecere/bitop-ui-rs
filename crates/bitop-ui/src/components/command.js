// Command palette: a search box over a listbox in a modal dialog. Focus stays in the
// search box; the item the keys are on is marked data-active and announced through
// aria-activedescendant. Matching and ranking follow upstream's commandMatches,
// commandRank and rankCommandGroups.

const COMMAND_ITEM = '[role="option"]';
let commandIds = 0;

// A typed word and its singular forms: "members" also looks for "member", "policies"
// for "policy", "searches" for "search".
function wordForms(word) {
  const forms = [word];
  if (word.length > 4 && word.endsWith("ies")) forms.push(`${word.slice(0, -3)}y`);
  if (word.length > 4 && word.endsWith("es")) forms.push(word.slice(0, -2));
  if (word.length > 3 && word.endsWith("s") && !word.endsWith("ss")) forms.push(word.slice(0, -1));
  return forms;
}

const commandWords = (text) =>
  text
    .toLowerCase()
    .split(/[^\p{L}\p{N}]+/u)
    .filter(Boolean);

function commandLabel(item) {
  return (item.querySelector("[data-command-label]") ?? item).textContent.trim();
}

function commandKeywords(item) {
  return (item.dataset.keywords ?? "")
    .split(",")
    .map((keyword) => keyword.trim())
    .filter(Boolean);
}

// Every word of the query is in the label or the keywords, in the singular or the plural.
function commandMatches(item, query) {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  const haystack = [commandLabel(item), ...commandKeywords(item)].join(" ").toLowerCase();
  return q.split(/\s+/).every((word) => wordForms(word).some((form) => haystack.includes(form)));
}

// How well a matching item fits, best first: 0 its label is the query, 1 its label
// starts with it, 2 every typed word starts a word of the label, 3 a keyword is the
// query, 4 any other match.
function commandRank(item, query) {
  const q = query.trim().toLowerCase().replace(/\s+/g, " ");
  if (!q) return 0;
  const label = commandLabel(item).toLowerCase();
  const forms = wordForms(q);
  if (forms.includes(label)) return 0;
  if (label.startsWith(q)) return 1;
  const labelWords = commandWords(label);
  const typed = commandWords(q);
  if (typed.length && typed.every((w) => wordForms(w).some((f) => labelWords.some((l) => l.startsWith(f))))) {
    return 2;
  }
  if (commandKeywords(item).some((k) => forms.includes(k.toLowerCase()))) return 3;
  return 4;
}

function paletteParts(target) {
  const dialog = target instanceof Element && target.closest("dialog[data-command-palette]");
  if (!dialog) return undefined;
  const input = dialog.querySelector("[data-command-input]");
  return {
    dialog,
    input,
    list: document.getElementById(input.getAttribute("aria-controls")),
    empty: dialog.querySelector("[data-command-empty]"),
  };
}

const paletteShown = (list) => [...list.querySelectorAll(COMMAND_ITEM)].filter((item) => !item.hidden);

function paletteActivate({ input, list }, item) {
  list.querySelector("[data-active]")?.removeAttribute("data-active");
  if (!item) {
    input.removeAttribute("aria-activedescendant");
    return;
  }
  item.id ||= `bitop-command-${++commandIds}`;
  item.toggleAttribute("data-active", true);
  input.setAttribute("aria-activedescendant", item.id);
  item.scrollIntoView({ block: "nearest" });
}

// Filters and ranks: items by rank in each group, and groups by their best item.
// Ties keep the order the server rendered, which each element remembers.
function paletteFilter(parts, query) {
  const { list, empty } = parts;
  const groups = [...list.querySelectorAll(':scope > [role="group"]')];
  const ranked = groups.map((group, index) => {
    group.dataset.order ??= String(index);
    const items = [...group.querySelectorAll(COMMAND_ITEM)].map((item, position) => {
      item.dataset.order ??= String(position);
      const match = commandMatches(item, query);
      item.hidden = !match;
      return { item, order: Number(item.dataset.order), rank: match ? commandRank(item, query) : 5 };
    });
    const keep = group.hasAttribute("data-keep-order") || !query.trim();
    items.sort((a, b) => (keep ? 0 : a.rank - b.rank) || a.order - b.order);
    for (const { item } of items) group.append(item);
    group.hidden = items.every(({ item }) => item.hidden);
    const best = Math.min(5, ...items.map(({ rank }) => rank));
    return { group, order: Number(group.dataset.order), best: query.trim() ? best : 0 };
  });
  ranked.sort((a, b) => a.best - b.best || a.order - b.order);
  for (const { group } of ranked) list.append(group);
  const shown = paletteShown(list);
  empty.textContent = shown.length ? "" : empty.dataset.empty;
  // Upstream's autoHighlight="always": the first match is highlighted.
  paletteActivate(parts, shown[0]);
}

// A palette starts each time with an empty search and its first item highlighted.
document.addEventListener(
  "toggle",
  (event) => {
    const parts = paletteParts(event.target);
    if (!parts || event.target !== parts.dialog || event.newState !== "open") return;
    parts.input.value = "";
    paletteFilter(parts, "");
  },
  true,
);

// Older engines send no toggle event for a dialog: also reset on close.
document.addEventListener(
  "close",
  (event) => {
    const parts = paletteParts(event.target);
    if (!parts || event.target !== parts.dialog) return;
    parts.input.value = "";
    paletteFilter(parts, "");
  },
  true,
);

// Where a dialog sends no toggle event, the first focus of the search box sets the
// highlight.
document.addEventListener("focusin", (event) => {
  const parts = paletteParts(event.target);
  if (parts && event.target === parts.input && !parts.list.querySelector("[data-active]")) {
    paletteFilter(parts, parts.input.value);
  }
});

document.addEventListener("input", (event) => {
  const parts = paletteParts(event.target);
  if (parts && event.target === parts.input) paletteFilter(parts, parts.input.value);
});

function runCommand(parts, item) {
  // Close first, as upstream runs the command after the palette has closed.
  parts.dialog.close();
  item.click();
}

document.addEventListener("keydown", (event) => {
  const parts = paletteParts(event.target);
  if (!parts || event.target !== parts.input) return;
  const items = paletteShown(parts.list);
  const index = items.indexOf(parts.list.querySelector("[data-active]"));
  switch (event.key) {
    case "ArrowDown":
    case "ArrowUp": {
      event.preventDefault();
      if (!items.length) return;
      const step = event.key === "ArrowDown" ? 1 : -1;
      const from = index === -1 && step === -1 ? 0 : index;
      paletteActivate(parts, items[(from + step + items.length) % items.length]);
      return;
    }
    case "Enter":
      event.preventDefault();
      if (index !== -1) runCommand(parts, items[index]);
      return;
    default:
  }
});

// A click on an item runs it: close the palette, then let the link or button act.
document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const item = event.target.closest(COMMAND_ITEM);
  const parts = item && paletteParts(item);
  if (parts) parts.dialog.close();
});

// Keep focus in the search box when the list is pressed.
document.addEventListener("pointerdown", (event) => {
  const parts = paletteParts(event.target);
  if (parts?.list.contains(event.target)) event.preventDefault();
});

// The pointer and the keys share one highlight.
document.addEventListener("pointermove", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const item = event.target.closest(COMMAND_ITEM);
  if (!item || item.hasAttribute("data-active")) return;
  const parts = paletteParts(item);
  if (parts) paletteActivate(parts, item);
});

// Ctrl+K or Command+K, or whichever key data-shortcut names, toggles the palette.
document.addEventListener("keydown", (event) => {
  if (!(event.ctrlKey || event.metaKey) || event.altKey || event.shiftKey) return;
  for (const palette of document.querySelectorAll("dialog[data-command-palette][data-shortcut]")) {
    if (palette.dataset.shortcut.toLowerCase() !== event.key.toLowerCase()) continue;
    event.preventDefault();
    if (palette.open) palette.close();
    else palette.showModal();
    return;
  }
});

window.bitopUi.command = { matches: commandMatches, rank: commandRank };
