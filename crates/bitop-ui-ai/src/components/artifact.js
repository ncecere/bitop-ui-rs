// Artifact: a close or action button that is not a submit button tells the page.
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-artifact-action]");
  if (!button || button.type === "submit" || button.disabled) return;
  const root = button.closest("[data-artifact]");
  const kind = button.dataset.artifactAction === "close" ? "close" : "action";
  root?.dispatchEvent(new CustomEvent(`bitop:artifact-${kind}`, { bubbles: true, detail: { action: button.dataset.artifactAction } }));
});
