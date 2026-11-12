// Image: the frame's data-state follows its <img>: loaded or error. Load events do not
// bubble, so they are heard in the capture phase; an image that finished before this
// script ran is settled from its `complete` flag.

function settleImage(img, failed) {
  const frame = img.closest("[data-image-frame]");
  if (!frame) return;
  frame.dataset.state = failed ? "error" : "loaded";
  frame.removeAttribute("aria-busy");
}

document.addEventListener("load", (event) => {
  if (event.target instanceof HTMLImageElement && event.target.matches("[data-image]")) settleImage(event.target, false);
}, true);

document.addEventListener("error", (event) => {
  if (event.target instanceof HTMLImageElement && event.target.matches("[data-image]")) settleImage(event.target, true);
}, true);

for (const img of document.querySelectorAll("[data-image-frame][data-state=loading] > img[data-image]")) {
  if (img.complete) settleImage(img, img.naturalWidth === 0);
}
