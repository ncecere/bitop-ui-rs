// The dismiss button removes its alert.
document.addEventListener("click", (event) => {
  const dismiss = event.target instanceof Element && event.target.closest("[data-alert-dismiss]");
  if (dismiss) dismiss.closest("[data-alert]")?.remove();
});
