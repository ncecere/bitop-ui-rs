// Number input: grouping while not editing, the plain number in the form, and the
// spin button keyboard pattern.

const ROOT = "[data-number-input]";
const controlOf = (root) => root.querySelector("input[role=spinbutton]");

function separators(locale) {
  const parts = new Intl.NumberFormat(locale).formatToParts(12345.6);
  return {
    group: parts.find((part) => part.type === "group")?.value ?? ",",
    decimal: parts.find((part) => part.type === "decimal")?.value ?? ".",
  };
}

// "50,000" becomes "50000", and "1,5" in German becomes "1.5". Text that is not a
// number comes back without separators only.
function parse(text, locale) {
  const { group, decimal } = separators(locale);
  let out = text.replace(/[\s  ]/g, "");
  if (group.trim() !== "") out = out.split(group).join("");
  if (decimal !== ".") out = out.split(decimal).join(".");
  return out;
}

// "50000" becomes "50,000". Text that is not a plain number, or that has more
// fraction digits than are shown, comes back unchanged, so rounding never hides what
// was typed.
function format(value, locale, digits) {
  const match = /^-?\d+(?:\.(\d+))?$/.exec(value.trim());
  if (!match || (match[1]?.length ?? 0) > digits) return value;
  // The decimal string itself is formatted: Number() would round past 2^53.
  return new Intl.NumberFormat(locale, { maximumFractionDigits: digits, useGrouping: true }).format(value.trim());
}

function settings(root) {
  return {
    locale: root.dataset.locale || undefined,
    digits: Number(root.dataset.fractionDigits ?? 3),
    step: Number(root.dataset.step) || 1,
  };
}

const isNumber = (plain) => /^-?\d+(?:\.\d+)?$/.test(plain);
const decimals = (text) => (String(text).split(".")[1] ?? "").length;
const bound = (input, name) => {
  const text = input.getAttribute(name);
  return text === null || text === "" ? undefined : Number(text);
};

// The plain value. The server renders it plain; once the script has shown it, or the
// person has typed, the text is in the locale's form, which data-local records.
function plainValue(root) {
  const input = controlOf(root);
  return "local" in input.dataset ? parse(input.value, settings(root).locale) : input.value.trim();
}

// Shows `plain` in the form that suits the focus state, and reports it as the value.
function show(root, plain) {
  const input = controlOf(root);
  const { locale, digits } = settings(root);
  const text =
    document.activeElement === input
      ? plain.replace(".", separators(locale).decimal)
      : format(plain, locale, digits);
  input.dataset.local = "";
  if (input.value !== text) input.value = text;
  if (isNumber(plain)) input.setAttribute("aria-valuenow", plain);
  else input.removeAttribute("aria-valuenow");
}

function stepBy(root, count) {
  const input = controlOf(root);
  if (input.disabled || input.readOnly) return;
  const { step } = settings(root);
  const plain = plainValue(root);
  const min = bound(input, "aria-valuemin");
  const max = bound(input, "aria-valuemax");
  let next;
  if (count === -Infinity) next = min;
  else if (count === Infinity) next = max;
  else next = (isNumber(plain) ? Number(plain) : (min ?? 0)) + count * step;
  if (next === undefined) return;
  if (min !== undefined) next = Math.max(min, next);
  if (max !== undefined) next = Math.min(max, next);
  const places = Math.max(decimals(root.dataset.step ?? ""), isNumber(plain) ? decimals(plain) : 0);
  show(root, next.toFixed(places));
  input.dispatchEvent(new Event("input", { bubbles: true }));
  input.dispatchEvent(new Event("change", { bubbles: true }));
}

for (const root of document.querySelectorAll(ROOT)) show(root, plainValue(root));

document.addEventListener("focusin", (event) => {
  const root = event.target instanceof Element && event.target.closest(ROOT);
  if (!root || event.target !== controlOf(root)) return;
  const before = event.target.value;
  show(root, plainValue(root));
  // Taking the grouping out drops the selection that Tab makes; put it back. A click
  // still places the caret, because it does so after this.
  if (event.target.value !== before) event.target.select();
});

document.addEventListener("focusout", (event) => {
  const root = event.target instanceof Element && event.target.closest(ROOT);
  if (root && event.target === controlOf(root)) show(root, plainValue(root));
});

document.addEventListener("input", (event) => {
  const root = event.target instanceof Element && event.target.closest(ROOT);
  if (!root || event.target !== controlOf(root)) return;
  event.target.dataset.local = "";
  const plain = plainValue(root);
  if (isNumber(plain)) event.target.setAttribute("aria-valuenow", plain);
  else event.target.removeAttribute("aria-valuenow");
});

document.addEventListener("keydown", (event) => {
  const root = event.target instanceof Element && event.target.closest(ROOT);
  if (!root || event.target !== controlOf(root) || event.altKey || event.ctrlKey || event.metaKey) return;
  const count = { ArrowUp: 1, ArrowDown: -1, PageUp: 10, PageDown: -10, Home: -Infinity, End: Infinity }[event.key];
  if (count === undefined) return;
  // Home and End move the caret unless the input has a minimum or a maximum.
  if (count === -Infinity && bound(event.target, "aria-valuemin") === undefined) return;
  if (count === Infinity && bound(event.target, "aria-valuemax") === undefined) return;
  event.preventDefault();
  stepBy(root, count);
});

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-number-step]");
  const root = button && button.closest(ROOT);
  if (root) stepBy(root, Number(button.dataset.numberStep));
});

// The form receives the plain number, whatever the input is showing.
document.addEventListener("formdata", (event) => {
  for (const root of document.querySelectorAll(ROOT)) {
    const input = controlOf(root);
    if (input.form === event.target && input.name && event.formData.has(input.name)) {
      event.formData.set(input.name, plainValue(root));
    }
  }
});
