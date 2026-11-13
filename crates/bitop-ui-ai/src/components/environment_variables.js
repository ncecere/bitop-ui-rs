// Environment variables: each row's toggle reveals or masks its value, and the switch
// in the header reveals or masks every row, resetting what the rows chose.

function setEnvShown(row, shown) {
  row.toggleAttribute("data-shown", shown);
  const value = row.querySelector("[data-env-value]");
  if (value) {
    value.toggleAttribute("data-masked", !shown);
    const mask = value.querySelector(":scope > [data-env-mask]");
    const plain = value.querySelector(":scope > [data-env-plain]");
    if (mask) mask.hidden = shown;
    if (plain) plain.hidden = !shown;
  }
  for (const button of row.querySelectorAll("[data-env-reveal]")) {
    button.setAttribute("aria-pressed", String(shown));
    button.querySelector("[data-icon]")?.setAttribute("data-icon", shown ? "eye-off" : "eye");
    const tip = button.dataset.tooltip && document.getElementById(button.dataset.tooltip);
    if (tip) tip.textContent = shown ? tip.dataset.hide : tip.dataset.show;
  }
}

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-env-reveal]");
  if (!button) return;
  const row = button.closest("[data-env-var]");
  if (row) setEnvShown(row, button.getAttribute("aria-pressed") !== "true");
});

document.addEventListener("change", (event) => {
  const toggle = event.target;
  if (!(toggle instanceof HTMLInputElement) || !toggle.hasAttribute("data-env-toggle")) return;
  const root = toggle.closest("[data-env]");
  if (!root) return;
  root.toggleAttribute("data-show-values", toggle.checked);
  for (const row of root.querySelectorAll("[data-env-var]")) setEnvShown(row, toggle.checked);
});
