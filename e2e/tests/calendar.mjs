export default async ({ page, check, axe, button, setDir }) => {
  const cal = (form) => page.locator(`#${form} [data-calendar]`);
  const single = cal("day-form");
  const formValue = (id, name) => page.evaluate(([i, n]) => new FormData(document.getElementById(i)).get(n), [id, name]);
  const focusedDay = () => page.evaluate(() => document.activeElement.dataset.day ?? document.activeElement.tagName);
  const caption = (root, index = 0) => root.locator("[data-calendar-caption]").nth(index).textContent().then((t) => t.trim());
  const selected = (root) => root.locator("td[aria-selected=true] > button[data-day]").evaluateAll((all) => all.map((b) => b.dataset.day));
  const day = (root, iso) => root.locator(`button[data-day="${iso}"]`);
  // What a grid shows, attribute by attribute, to compare the server's rendering with the script's.
  const snapshot = (root) =>
    root.evaluate((r) =>
      [...r.querySelectorAll("table")].map((table) => [
        document.getElementById(table.getAttribute("aria-labelledby")).textContent.trim(),
        ...[...table.querySelectorAll("tbody td")].map((td) => {
          const attrs = (el) => (el ? [...el.attributes].map((a) => `${a.name}=${a.value}`).sort().join(" ") : "");
          return `${attrs(td)} | ${attrs(td.firstElementChild)} | ${td.textContent.trim()}`;
        }),
      ]),
    );

  // The server's grid: November 2026, which starts on a Sunday.
  check("caption", await caption(single), "November 2026");
  check("grid", await single.getByRole("grid", { name: "November 2026" }).count(), 1);
  check("weekdays", await single.locator("th").evaluateAll((all) => all.map((th) => `${th.textContent.trim()}:${th.abbr}:${th.scope}`)), [
    "Sun:Sunday:col", "Mon:Monday:col", "Tue:Tuesday:col", "Wed:Wednesday:col", "Thu:Thursday:col", "Fri:Friday:col", "Sat:Saturday:col",
  ]);
  check("weeks", [await single.locator("tbody tr").count(), await single.locator("tbody td").count()], [5, 35]);
  check("firstAndLast", await single.locator("tbody button").evaluateAll((all) => [all[0].dataset.date, all.at(-1).dataset.date, all.at(-1).dataset.day ?? null]), ["2026-11-01", "2026-12-05", null]);
  check("selected", [await selected(single), await formValue("day-form", "day")], [["2026-11-18"], "2026-11-18"]);
  check("names", [await day(single, "2026-11-18").getAttribute("aria-label"), await day(single, "2026-11-12").getAttribute("aria-label")], ["Wednesday, November 18, 2026, selected", "Thursday, November 12, 2026"]);
  check("today", await single.locator("[aria-current=date]").evaluateAll((all) => all.map((b) => [b.dataset.day, getComputedStyle(b).fontWeight, getComputedStyle(b.firstElementChild).textDecorationLine])), [["2026-11-12", "700", "underline"]]);
  check("oneTabStop", await single.locator("button[tabindex='0'][data-date]").evaluateAll((all) => all.map((b) => b.dataset.day)), ["2026-11-18"]);
  const cell = await day(single, "2026-11-18").boundingBox();
  check("cellSize", [Math.round(cell.width), Math.round(cell.height)], [36, 36]);

  // The date-grid keyboard pattern.
  await day(single, "2026-11-18").focus();
  await page.keyboard.press("ArrowRight");
  check("right", await focusedDay(), "2026-11-19");
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowLeft");
  check("left", await focusedDay(), "2026-11-17");
  await page.keyboard.press("ArrowDown");
  check("down", await focusedDay(), "2026-11-24");
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("ArrowUp");
  check("up", await focusedDay(), "2026-11-10");
  await page.keyboard.press("Home");
  check("home", await focusedDay(), "2026-11-08");
  await page.keyboard.press("End");
  check("end", await focusedDay(), "2026-11-14");
  check("tabStopFollows", await single.locator("button[tabindex='0'][data-date]").evaluateAll((all) => all.map((b) => b.dataset.day)), ["2026-11-14"]);
  check("arrowsDoNotSelect", await selected(single), ["2026-11-18"]);
  // Moving past the end of the month shows the next month, and keeps focus on the day.
  const server = await snapshot(single);
  await page.keyboard.press("PageDown");
  check("pageDown", [await focusedDay(), await caption(single)], ["2026-12-14", "December 2026"]);
  await page.keyboard.press("Shift+PageDown");
  check("shiftPageDown", [await focusedDay(), await caption(single)], ["2027-12-14", "December 2027"]);
  await page.keyboard.press("Shift+PageUp");
  await page.keyboard.press("PageUp");
  check("pageUp", [await focusedDay(), await caption(single)], ["2026-11-14", "November 2026"]);
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("ArrowLeft");
  check("arrowIntoPreviousMonth", [await focusedDay(), await caption(single), await single.locator("tbody tr").count()], ["2026-10-31", "October 2026", 5]);
  await page.keyboard.press("ArrowRight");
  check("arrowBack", [await focusedDay(), await caption(single)], ["2026-11-01", "November 2026"]);
  // Back on the first month, the script's grid is the server's, but for the tab stop.
  const tabless = (grids) => grids.map((grid) => grid.map((line) => line.replace(/tabindex=-?\d+ /, "")));
  check("scriptRendersWhatTheServerDid", tabless(await snapshot(single)), tabless(server));
  // The day clamps to the month's length.
  await day(single, "2026-11-01").focus();
  for (let i = 0; i < 2; i++) await page.keyboard.press("PageUp");
  await page.keyboard.press("ArrowLeft");
  check("endOfAugust", await focusedDay(), "2026-08-31");
  await page.keyboard.press("PageDown");
  check("clampsToShorterMonth", [await focusedDay(), await caption(single)], ["2026-09-30", "September 2026"]);

  // Space and Enter select; the hidden input carries the ISO date and fires change.
  await page.evaluate(() => {
    window.calendarChanges = [];
    document.getElementById("day-form").addEventListener("change", (event) => window.calendarChanges.push(event.target.value));
  });
  await page.keyboard.press("Space");
  check("spaceSelects", [await selected(single), await formValue("day-form", "day"), await focusedDay()], [["2026-09-30"], "2026-09-30", "2026-09-30"]);
  check("selectedName", await day(single, "2026-09-30").getAttribute("aria-label"), "Wednesday, September 30, 2026, selected");
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("Enter");
  check("enterSelects", [await selected(single), await formValue("day-form", "day")], [["2026-09-29"], "2026-09-29"]);
  // Selecting the selected day again clears it, unless the calendar is required.
  await page.keyboard.press("Enter");
  check("clears", [await selected(single), await formValue("day-form", "day")], [[], ""]);
  check("changeEvents", await page.evaluate(() => window.calendarChanges), ["2026-09-30", "2026-09-29", ""]);

  // The buttons change the month without moving focus; a click selects.
  await single.getByRole("button", { name: "Next month" }).click();
  await single.getByRole("button", { name: "Next month" }).click();
  check("nextButton", [await caption(single), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], ["November 2026", "Next month"]);
  await single.getByRole("button", { name: "Previous month" }).click();
  check("previousButton", [await caption(single), await single.locator("button[tabindex='0'][data-date]").count()], ["October 2026", 1]);
  await single.getByRole("button", { name: "Next month" }).click();
  await day(single, "2026-11-20").click();
  check("clickSelects", [await selected(single), await formValue("day-form", "day"), await focusedDay()], [["2026-11-20"], "2026-11-20", "2026-11-20"]);
  // The fill fades in.
  const filled = await page
    .waitForFunction((b) => getComputedStyle(b).backgroundColor !== "rgba(0, 0, 0, 0)", await day(single, "2026-11-20").elementHandle())
    .then(() => true, () => false);
  check("selectedIsFilled", filled);
  // A day of the next month can be clicked: it is selected and its month is shown.
  await single.locator('button[data-date="2026-12-03"]').click();
  check("outsideDay", [await selected(single), await caption(single), await focusedDay()], [["2026-12-03"], "December 2026", "2026-12-03"]);
  check("axeSingle", await axe(), []);

  // A range over two months.
  const range = cal("stay-form");
  check("twoMonths", [await caption(range, 0), await caption(range, 1), await range.getByRole("grid").count()], ["November 2026", "December 2026", 2]);
  check("multiselectable", await range.getByRole("grid").first().getAttribute("aria-multiselectable"), "true");
  check("rangeValue", await formValue("stay-form", "stay"), "2026-11-24/2026-12-03");
  const band = (root) => root.evaluate((r) => ["start", "middle", "end"].map((part) => [...r.querySelectorAll(`td[data-range-${part}] > button[data-day]`)].map((b) => b.dataset.day)));
  const stay = await band(range);
  check("rangeBand", [stay[0], stay[1].length, stay[2]], [["2026-11-24"], 8, ["2026-12-03"]]);
  // The first click starts a new range, the second ends it; an earlier day becomes the start.
  await day(range, "2026-12-10").click();
  check("rangeStarts", [await formValue("stay-form", "stay"), await selected(range)], ["2026-12-10", ["2026-12-10"]]);
  await day(range, "2026-12-07").click();
  check("rangeEndsEitherWay", await formValue("stay-form", "stay"), "2026-12-07/2026-12-10");
  check("rangeBandAfter", await band(range), [["2026-12-07"], ["2026-12-08", "2026-12-09"], ["2026-12-10"]]);
  // The keyboard crosses from one month to the other without changing the months shown.
  await day(range, "2026-12-01").focus();
  await page.keyboard.press("ArrowLeft");
  check("crossesMonths", [await focusedDay(), await caption(range, 0)], ["2026-11-30", "November 2026"]);
  await page.keyboard.press("PageDown");
  await page.keyboard.press("PageDown");
  check("scrollsTwoMonths", [await focusedDay(), await caption(range, 0), await caption(range, 1)], ["2027-01-30", "December 2026", "January 2027"]);
  check("axeRange", await axe(), []);

  // Several days, three at most.
  const multiple = cal("shifts-form");
  check("multipleValue", await formValue("shifts-form", "shifts"), "2026-11-05,2026-11-20");
  await day(multiple, "2026-11-11").click();
  check("adds", await formValue("shifts-form", "shifts"), "2026-11-05,2026-11-11,2026-11-20");
  await day(multiple, "2026-11-12").click();
  check("limit", await formValue("shifts-form", "shifts"), "2026-11-05,2026-11-11,2026-11-20");
  await day(multiple, "2026-11-05").click();
  check("removes", [await formValue("shifts-form", "shifts"), await selected(multiple)], ["2026-11-11,2026-11-20", ["2026-11-11", "2026-11-20"]]);

  // Limits: days outside them, weekends and a holiday cannot be chosen, and the keyboard stops at them.
  const limits = page.locator("[data-calendar][data-min]");
  const off = (iso) => limits.locator(`button[data-date="${iso}"]`).getAttribute("aria-disabled");
  check("unavailable", [await off("2026-11-09"), await off("2026-11-10"), await off("2026-11-14"), await off("2026-11-26"), await off("2026-11-27")], ["true", null, "true", "true", null]);
  check("struckThrough", await limits.locator('button[data-date="2026-11-26"] span').evaluate((s) => getComputedStyle(s).textDecorationLine), "line-through");
  check("previousDisabled", [await limits.getByRole("button", { name: "Previous month" }).isDisabled(), await limits.getByRole("button", { name: "Next month" }).isDisabled()], [true, false]);
  // Playwright does not click what is aria-disabled unless it is forced to.
  await limits.locator('button[data-date="2026-11-26"]').click({ force: true });
  await limits.locator('button[data-date="2026-11-09"]').click({ force: true });
  check("cannotChoose", await limits.locator("input[type=hidden]").inputValue(), "");
  await day(limits, "2026-11-12").focus();
  await page.keyboard.press("PageUp");
  check("stopsAtMin", [await focusedDay(), await caption(limits)], ["2026-11-10", "November 2026"]);
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Enter");
  await page.keyboard.press("Enter");
  check("requiredKeepsSelection", await limits.locator("input[type=hidden]").inputValue(), "2026-11-11");
  await page.keyboard.press("Shift+PageDown");
  check("stopsAtMax", [await focusedDay(), await caption(limits), await limits.getByRole("button", { name: "Next month" }).isDisabled()], ["2026-12-20", "December 2026", true]);
  // Unavailable days can be focused, so that they are announced.
  await page.keyboard.press("ArrowLeft");
  check("focusesUnavailable", [await focusedDay(), await page.evaluate(() => document.activeElement.getAttribute("aria-disabled"))], ["2026-12-19", "true"]);

  // Month and year selects.
  const dropdown = page.locator("[data-calendar]:has(select)");
  const month = dropdown.getByRole("combobox", { name: "Month" });
  const year = dropdown.getByRole("combobox", { name: "Year" });
  check("selects", [await month.inputValue(), await year.inputValue(), await caption(dropdown), await year.locator("option").count()], ["5", "1990", "June 1990", 47]);
  check("captionForScreenReaders", await dropdown.locator("[data-calendar-caption]").evaluate((c) => c.classList.contains("sr-only")));
  await year.selectOption("2000");
  await month.selectOption({ label: "February" });
  check("selectChanges", [await caption(dropdown), await dropdown.locator("tbody button[data-day]").count()], ["February 2000", 29]);
  await dropdown.getByRole("button", { name: "Next month" }).click();
  check("selectsFollowButtons", [await month.inputValue(), await year.inputValue()], ["2", "2000"]);
  await year.selectOption("2026");
  check("yearClampsToMax", [await caption(dropdown), await month.inputValue(), await month.locator("option:disabled").count()], ["March 2026", "2", 1]);
  await month.selectOption({ label: "November" });
  check("lastMonth", [await caption(dropdown), await dropdown.getByRole("button", { name: "Next month" }).isDisabled()], ["November 2026", true]);

  // Weeks from Monday, without the neighbouring months' days.
  const monday = page.locator("[data-calendar][data-week-start='1']");
  check("mondayFirst", await monday.locator("th").first().textContent().then((t) => t.trim()), "Mon");
  check("noOutsideDays", [await monday.locator("tbody td").count(), await monday.locator("tbody button").count(), await monday.locator("tbody tr").count()], [42, 30, 6]);
  await day(monday, "2026-11-12").focus();
  await page.keyboard.press("Home");
  check("weekStartsMonday", await focusedDay(), "2026-11-09");
  await page.keyboard.press("End");
  check("weekEndsSunday", await focusedDay(), "2026-11-15");
  await page.keyboard.press("PageDown");
  check("rebuildsWithoutOutsideDays", [await monday.locator("tbody td").count(), await monday.locator("tbody button").count()], [35, 31]);

  // Disabled: nothing can be chosen or changed.
  const disabled = page.locator("[data-calendar][data-disabled]");
  await disabled.locator('button[data-date="2026-11-20"]').click({ force: true });
  check("disabled", [await disabled.getByRole("button", { name: "Next month" }).isDisabled(), await disabled.getByRole("grid").getAttribute("aria-disabled"), await disabled.locator("input[type=hidden]").evaluate((i) => [i.value, i.disabled])], [true, "true", ["2026-11-18", true]]);

  // A script can read and set the selection.
  check("api", await single.evaluate((root) => {
    window.bitopUi.calendar.setValue(root, ["2027-02-14"]);
    return [window.bitopUi.calendar.values(root), root.querySelector("[data-calendar-caption]").textContent.trim()];
  }), [["2027-02-14"], "February 2027"]);

  // Right to left: the days run from the right, and the arrows follow them.
  await setDir("rtl");
  const first = await day(monday, "2026-12-01").boundingBox();
  const second = await day(monday, "2026-12-02").boundingBox();
  check("rtlMirrors", second.x < first.x);
  await day(monday, "2026-12-02").focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlLeftIsNextDay", await focusedDay(), "2026-12-03");
  await page.keyboard.press("ArrowRight");
  check("rtlRightIsPreviousDay", await focusedDay(), "2026-12-02");
  const previous = await monday.getByRole("button", { name: "Previous month" }).boundingBox();
  const next = await monday.getByRole("button", { name: "Next month" }).boundingBox();
  check("rtlButtons", previous.x > next.x);
};
