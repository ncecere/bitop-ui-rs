// Model selector: a search box over a listbox in a popover. Focus stays in the search
// box; the option the keys are on is marked data-active and announced through
// aria-activedescendant.

const OPTION = '[role="option"]';

function selectorParts(target) {
  const root = target instanceof Element && target.closest("[data-model-selector]");
  if (!root) return undefined;
  return {
    root,
    trigger: root.querySelector("[data-model-trigger]"),
    hidden: root.querySelector(':scope > input[type="hidden"]'),
    popup: root.querySelector(":scope > [popover]"),
    search: root.querySelector("[data-model-search]"),
    list: root.querySelector('[role="listbox"]'),
    empty: root.querySelector("[data-model-empty]"),
  };
}

const shown = (list) => [...list.querySelectorAll(OPTION)].filter((option) => !option.hidden);

function activate(parts, option) {
  parts.list.querySelector("[data-active]")?.removeAttribute("data-active");
  if (!option) {
    parts.search.removeAttribute("aria-activedescendant");
    return;
  }
  option.toggleAttribute("data-active", true);
  parts.search.setAttribute("aria-activedescendant", option.id);
  option.scrollIntoView({ block: "nearest" });
}

// Every word of the query must appear in the model's name, provider, id, description
// or capabilities.
function filter(parts, query) {
  const words = query.trim().toLowerCase().split(/\s+/).filter(Boolean);
  for (const option of parts.list.querySelectorAll(OPTION)) {
    const hay = option.dataset.search ?? "";
    option.hidden = !words.every((word) => hay.includes(word));
  }
  for (const group of parts.list.querySelectorAll('[role="group"]')) {
    group.hidden = !group.querySelector(`${OPTION}:not([hidden])`);
  }
  const visible = shown(parts.list);
  parts.empty.hidden = visible.length > 0;
  activate(parts, visible[0]);
}

function choose(parts, option) {
  const { root, trigger, hidden, popup, list } = parts;
  for (const other of list.querySelectorAll(OPTION)) other.setAttribute("aria-selected", String(other === option));
  hidden.value = option.dataset.value ?? "";
  const value = trigger.querySelector("[data-placeholder], span");
  value.textContent = option.dataset.name ?? option.textContent.trim();
  value.removeAttribute("data-placeholder");
  trigger.setAttribute("aria-label", `${trigger.dataset.label}: ${option.dataset.name}`);
  trigger.focus();
  popup.hidePopover();
  hidden.dispatchEvent(new Event("change", { bubbles: true }));
  root.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { value: hidden.value } }));
}

document.addEventListener(
  "beforetoggle",
  (event) => {
    const parts = selectorParts(event.target);
    if (parts && event.target === parts.popup) {
      parts.trigger.setAttribute("aria-expanded", String(event.newState === "open"));
    }
  },
  true,
);

document.addEventListener(
  "toggle",
  (event) => {
    const parts = selectorParts(event.target);
    if (!parts || event.target !== parts.popup) return;
    if (event.newState === "open") {
      parts.search.value = "";
      filter(parts, "");
      activate(parts, parts.list.querySelector(`${OPTION}[aria-selected="true"]`) ?? shown(parts.list)[0]);
      // The event is queued: keys pressed since may already have moved focus inside.
      if (!parts.popup.contains(document.activeElement)) parts.search.focus();
    } else {
      activate(parts, undefined);
    }
  },
  true,
);

document.addEventListener("input", (event) => {
  const parts = selectorParts(event.target);
  if (parts && event.target === parts.search) filter(parts, parts.search.value);
});

document.addEventListener("keydown", (event) => {
  const parts = selectorParts(event.target);
  if (!parts || event.target !== parts.search) return;
  const options = shown(parts.list);
  const index = options.indexOf(parts.list.querySelector("[data-active]"));
  switch (event.key) {
    case "ArrowDown":
    case "ArrowUp": {
      event.preventDefault();
      if (!options.length) return;
      const step = event.key === "ArrowDown" ? 1 : -1;
      const from = index === -1 && step === -1 ? 0 : index;
      activate(parts, options[(from + step + options.length) % options.length]);
      return;
    }
    case "Home":
    case "End":
      event.preventDefault();
      activate(parts, event.key === "Home" ? options[0] : options.at(-1));
      return;
    case "Enter":
      event.preventDefault();
      if (index !== -1) choose(parts, options[index]);
      return;
    case "Escape":
      // Safari would return focus to the page: close here and focus the trigger.
      event.preventDefault();
      parts.trigger.focus();
      parts.popup.hidePopover();
      return;
    default:
  }
});

document.addEventListener("click", (event) => {
  const option = event.target instanceof Element && event.target.closest(OPTION);
  if (!option) return;
  const parts = selectorParts(option);
  if (parts) choose(parts, option);
});

// Keep focus in the search box when the list is pressed.
document.addEventListener("pointerdown", (event) => {
  const parts = selectorParts(event.target);
  if (parts?.list.contains(event.target)) event.preventDefault();
});

document.addEventListener("pointermove", (event) => {
  const option = event.target instanceof Element && event.target.closest(OPTION);
  if (!option || option.hasAttribute("data-active")) return;
  const parts = selectorParts(option);
  if (parts) activate(parts, option);
});
