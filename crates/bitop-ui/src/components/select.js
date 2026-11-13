// Select: a button that opens a popover listbox. Focus moves into the list, onto the
// options, as in upstream's Base UI Select; the arrow keys do not wrap.

const SELECT_OPTION = '[role="option"]';

function selectParts(target) {
  const root = target instanceof Element && target.closest("[data-select]");
  if (!root) return undefined;
  return {
    root,
    trigger: root.querySelector('[role="combobox"]'),
    input: root.querySelector("[data-select-value]"),
    popup: root.querySelector("[data-select-popup]"),
    list: root.querySelector('[role="listbox"]'),
  };
}

const selectOptions = (list) => [...list.querySelectorAll(SELECT_OPTION)];
const selectable = (option) => option.getAttribute("aria-disabled") !== "true";
const chosenOption = (list) => list.querySelector(`${SELECT_OPTION}[aria-selected="true"]`);

// Options are found by typing their first letters, as in a native select.
let selectTyped = "";
let selectTypedAt = 0;

function typeahead(options, key, from) {
  const now = Date.now();
  if (now - selectTypedAt > 500) selectTyped = "";
  selectTypedAt = now;
  selectTyped += key.toLowerCase();
  // Repeating one letter cycles through the options that start with it.
  const repeated = [...selectTyped].every((char) => char === selectTyped[0]);
  const search = repeated ? selectTyped[0] : selectTyped;
  const start = repeated ? from + 1 : Math.max(from, 0);
  const ordered = [...options.slice(start), ...options.slice(0, start)];
  return ordered.find((option) => selectable(option) && option.dataset.label.toLowerCase().startsWith(search));
}

function chooseSelect(parts, option) {
  const { root, trigger, input, popup, list } = parts;
  if (!selectable(option)) return;
  const changed = input.value !== option.dataset.value;
  for (const other of selectOptions(list)) other.setAttribute("aria-selected", String(other === option));
  input.value = option.dataset.value;
  const value = trigger.querySelector("span");
  value.textContent = option.dataset.label;
  value.removeAttribute("data-placeholder");
  if (popup.matches(":popover-open")) {
    // The browser would return focus to the page in Safari, where a click does not
    // focus a button.
    trigger.focus();
    popup.hidePopover();
  }
  if (!changed) return;
  input.dispatchEvent(new Event("change", { bubbles: true }));
  root.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { value: input.value } }));
}

// Opened by ArrowUp: focus the last option when none is chosen.
const selectOpenOnLast = new WeakSet();

document.addEventListener(
  "beforetoggle",
  (event) => {
    const parts = selectParts(event.target);
    if (!parts || event.target !== parts.popup) return;
    parts.trigger.setAttribute("aria-expanded", String(event.newState === "open"));
  },
  true,
);

// Opening moves focus to the chosen option, or the first. The event is queued: keys
// pressed since may already have moved focus inside.
document.addEventListener(
  "toggle",
  (event) => {
    const parts = selectParts(event.target);
    if (!parts || event.target !== parts.popup || event.newState !== "open") return;
    const last = selectOpenOnLast.delete(parts.popup);
    if (parts.list.contains(document.activeElement)) return;
    const options = selectOptions(parts.list);
    (chosenOption(parts.list) ?? (last ? options.at(-1) : options[0]) ?? parts.list).focus({ preventScroll: true });
    document.activeElement?.scrollIntoView({ block: "nearest" });
  },
  true,
);

document.addEventListener("keydown", (event) => {
  const parts = selectParts(event.target);
  if (!parts || event.ctrlKey || event.metaKey || event.altKey) return;
  const { trigger, popup, list } = parts;
  const options = selectOptions(list);

  if (event.target === trigger) {
    if (event.key === "ArrowDown" || event.key === "ArrowUp") {
      event.preventDefault();
      if (event.key === "ArrowUp") selectOpenOnLast.add(popup);
      if (!popup.matches(":popover-open")) trigger.click();
    } else if (event.key.length === 1 && event.key !== " ") {
      // Typing on the closed button chooses, as in a native select.
      const next = typeahead(options, event.key, options.indexOf(chosenOption(list)));
      if (next) {
        event.preventDefault();
        chooseSelect(parts, next);
      }
    }
    return;
  }

  if (!list.contains(event.target)) return;
  const index = options.indexOf(event.target.closest(SELECT_OPTION));
  let next;
  switch (event.key) {
    case "ArrowDown":
      next = options[Math.min(index + 1, options.length - 1)];
      break;
    case "ArrowUp":
      next = index === -1 ? options.at(-1) : options[Math.max(index - 1, 0)];
      break;
    case "Home":
      next = options[0];
      break;
    case "End":
      next = options.at(-1);
      break;
    case " ":
      // A space is part of a search being typed; otherwise it chooses.
      if (selectTyped && Date.now() - selectTypedAt <= 500) {
        next = typeahead(options, " ", index);
        if (!next) return;
        break;
      }
    // falls through
    case "Enter":
      event.preventDefault();
      if (index !== -1) chooseSelect(parts, options[index]);
      return;
    case "Escape":
      event.preventDefault();
      trigger.focus();
      popup.hidePopover();
      return;
    case "Tab":
      // The list is not a tab stop: Tab closes it and moves on from the button.
      trigger.focus();
      popup.hidePopover();
      return;
    default:
      if (event.key.length !== 1) return;
      next = typeahead(options, event.key, index);
      if (!next) return;
  }
  event.preventDefault();
  next?.focus();
  next?.scrollIntoView({ block: "nearest" });
});

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  // The label focuses the button, as upstream's, rather than clicking it.
  const label = event.target.closest("[data-select] > label");
  if (label) {
    event.preventDefault();
    selectParts(label)?.trigger.focus();
    return;
  }
  const option = event.target.closest(SELECT_OPTION);
  const parts = option && selectParts(option);
  if (parts) chooseSelect(parts, option);
});

// The pointer moves the highlight, which is focus.
document.addEventListener("pointermove", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const option = event.target.closest(`[data-select-popup] ${SELECT_OPTION}`);
  if (option && option !== document.activeElement) option.focus({ preventScroll: true });
});

// A form that is sent without a required value shows the browser's message at the
// button, and focuses the button rather than the hidden input.
document.addEventListener(
  "invalid",
  (event) => {
    const parts = selectParts(event.target);
    if (parts && event.target === parts.input) parts.trigger.focus();
  },
  true,
);
