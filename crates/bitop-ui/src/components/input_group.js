// Input group: a click on an addon, other than on something interactive in it,
// focuses the group's control.
document.addEventListener("click", (event) => {
  if (event.defaultPrevented || !(event.target instanceof Element)) return;
  const addon = event.target.closest("[data-input-group-addon]");
  if (!addon || event.target.closest("button, a, input, textarea, select, [role='button']")) return;
  addon.closest("[data-input-group]")?.querySelector("[data-group-control]")?.focus();
});
