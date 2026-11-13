// A button with data-collapsible and aria-controls shows and hides the panel it names
// (the WAI-ARIA disclosure pattern), for parts whose trigger shares a row with other
// buttons and so cannot be a <summary>: commit, stack trace. The button's aria-expanded
// and data-open follow, and a bitop:collapsible-toggle event says the new state.

document.addEventListener("click", (event) => {
  const trigger = event.target instanceof Element && event.target.closest("[data-collapsible]");
  if (!trigger) return;
  const panel = document.getElementById(trigger.getAttribute("aria-controls"));
  if (!panel) return;
  const open = trigger.getAttribute("aria-expanded") !== "true";
  trigger.setAttribute("aria-expanded", String(open));
  trigger.toggleAttribute("data-open", open);
  panel.hidden = !open;
  trigger.dispatchEvent(new CustomEvent("bitop:collapsible-toggle", { bubbles: true, detail: { open } }));
});
