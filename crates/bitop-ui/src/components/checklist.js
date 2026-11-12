// Checklist: the dismiss button removes its checklist. An application that remembers
// the choice listens for `bitop-dismiss`; cancelling the event keeps the checklist.
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-checklist-dismiss]");
  const list = button && button.closest("[data-checklist]");
  if (!list) return;
  if (list.dispatchEvent(new CustomEvent("bitop-dismiss", { bubbles: true, cancelable: true }))) list.remove();
});
