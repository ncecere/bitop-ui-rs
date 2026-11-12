// Combobox and command palette: one text input filtering one listbox.
//
// Focus stays in the input. The option the arrow keys are on is marked with
// data-active and announced through aria-activedescendant.

const OPTION = '[role="option"]:not([aria-disabled="true"])';
let optionIds = 0;

function comboboxParts(target) {
  const root = target instanceof Element && target.closest("[data-combobox]");
  if (!root) return undefined;
  const input = root.querySelector('[role="combobox"]');
  return {
    root,
    input,
    list: document.getElementById(input.getAttribute("aria-controls")),
    popup: root.querySelector(":scope > [popover]"),
    hidden: root.querySelector(':scope > input[type="hidden"]'),
    empty: root.querySelector("[data-combobox-empty]"),
  };
}

function visibleOptions(list) {
  return [...list.querySelectorAll(OPTION)].filter((option) => !option.hidden);
}

function setActive({ input, list }, option) {
  list.querySelector("[data-active]")?.removeAttribute("data-active");
  if (!option) {
    input.removeAttribute("aria-activedescendant");
    return;
  }
  option.id ||= `bitop-option-${++optionIds}`;
  option.toggleAttribute("data-active", true);
  input.setAttribute("aria-activedescendant", option.id);
  option.scrollIntoView({ block: "nearest" });
}

function filterOptions(parts, query) {
  const { list, empty } = parts;
  const needle = query.trim().toLowerCase();
  let shown = 0;
  for (const option of list.querySelectorAll('[role="option"]')) {
    const text = `${option.textContent} ${option.dataset.keywords ?? ""}`.toLowerCase();
    option.hidden = !text.includes(needle);
    if (!option.hidden) shown += 1;
  }
  for (const group of list.querySelectorAll('[role="group"]')) {
    group.hidden = !group.querySelector('[role="option"]:not([hidden])');
  }
  if (empty) empty.hidden = shown > 0;
  setActive(parts, visibleOptions(list)[0]);
}

function selectedOption(list) {
  return list.querySelector('[role="option"][aria-selected="true"]');
}

function openCombobox(parts) {
  const { input, popup, list } = parts;
  if (!popup || popup.matches(":popover-open")) return;
  input.style.anchorName = "--bitop-combobox";
  popup.showPopover();
  input.setAttribute("aria-expanded", "true");
  setActive(parts, selectedOption(list) ?? visibleOptions(list)[0]);
}

function closeCombobox(parts) {
  const { input, popup } = parts;
  if (!popup || !popup.matches(":popover-open")) return;
  popup.hidePopover();
  input.style.anchorName = "";
  input.setAttribute("aria-expanded", "false");
  setActive(parts, undefined);
}

// Puts the text back in step with the chosen value when the person leaves without
// choosing. Clearing the text clears the value.
function settleCombobox(parts) {
  const { input, list, hidden } = parts;
  if (!hidden) return;
  const selected = selectedOption(list);
  if (input.value.trim() === "" && selected) {
    selected.setAttribute("aria-selected", "false");
    hidden.value = "";
    hidden.dispatchEvent(new Event("change", { bubbles: true }));
  } else {
    input.value = selected ? selected.textContent.trim() : "";
  }
  filterOptions(parts, "");
  setActive(parts, undefined);
}

function chooseOption(parts, option) {
  const { input, list, hidden, root } = parts;
  if (!hidden) {
    // A palette item: run the link or button itself, then close the palette.
    option.click();
    return;
  }
  for (const other of list.querySelectorAll('[role="option"]')) {
    other.setAttribute("aria-selected", String(other === option));
  }
  input.value = option.textContent.trim();
  hidden.value = option.dataset.value ?? "";
  closeCombobox(parts);
  filterOptions(parts, "");
  setActive(parts, undefined);
  hidden.dispatchEvent(new Event("change", { bubbles: true }));
  root.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { value: hidden.value } }));
}

document.addEventListener("input", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts || event.target !== parts.input) return;
  openCombobox(parts);
  filterOptions(parts, parts.input.value);
});

document.addEventListener("keydown", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts || event.target !== parts.input) return;
  const { input, list, popup } = parts;
  const open = !popup || popup.matches(":popover-open");
  const options = visibleOptions(list);
  const index = options.indexOf(list.querySelector("[data-active]"));

  switch (event.key) {
    case "ArrowDown":
    case "ArrowUp": {
      event.preventDefault();
      if (!open) {
        openCombobox(parts);
        return;
      }
      if (!options.length) return;
      const step = event.key === "ArrowDown" ? 1 : -1;
      // From no active option, Down goes to the first and Up to the last.
      const from = index === -1 && step === -1 ? 0 : index;
      setActive(parts, options[(from + step + options.length) % options.length]);
      return;
    }
    case "Enter":
      if (open && index !== -1) {
        event.preventDefault();
        chooseOption(parts, options[index]);
      }
      return;
    case "Escape":
      if (popup && open) {
        // Close only the list, not a dialog the combobox may be in.
        event.preventDefault();
        closeCombobox(parts);
        settleCombobox(parts);
      } else if (popup && input.value) {
        event.preventDefault();
        input.value = "";
        settleCombobox(parts);
      }
      return;
    default:
  }
});

// A click on the input opens the list; a click on an option chooses it.
document.addEventListener("click", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts) return;
  if (event.target === parts.input) {
    openCombobox(parts);
    return;
  }
  const option = event.target.closest(OPTION);
  if (!option) return;
  if (parts.hidden) chooseOption(parts, option);
  else parts.root.closest("dialog")?.close();
});

// Keep focus in the input when the list is pressed, on an option or between them.
document.addEventListener("pointerdown", (event) => {
  const parts = comboboxParts(event.target);
  if (parts?.popup?.contains(event.target)) event.preventDefault();
});

// The pointer and the keys share one highlight.
document.addEventListener("pointermove", (event) => {
  const option = event.target instanceof Element && event.target.closest(OPTION);
  if (!option || option.hasAttribute("data-active")) return;
  const parts = comboboxParts(option);
  if (parts) setActive(parts, option);
});

document.addEventListener("focusout", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts?.popup || parts.root.contains(event.relatedTarget)) return;
  closeCombobox(parts);
  settleCombobox(parts);
});

// A palette starts each time with an empty search and its first item active.
document.addEventListener("focusin", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts || parts.popup || event.target !== parts.input) return;
  if (!parts.list.querySelector("[data-active]")) setActive(parts, visibleOptions(parts.list)[0]);
});

// For the command palette, which shares this controller.
window.bitopUi.combobox = { parts: comboboxParts, filter: filterOptions, activate: setActive };
