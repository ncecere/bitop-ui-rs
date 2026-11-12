// One-time-code field: the slots behave as one field.

const ROOT = "[data-otp]";
const slotsOf = (root) => [...root.querySelectorAll("[data-otp-slot]")];

function accepted(root, text) {
  const pattern = root.hasAttribute("data-alphanumeric") ? /[^\p{L}\p{N}]/gu : /[^0-9]/g;
  return text.replace(pattern, "");
}

// Copies the slots into the hidden input that is submitted.
function sync(root) {
  const hidden = root.querySelector(":scope > input[type=hidden]");
  const code = slotsOf(root)
    .map((slot) => slot.value)
    .join("");
  if (!hidden || hidden.value === code) return;
  hidden.value = code;
  hidden.dispatchEvent(new Event("change", { bubbles: true }));
}

// Writes `text` into the slots from `start` on, and focuses the slot after it.
function fill(root, start, text) {
  const slots = slotsOf(root);
  const chars = [...text].slice(0, slots.length - start);
  chars.forEach((char, offset) => (slots[start + offset].value = char));
  const next = slots[Math.min(start + chars.length, slots.length - 1)];
  next.focus();
  next.select();
  sync(root);
}

function slotFor(event) {
  const slot = event.target instanceof Element && event.target.closest("[data-otp-slot]");
  const root = slot && slot.closest(ROOT);
  return root ? [root, slot, slotsOf(root)] : [];
}

document.addEventListener("input", (event) => {
  const [root, slot, slots] = slotFor(event);
  if (!root) return;
  const typed = event.inputType === "insertText" && [...(event.data ?? "")].length === 1;
  // One typed character replaces the slot's; anything longer is a pasted or filled-in code.
  const text = accepted(root, typed ? event.data : slot.value);
  slot.value = "";
  if (text) fill(root, slots.indexOf(slot), text);
  else sync(root);
});

document.addEventListener("paste", (event) => {
  const [root, slot, slots] = slotFor(event);
  if (!root) return;
  event.preventDefault();
  const text = accepted(root, event.clipboardData?.getData("text") ?? "");
  // A whole code replaces everything, wherever the caret is.
  if (text) fill(root, text.length >= slots.length ? 0 : slots.indexOf(slot), text);
});

// A focused slot is selected, so that typing replaces its character.
document.addEventListener("focusin", (event) => {
  const [root, slot] = slotFor(event);
  if (root) slot.select();
});

document.addEventListener("keydown", (event) => {
  const [root, slot, slots] = slotFor(event);
  if (!root || event.altKey || event.ctrlKey || event.metaKey) return;
  const index = slots.indexOf(slot);
  // In a right-to-left page the arrow keys follow the visual order.
  const forward = getComputedStyle(root).direction === "rtl" ? "ArrowLeft" : "ArrowRight";
  const backward = forward === "ArrowLeft" ? "ArrowRight" : "ArrowLeft";
  let next;
  if (event.key === "Backspace") {
    // An empty slot hands Backspace to the one before it.
    next = slot.value === "" ? slots[index - 1] : slot;
    if (next) next.value = "";
    sync(root);
  } else if (event.key === "Delete") {
    slot.value = "";
    sync(root);
    next = slot;
  } else if (event.key === forward) next = slots[index + 1];
  else if (event.key === backward) next = slots[index - 1];
  else if (event.key === "Home") next = slots[0];
  else if (event.key === "End") next = slots.at(-1);
  else return;
  event.preventDefault();
  next?.focus();
  next?.select();
});
