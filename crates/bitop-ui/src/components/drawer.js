// Drawer: drag it toward its edge to dismiss it. A touch can start on the handle or the
// header; a mouse only on the handle, so that text in the drawer can still be selected.

const SWIPE = {
  bottom: { axis: "y", sign: 1 },
  top: { axis: "y", sign: -1 },
  left: { axis: "x", sign: -1 },
  right: { axis: "x", sign: 1 },
};
// How far the pointer moves before a press becomes a swipe, in pixels.
const SWIPE_SLOP = 4;
let swipe;

function swipeOffset(event) {
  const moved = swipe.axis === "y" ? event.clientY - swipe.startY : event.clientX - swipe.startX;
  // Dragging away from the edge does nothing.
  return Math.max(0, moved * swipe.sign);
}

function endSwipe() {
  const { drawer, axis } = swipe;
  drawer.removeAttribute("data-swiping");
  drawer.style.removeProperty(`--drawer-swipe-movement-${axis}`);
  drawer.style.removeProperty("--drawer-swipe-progress");
  swipe = undefined;
}

document.addEventListener("pointerdown", (event) => {
  if (swipe || event.button !== 0 || !(event.target instanceof Element)) return;
  const drawer = event.target.closest("dialog[data-drawer]");
  const grip = event.target.closest(
    event.pointerType === "mouse" ? "[data-drawer-handle]" : "[data-drawer-handle], [data-drawer-header]",
  );
  if (!drawer || !grip || event.target.closest("button, a, input, select, textarea")) return;
  swipe = {
    drawer,
    ...SWIPE[drawer.dataset.side ?? "bottom"],
    pointer: event.pointerId,
    startX: event.clientX,
    startY: event.clientY,
    startedAt: event.timeStamp,
    active: false,
  };
});

document.addEventListener("pointermove", (event) => {
  if (!swipe || event.pointerId !== swipe.pointer) return;
  const offset = swipeOffset(event);
  if (!swipe.active) {
    if (offset < SWIPE_SLOP) return;
    swipe.active = true;
    swipe.drawer.setAttribute("data-swiping", "");
  }
  const { drawer, axis, sign } = swipe;
  const box = drawer.getBoundingClientRect();
  const size = axis === "y" ? box.height : box.width;
  drawer.style.setProperty(`--drawer-swipe-movement-${axis}`, `${offset * sign}px`);
  drawer.style.setProperty("--drawer-swipe-progress", String(Math.min(1, offset / size)));
});

function releaseSwipe(event) {
  if (!swipe || event.pointerId !== swipe.pointer) return;
  const { drawer, axis, active, startedAt } = swipe;
  if (!active) {
    swipe = undefined;
    return;
  }
  const offset = swipeOffset(event);
  const box = drawer.getBoundingClientRect();
  const size = axis === "y" ? box.height : box.width;
  const speed = offset / Math.max(1, event.timeStamp - startedAt);
  // Released a third of the way out, or flicked: close. Otherwise it springs back.
  const dismiss = event.type === "pointerup" && (offset > size / 3 || (speed > 0.5 && offset > 24));
  endSwipe();
  if (dismiss) drawer.close();
}

document.addEventListener("pointerup", releaseSwipe);
document.addEventListener("pointercancel", releaseSwipe);
