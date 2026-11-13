// Checkpoint: a restore button that is not a submit button tells the page, which rolls
// the chat back itself.
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-checkpoint-restore='']");
  if (!button || button.disabled) return;
  const root = button.closest("[data-checkpoint]");
  root?.dispatchEvent(new CustomEvent("bitop:checkpoint-restore", { bubbles: true, detail: { id: root.dataset.checkpointId ?? "" } }));
});
