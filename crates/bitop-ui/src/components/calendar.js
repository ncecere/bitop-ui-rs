// Calendar: month navigation, selection and the date-grid keyboard pattern.
//
// The server renders the first grid. This script renders the same grid for another
// month or another selection, from the data attributes on the calendar. The selection
// is the value of the calendar's hidden input, as ISO dates.

const DAY_MS = 86_400_000;

// Dates are UTC midnights, so that no time zone or clock change can shift a day. As
// ISO text they also compare in order.
function parseDay(text) {
  const match = /^(\d{4})-(\d{2})-(\d{2})$/.exec(text ?? "");
  if (!match) return undefined;
  const date = new Date(Date.UTC(+match[1], match[2] - 1, +match[3]));
  return date.getUTCMonth() === match[2] - 1 ? date : undefined;
}

function isoDay(date) {
  return date.toISOString().slice(0, 10);
}

function addDays(date, days) {
  return new Date(date.getTime() + days * DAY_MS);
}

function monthStart(date) {
  return new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), 1));
}

function monthEnd(date) {
  return new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth() + 1, 0));
}

// The same day some months later, or the month's last day when it is shorter.
function addMonths(date, months) {
  const first = new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth() + months, 1));
  return new Date(first.getTime() + (Math.min(date.getUTCDate(), monthEnd(first).getUTCDate()) - 1) * DAY_MS);
}

function calendarState(root) {
  const data = root.dataset;
  const list = (text) => (text ? text.split(",") : []);
  const hidden = root.querySelector(':scope > input[type="hidden"]');
  const mode = data.mode ?? "single";
  return {
    root,
    hidden,
    mode,
    values: hidden.value ? hidden.value.split(mode === "range" ? "/" : ",") : [],
    month: parseDay(`${data.month}-01`),
    count: root.querySelectorAll("[data-calendar-month]").length,
    today: data.today,
    min: data.min,
    max: data.max,
    off: new Set(list(data.disabledDates)),
    offWeekdays: new Set(list(data.disabledWeekdays).map(Number)),
    weekStart: Number(data.weekStart ?? 0),
    outsideDays: !root.hasAttribute("data-hide-outside"),
    required: root.hasAttribute("data-required"),
    maxSelected: Number(data.maxSelected ?? Infinity),
    disabled: root.hasAttribute("data-disabled"),
    monthNames: data.monthNames.split("|"),
    weekdayNames: data.weekdayNames.split("|"),
  };
}

function viewEnd(state) {
  return monthEnd(addMonths(state.month, state.count - 1));
}

function inView(state, key) {
  return key >= isoDay(state.month) && key <= isoDay(viewEnd(state));
}

function clampDay(state, key) {
  if (state.min && key < state.min) return state.min;
  if (state.max && key > state.max) return state.max;
  return key;
}

// The day that is in the tab order: the focused day while it is shown, else the first
// selected day or today, else the first day shown.
function tabStop(state, focused) {
  if (focused && inView(state, focused)) return focused;
  const selected = state.mode === "range" ? state.values.slice(0, 1) : state.values;
  const shown = [...selected, state.today].find((key) => key && inView(state, key)) ?? isoDay(state.month);
  const clamped = clampDay(state, shown);
  return inView(state, clamped) ? clamped : shown;
}

function dayModel(state, date, month) {
  const key = isoDay(date);
  const { values, root } = state;
  let selected = values.includes(key);
  let start = false;
  let end = false;
  let middle = false;
  if (state.mode === "range" && values.length) {
    const [from, to = from] = values;
    middle = key > from && key < to;
    selected = key === from || key === to || middle;
    // A one-day range is drawn as a single selected day, with no band.
    start = key === from && from !== to;
    end = key === to && from !== to;
  }
  const full = root.dataset.dateFormat
    .replace("{weekday}", state.weekdayNames[date.getUTCDay()])
    .replace("{month}", state.monthNames[date.getUTCMonth()])
    .replace("{day}", date.getUTCDate())
    .replace("{year}", date.getUTCFullYear());
  return {
    key,
    number: date.getUTCDate(),
    outside: date.getUTCMonth() !== month.getUTCMonth(),
    selected,
    start,
    end,
    middle,
    today: key === state.today,
    disabled:
      state.disabled ||
      (!!state.min && key < state.min) ||
      (!!state.max && key > state.max) ||
      state.off.has(key) ||
      state.offWeekdays.has(date.getUTCDay()),
    label: selected ? root.dataset.selectedFormat.replace("{date}", full) : full,
  };
}

function setOptional(element, name, value) {
  if (value === undefined || value === false) element.removeAttribute(name);
  else element.setAttribute(name, value === true ? "" : value);
}

function applyDay(cell, day, stop) {
  const button = cell.firstElementChild;
  cell.setAttribute("aria-selected", String(day.selected));
  setOptional(cell, "data-outside", day.outside);
  setOptional(cell, "data-range-start", day.start);
  setOptional(cell, "data-range-end", day.end);
  setOptional(cell, "data-range-middle", day.middle);
  setOptional(button, "data-day", day.outside ? undefined : day.key);
  button.setAttribute("data-date", day.key);
  button.tabIndex = !day.outside && day.key === stop ? 0 : -1;
  button.setAttribute("aria-label", day.label);
  setOptional(button, "aria-current", day.today ? "date" : undefined);
  setOptional(button, "aria-disabled", day.disabled ? "true" : undefined);
  setOptional(button, "data-outside", day.outside);
  button.firstElementChild.textContent = day.number;
}

// Replaces the weeks of one month. New cells are copies of a cell the server rendered,
// so they carry its classes.
function rebuildWeeks(state, body, month) {
  const dayCell = body.querySelector("td > button").parentElement;
  const emptyCell = document.createElement("td");
  emptyCell.className = dayCell.className;
  emptyCell.setAttribute("role", "gridcell");
  const end = monthEnd(month);
  const weeks = [];
  let date = addDays(month, -((month.getUTCDay() - state.weekStart + 7) % 7));
  while (date <= end) {
    const week = document.createElement("tr");
    for (let i = 0; i < 7; i += 1) {
      const outside = date.getUTCMonth() !== month.getUTCMonth();
      const cell = outside && !state.outsideDays ? emptyCell.cloneNode() : dayCell.cloneNode(true);
      cell.firstElementChild?.setAttribute("data-date", isoDay(date));
      week.append(cell);
      date = addDays(date, 1);
    }
    weeks.push(week);
  }
  body.replaceChildren(...weeks);
}

function updateSelects(state, block, month) {
  const monthSelect = block.querySelector('[data-calendar-select="month"]');
  const yearSelect = block.querySelector('[data-calendar-select="year"]');
  if (!monthSelect || !yearSelect) return;
  const year = month.getUTCFullYear();
  for (const option of monthSelect.options) {
    const first = new Date(Date.UTC(year, Number(option.value), 1));
    option.disabled = (!!state.min && isoDay(monthEnd(first)) < state.min) || (!!state.max && isoDay(first) > state.max);
  }
  monthSelect.value = String(month.getUTCMonth());
  if (![...yearSelect.options].some((option) => Number(option.value) === year)) {
    const before = [...yearSelect.options].find((option) => Number(option.value) > year);
    yearSelect.add(new Option(String(year), String(year)), before ?? null);
  }
  yearSelect.value = String(year);
}

// Renders the months from `state`. `rebuild` when the month shown has changed;
// otherwise the days are updated where they stand, and focus stays where it is.
function renderCalendar(state, focused, rebuild) {
  const { root } = state;
  const stop = tabStop(state, focused);
  root.dataset.month = isoDay(state.month).slice(0, 7);
  root.querySelectorAll("[data-calendar-month]").forEach((block, index) => {
    const month = addMonths(state.month, index);
    if (rebuild) {
      block.querySelector("[data-calendar-caption]").textContent = root.dataset.captionFormat
        .replace("{month}", state.monthNames[month.getUTCMonth()])
        .replace("{year}", month.getUTCFullYear());
      updateSelects(state, block, month);
      rebuildWeeks(state, block.querySelector("tbody"), month);
    }
    for (const button of block.querySelectorAll("td > button")) {
      applyDay(button.parentElement, dayModel(state, parseDay(button.dataset.date), month), stop);
    }
  });
  const previous = root.querySelector("[data-calendar-previous]");
  const next = root.querySelector("[data-calendar-next]");
  previous.disabled = state.disabled || (!!state.min && isoDay(addDays(state.month, -1)) < state.min);
  next.disabled = state.disabled || (!!state.max && isoDay(addDays(viewEnd(state), 1)) > state.max);
}

// Changes the months shown so that they include `key`. Returns whether they changed.
function revealDay(state, key) {
  if (inView(state, key)) return false;
  const month = monthStart(parseDay(key));
  state.month = key < isoDay(state.month) ? month : addMonths(month, -(state.count - 1));
  return true;
}

function focusDay(state, date) {
  const key = clampDay(state, isoDay(date));
  renderCalendar(state, key, revealDay(state, key));
  state.root.querySelector(`[data-day="${key}"]`)?.focus();
}

function currentStop(root) {
  return root.querySelector('[data-day][tabindex="0"]')?.dataset.day;
}

// Stores a selection, shows it, and tells the page.
function commitCalendar(state, values, focused) {
  state.values = values;
  state.hidden.value = values.join(state.mode === "range" ? "/" : ",");
  renderCalendar(state, focused, focused ? revealDay(state, focused) : false);
  state.hidden.dispatchEvent(new Event("change", { bubbles: true }));
  state.root.dispatchEvent(
    new CustomEvent("bitop:change", { bubbles: true, detail: { value: state.hidden.value, values } }),
  );
}

function selectDay(state, key) {
  const { values, mode } = state;
  let next;
  if (mode === "multiple") {
    if (values.includes(key)) next = values.filter((value) => value !== key);
    else if (values.length < state.maxSelected) next = [...values, key].sort();
    else return;
  } else if (mode === "range") {
    // The first click starts a range, the second ends it, the third starts again.
    if (values.length !== 1) next = [key];
    else next = key < values[0] ? [key, values[0]] : [values[0], key];
  } else next = values[0] === key && !state.required ? [] : [key];
  commitCalendar(state, next, key);
  state.root.querySelector(`[data-day="${key}"]`)?.focus();
}

document.addEventListener("click", (event) => {
  const root = event.target instanceof Element && event.target.closest("[data-calendar]");
  if (!root) return;
  const step = event.target.closest("[data-calendar-previous], [data-calendar-next]");
  if (step) {
    const state = calendarState(root);
    state.month = addMonths(state.month, step.hasAttribute("data-calendar-next") ? 1 : -1);
    renderCalendar(state, currentStop(root), true);
    return;
  }
  const day = event.target.closest("button[data-date]");
  if (!day || day.getAttribute("aria-disabled") === "true") return;
  selectDay(calendarState(root), day.dataset.date);
});

document.addEventListener("change", (event) => {
  const select = event.target instanceof Element && event.target.closest("[data-calendar-select]");
  const root = select && select.closest("[data-calendar]");
  if (!root) return;
  const state = calendarState(root);
  if (select.dataset.calendarSelect === "month") {
    state.month = new Date(Date.UTC(state.month.getUTCFullYear(), Number(select.value), 1));
  } else {
    // The same month of the chosen year, or the nearest month that can be chosen.
    let month = new Date(Date.UTC(Number(select.value), state.month.getUTCMonth(), 1));
    if (state.min && isoDay(monthEnd(month)) < state.min) month = monthStart(parseDay(state.min));
    if (state.max && isoDay(month) > state.max) month = monthStart(parseDay(state.max));
    state.month = month;
  }
  renderCalendar(state, currentStop(root), true);
});

document.addEventListener("keydown", (event) => {
  const day = event.target instanceof Element && event.target.closest("[data-calendar] button[data-day]");
  if (!day || event.ctrlKey || event.metaKey || event.altKey) return;
  const state = calendarState(day.closest("[data-calendar]"));
  const from = parseDay(day.dataset.day);
  // In a right-to-left page the days run the other way.
  const forward = getComputedStyle(state.root).direction === "rtl" ? -1 : 1;
  const weekStart = addDays(from, -((from.getUTCDay() - state.weekStart + 7) % 7));
  let next;
  switch (event.key) {
    case "ArrowLeft":
      next = addDays(from, -forward);
      break;
    case "ArrowRight":
      next = addDays(from, forward);
      break;
    case "ArrowUp":
      next = addDays(from, -7);
      break;
    case "ArrowDown":
      next = addDays(from, 7);
      break;
    case "Home":
      next = weekStart;
      break;
    case "End":
      next = addDays(weekStart, 6);
      break;
    case "PageUp":
      next = addMonths(from, event.shiftKey ? -12 : -1);
      break;
    case "PageDown":
      next = addMonths(from, event.shiftKey ? 12 : 1);
      break;
    default:
      return;
  }
  event.preventDefault();
  focusDay(state, next);
});

// For the date picker, and for scripts on the page.
window.bitopUi.calendar = {
  values: (root) => calendarState(root).values,
  setValue: (root, values) => commitCalendar(calendarState(root), values, values[0]),
};
