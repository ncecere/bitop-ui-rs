export default async ({ page, base, check }) => {
  const NOON = "2026-09-26T12:00:00Z";
  const texts = (name) => page.locator(`[data-example=${name}] time`).evaluateAll((all) => all.map((t) => t.textContent.trim().replace(/ /g, " ")));
  // What Intl gives in this browser, to compare with: the checks do not assume a zone.
  const intl = (iso, options, locale) => page.evaluate(([i, o, l]) => new Intl.DateTimeFormat(l, o).format(new Date(i)).replace(/ /g, " "), [iso, options, locale]);

  // The script localises every format; the datetime attribute is the instant in UTC.
  const formats = page.locator("[data-example=formats] time");
  check("datetime", await formats.evaluateAll((all) => all.map((t) => t.getAttribute("datetime"))), Array(5).fill(NOON));
  check("formats", await texts("formats"), [
    await intl(NOON, { dateStyle: "medium", timeStyle: "short" }),
    await intl(NOON, { dateStyle: "medium" }),
    await intl(NOON, { timeStyle: "short" }),
    await intl(NOON, { dateStyle: "full", timeStyle: "long" }),
    await intl(NOON, { dateStyle: "medium", timeStyle: "short" }),
  ]);
  check("formatNames", await formats.evaluateAll((all) => all.map((t) => t.dataset.time)), ["datetime", "date", "time", "long", "datetime"]);
  check("noTitleOnAbsolute", await formats.first().getAttribute("title"), null);
  check("tabularFigures", await formats.first().evaluate((t) => getComputedStyle(t).fontVariantNumeric), "tabular-nums");

  // Relative, from a fixed time: the text does not depend on when the check runs.
  const fixed = await texts("fixed");
  check("relative", [fixed[0], fixed[2], fixed[3], fixed[4]], ["3 hours ago", "in 2 weeks", "now", "2 minutes ago"]);
  // 22:00 UTC the day before is yesterday, except in a zone where it is the same day.
  check("relativeDay", ["yesterday", "14 hours ago"].includes(fixed[1]));
  const titles = await page.locator("[data-example=fixed] time").evaluateAll((all) => all.map((t) => t.title.replace(/ /g, " ")));
  check("relativeTitle", titles[0], await intl("2026-09-26T09:00:00Z", { dateStyle: "full", timeStyle: "long" }));
  check("ownTitleKept", titles[4], "When the build started");

  // A live relative time ticks: every second while it is under a minute old.
  const live = page.locator("[data-example=live] time").first();
  const before = await live.textContent();
  check("liveStarts", /^(now|\d+ seconds? ago)$/.test(before));
  await page.waitForFunction((text) => document.querySelector("[data-example=live] time").textContent !== text, before);
  check("liveTicks", /^\d+ seconds ago$/.test(await live.textContent()));
  check("hours", (await texts("live"))[1], "3 hours ago");

  // A locale and a time zone given by the server win over the visitor's.
  const locale = await texts("locale");
  check("german", locale[0], "26.09.2026, 14:00");
  check("french", locale[1], "il y a 3 heures");
  check("frenchTitle", await page.locator("[data-example=locale] time").nth(1).getAttribute("title"), "samedi 26 septembre 2026 à 18:00:00 UTC+9");
  // An unknown zone cannot be formatted: the server's UTC text stays.
  check("badZoneKeepsServerText", locale[2], "Sep 26, 2026, 12:00 PM UTC");

  // A missing or invalid value renders the fallback, or nothing.
  check("missing", [await page.locator("[data-example=missing]").textContent().then((t) => t.replace(/\s+/g, " ").trim()), await page.locator("[data-example=missing] time").count()], ["Last sign-in: never", 0]);

  // A time added to the page later is localised too.
  await page.evaluate((iso) => {
    const time = document.createElement("time");
    time.dateTime = iso;
    time.dataset.time = "date";
    time.dataset.locale = "en-GB";
    time.dataset.timeZone = "UTC";
    time.id = "added";
    time.textContent = "server text";
    document.querySelector("main").append(time);
  }, NOON);
  await page.waitForFunction(() => document.getElementById("added").textContent !== "server text");
  check("addedLater", await page.locator("#added").textContent(), "26 Sept 2026");

  // Another visitor: German, in Berlin. Same page, their own locale and zone.
  const context = await page.context().browser().newContext({ locale: "de-DE", timezoneId: "Europe/Berlin" });
  const berlin = await context.newPage();
  await berlin.goto(`${base}/c/time`);
  const german = berlin.locator("[data-example=formats] time");
  await berlin.waitForFunction(() => !document.querySelector("[data-example=formats] time").textContent.includes("UTC"));
  check("visitorLocale", await german.evaluateAll((all) => all.map((t) => t.textContent.trim())), [
    "26.09.2026, 14:00", "26.09.2026", "14:00", "Samstag, 26. September 2026 um 14:00:00 MESZ", "26.09.2026, 14:00",
  ]);
  // In Berlin the reference time is 14:00, so 14 hours before is midnight of the same day.
  check("visitorRelative", await berlin.locator("[data-example=fixed] time").evaluateAll((all) => all.slice(0, 3).map((t) => t.textContent.trim())), ["vor 3 Stunden", "vor 14 Stunden", "in 2 Wochen"]);
  await context.close();

  // Without a script the server's text stays: English, in UTC, and it says so.
  const plain = await page.context().browser().newContext({ javaScriptEnabled: false });
  const still = await plain.newPage();
  await still.goto(`${base}/c/time`);
  check("withoutScript", await still.locator("[data-example=formats] time").evaluateAll((all) => all.map((t) => t.textContent.trim())), [
    "Sep 26, 2026, 12:00 PM UTC", "Sep 26, 2026", "12:00 PM UTC", "Saturday, September 26, 2026 at 12:00:00 PM UTC", "Sep 26, 2026, 12:00 PM UTC",
  ]);
  check("withoutScriptRelative", await still.locator("[data-example=fixed] time").evaluateAll((all) => all.map((t) => [t.textContent.trim(), t.title])).then((all) => [all[0], all[1][0]]), [["3 hours ago", "Saturday, September 26, 2026 at 9:00:00 AM UTC"], "yesterday"]);
  await plain.close();
};
