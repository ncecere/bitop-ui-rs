export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const formValue = (id, name) => page.evaluate(([i, n]) => new FormData(document.getElementById(i)).get(n), [id, name]);
  const focusedDay = () => page.evaluate(() => document.activeElement.dataset.day ?? document.activeElement.tagName);
  const isOpen = (popup) => popup.evaluate((el) => el.matches(":popover-open"));
  const untilOpen = async (popup, want = true) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await popup.elementHandle(), want]);
  // Focus moves into the calendar a moment after the popup opens.
  const untilDayFocused = () => page.waitForFunction(() => document.activeElement?.hasAttribute("data-day"));

  // One date, in a field: the button is named by the label and its value.
  const start = page.getByRole("button", { name: "Start date Pick a date" });
  const startPopup = page.locator("#start-form [popover]");
  check("named", await start.count(), 1);
  check("button", [await start.getAttribute("aria-haspopup"), await start.getAttribute("aria-expanded"), await start.evaluate((b) => b.hasAttribute("data-placeholder"))], ["dialog", null, true]);
  check("described", await start.evaluate((b) => document.getElementById(b.getAttribute("aria-describedby")).textContent.trim()), "The first day of the rollout.");
  check("emptyValue", await formValue("start-form", "start"), "");
  // A click on the label is a click on the button, as for any labelled button.
  await page.getByText("Start date", { exact: true }).click();
  await untilOpen(startPopup);
  check("labelOpens", await isOpen(startPopup));
  await page.keyboard.press("Escape");
  check("labelFocusesButton", await page.evaluate(() => document.activeElement.hasAttribute("data-date-picker-trigger")));

  // Opening moves focus to today, in a dialog under the button.
  await page.keyboard.press("Enter");
  await untilOpen(startPopup);
  await untilDayFocused();
  check("opensOnToday", [await focusedDay(), await start.getAttribute("aria-expanded")], ["2026-11-12", "true"]);
  check("dialog", await page.getByRole("dialog", { name: "Choose date" }).count(), 1);
  await settled(startPopup);
  let t = await start.boundingBox();
  let p = await startPopup.boundingBox();
  check("anchored", { below: p.y >= t.y + t.height, gap: Math.round(p.y - t.y - t.height), aligned: Math.abs(p.x - t.x) < 2 }, { below: true, gap: 6, aligned: true });
  check("axeOpen", await axe(), []);
  // Escape closes without choosing, and returns focus.
  await page.keyboard.press("Escape");
  check("esc", [await isOpen(startPopup), await page.evaluate(() => document.activeElement.hasAttribute("data-date-picker-trigger")), await start.getAttribute("aria-expanded")], [false, true, "false"]);

  // Choosing a day from the keyboard closes the popup, fills the button and the form.
  await page.keyboard.press("Space");
  await untilOpen(startPopup);
  await untilDayFocused();
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Enter");
  check("chosen", [await isOpen(startPopup), await formValue("start-form", "start")], [false, "2026-11-20"]);
  const chosen = page.getByRole("button", { name: "Start date Nov 20, 2026" });
  check("buttonShowsDate", [await chosen.count(), await chosen.evaluate((b) => b.hasAttribute("data-placeholder"))], [1, false]);
  check("focusReturned", await page.evaluate(() => document.activeElement.hasAttribute("data-date-picker-trigger")));
  // Opening again starts on the chosen day; choosing it again keeps it.
  await page.keyboard.press("Enter");
  await untilOpen(startPopup);
  await untilDayFocused();
  check("reopensOnChosen", await focusedDay(), "2026-11-20");
  await page.keyboard.press("Enter");
  check("keepsWhenChosenAgain", [await isOpen(startPopup), await formValue("start-form", "start")], [false, "2026-11-20"]);
  // The pointer: a click on a day; days before the minimum cannot be chosen.
  await chosen.click();
  await untilOpen(startPopup);
  await startPopup.locator('button[data-date="2026-11-01"]').click({ force: true });
  check("minHolds", [await isOpen(startPopup), await formValue("start-form", "start")], [true, "2026-11-20"]);
  await startPopup.locator('button[data-day="2026-11-03"]').click();
  check("clickChooses", [await isOpen(startPopup), await formValue("start-form", "start"), await page.getByRole("button", { name: "Start date Nov 3, 2026" }).count()], [false, "2026-11-03", 1]);
  await page.getByRole("button", { name: "Start date Nov 3, 2026" }).click();
  await untilOpen(startPopup);
  await page.mouse.click(5, 5);
  check("clickOutsideCloses", await isOpen(startPopup), false);

  // A range with presets: two months, and the first click keeps the popup open.
  const period = page.getByRole("button", { name: "Report period Pick a date range" });
  const periodPopup = page.locator("#period-form [popover]");
  await period.click();
  await untilOpen(periodPopup);
  await settled(periodPopup);
  check("rangeDialog", [await page.getByRole("dialog", { name: "Choose dates" }).count(), await periodPopup.getByRole("grid").count()], [1, 2]);
  const presets = periodPopup.getByRole("group", { name: "Presets" }).getByRole("button");
  check("presets", [await presets.allTextContents().then((all) => all.map((text) => text.trim())), await periodPopup.locator("[aria-pressed=true]").count()], [["Today", "Last 7 days", "Last 30 days", "Last 90 days"], 0]);
  const g = await periodPopup.getByRole("group", { name: "Presets" }).boundingBox();
  const c = await periodPopup.locator("[data-calendar]").boundingBox();
  check("presetsBeside", g.x + g.width <= c.x + 1);
  check("axeRange", await axe(), []);
  // The months shown end with today's, because later days cannot be chosen.
  await periodPopup.locator('button[data-day="2026-11-03"]').click();
  check("firstClickKeepsOpen", [await isOpen(periodPopup), await formValue("period-form", "period"), await page.getByRole("button", { name: "Report period Nov 3, 2026 – …" }).count()], [true, "2026-11-03", 1]);
  await periodPopup.locator('button[data-day="2026-11-09"]').click();
  check("secondClickCloses", [await isOpen(periodPopup), await formValue("period-form", "period"), await page.getByRole("button", { name: "Report period Nov 3, 2026 – Nov 9, 2026" }).count()], [false, "2026-11-03/2026-11-09", 1]);
  // A preset sets the whole range and closes; it is then shown as pressed.
  await page.getByRole("button", { name: "Report period Nov 3, 2026 – Nov 9, 2026" }).click();
  await untilOpen(periodPopup);
  await presets.nth(1).click();
  check("preset", [await isOpen(periodPopup), await formValue("period-form", "period"), await page.getByRole("button", { name: "Report period Nov 6, 2026 – Nov 12, 2026" }).count()], [false, "2026-11-06/2026-11-12", 1]);
  check("presetFocusReturned", await page.evaluate(() => document.activeElement.hasAttribute("data-date-picker-trigger")));
  await page.keyboard.press("Enter");
  await untilOpen(periodPopup);
  await untilDayFocused();
  check("presetPressed", [await periodPopup.locator("[aria-pressed=true]").allTextContents().then((all) => all.map((text) => text.trim())), await focusedDay()], [["Last 7 days"], "2026-11-06"]);
  // A preset that reaches back shows its first month.
  await presets.nth(3).click();
  check("longPreset", await formValue("period-form", "period"), "2026-08-15/2026-11-12");
  await page.keyboard.press("Enter");
  await untilOpen(periodPopup);
  check("showsRangeStart", await periodPopup.locator("[data-calendar-caption]").first().textContent().then((text) => text.trim()), "August 2026");
  await page.keyboard.press("Escape");

  // Kept open: the popup stays after a range is chosen, until the footer's button closes it.
  const trip = page.getByRole("button", { name: "Trip Nov 24, 2026 – Dec 3, 2026" });
  const tripPopup = page.locator("[data-date-picker][data-keep-open] [popover]");
  await trip.click();
  await untilOpen(tripPopup);
  await untilDayFocused();
  check("opensOnRangeStart", await focusedDay(), "2026-11-24");
  await tripPopup.locator('button[data-day="2026-11-10"]').click();
  await tripPopup.locator('button[data-day="2026-11-12"]').click();
  check("keepOpen", [await isOpen(tripPopup), await page.locator("input[name=trip]").inputValue()], [true, "2026-11-10/2026-11-12"]);
  await tripPopup.getByRole("button", { name: "Done" }).click();
  check("footerCloses", [await isOpen(tripPopup), await page.getByRole("button", { name: "Trip Nov 10, 2026 – Nov 12, 2026" }).count()], [false, 1]);

  // Sizes, another format, and the full-width picker with selects.
  const due = page.getByRole("button", { name: "Due date 30 Nov 2026" });
  check("format", await due.count(), 1);
  const birthday = page.getByRole("button", { name: "Birthday Pick a date" });
  const dueBox = await due.boundingBox();
  const startBox = await page.getByRole("button", { name: /^Start date/ }).boundingBox();
  check("sizes", [Math.round(dueBox.height), Math.round(startBox.height)], [32, 36]);
  const frame = await birthday.locator("xpath=../..").boundingBox();
  const wide = await birthday.boundingBox();
  check("block", wide.width > 400 && wide.width <= frame.width);
  await birthday.click();
  const birthdayPopup = page.locator("[data-date-picker][data-block] [popover]");
  await untilOpen(birthdayPopup);
  await birthdayPopup.getByRole("combobox", { name: "Year" }).selectOption("1990");
  await birthdayPopup.getByRole("combobox", { name: "Month" }).selectOption({ label: "June" });
  check("selectsKeepOpen", await isOpen(birthdayPopup));
  await birthdayPopup.locator('button[data-day="1990-06-15"]').click();
  check("selectsThenDay", [await isOpen(birthdayPopup), await page.locator("input[name=birthday]").inputValue(), await page.getByRole("button", { name: "Birthday Jun 15, 1990" }).count()], [false, "1990-06-15", 1]);

  check("invalid", await page.getByRole("button", { name: "Expiry date Pick a date" }).getAttribute("aria-invalid"), "true");
  const locked = page.getByRole("button", { name: "Locked date Nov 18, 2026" });
  check("disabled", [await locked.isDisabled(), await page.locator("input[name=locked]").isDisabled()], [true, true]);

  // Right to left: the popup lines up with the right edge of the button.
  await setDir("rtl");
  const rtlStart = page.getByRole("button", { name: /^Start date/ });
  await rtlStart.click();
  await untilOpen(startPopup);
  await settled(startPopup);
  t = await rtlStart.boundingBox();
  p = await startPopup.boundingBox();
  check("rtlAligned", Math.abs(p.x + p.width - (t.x + t.width)) < 2);
  await untilDayFocused();
  await page.keyboard.press("ArrowLeft");
  check("rtlArrow", await focusedDay(), "2026-11-04");
  await page.keyboard.press("Escape");
};
