// Slider: keeps the filled part of the track, the readout and the value text in
// step with the thumbs. A range keeps its thumbs apart, and a press on its track
// moves the nearer thumb and drags it.

const sliderInputs = (root) => [...root.querySelectorAll("input[type=range]")];

function sliderUpdate(input) {
  const root = input.closest("[data-slider]");
  const inputs = sliderInputs(root);
  const min = Number(input.min || 0);
  const max = Number(input.max || 100);
  const at = (one) => (max > min ? (one.valueAsNumber - min) / (max - min) : 0);
  const first = inputs[0];
  if (inputs.length > 1) {
    first.style.setProperty("--slider-from", String(at(first)));
    first.style.setProperty("--slider-fill", String(at(inputs[1])));
  } else {
    first.style.setProperty("--slider-fill", String(at(first)));
  }
  const template = input.dataset.valueText;
  if (template) input.setAttribute("aria-valuetext", template.replace("{value}", input.value));
  const output = root.querySelector("output");
  if (output) {
    const format = output.dataset.format || "{value}";
    output.textContent = inputs.map((one) => format.replace("{value}", one.value)).join(" – ");
  }
}

// Keeps a range's thumbs `data-min-steps` steps apart: the moved one stops there.
function sliderKeepApart(input) {
  const control = input.closest("[data-range]");
  if (!control) return;
  const [start, end] = sliderInputs(control);
  const gap = Number(control.dataset.minSteps || 0) * (Number(input.step) || 1);
  if (input === start && start.valueAsNumber > end.valueAsNumber - gap) start.value = String(end.valueAsNumber - gap);
  if (input === end && end.valueAsNumber < start.valueAsNumber + gap) end.value = String(start.valueAsNumber + gap);
  for (const one of [start, end]) one.toggleAttribute("data-top", one === input);
}

document.addEventListener("input", (event) => {
  if (!(event.target instanceof Element) || !event.target.matches("[data-slider] input[type=range]")) return;
  sliderKeepApart(event.target);
  sliderUpdate(event.target);
});

// A form reset restores the values after the event, so wait for that.
document.addEventListener("reset", (event) => {
  const form = event.target;
  setTimeout(() => {
    for (const input of document.querySelectorAll("[data-slider] input[type=range]")) {
      if (input.form === form) sliderUpdate(input);
    }
  });
});

// The value under the pointer, from the input's own geometry: the thumb's centre
// travels between half a thumb from each end.
function sliderValueAt(input, event) {
  const box = input.getBoundingClientRect();
  const vertical = input.getAttribute("aria-orientation") === "vertical";
  const thumb = vertical ? box.width : box.height;
  const length = (vertical ? box.height : box.width) - thumb;
  let offset = vertical ? box.bottom - event.clientY : event.clientX - box.left;
  if (!vertical && getComputedStyle(input).direction === "rtl") offset = box.right - event.clientX;
  const fraction = Math.min(1, Math.max(0, (offset - thumb / 2) / (length || 1)));
  const min = Number(input.min || 0);
  const max = Number(input.max || 100);
  const step = Number(input.step) || 1;
  return min + Math.round((fraction * (max - min)) / step) * step;
}

function sliderSet(input, value) {
  const before = input.value;
  input.value = String(value);
  if (input.value === before) return;
  input.dispatchEvent(new Event("input", { bubbles: true }));
}

// A press on a range's track: the nearer thumb jumps there, takes focus, and follows
// the pointer until it is released. A press on a thumb is the browser's own drag.
document.addEventListener("pointerdown", (event) => {
  const control = event.target instanceof Element && event.target.closest("[data-slider] [data-range]");
  // Only a thumb takes the pointer on the inputs: a press there is the browser's drag.
  if (!control || event.button !== 0 || event.target.matches("input")) return;
  const inputs = sliderInputs(control);
  if (inputs.some((one) => one.disabled)) return;
  const value = sliderValueAt(inputs[0], event);
  const [start, end] = inputs;
  const nearer =
    Math.abs(value - start.valueAsNumber) < Math.abs(value - end.valueAsNumber) ||
    (value < start.valueAsNumber && start.valueAsNumber === end.valueAsNumber)
      ? start
      : end;
  event.preventDefault();
  nearer.focus();
  sliderSet(nearer, value);
  control.setPointerCapture(event.pointerId);
  control.dataset.dragging = "";
  const move = (moved) => sliderSet(nearer, sliderValueAt(nearer, moved));
  const stop = () => {
    control.removeEventListener("pointermove", move);
    control.removeEventListener("pointerup", stop);
    control.removeEventListener("pointercancel", stop);
    delete control.dataset.dragging;
    nearer.dispatchEvent(new Event("change", { bubbles: true }));
  };
  control.addEventListener("pointermove", move);
  control.addEventListener("pointerup", stop);
  control.addEventListener("pointercancel", stop);
});
