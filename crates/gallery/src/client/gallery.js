// Controls for the gallery shell. Not part of the library.

const root = document.documentElement;

// Mark the current page in the sidebar.
for (const link of document.querySelectorAll("nav a[href]")) {
  if (link.getAttribute("href") === location.pathname) link.setAttribute("aria-current", "page");
}

const theme = document.querySelector('[data-gallery-set="theme"]');
if (theme) theme.value = window.bitopUi.colorMode();

document.addEventListener("change", (event) => {
  const control = event.target;
  if (!(control instanceof HTMLSelectElement)) return;
  if (control.dataset.gallerySet === "dir") root.dir = control.value;
  if (control.dataset.gallerySet === "theme") window.bitopUi.setColorMode(control.value);
});

// Buttons on the toast page, and palette actions on the command page.
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-toast]");
  if (!button) return;
  const variant = button.dataset.galleryToast;
  window.bitopUi.toast({
    title: { neutral: "Link copied", success: "Changes saved", danger: "Could not save" }[variant],
    description: variant === "danger" ? "The server did not respond. Try again." : "",
    variant,
  });
});
