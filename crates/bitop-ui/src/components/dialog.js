// Dialog: the backdrop click where the browser lacks closedby, focus on close
// (final_focus), and bitopUi.dialog for an alert dialog whose action runs in the page.

// Safari does not support <dialog closedby>. Where it is missing, close a
// closedby="any" dialog when its backdrop is clicked; the browser already handles Escape.
if (!("closedBy" in HTMLDialogElement.prototype)) {
  document.addEventListener("click", (event) => {
    const dialog = event.target;
    if (!(dialog instanceof HTMLDialogElement) || dialog.getAttribute("closedby") !== "any") return;
    // A click whose target is the dialog itself is on its padding or its backdrop.
    const box = dialog.getBoundingClientRect();
    const inside =
      event.clientX >= box.left &&
      event.clientX <= box.right &&
      event.clientY >= box.top &&
      event.clientY <= box.bottom;
    if (!inside) dialog.close();
  });
}

// `close` does not bubble. The browser has already returned focus to the trigger by
// the time it is sent, so this only moves it on.
document.addEventListener(
  "close",
  (event) => {
    const dialog = event.target;
    if (!(dialog instanceof HTMLDialogElement) || !dialog.dataset.finalFocus) return;
    document.getElementById(dialog.dataset.finalFocus)?.focus();
  },
  true,
);

window.bitopUi.dialog = {
  // The confirm button of an alert dialog shows a spinner and is disabled.
  setBusy(dialog, busy) {
    const confirm = dialog.querySelector("[data-dialog-confirm]");
    if (confirm) window.bitopUi.button.setLoading(confirm, busy);
  },
  // Shows `message` as the alert dialog's error, or hides the error when it is empty.
  setError(dialog, message) {
    const alert = dialog.querySelector("[data-dialog-error]");
    if (!alert) return;
    const content = alert.querySelector(":scope > div > div:last-child") ?? alert;
    content.textContent = message ?? "";
    alert.hidden = !message;
  },
};
