// Date picker: a button, and a calendar in a popover. The calendar keeps the value
// (calendar.js); this script keeps the button's text and the presets in step with it,
// moves focus into the calendar, and closes the popup when a date has been chosen.

function pickerParts(target) {
  const root = target instanceof Element && target.closest("[data-date-picker]");
  if (!root) return undefined;
  return {
    root,
    trigger: root.querySelector(":scope > [data-date-picker-trigger]"),
    popup: root.querySelector(":scope > [popover]"),
    calendar: root.querySelector("[data-calendar]"),
  };
}

// A date as the button shows it, from the picker's format and the calendar's names.
function pickerText({ root, calendar }, key) {
  const [year, month, day] = key.split("-").map(Number);
  return root.dataset.format
    .replace("{month}", calendar.dataset.monthNamesShort.split("|")[month - 1])
    .replace("{day}", day)
    .replace("{year}", year);
}

document.addEventListener("bitop:change", (event) => {
  const parts = pickerParts(event.target);
  if (!parts || event.target !== parts.calendar) return;
  const { root, trigger, popup } = parts;
  const range = root.dataset.mode === "range";
  const [from, to] = event.detail.values;
  const separator = root.dataset.rangeSeparator;
  let text = root.dataset.placeholder;
  if (from && !range) text = pickerText(parts, from);
  else if (from) text = `${pickerText(parts, from)}${separator}${to ? pickerText(parts, to) : "…"}`;
  trigger.querySelector(`#${CSS.escape(trigger.id)}-value`).textContent = text;
  // The button's name is its label followed by the value.
  trigger.setAttribute("aria-label", `${root.dataset.label} ${text}`);
  trigger.toggleAttribute("data-placeholder", !from);
  for (const preset of popup.querySelectorAll("button[data-from]")) {
    preset.setAttribute("aria-pressed", String(preset.dataset.from === from && preset.dataset.to === to));
  }
  const complete = range ? !!to : !!from;
  if (complete && !root.hasAttribute("data-keep-open") && popup.matches(":popover-open")) {
    popup.hidePopover();
    trigger.focus();
  }
});

// A preset sets the whole range; the listener above then closes the popup.
document.addEventListener("click", (event) => {
  const preset = event.target instanceof Element && event.target.closest("[data-date-picker] button[data-from]");
  if (!preset) return;
  const { calendar } = pickerParts(preset);
  window.bitopUi.calendar.setValue(calendar, [preset.dataset.from, preset.dataset.to]);
});

// beforetoggle is sent as the popover opens or closes, so aria-expanded is never behind.
document.addEventListener(
  "beforetoggle",
  (event) => {
    const parts = pickerParts(event.target);
    if (parts && event.target === parts.popup) parts.trigger.setAttribute("aria-expanded", String(event.newState === "open"));
  },
  true,
);

// Opening moves focus to the selected day, or to today.
document.addEventListener(
  "toggle",
  (event) => {
    const parts = pickerParts(event.target);
    if (!parts || event.target !== parts.popup || event.newState !== "open") return;
    // The event is queued; keys pressed since the popup opened have already moved focus.
    if (parts.popup.matches(":popover-open") && !parts.popup.contains(document.activeElement)) {
      parts.calendar.querySelector('[data-day][tabindex="0"]')?.focus();
    }
  },
  true,
);

// Escape returns focus to the button. The browser would return it to what had focus
// before the popup opened, and in Safari a click does not focus a button.
document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape") return;
  const parts = pickerParts(event.target);
  if (!parts || !parts.popup.contains(event.target) || !parts.popup.matches(":popover-open")) return;
  event.preventDefault();
  parts.popup.hidePopover();
  parts.trigger.focus();
});
