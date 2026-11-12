// Slider: keeps the filled part of the track, the readout and the value text in
// step with the thumb.
function update(input) {
  const min = Number(input.min || 0);
  const max = Number(input.max || 100);
  const fill = max > min ? (input.valueAsNumber - min) / (max - min) : 0;
  input.style.setProperty("--slider-fill", String(fill));
  const template = input.dataset.valueText;
  if (template) input.setAttribute("aria-valuetext", template.replace("{value}", input.value));
  const output = input.closest("[data-slider]").querySelector("output");
  if (output) output.textContent = (output.dataset.format || "{value}").replace("{value}", input.value);
}

document.addEventListener("input", (event) => {
  if (event.target instanceof Element && event.target.matches("[data-slider] input[type=range]")) update(event.target);
});

// A form reset restores the values after the event, so wait for that.
document.addEventListener("reset", (event) => {
  const form = event.target;
  setTimeout(() => {
    for (const input of document.querySelectorAll("[data-slider] input[type=range]")) {
      if (input.form === form) update(input);
    }
  });
});
