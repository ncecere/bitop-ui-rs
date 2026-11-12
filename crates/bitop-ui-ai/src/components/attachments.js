// Attachments: a remove button without a form name takes its chip out of the list,
// after telling the page, and keeps focus in the list.

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest('[data-attachment-remove=""]');
  if (!button || button.disabled) return;
  const chip = button.closest("[data-attachment]");
  const list = chip?.closest("[data-attachments]");
  if (!chip || !list) return;
  const proceed = list.dispatchEvent(
    new CustomEvent("bitop:attachment-remove", { bubbles: true, cancelable: true, detail: { id: chip.dataset.attachmentId ?? "", chip } }),
  );
  if (!proceed) return;
  const chips = [...list.querySelectorAll(":scope > [data-attachment]")];
  const at = chips.indexOf(chip);
  const neighbour = chips[at + 1] ?? chips[at - 1];
  chip.remove();
  const next =
    neighbour?.querySelector("[data-attachment-remove]:not(:disabled)") ??
    (list.dataset.attachmentsFocus && document.getElementById(list.dataset.attachmentsFocus));
  next?.focus();
});
