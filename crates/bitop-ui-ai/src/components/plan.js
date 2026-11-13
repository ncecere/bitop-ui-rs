// Plan: the toggle shows and hides the steps, and the card follows data-streaming on
// its root, which an app that streams into the page sets and removes.

document.addEventListener("click", (event) => {
  const toggle = event.target instanceof Element && event.target.closest("[data-plan-toggle]");
  if (!toggle) return;
  const panel = document.getElementById(toggle.getAttribute("aria-controls"));
  if (!panel) return;
  const open = toggle.getAttribute("aria-expanded") !== "true";
  toggle.setAttribute("aria-expanded", String(open));
  panel.hidden = !open;
  const plan = toggle.closest("[data-plan]");
  plan?.toggleAttribute("data-open", open);
  plan?.dispatchEvent(new CustomEvent("bitop:plan-toggle", { bubbles: true, detail: { open } }));
});

function followStreaming(plan) {
  const streaming = plan.hasAttribute("data-streaming");
  if (streaming) plan.setAttribute("aria-busy", "true");
  else plan.removeAttribute("aria-busy");
  for (const shimmer of plan.querySelectorAll("[data-plan-header] [data-shimmer]")) {
    shimmer.toggleAttribute("data-active", streaming);
  }
}

new MutationObserver((records) => {
  for (const record of records) {
    if (record.target instanceof Element && record.target.matches("[data-plan]")) followStreaming(record.target);
  }
}).observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-streaming"] });
