// Colour field: keeps the swatch, the picker, the presets and the contrast line in
// step with the hex input.

const ROOT = "[data-color-field]";

// "#rrggbb" in lower case for "#RGB", "rrggbb" or "#RRGGBB"; undefined when not a colour.
function normalizeHex(value) {
  const hex = value.trim().replace(/^#/, "").toLowerCase();
  if (/^[0-9a-f]{6}$/.test(hex)) return `#${hex}`;
  if (/^[0-9a-f]{3}$/.test(hex)) return `#${[...hex].map((c) => c + c).join("")}`;
  return undefined;
}

function luminance(hex) {
  const n = parseInt(hex.slice(1), 16);
  const channel = (c) => {
    const s = c / 255;
    return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel((n >> 16) & 255) + 0.7152 * channel((n >> 8) & 255) + 0.0722 * channel(n & 255);
}

// Rounded down to one decimal, so that 4.46 never reads as a passing "4.5".
const formatRatio = (ratio) => `${(Math.floor(ratio * 10) / 10).toFixed(1)}:1`;

function update(root) {
  const input = root.querySelector("input[type=text]");
  const value = input.value.trim().toLowerCase();
  const hex = normalizeHex(value);
  const shown = hex ?? root.dataset.default;
  const invalid = value !== "" && !hex;

  root.querySelector("input[type=color]").value = shown;
  for (const part of root.querySelectorAll("[data-color-shown]")) part.style.backgroundColor = shown;
  if (invalid) input.setAttribute("aria-invalid", "true");
  else input.removeAttribute("aria-invalid");
  for (const preset of root.querySelectorAll("button[data-value]")) {
    preset.setAttribute("aria-pressed", String(hex !== undefined && preset.dataset.value === hex));
  }

  const line = root.querySelector("[data-pass-message]");
  const text = root.dataset.contrastWith;
  const ratio = text ? (Math.max(luminance(shown), luminance(text)) + 0.05) / (Math.min(luminance(shown), luminance(text)) + 0.05) : undefined;
  const passes = ratio !== undefined && ratio >= Number(root.dataset.minContrast);
  line.toggleAttribute("data-pass", passes && !invalid);
  let message = "";
  if (invalid) message = line.dataset.invalidMessage;
  else if (ratio !== undefined) {
    message = (passes ? line.dataset.passMessage : line.dataset.failMessage).replace("{ratio}", formatRatio(ratio));
  }
  line.querySelector("[data-color-message]").textContent = message;
}

// Sets the hex input, and tells the page that it changed.
function setValue(root, value) {
  const input = root.querySelector("input[type=text]");
  input.value = value;
  update(root);
  input.dispatchEvent(new Event("input", { bubbles: true }));
  input.dispatchEvent(new Event("change", { bubbles: true }));
}

document.addEventListener("input", (event) => {
  const root = event.target instanceof Element && event.target.closest(ROOT);
  if (!root) return;
  if (event.target.matches("input[type=color]")) setValue(root, event.target.value.toLowerCase());
  else update(root);
});

document.addEventListener("click", (event) => {
  const preset = event.target instanceof Element && event.target.closest("button[data-value]");
  const root = preset && preset.closest(ROOT);
  if (root) setValue(root, preset.dataset.value);
});
