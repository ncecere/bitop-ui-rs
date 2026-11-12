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
