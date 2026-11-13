// Filter bar: a change submits the form at once, empty parameters are left out of
// the URL, and focus comes back to the control that was used once the page returns.

const FILTER_FOCUS = "bitop-filter-focus";

function filterRemember(root, note) {
  if (!root?.id) return;
  try {
    sessionStorage.setItem(FILTER_FOCUS, JSON.stringify({ id: root.id, ...note }));
  } catch {
    // Storage can be turned off; focus then starts at the top of the page.
  }
}

function filterSubmit(form, control) {
  filterRemember(form.closest("[data-filter-bar]"), { name: control.name, value: control.value });
  form.requestSubmit();
}

document.addEventListener("change", (event) => {
  const control = event.target;
  const form = control instanceof Element && control.closest("form[data-filter-form]");
  if (!form) return;
  // A dropdown of checkboxes has its own Apply, so that several can be chosen.
  if (control.closest("[data-filter-multi]")) return;
  // Text is submitted with Enter, not on every change.
  if (control.matches("input:not([type=radio], [type=checkbox]), textarea")) return;
  // "All" among checkboxes clears the others; another choice clears "All".
  if (control.type === "checkbox") {
    const group = control.closest("[role=group]");
    if (control.matches("[data-filter-all]")) {
      for (const box of group?.querySelectorAll("input[type=checkbox]") ?? []) if (box !== control) box.checked = false;
    } else {
      const all = group?.querySelector("[data-filter-all]");
      if (all) all.checked = false;
    }
  }
  // A choice replaces a value the options do not offer, such as a custom range.
  control.closest("[role=radiogroup]")?.querySelector("[data-filter-extra]")?.remove();
  filterSubmit(form, control);
});

// A single choice with no "All" item is cleared by choosing it again.
let filterWasChecked = null;
document.addEventListener("pointerdown", (event) => {
  const label = event.target instanceof Element && event.target.closest("[data-filter-form] [data-clearable] label");
  const radio = label?.querySelector("input[type=radio]");
  filterWasChecked = radio?.checked ? radio : null;
});
document.addEventListener("click", (event) => {
  const target = event.target;
  if (!(target instanceof Element)) return;
  if (target.matches("[data-clearable] input[type=radio]") && target === filterWasChecked) {
    filterWasChecked = null;
    target.checked = false;
    filterSubmit(target.form, target);
    return;
  }
  // A chip or "Clear all": note where it was, to put focus near it afterwards.
  const link = target.closest("[data-filter-chip], [data-filter-clear]");
  const list = link?.closest("[data-filter-chips]");
  if (list) {
    const index = link.matches("[data-filter-clear]") ? 0 : [...list.querySelectorAll("a")].indexOf(link);
    filterRemember(link.closest("[data-filter-bar]"), { chip: index });
  }
  // A click outside an open dropdown closes it.
  for (const open of document.querySelectorAll("details[data-filter-multi][open]")) {
    if (!open.contains(target)) open.open = false;
  }
});

document.addEventListener("keydown", (event) => {
  const target = event.target;
  if (!(target instanceof Element)) return;
  if (event.key === " " && target.matches("[data-filter-form] [data-clearable] input[type=radio]:checked")) {
    event.preventDefault();
    target.checked = false;
    filterSubmit(target.form, target);
  }
  if (event.key === "Escape") {
    const open = target.closest("details[data-filter-multi][open]");
    if (open) {
      open.open = false;
      open.querySelector("summary")?.focus();
    }
  }
});

document.addEventListener("submit", (event) => {
  const form = event.target instanceof Element && event.target.closest("form[data-filter-form]");
  const active = document.activeElement;
  // With nothing chosen the browser would go to "path?": go to the path.
  if (form && ![...new FormData(form, event.submitter).values()].some((value) => value !== "")) {
    event.preventDefault();
    location.assign(form.getAttribute("action") || location.pathname);
  }
  // A submit from the keyboard or from Apply: come back to where the focus was.
  if (form && active && form.contains(active) && sessionStorage.getItem(FILTER_FOCUS) === null) {
    // From inside a dropdown, that is its button: the panel is closed when the page returns.
    const inside = active.closest("[data-filter-multi]")?.querySelector("input[name]");
    filterRemember(form.closest("[data-filter-bar]"), { name: (inside ?? active).name ?? "", value: inside ? "" : (active.value ?? "") });
  }
});

// Leave empty parameters out of the URL: "All", "Any", an empty search box.
document.addEventListener("formdata", (event) => {
  if (!(event.target instanceof Element) || !event.target.matches("form[data-filter-form]")) return;
  const data = event.formData;
  const entries = [...data.entries()];
  for (const [name] of entries) data.delete(name);
  for (const [name, value] of entries) if (value !== "") data.append(name, value);
});

function filterRestoreFocus() {
  let note = null;
  try {
    note = JSON.parse(sessionStorage.getItem(FILTER_FOCUS) ?? "null");
    sessionStorage.removeItem(FILTER_FOCUS);
  } catch {
    return;
  }
  const root = note && document.getElementById(note.id);
  if (!root?.matches("[data-filter-bar]")) return;
  const first = () => root.querySelector("form :is(input:not([type=hidden]), select, summary, button):not(:disabled)");
  let target = null;
  if (typeof note.chip === "number") {
    const links = [...root.querySelectorAll("[data-filter-chips] a")];
    target = links[Math.min(note.chip, links.length - 1)] ?? first();
  } else {
    const named = [...root.querySelectorAll("form [name]")].filter((control) => control.name === note.name && control.type !== "hidden");
    target = named.find((control) => control.value === note.value) ?? named.find((control) => control.checked) ?? named[0];
    // A dropdown's checkbox is inside a closed panel after the reload: its button is not.
    const multi = target?.closest("[data-filter-multi]");
    if (multi && !multi.open) target = multi.querySelector("summary");
    target ??= first();
  }
  target?.focus();
}

for (const root of document.querySelectorAll("[data-filter-bar]")) root.setAttribute("data-enhanced", "");
filterRestoreFocus();
