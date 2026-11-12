// Avatar: drop an image that failed to load, which leaves the initials.

document.addEventListener(
  "error",
  (event) => {
    if (event.target instanceof HTMLImageElement && event.target.matches("[data-avatar-image]")) {
      event.target.remove();
    }
  },
  true,
);

// Images that failed before this script ran.
for (const image of document.querySelectorAll("img[data-avatar-image]")) {
  if (image.complete && image.naturalWidth === 0) image.remove();
}
