// Combobox: one text input filtering one listbox, with one value, several (chips) or
// free text. Focus stays in the input; the option the keys are on is marked with
// data-active and announced through aria-activedescendant.

const CB_OPTION = '[role="option"]';

function comboboxParts(target) {
  const root = target instanceof Element && target.closest("[data-combobox]");
  if (!root) return undefined;
  const input = root.querySelector('[role="combobox"]');
  return {
    root,
    input,
    mode: root.dataset.mode,
    control: root.querySelector("[data-combobox-control]"),
    list: document.getElementById(input.getAttribute("aria-controls")),
    popup: root.querySelector(":scope > [popover]"),
    hidden: root.querySelector("[data-combobox-value]"),
    chips: root.querySelector("[data-combobox-chips]"),
    template: root.querySelector("template[data-combobox-chip]"),
    clear: root.querySelector("[data-combobox-clear]"),
    toggle: root.querySelector("[data-combobox-toggle]"),
    empty: root.querySelector("[data-combobox-empty]"),
  };
}

const cbOptions = (list) => [...list.querySelectorAll(CB_OPTION)];
const cbShown = (list) => cbOptions(list).filter((option) => !option.hidden);
const cbEnabled = (option) => option.getAttribute("aria-disabled") !== "true";
const cbFold = (text) =>
  text
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();
const isOpen = (parts) => parts.popup.matches(":popover-open");

function cbActivate({ input, list }, option) {
  list.querySelector("[data-active]")?.removeAttribute("data-active");
  if (!option) {
    input.removeAttribute("aria-activedescendant");
    return;
  }
  option.toggleAttribute("data-active", true);
  input.setAttribute("aria-activedescendant", option.id);
  option.scrollIntoView({ block: "nearest" });
}

// Shows the options whose label contains the text, at most `limit` of them.
function cbFilter(parts, query) {
  const { root, list, empty } = parts;
  const needle = cbFold(query.trim());
  const limit = Number(root.dataset.limit ?? Infinity);
  let shown = 0;
  for (const option of cbOptions(list)) {
    const match = cbFold(option.dataset.label).includes(needle);
    option.hidden = !match || shown >= limit;
    if (!option.hidden) shown += 1;
  }
  for (const group of list.children) group.hidden = !group.querySelector(`${CB_OPTION}:not([hidden])`);
  empty.textContent = shown ? "" : (root.dataset.empty ?? "");
  // Free text with no suggestion left and no message: nothing to show.
  if (!shown && !empty.textContent && isOpen(parts)) cbClose(parts);
  return shown;
}

function cbOpen(parts, highlight) {
  const { input, popup, list, control, toggle } = parts;
  if (isOpen(parts)) return;
  for (const other of document.querySelectorAll("[data-combobox-control]")) other.style.anchorName = "";
  control.style.anchorName = "--bitop-combobox";
  // The input shows the chosen label: show every option, not those matching it.
  const shown = cbFilter(parts, parts.mode === "single" ? "" : input.value);
  if (!shown && !parts.empty.textContent) return;
  popup.showPopover();
  control.toggleAttribute("data-open", true);
  input.setAttribute("aria-expanded", "true");
  toggle?.setAttribute("aria-expanded", "true");
  const options = cbShown(list);
  const chosen = options.find((option) => option.getAttribute("aria-selected") === "true");
  // The chosen option is highlighted; ArrowDown and ArrowUp, with none chosen,
  // highlight the first or the last.
  const single = parts.mode === "free-text" ? undefined : chosen;
  if (highlight === "first") cbActivate(parts, single ?? options[0]);
  else if (highlight === "last") cbActivate(parts, single ?? options.at(-1));
  else cbActivate(parts, single);
}

function cbClose(parts) {
  const { input, popup, control, toggle } = parts;
  if (!isOpen(parts)) return;
  popup.hidePopover();
  control.style.anchorName = "";
  control.removeAttribute("data-open");
  input.setAttribute("aria-expanded", "false");
  toggle?.setAttribute("aria-expanded", "false");
  cbActivate(parts, undefined);
}

function chipsOf(parts) {
  return parts.chips ? [...parts.chips.querySelectorAll(":scope > [data-value]")] : [];
}

function cbValue(parts) {
  if (parts.mode === "multiple") return chipsOf(parts).map((chip) => chip.dataset.value);
  if (parts.mode === "free-text") return parts.input.value;
  return parts.hidden?.value ?? "";
}

// After a change: the clear button, the chips' name and the placeholder.
function cbRefresh(parts) {
  const { input, chips, clear, root } = parts;
  const value = cbValue(parts);
  const has = Array.isArray(value) ? value.length > 0 : value !== "";
  if (clear) clear.hidden = !has;
  if (chips) {
    const any = chipsOf(parts).length > 0;
    if (any) {
      chips.setAttribute("role", "toolbar");
      chips.setAttribute("aria-label", chips.dataset.chipsLabel);
    } else {
      chips.removeAttribute("role");
      chips.removeAttribute("aria-label");
    }
    if (input.dataset.placeholder) {
      if (any) input.removeAttribute("placeholder");
      else input.setAttribute("placeholder", input.dataset.placeholder);
    }
  }
  root.toggleAttribute("data-filled", has);
}

function cbChanged(parts, from) {
  cbRefresh(parts);
  from.dispatchEvent(new Event("change", { bubbles: true }));
  parts.root.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { value: cbValue(parts) } }));
}

function addChip(parts, option) {
  const chip = parts.template.content.firstElementChild.cloneNode(true);
  chip.dataset.value = option.dataset.value;
  chip.querySelector("span").textContent = option.dataset.label;
  chip
    .querySelector("[data-combobox-remove]")
    .setAttribute("aria-label", parts.root.dataset.removeLabel.replace("{label}", option.dataset.label));
  const hidden = chip.querySelector('input[type="hidden"]');
  hidden.value = option.dataset.value;
  parts.chips.insertBefore(chip, parts.input);
  return hidden;
}

function removeChip(parts, chip) {
  const option = cbOptions(parts.list).find((other) => other.dataset.value === chip.dataset.value);
  option?.setAttribute("aria-selected", "false");
  const input = chip.querySelector('input[type="hidden"]');
  chip.remove();
  // The hidden input has left the form; the change is sent from the text input.
  cbChanged(parts, parts.input);
  return input;
}

function cbChoose(parts, option) {
  const { input, hidden, list, mode } = parts;
  if (!cbEnabled(option)) return;
  if (mode === "multiple") {
    if (option.getAttribute("aria-selected") === "true") {
      const chip = chipsOf(parts).find((other) => other.dataset.value === option.dataset.value);
      if (chip) removeChip(parts, chip);
      return;
    }
    option.setAttribute("aria-selected", "true");
    const added = addChip(parts, option);
    input.value = "";
    cbFilter(parts, "");
    cbActivate(parts, option);
    cbChanged(parts, added);
    return;
  }
  if (mode === "free-text") {
    input.value = option.dataset.value;
    cbClose(parts);
    cbChanged(parts, input);
    return;
  }
  const changed = hidden.value !== option.dataset.value;
  for (const other of cbOptions(list)) other.setAttribute("aria-selected", String(other === option));
  input.value = option.dataset.label;
  hidden.value = option.dataset.value;
  cbClose(parts);
  if (changed) cbChanged(parts, hidden);
}

function cbClear(parts) {
  const { input, hidden, mode } = parts;
  input.value = "";
  if (mode === "multiple") {
    for (const chip of chipsOf(parts)) chip.remove();
    for (const option of cbOptions(parts.list)) option.setAttribute("aria-selected", "false");
    cbChanged(parts, input);
  } else if (mode === "free-text") {
    cbChanged(parts, input);
  } else {
    for (const option of cbOptions(parts.list)) option.setAttribute("aria-selected", "false");
    hidden.value = "";
    cbChanged(parts, hidden);
  }
  cbFilter(parts, "");
  input.focus();
}

// Leaving without choosing: one value puts the text back in step with it, and an
// emptied text clears it. Several values drop the text; free text keeps it.
function cbSettle(parts) {
  const { input, hidden, list, mode } = parts;
  if (mode === "multiple") input.value = "";
  if (mode !== "single") return;
  const chosen = list.querySelector(`${CB_OPTION}[aria-selected="true"]`);
  if (input.value.trim() === "" && chosen) {
    chosen.setAttribute("aria-selected", "false");
    hidden.value = "";
    cbChanged(parts, hidden);
  } else {
    input.value = chosen ? chosen.dataset.label : "";
  }
}

document.addEventListener("input", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts || event.target !== parts.input) return;
  if (!isOpen(parts)) cbOpen(parts);
  cbFilter(parts, parts.input.value);
  if (parts.mode === "free-text") cbRefresh(parts);
  if (!isOpen(parts) && cbShown(parts.list).length) cbOpen(parts);
  // Upstream's autoHighlight: the first match is highlighted while typing.
  cbActivate(parts, parts.root.hasAttribute("data-auto-highlight") ? cbShown(parts.list).find(cbEnabled) : undefined);
});

// Moves among the chips and the input of a combobox with several values.
function chipKeys(parts, event) {
  const chips = chipsOf(parts);
  const chip = event.target.closest("[data-value]");
  const index = chips.indexOf(chip);
  const rtl = getComputedStyle(parts.root).direction === "rtl";
  const back = rtl ? "ArrowRight" : "ArrowLeft";
  const forward = rtl ? "ArrowLeft" : "ArrowRight";
  if (event.key === back) chips[Math.max(index - 1, 0)].focus();
  else if (event.key === forward) (chips[index + 1] ?? parts.input).focus();
  else if (event.key === "Backspace" || event.key === "Delete") {
    // Focus moves on before the chip goes, so that it never leaves the combobox.
    (chips[index - 1] ?? chips[index + 1] ?? parts.input).focus();
    removeChip(parts, chip);
  } else return;
  event.preventDefault();
}

document.addEventListener("keydown", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts) return;
  if (parts.chips && event.target !== parts.input && event.target.closest("[data-combobox-chips] > [data-value]")) {
    chipKeys(parts, event);
    return;
  }
  if (event.target !== parts.input) return;
  const { input, list, root } = parts;
  const open = isOpen(parts);
  const options = cbShown(list);
  const index = options.indexOf(list.querySelector("[data-active]"));

  switch (event.key) {
    case "ArrowDown":
    case "ArrowUp": {
      if (event.altKey && event.key === "ArrowUp") {
        event.preventDefault();
        cbClose(parts);
        return;
      }
      event.preventDefault();
      const down = event.key === "ArrowDown";
      if (!open) {
        cbOpen(parts, event.altKey ? undefined : down ? "first" : "last");
        return;
      }
      if (!options.length) return;
      // Past either end, the highlight goes back to the text, as upstream's loop.
      let next;
      if (index === -1) next = down ? options[0] : options.at(-1);
      else next = options[index + (down ? 1 : -1)];
      cbActivate(parts, next);
      return;
    }
    case "Enter":
      if (open && index !== -1) {
        event.preventDefault();
        cbChoose(parts, options[index]);
      } else if (!root.hasAttribute("data-submit-on-enter")) {
        // A closed list never sends a half-filled form.
        event.preventDefault();
      } else if (open) {
        cbClose(parts);
      }
      return;
    case "Escape":
      if (open) {
        // Close only the list, not a dialog the combobox may be in.
        event.preventDefault();
        cbClose(parts);
        cbSettle(parts);
      } else if (parts.mode !== "multiple" && input.value) {
        event.preventDefault();
        input.value = "";
        if (parts.mode === "single") cbSettle(parts);
        else cbChanged(parts, input);
      }
      return;
    case "Backspace":
      // Backspace in the empty input removes the last chip.
      if (parts.chips && input.selectionStart === 0 && input.selectionEnd === 0) {
        const last = chipsOf(parts).at(-1);
        if (last) {
          event.preventDefault();
          removeChip(parts, last);
        }
      }
      return;
    case "ArrowLeft":
    case "ArrowRight": {
      if (!parts.chips || input.selectionStart !== 0 || input.selectionEnd !== 0) return;
      const rtl = getComputedStyle(root).direction === "rtl";
      if (event.key !== (rtl ? "ArrowRight" : "ArrowLeft")) return;
      const last = chipsOf(parts).at(-1);
      if (last) {
        event.preventDefault();
        last.focus();
      }
      return;
    }
    default:
  }
});

document.addEventListener("click", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts) return;
  const { input } = parts;
  if (event.target.closest("[data-combobox-toggle]")) {
    input.focus();
    if (isOpen(parts)) cbClose(parts);
    else cbOpen(parts);
    return;
  }
  if (event.target.closest("[data-combobox-clear]")) {
    cbClear(parts);
    return;
  }
  const remove = event.target.closest("[data-combobox-remove]");
  if (remove) {
    removeChip(parts, remove.closest("[data-value]"));
    input.focus();
    return;
  }
  if (event.target === input) {
    cbOpen(parts);
    return;
  }
  const option = event.target.closest(CB_OPTION);
  if (option) {
    cbChoose(parts, option);
    return;
  }
  // A click on the control around the input focuses it.
  if (event.target.closest("[data-combobox-control]") && !input.disabled) input.focus();
});

// Keep focus in the input when the list or the buttons are pressed.
document.addEventListener("pointerdown", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts) return;
  if (parts.popup.contains(event.target) || event.target.closest("[data-combobox-control] button")) {
    event.preventDefault();
  }
});

// The pointer and the keys share one highlight.
document.addEventListener("pointermove", (event) => {
  if (event.pointerType === "touch" || !(event.target instanceof Element)) return;
  const option = event.target.closest(CB_OPTION);
  if (!option || option.hasAttribute("data-active")) return;
  const parts = comboboxParts(option);
  if (parts && parts.popup.contains(option)) cbActivate(parts, option);
});

document.addEventListener("focusout", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts || parts.root.contains(event.relatedTarget)) return;
  cbClose(parts);
  cbSettle(parts);
});

// With one value, focusing the input selects its text, so that typing searches
// afresh rather than adding to the chosen label. The mouseup after a click's focus
// would put the caret back, so it is skipped once.
let cbJustFocused = false;

document.addEventListener("focusin", (event) => {
  const parts = comboboxParts(event.target);
  if (!parts || event.target !== parts.input || parts.mode !== "single") return;
  parts.input.select();
  cbJustFocused = true;
});

document.addEventListener("mouseup", (event) => {
  const parts = comboboxParts(event.target);
  if (parts && event.target === parts.input && cbJustFocused) event.preventDefault();
  cbJustFocused = false;
});
