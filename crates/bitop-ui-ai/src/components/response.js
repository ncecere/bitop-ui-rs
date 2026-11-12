// Response: an image from another host loads when its button is pressed; an app that
// streams can put changed blocks in place with bitopUi.patchResponse.

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-response-image]");
  if (!button) return;
  const frame = document.createElement("span");
  frame.className = button.dataset.frameClass ?? "";
  frame.tabIndex = -1;
  const img = document.createElement("img");
  img.src = button.dataset.responseImage;
  img.alt = button.dataset.alt ?? "";
  img.className = button.dataset.imageClass ?? "";
  frame.append(img);
  button.replaceWith(frame);
  // The button is gone: keep focus where it was.
  frame.focus();
});

// Puts rendered blocks into a response: `patches` is a list of { index, html }, where
// html is one block as response_block renders it. Blocks at or past `len` are removed.
window.bitopUi.patchResponse = (root, patches, len) => {
  const block = (index) => root.querySelector(`:scope > [data-block="${index}"]`);
  for (const { index, html } of patches) {
    const template = document.createElement("template");
    template.innerHTML = html;
    const fresh = template.content.firstElementChild;
    if (!fresh) continue;
    const old = block(index);
    if (old) {
      old.replaceWith(fresh);
      continue;
    }
    // A new block goes after the last one before it.
    let before;
    for (const child of root.querySelectorAll(":scope > [data-block]")) {
      if (Number(child.dataset.block) > index) {
        before = child;
        break;
      }
    }
    root.insertBefore(fresh, before ?? null);
  }
  if (len !== undefined) {
    for (const child of root.querySelectorAll(":scope > [data-block]")) {
      if (Number(child.dataset.block) >= len) child.remove();
    }
  }
};
