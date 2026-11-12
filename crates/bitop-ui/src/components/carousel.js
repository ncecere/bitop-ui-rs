// Carousel: previous and next buttons, dots, arrow keys and announcements for a
// scroll-snap track. The browser does the scrolling and the snapping.

const CAROUSEL = "[data-carousel]";
const carousels = new WeakMap();

function carouselTrack(root) {
  return root.querySelector("[data-carousel-track]");
}

function carouselText(template, index, count) {
  return template.replace("{index}", String(index + 1)).replace("{count}", String(count));
}

// How far each slide is from the start edge of the track, in the reading direction.
function carouselGeometry(root) {
  const track = carouselTrack(root);
  if (!track) return undefined;
  const horizontal = root.dataset.orientation !== "vertical";
  const rtl = horizontal && getComputedStyle(track).direction === "rtl";
  const box = track.getBoundingClientRect();
  const offset = (slide) => {
    const r = slide.getBoundingClientRect();
    if (!horizontal) return r.top - box.top;
    return rtl ? box.right - r.right : r.left - box.left;
  };
  return { track, horizontal, rtl, offset };
}

// Writes the state into the page: the current slide, the buttons at the ends, the
// current dot, and the slides' labels.
function renderCarousel(root) {
  const state = carousels.get(root);
  const track = carouselTrack(root);
  if (!state || !track) return;
  const slides = [...track.children];
  slides.forEach((slide, index) => {
    slide.toggleAttribute("data-current", index === state.index);
    if (!slide.hasAttribute("aria-label") || slide.hasAttribute("data-carousel-labelled")) {
      slide.setAttribute("data-carousel-labelled", "");
      slide.setAttribute("aria-label", carouselText(root.dataset.slideLabel, index, slides.length));
    }
  });
  const canPrevious = state.index > 0 && state.edges?.start !== true;
  const canNext = state.index < slides.length - 1 && state.edges?.end !== true;
  for (const button of root.querySelectorAll("[data-carousel-step]")) {
    if (button.closest(CAROUSEL) !== root) continue;
    const enabled = button.dataset.carouselStep === "prev" ? canPrevious : canNext;
    // Still focusable at the ends, so that focus is not lost; it does nothing.
    if (enabled) button.removeAttribute("aria-disabled");
    else button.setAttribute("aria-disabled", "true");
  }
  for (const dot of root.querySelectorAll("[data-carousel-indicator]")) {
    if (dot.closest(CAROUSEL) !== root) continue;
    if (Number(dot.dataset.carouselIndicator) === state.index) dot.setAttribute("aria-current", "true");
    else dot.removeAttribute("aria-current");
  }
  if (state.reported !== state.index) {
    const first = state.reported === undefined;
    state.reported = state.index;
    if (!first) root.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { index: state.index } }));
  }
}

// Reads the scroll position: which slide is nearest the start edge, and whether the
// track is at either end.
function measureCarousel(root) {
  const state = carousels.get(root);
  const geometry = carouselGeometry(root);
  if (!state || !geometry) return;
  const { track, horizontal, offset } = geometry;
  const size = horizontal ? track.clientWidth : track.clientHeight;
  if (size === 0) {
    // Not laid out, so nothing to measure: the index alone decides.
    state.edges = undefined;
    renderCarousel(root);
    return;
  }
  const max = horizontal ? track.scrollWidth - track.clientWidth : track.scrollHeight - track.clientHeight;
  const position = Math.abs(horizontal ? track.scrollLeft : track.scrollTop);
  // Two pixels of slack: slides can be a fraction of a pixel wide, and the scroll
  // position is then a fraction short of the end.
  state.edges = { start: position <= 2, end: position >= max - 2 };
  let nearest = 0;
  let best = Infinity;
  [...track.children].forEach((slide, index) => {
    const distance = Math.abs(offset(slide));
    if (distance < best) {
      best = distance;
      nearest = index;
    }
  });
  // While the track glides to a slide, that slide stays the current one; the slides
  // it passes are not reported.
  if (state.target !== undefined) {
    if (nearest !== state.target) {
      renderCarousel(root);
      return;
    }
    state.target = undefined;
  }
  state.index = nearest;
  renderCarousel(root);
}

function scrollCarousel(root, to, { announce = false, instant = false } = {}) {
  const state = carouselState(root);
  const geometry = carouselGeometry(root);
  if (!geometry) return;
  const { track, horizontal, rtl, offset } = geometry;
  const count = track.children.length;
  if (count === 0) return;
  const index = Math.min(Math.max(0, to), count - 1);
  const delta = offset(track.children[index]);
  const still = instant || matchMedia("(prefers-reduced-motion: reduce)").matches;
  const behavior = still ? "instant" : "smooth";
  track.scrollBy(horizontal ? { left: rtl ? -delta : delta, behavior } : { top: delta, behavior });
  state.target = index;
  clearTimeout(state.timer);
  state.timer = setTimeout(() => {
    state.target = undefined;
    measureCarousel(root);
  }, 1000);
  state.index = index;
  if (announce) {
    const status = [...root.children].find((child) => child.matches('[role="status"]'));
    if (status) status.textContent = carouselText(root.dataset.announcement, index, count);
  }
  renderCarousel(root);
}

// The state of a carousel, made when the carousel is first seen, so that one the
// server adds later works too.
function carouselState(root) {
  let state = carousels.get(root);
  if (state) return state;
  state = { index: 0, target: undefined, edges: undefined, timer: undefined, reported: undefined, frame: 0 };
  carousels.set(root, state);
  const first = Number(root.dataset.index) || 0;
  if (first > 0) scrollCarousel(root, first, { instant: true });
  measureCarousel(root);
  return state;
}

function carouselOf(target) {
  const root = target instanceof Element && target.closest(CAROUSEL);
  if (!root) return undefined;
  carouselState(root);
  return root;
}

for (const root of document.querySelectorAll(CAROUSEL)) carouselState(root);

window.bitopUi.carousel = {
  scrollTo: (root, index) => scrollCarousel(root, index, { announce: true }),
  index: (root) => carouselState(root).index,
};

// scroll does not bubble, but it can be captured on its way down.
document.addEventListener(
  "scroll",
  (event) => {
    const track = event.target;
    if (!(track instanceof Element) || !track.matches("[data-carousel-track]")) return;
    const root = carouselOf(track);
    if (!root) return;
    const state = carousels.get(root);
    cancelAnimationFrame(state.frame);
    state.frame = requestAnimationFrame(() => measureCarousel(root));
  },
  { capture: true, passive: true },
);

window.addEventListener("resize", () => {
  for (const root of document.querySelectorAll(CAROUSEL)) {
    carouselState(root);
    measureCarousel(root);
  }
});

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const control = event.target.closest("[data-carousel-step], [data-carousel-indicator]");
  const root = control && carouselOf(control);
  if (!root) return;
  const state = carousels.get(root);
  if (control.hasAttribute("data-carousel-indicator")) {
    scrollCarousel(root, Number(control.dataset.carouselIndicator), { announce: true });
  } else if (control.getAttribute("aria-disabled") !== "true") {
    scrollCarousel(root, state.index + (control.dataset.carouselStep === "prev" ? -1 : 1), { announce: true });
  }
});

document.addEventListener("keydown", (event) => {
  if (event.defaultPrevented || event.ctrlKey || event.metaKey || event.altKey) return;
  if (!(event.target instanceof Element)) return;
  if (event.target.closest("input, textarea, select, [contenteditable='true']")) return;
  const root = carouselOf(event.target);
  const geometry = root && carouselGeometry(root);
  if (!geometry) return;
  const { horizontal, rtl } = geometry;
  // In a right-to-left page the arrow keys follow what is seen.
  const previousKey = horizontal ? (rtl ? "ArrowRight" : "ArrowLeft") : "ArrowUp";
  const nextKey = horizontal ? (rtl ? "ArrowLeft" : "ArrowRight") : "ArrowDown";
  if (event.key !== previousKey && event.key !== nextKey) return;
  // Handled even at an end, so that the track does not scroll by itself.
  event.preventDefault();
  const step = root.querySelector(`[data-carousel-step="${event.key === previousKey ? "prev" : "next"}"]`);
  const state = carousels.get(root);
  const count = geometry.track.children.length;
  const blocked =
    event.key === previousKey
      ? state.index <= 0 || state.edges?.start === true
      : state.index >= count - 1 || state.edges?.end === true;
  if (blocked || step?.getAttribute("aria-disabled") === "true") return;
  scrollCarousel(root, state.index + (event.key === previousKey ? -1 : 1), { announce: true });
});
