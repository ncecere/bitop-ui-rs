// Radio group: the arrow keys move between the radios of one name and select them.
// Browsers do this themselves, but not alike: WebKit does not wrap at the ends, and
// they differ on which way left and right go in a right-to-left page.
document.addEventListener("keydown", (event) => {
  const radio = event.target instanceof Element && event.target.closest('[role="radiogroup"] input[type="radio"]');
  if (!radio || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
  const group = radio.closest('[role="radiogroup"]');
  const radios = [...group.querySelectorAll('input[type="radio"]')].filter(
    (other) => other.name === radio.name && other.form === radio.form && !other.matches(":disabled"),
  );
  const rtl = getComputedStyle(group).direction === "rtl";
  const step = { ArrowDown: 1, ArrowUp: -1, ArrowRight: rtl ? -1 : 1, ArrowLeft: rtl ? 1 : -1 }[event.key];
  if (!step || radios.length < 2) return;
  event.preventDefault();
  const next = radios[(radios.indexOf(radio) + step + radios.length) % radios.length];
  next.focus();
  // A click checks the radio and fires the events a person's choice would.
  next.click();
});
