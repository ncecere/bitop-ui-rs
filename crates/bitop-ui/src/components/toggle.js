// Toggle: a click flips aria-pressed. A toggle in a toggle group is handled there.

// Sets the pressed state, and with it whether the toggle's hidden input is submitted.
window.bitopUi.setPressed = (toggle, pressed) => {
  toggle.setAttribute("aria-pressed", String(pressed));
  const input = toggle.querySelector(":scope > input[type=hidden]");
  if (input) input.disabled = !pressed;
};

document.addEventListener("click", (event) => {
  const toggle = event.target instanceof Element && event.target.closest("[data-toggle]");
  if (!toggle || toggle.disabled || toggle.closest("[data-toggle-group]")) return;
  window.bitopUi.setPressed(toggle, toggle.getAttribute("aria-pressed") !== "true");
});
