// Time: rewrites the text of <time data-time> in the visitor's locale and time zone,
// and keeps relative times fresh. The server rendered English in UTC.

const TIME_STYLES = {
  date: { dateStyle: "medium" },
  datetime: { dateStyle: "medium", timeStyle: "short" },
  time: { timeStyle: "short" },
  long: { dateStyle: "full", timeStyle: "long" },
};
const TIME_DAY = 86_400;
const TIME_MONTH = 30.436875 * TIME_DAY;
const TIME_YEAR = 365.2425 * TIME_DAY;
const timeTimers = new WeakMap();

// An unknown locale or time zone throws; the server's text then stays.
function timeAbsolute(date, style, locale, timeZone) {
  try {
    return new Intl.DateTimeFormat(locale || undefined, { ...TIME_STYLES[style], timeZone: timeZone || undefined }).format(date);
  } catch {
    return "";
  }
}

// Each unit is used until the rounded value would reach the next one.
function timeUnit(seconds) {
  const abs = Math.abs(seconds);
  const pick = (size, unit) => [Math.sign(seconds) * Math.round(abs / size), unit];
  if (abs < 59.5) return pick(1, "second");
  if (abs < 3600 - 30) return pick(60, "minute");
  if (abs < TIME_DAY - 1800) return pick(3600, "hour");
  if (abs < 7 * TIME_DAY - TIME_DAY / 2) return pick(TIME_DAY, "day");
  if (abs < TIME_MONTH) return pick(7 * TIME_DAY, "week");
  if (abs < TIME_YEAR - TIME_MONTH / 2) return pick(TIME_MONTH, "month");
  return pick(TIME_YEAR, "year");
}

// From half a day to a week away it counts calendar days in the visitor's zone, so
// "yesterday" is always the day before today.
function timeRelative(date, now, locale) {
  const seconds = (date.getTime() - now.getTime()) / 1000;
  const midnight = (d) => new Date(d.getFullYear(), d.getMonth(), d.getDate()).getTime();
  const days = Math.round((midnight(date) - midnight(now)) / (TIME_DAY * 1000));
  const byDay = Math.abs(seconds) >= TIME_DAY / 2 && days !== 0 && Math.abs(days) < 7;
  const [n, unit] = byDay ? [days, "day"] : timeUnit(seconds);
  try {
    // `+ 0` turns -0 into 0, so that "now" is not "0 seconds ago".
    return new Intl.RelativeTimeFormat(locale || undefined, { numeric: "auto" }).format(n + 0, unit);
  } catch {
    return "";
  }
}

// How long to wait before refreshing a relative time that is `age` milliseconds away.
function timeRefreshInterval(age) {
  age = Math.abs(age);
  if (age < 60_000) return 1_000;
  if (age < 3_600_000) return 15_000;
  if (age < 86_400_000) return 300_000;
  return 3_600_000;
}

function timeUpdate(element) {
  const date = new Date(element.dateTime);
  if (Number.isNaN(date.getTime())) return;
  const { time: style, locale, timeZone, now } = element.dataset;
  const set = (text) => {
    if (text && element.textContent !== text) element.textContent = text;
  };
  if (style !== "relative") {
    set(timeAbsolute(date, style in TIME_STYLES ? style : "datetime", locale, timeZone));
    return;
  }
  const fixed = now !== undefined && now !== "";
  set(timeRelative(date, fixed ? new Date(Number(now) * 1000) : new Date(), locale));
  if (element.dataset.title !== "fixed") {
    const title = timeAbsolute(date, "long", locale, timeZone);
    if (title && element.title !== title) element.title = title;
  }
  clearTimeout(timeTimers.get(element));
  if (fixed) return;
  timeTimers.set(
    element,
    setTimeout(() => {
      // A time that has left the page stops ticking.
      if (element.isConnected) timeUpdate(element);
    }, timeRefreshInterval(Date.now() - date.getTime())),
  );
}

function timeScan(root) {
  if (root.matches?.("time[data-time]")) timeUpdate(root);
  for (const element of root.querySelectorAll?.("time[data-time]") ?? []) timeUpdate(element);
}

timeScan(document);
// Times that the server adds later are localised as they arrive.
new MutationObserver((records) => {
  for (const record of records) for (const node of record.addedNodes) if (node.nodeType === 1) timeScan(node);
}).observe(document.documentElement, { childList: true, subtree: true });
