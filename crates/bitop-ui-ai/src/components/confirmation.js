// Confirmation: the actions send a form. Before it goes, a cancelable
// bitop:confirmation event tells the page, which may answer in script instead and set
// data-state on the card. When the state changes, the outcome is announced and focus
// that was on a button that has gone moves to the card.

const confirmationFocus = new WeakSet();

document.addEventListener("focusin", (event) => {
  const root = event.target instanceof Element && event.target.closest("[data-confirmation]");
  if (root) confirmationFocus.add(root);
});

document.addEventListener("focusout", (event) => {
  const root = event.target instanceof Element && event.target.closest("[data-confirmation]");
  if (root && !root.contains(event.relatedTarget)) confirmationFocus.delete(root);
});

document.addEventListener("submit", (event) => {
  const form = event.target;
  if (!(form instanceof HTMLFormElement) || !form.hasAttribute("data-confirmation-actions")) return;
  const root = form.closest("[data-confirmation]");
  if (!root || root.dataset.state !== "requested") {
    event.preventDefault();
    return;
  }
  const button = event.submitter;
  const proceed = root.dispatchEvent(
    new CustomEvent("bitop:confirmation", {
      bubbles: true,
      cancelable: true,
      detail: { name: button?.name ?? "", value: button?.value ?? "" },
    }),
  );
  if (!proceed) event.preventDefault();
});

function confirmationChanged(root) {
  const state = root.dataset.state;
  const live = root.querySelector("[data-confirmation-status]");
  const outcome = root.querySelector(`[data-confirmation-outcome][data-when="${CSS.escape(state ?? "")}"]`);
  if (live) live.textContent = outcome?.textContent.trim() ?? "";
  if (state === "requested") return;
  const active = document.activeElement;
  const lost = !root.contains(active) || active?.closest("[data-confirmation-actions]");
  if (confirmationFocus.has(root) && lost) root.focus({ preventScroll: true });
}

new MutationObserver((records) => {
  for (const record of records) {
    const root = record.target;
    if (root instanceof Element && root.matches("[data-confirmation]") && root.dataset.state !== record.oldValue) {
      confirmationChanged(root);
    }
  }
}).observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-state"], attributeOldValue: true });
