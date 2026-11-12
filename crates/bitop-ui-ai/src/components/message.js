// Message: exclusive feedback toggles, and switching between versions of a response.

// The base set's toggle script has already flipped the button (its script loads first).
// A pressed exclusive action releases the others of its group.
document.addEventListener("click", (event) => {
  const action = event.target instanceof Element && event.target.closest("[data-message-exclusive][data-toggle]");
  if (!action || action.getAttribute("aria-pressed") !== "true") return;
  const group = action.closest('[role="group"]') ?? action.parentElement;
  for (const other of group.querySelectorAll('[data-message-exclusive][aria-pressed="true"]')) {
    if (other !== action) window.bitopUi.setPressed(other, false);
  }
});

const ownedBy = (root) => (element) => element.closest("[data-message-branch]") === root;

function branchParts(root) {
  const mine = ownedBy(root);
  const content = [...root.querySelectorAll("[data-message-branch-content]")].find(mine);
  return {
    versions: content ? [...content.children] : [],
    selectors: [...root.querySelectorAll("[data-message-branch-selector]")].filter(mine),
  };
}

// Shows version `index` of a branch, kept inside the versions there are. Returns the
// index shown.
function showBranch(root, index) {
  const { versions, selectors } = branchParts(root);
  const count = versions.length;
  const shown = count ? Math.min(Math.max(index, 0), count - 1) : 0;
  root.dataset.index = String(shown);
  versions.forEach((version, i) => (version.hidden = i !== shown));
  for (const selector of selectors) {
    selector.hidden = count < 2;
    const page = selector.querySelector("[data-branch-page]");
    if (page) page.textContent = `${shown + 1} / ${count}`;
    const status = selector.querySelector("[data-branch-status]");
    if (status) status.textContent = selector.dataset.statusLabel.replace("{index}", String(shown + 1)).replace("{count}", String(count));
    const previous = selector.querySelector('[data-branch-step="-1"]');
    const next = selector.querySelector('[data-branch-step="1"]');
    if (previous) previous.disabled = shown === 0;
    if (next) next.disabled = shown >= count - 1;
  }
  root.toggleAttribute("data-ready", true);
  return shown;
}

function changed(root, index) {
  root.dispatchEvent(new CustomEvent("bitop:branch-change", { bubbles: true, detail: { index } }));
}

// Applies the server's index, and reports it if it had to be clamped.
function settleBranch(root) {
  const wanted = Number(root.dataset.index) || 0;
  const shown = showBranch(root, wanted);
  if (shown !== wanted) changed(root, shown);
}

document.querySelectorAll("[data-message-branch]").forEach(settleBranch);

document.addEventListener("click", (event) => {
  const step = event.target instanceof Element && event.target.closest("[data-branch-step]");
  if (!step || step.disabled) return;
  const root = step.closest("[data-message-branch]");
  if (!root) return;
  const direction = step.dataset.branchStep;
  const shown = showBranch(root, Number(root.dataset.index) + Number(direction));
  changed(root, shown);
  // The button pressed may now be disabled, or hidden with the version it was in: keep
  // focus on the selector that is shown.
  const focused = document.activeElement;
  if (focused === step && !step.disabled && step.checkVisibility()) return;
  const selector = branchParts(root).selectors.find((s) => s.checkVisibility()) ?? step.closest("[data-message-branch-selector]");
  const same = selector?.querySelector(`[data-branch-step="${direction}"]`);
  const other = selector?.querySelector(`[data-branch-step="${-Number(direction)}"]`);
  (same && !same.disabled ? same : other)?.focus();
});

// Versions added or removed later, by a server streaming into the page.
new MutationObserver((records) => {
  const roots = new Set();
  for (const record of records) {
    const target = record.target instanceof Element ? record.target : null;
    if (!target) continue;
    const content = target.closest("[data-message-branch-content]");
    if (content && record.target === content) roots.add(content.closest("[data-message-branch]"));
    for (const node of record.addedNodes) {
      if (!(node instanceof Element)) continue;
      if (node.matches("[data-message-branch]")) roots.add(node);
      node.querySelectorAll("[data-message-branch]").forEach((root) => roots.add(root));
    }
  }
  for (const root of roots) if (root) settleBranch(root);
}).observe(document.documentElement, { subtree: true, childList: true });
