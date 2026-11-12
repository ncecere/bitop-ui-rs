// Charts: the values of the point under the pointer or the keyboard, in a readout
// over the plot. The server writes each point's text in its slot's
// data-chart-value; the arrow keys move along the slots as they are laid out, so
// they follow a time axis that runs from the right.

function chartSlots(plot) {
  return [...plot.querySelectorAll("[data-chart-slot]")];
}

// The keyboard's point is kept in data-index; the pointer's is not, so that focus
// comes back to where the keys left it.
function chartShow(plot, index, keyboard) {
  const slots = chartSlots(plot);
  const slot = slots[index];
  const readout = plot.parentElement.querySelector("[data-chart-readout]");
  if (!slot || !readout) return;
  if (keyboard) plot.dataset.index = String(index);
  for (const other of slots) other.toggleAttribute("data-active", other === slot);
  if (readout.textContent !== slot.dataset.chartValue) readout.textContent = slot.dataset.chartValue;
  // Centre it over the slot, kept inside the frame.
  const frame = readout.parentElement.getBoundingClientRect();
  const box = slot.getBoundingClientRect();
  const half = readout.offsetWidth / 2;
  const centre = box.left + box.width / 2 - frame.left;
  readout.style.left = `${Math.min(Math.max(centre, half), frame.width - half)}px`;
}

function chartHide(plot) {
  for (const slot of chartSlots(plot)) slot.removeAttribute("data-active");
  const readout = plot.parentElement.querySelector("[data-chart-readout]");
  if (readout) readout.textContent = "";
}

document.addEventListener("pointerover", (event) => {
  const slot = event.target instanceof Element && event.target.closest("[data-chart-slot]");
  if (!slot) return;
  const plot = slot.closest("[data-chart-plot]");
  chartShow(plot, chartSlots(plot).indexOf(slot));
});

document.addEventListener("pointerout", (event) => {
  const plot = event.target instanceof Element && event.target.closest("[data-chart-plot]");
  if (!plot || plot.contains(event.relatedTarget)) return;
  // The keyboard's point stays shown while the plot has focus.
  if (plot.matches(":focus-visible")) chartShow(plot, Number(plot.dataset.index), true);
  else chartHide(plot);
});

document.addEventListener("focusin", (event) => {
  const plot = event.target instanceof Element && event.target.closest("[data-chart-plot]");
  if (!plot || !plot.matches(":focus-visible")) return;
  // Start at the last point, the latest of a time series.
  const index = plot.dataset.index ?? String(chartSlots(plot).length - 1);
  chartShow(plot, Number(index), true);
});

document.addEventListener("focusout", (event) => {
  const plot = event.target instanceof Element && event.target.closest("[data-chart-plot]");
  if (plot) chartHide(plot);
});

document.addEventListener("keydown", (event) => {
  const plot = event.target instanceof Element && event.target.closest("[data-chart-plot]");
  if (!plot || event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) return;
  const slots = chartSlots(plot);
  if (slots.length === 0) return;
  const last = slots.length - 1;
  const current = Number(plot.dataset.index ?? last);
  // Whether the first point is drawn on the right (a bar chart in right-to-left).
  const reversed = slots[0].getBoundingClientRect().left > slots[last].getBoundingClientRect().left;
  const step = { ArrowRight: 1, ArrowLeft: -1 }[event.key];
  let index;
  if (step) index = Math.min(Math.max(current + (reversed ? -step : step), 0), last);
  else if (event.key === "Home") index = 0;
  else if (event.key === "End") index = last;
  else if (event.key === "Escape") {
    chartHide(plot);
    return;
  } else return;
  event.preventDefault();
  chartShow(plot, index, true);
});
