// Sources: a selectable card tells the page which source was chosen.

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-source-select]");
  if (!button) return;
  const source = button.closest("[data-source]");
  const index = source?.dataset.index ? Number(source.dataset.index) : null;
  source?.dispatchEvent(new CustomEvent("bitop:source-select", { bubbles: true, detail: { index } }));
});
