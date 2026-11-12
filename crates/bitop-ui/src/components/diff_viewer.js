// Diff viewer: the layout toggle switches between the two tables the server rendered,
// and a fold button shows the unchanged lines behind it.

document.addEventListener("click", (event) => {
  const target = event.target;
  if (!(target instanceof Element)) return;

  const item = target.closest("[data-diff-modes] [data-toggle]");
  if (item) {
    const root = item.closest("[data-diff-viewer]");
    const group = item.closest("[data-diff-modes]");
    if (!root) return;
    root.dataset.mode = item.dataset.value;
    // One layout is always chosen: after the toggle group has handled the click,
    // which unpresses a pressed item, press the chosen one again.
    setTimeout(() => {
      for (const other of group.querySelectorAll("[data-toggle]")) window.bitopUi.setPressed(other, other === item);
    });
    return;
  }

  const fold = target.closest("[data-diff-fold]");
  const root = fold?.closest("[data-diff-viewer]");
  if (!root) return;
  const id = fold.dataset.diffFold;
  // Both layouts fold the same lines; unfold both, so that switching keeps them.
  for (const row of root.querySelectorAll(`tr[data-fold="${id}"]`)) row.hidden = false;
  for (const row of root.querySelectorAll(`tr[data-fold-row="${id}"]`)) row.remove();
  // The button is gone: focus goes to the scroll region, where the arrow keys scroll.
  root.querySelector("[role=region]")?.focus();
});
