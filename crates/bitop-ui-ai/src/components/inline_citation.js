// Inline citation: the chip toggles its card (popovertarget) and the card takes focus;
// hovering opens it without taking focus. The pager shows one source at a time. A
// chip with data-citation-activate, and the card's source action, go to an element
// elsewhere on the page instead.

const OPEN_DELAY = 150;
const CLOSE_DELAY = 150;
let hoverChip;
let openTimer;
let closeTimer;

const cardOf = (chip) => document.getElementById(chip.dataset.citationTrigger);
const chipOf = (card) => document.querySelector(`[data-citation-trigger="${CSS.escape(card.id)}"]`);

function anchorTo(chip) {
  for (const other of document.querySelectorAll("[data-citation-trigger][data-open]")) {
    if (other !== chip) {
      other.style.anchorName = "";
      other.removeAttribute("data-open");
    }
  }
  chip.style.anchorName = "--bitop-citation";
  chip.toggleAttribute("data-open", true);
}

// Shows page `n` of a card, kept inside the sources there are now: the server may
// have rendered fewer since the card was last opened.
function showPage(card, n) {
  const pages = [...card.querySelectorAll("[data-citation-page]")];
  if (!pages.length) return;
  const page = Math.min(Math.max(n, 0), pages.length - 1);
  card.dataset.page = String(page);
  pages.forEach((p, i) => (p.hidden = i !== page));
  const title = pages[page].querySelector("[id]");
  if (title) card.setAttribute("aria-labelledby", title.id);
  const index = card.querySelector("[data-citation-index]");
  if (index) {
    index.textContent = card.dataset.pageLabel.replace("{page}", String(page + 1)).replace("{count}", String(pages.length));
  }
  const previous = card.querySelector('[data-citation-step="-1"]');
  const next = card.querySelector('[data-citation-step="1"]');
  // A step button that becomes disabled would drop focus to the page: keep it in the card.
  const focused = document.activeElement;
  if (previous) previous.disabled = page === 0;
  if (next) next.disabled = page === pages.length - 1;
  if (focused?.disabled) {
    const other = focused === previous ? next : previous;
    if (other && !other.disabled) other.focus();
    else card.focus();
  }
}

// Opens the element with this ID: the <details> around it first, then focus. Content
// of a <details> that has just opened cannot take focus until the next frame in
// Chromium (the content box is still hidden while its transition starts), so focus is
// tried again for a few frames.
function goTo(id) {
  const target = document.getElementById(id);
  if (!target) return;
  for (let details = target.closest("details"); details; details = details.parentElement?.closest("details")) {
    details.open = true;
  }
  let tries = 10;
  const attempt = () => {
    target.focus({ preventScroll: true });
    if (document.activeElement !== target && tries-- > 0) requestAnimationFrame(attempt);
    else target.scrollIntoView({ block: "nearest" });
  };
  attempt();
}
window.bitopUi.goTo = goTo;

function openByHover(chip) {
  const card = cardOf(chip);
  if (!card || card.matches(":popover-open")) return;
  anchorTo(chip);
  card.dataset.hover = "";
  card.showPopover();
  hoverChip = chip;
}

function closeHover() {
  clearTimeout(openTimer);
  clearTimeout(closeTimer);
  if (!hoverChip) return;
  const card = cardOf(hoverChip);
  hoverChip = undefined;
  if (card?.matches(":popover-open") && card.hasAttribute("data-hover")) card.hidePopover();
}

// The chip and the card count as one area for the pointer.
function owner(target) {
  if (!(target instanceof Element)) return undefined;
  const chip = target.closest("[data-citation-trigger]");
  if (chip) return chip;
  const card = target.closest("[data-citation-card]");
  return card ? chipOf(card) : undefined;
}

document.addEventListener("pointerover", (event) => {
  if (event.pointerType === "touch") return;
  const chip = owner(event.target);
  if (!chip) return;
  if (chip === hoverChip) {
    clearTimeout(closeTimer);
    return;
  }
  clearTimeout(openTimer);
  openTimer = setTimeout(() => openByHover(chip), OPEN_DELAY);
});

document.addEventListener("pointerout", (event) => {
  const chip = owner(event.target);
  if (!chip || owner(event.relatedTarget) === chip) return;
  clearTimeout(openTimer);
  if (chip !== hoverChip) return;
  clearTimeout(closeTimer);
  closeTimer = setTimeout(closeHover, CLOSE_DELAY);
});

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const chip = event.target.closest("[data-citation-trigger]");
  if (chip) {
    clearTimeout(openTimer);
    const card = cardOf(chip);
    if (chip.dataset.citationActivate) {
      closeHover();
      goTo(chip.dataset.citationActivate);
      return;
    }
    if (card?.hasAttribute("data-hover") && card.matches(":popover-open")) {
      // Opened by hover: a press keeps it open and moves focus in, as a press on a
      // closed chip does.
      event.preventDefault();
      hoverChip = undefined;
      delete card.dataset.hover;
      card.focus();
      return;
    }
    hoverChip = undefined;
    if (card) {
      delete card.dataset.hover;
      anchorTo(chip);
    }
    return;
  }
  const step = event.target.closest("[data-citation-step]");
  if (step) {
    const card = step.closest("[data-citation-card]");
    showPage(card, Number(card.dataset.page) + Number(step.dataset.citationStep));
    return;
  }
  const go = event.target.closest("[data-citation-go]");
  if (go) {
    const card = go.closest("[data-citation-card]");
    // Focus goes to the source, not back to the chip.
    chipOf(card)?.focus({ preventScroll: true });
    card.hidePopover();
    goTo(go.dataset.citationGo);
  }
});

document.addEventListener(
  "toggle",
  (event) => {
    const card = event.target;
    if (!(card instanceof Element) || !card.matches("[data-citation-card]")) return;
    const chip = chipOf(card);
    if (event.newState === "open") {
      showPage(card, Number(card.dataset.page));
      // A press moves focus into the card; hover leaves it where it is. The event is
      // queued, so focus may already be inside.
      if (!card.hasAttribute("data-hover") && !card.contains(document.activeElement)) card.focus();
    } else {
      delete card.dataset.hover;
      showPage(card, 0);
      if (chip) {
        chip.style.anchorName = "";
        chip.removeAttribute("data-open");
      }
      if (hoverChip === chip) hoverChip = undefined;
    }
  },
  true,
);

// Safari does not focus a button when it is clicked, so the browser would return focus
// to the page when Escape closes a card opened by a click.
document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape") return;
  const card = event.target instanceof Element && event.target.closest("[data-citation-card]");
  if (!card?.matches(":popover-open")) return;
  event.preventDefault();
  card.hidePopover();
  chipOf(card)?.focus();
});
