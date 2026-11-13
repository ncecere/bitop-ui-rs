export default async ({ page, check, setDir }) => {
  const plot = (name) => page.getByRole("img", { name, exact: true });
  const figure = (name) => page.locator("figure").filter({ has: plot(name) });
  const readout = (name) => figure(name).locator("[data-chart-readout]");
  // Each series' bars, as fractions of the plot's height (rounded to a pixel's worth).
  const bars = (name) =>
    figure(name).locator("svg").evaluate((svg) => {
      const area = svg.getBoundingClientRect();
      return [...svg.querySelectorAll("[data-series]")].map((g) =>
        [...g.querySelectorAll(":scope > rect")].map((r) => {
          // The fill's box: Firefox counts the stroke of a patterned bar in its client rect.
          const g = r.getBBox();
          const b = r.getBoundingClientRect();
          return { height: g.height, bottom: Math.round(area.height - g.y - g.height), share: g.height / area.height, missing: r.hasAttribute("data-missing"), centre: b.left + b.width / 2, width: g.width / area.width };
        }),
      );
    });
  const near = (got, want, height) => got.every((g, i) => Math.abs(g * height - want[i] * height) <= 1);
  const areaHeight = (name) => figure(name).locator("svg").evaluate((svg) => svg.getBoundingClientRect().height);

  // Overlapping series: a bar per point and series, as tall as its share of the peak.
  const week = "Answers per day, Sep 1 to 7: 441 in total, most on Sep 4 (96)";
  check("name", await plot(week).count(), 1);
  const answers = [61, 74, 50, 96, 80, 32, 48];
  const escalations = [12, 20, 8, 30, 16, 4, 10];
  const [back, front] = await bars(week);
  const h = await areaHeight(week);
  check("barCount", [back.length, front.length], [7, 7]);
  check("proportional", near(back.map((b) => b.share), answers.map((v) => v / 96), h) && near(front.map((b) => b.share), escalations.map((v) => v / 96), h));
  check("fromTheBase", [...back, ...front].every((b) => Math.abs(b.bottom) <= 1));
  check("peak", await figure(week).locator("[data-chart-plot] > span").first().textContent(), "96");
  check("axis", await figure(week).locator("[aria-hidden=true] > span").allTextContents(), ["Sep 1", "Sep 7"]);
  // Series differ by pattern, not colour alone: solid, then striped; the back one is paler.
  const fills = await figure(week).locator("[data-series]").evaluateAll((all) => all.map((g) => [g.dataset.pattern, g.querySelector(":scope > rect").getAttribute("fill")?.startsWith("url(#") ?? false, g.querySelector("pattern") !== null]));
  check("patterns", fills, [["solid", false, false], ["dashed", true, true]]);
  const legend = await figure(week).locator("[data-chart-legend] li span").evaluateAll((all) => all.map((s) => [s.dataset.pattern, getComputedStyle(s).backgroundImage === "none"]));
  check("legendPatterns", legend, [["solid", true], ["dashed", false]]);
  const colours = await figure(week).locator("[data-series]").evaluateAll((all) => all.map((g) => getComputedStyle(g.querySelector(":scope > rect")).fill));
  const soft = await page.evaluate(() => { const p = document.createElement("span"); document.body.append(p); p.style.color = "var(--color-chart-primary-soft)"; const c = getComputedStyle(p).color; p.remove(); return c; });
  check("backIsSoft", colours[0], soft);
  // The table, closed until asked for.
  check("tableClosed", await figure(week).locator("details").evaluate((d) => d.open), false);

  // The readout: on hover, the point's values over its slot.
  const slot = (name, i) => figure(name).locator("[data-chart-slot]").nth(i);
  check("readoutLive", [await readout(week).getAttribute("aria-live"), await readout(week).isVisible()], ["polite", false]);
  await slot(week, 3).hover();
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent !== "");
  check("hover", [await readout(week).textContent(), await slot(week, 3).evaluate((s) => s.hasAttribute("data-active"))], ["Sep 4: 96 Answers, 30 Escalations", true]);
  // Over its slot: the readout spans the slot's centre and stays inside the chart's frame.
  const over = async (name, i) => { const r = await readout(name).boundingBox(); const s = await slot(name, i).boundingBox(); const f = await readout(name).locator("xpath=..").boundingBox(); const c = s.x + s.width / 2; return r.x <= c && c <= r.x + r.width && r.x >= f.x - 1 && r.x + r.width <= f.x + f.width + 1 && (Math.abs(r.x + r.width / 2 - c) <= 1 || Math.abs(r.x - f.x) <= 1 || Math.abs(r.x + r.width - f.x - f.width) <= 1); };
  check("hoverPlaced", await over(week, 3));
  await page.mouse.move(5, 890);
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent === "");
  check("leaveHides", [await readout(week).isVisible(), await figure(week).locator("[data-active]").count()], [false, 0]);

  // The keyboard: the plot is a tab stop; it starts at the last point, and the arrows,
  // Home and End move.
  check("tabStop", await plot(week).getAttribute("tabindex"), "0");
  await figure(week).locator("[data-chart-legend]").evaluate((l) => { l.tabIndex = -1; l.focus(); });
  await page.keyboard.press("Tab");
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent !== "");
  check("focusShowsLast", await readout(week).textContent(), "Sep 7: 48 Answers, 10 Escalations");
  check("focusPlaced", await over(week, 6));
  await page.keyboard.press("ArrowLeft");
  check("arrowLeft", await readout(week).textContent(), "Sep 6: 32 Answers, 4 Escalations");
  await page.keyboard.press("Home");
  check("home", await readout(week).textContent(), "Sep 1: 61 Answers, 12 Escalations");
  check("homePlaced", await over(week, 0));
  await page.keyboard.press("ArrowLeft");
  check("stopsAtStart", await readout(week).textContent(), "Sep 1: 61 Answers, 12 Escalations");
  await page.keyboard.press("ArrowRight");
  check("arrowRight", await readout(week).textContent(), "Sep 2: 74 Answers, 20 Escalations");
  await page.keyboard.press("End");
  check("end", await readout(week).textContent(), "Sep 7: 48 Answers, 10 Escalations");
  await page.keyboard.press("Escape");
  check("escapeHides", await readout(week).textContent(), "");
  await page.keyboard.press("Tab");
  await figure(week).locator("[data-chart-legend]").evaluate((l) => l.removeAttribute("tabindex"));

  // Stacked: the segments add up, each on the one below; the tallest stack fills the plot.
  const stacked = "Conversations by channel per quarter: 350 in total, most in Q4 (120)";
  const layers = await bars(stacked);
  const sh = await areaHeight(stacked);
  const totals = [[40, 20, 10], [50, 30, 20], [30, 25, 5], [60, 35, 25]];
  check("stackShares", layers.every((series, i) => near(series.map((b) => b.share), totals.map((t) => t[i] / 120), sh)));
  check("stackedOnTop", layers[1].every((b, j) => Math.abs(b.bottom - Math.round(layers[0][j].height)) <= 1) && layers[2].every((b, j) => Math.abs(b.bottom - Math.round(layers[0][j].height + layers[1][j].height)) <= 1));
  check("stackPatterns", await figure(stacked).locator("[data-series]").evaluateAll((all) => all.map((g) => [g.dataset.pattern, g.dataset.tone])), [["solid", "primary"], ["dashed", "success"], ["dotted", "accent"]]);
  check("onlyTopRounded", await figure(stacked).locator("[data-series]").nth(2).locator("rect").evaluateAll((all) => all.every((r) => r.hasAttribute("data-top"))) && (await figure(stacked).locator("[data-series]").first().locator("rect[data-top]").count()) === 0);
  check("stackHeader", await figure(stacked).locator("details summary").textContent().then((t) => t.trim()), "Show data");

  // A fixed scale: bars are shares of 100, a taller one stops at the top; the table keeps the value.
  const cpu = "CPU use per hour, 09:00 to 12:00: peaking at 120% at 11:00";
  const [usage] = await bars(cpu);
  check("fixedScale", near(usage.map((b) => b.share), [0.42, 0.875, 1, 0.64], await areaHeight(cpu)));
  check("fixedPeak", await figure(cpu).locator("[data-chart-plot] > span").first().textContent(), "100%");
  const table = page.getByRole("table", { name: "CPU use per hour", exact: true });
  check("tableOpen", await table.isVisible());
  check("table", await table.locator("tr").evaluateAll((all) => all.map((tr) => [...tr.cells].map((c) => c.textContent.trim()))), [["Hour", "CPU"], ["09:00", "42%"], ["10:00", "87.5%"], ["11:00", "120%"], ["12:00", "64%"]]);
  await slot(cpu, 2).hover();
  await page.waitForFunction(() => [...document.querySelectorAll("[data-chart-readout]")].some((r) => r.textContent === "11:00: 120% CPU"));
  check("hoverRealValue", await readout(cpu).textContent(), "11:00: 120% CPU");
  await page.mouse.move(5, 890);

  // No data and a negative value: empty bars, the missing one marked; the readout and table say so.
  const errors = "Errors per day, Mon to Fri: no data on Tue, a correction of -2 on Wed";
  const [days] = await bars(errors);
  check("gapBars", [near(days.map((b) => b.share), [0.5, 0, 0, 1, 0.25], await areaHeight(errors)), days.map((b) => b.missing)], [true, [false, true, false, false, false]]);
  await slot(errors, 1).hover();
  await page.waitForFunction(() => [...document.querySelectorAll("[data-chart-readout]")].some((r) => r.textContent === "Tue: Errors: No data"));
  check("missingReadout", await readout(errors).textContent(), "Tue: Errors: No data");
  await slot(errors, 2).hover();
  await page.waitForFunction(() => [...document.querySelectorAll("[data-chart-readout]")].some((r) => r.textContent === "Wed: -2 Errors"));
  check("negativeReadout", await readout(errors).textContent(), "Wed: -2 Errors");
  await page.mouse.move(5, 890);
  await figure(errors).locator("summary").click();
  const errorTable = page.getByRole("table", { name: "Errors per day", exact: true });
  await errorTable.waitFor();
  check("gapTable", await errorTable.locator("tbody td").allTextContents(), ["4", "No data", "-2", "8", "2"]);

  // One point fills the width less an inset; sizes; no legend or axis.
  const [single] = await bars("Errors today: 18");
  check("single", [single.length, Math.round(single[0].width * 100), Math.round(single[0].share * 100)], [1, 80, 100]);
  check("singleAxis", await figure("Errors today: 18").locator("[aria-hidden=true] > span").allTextContents(), ["Today"]);
  const plotHeight = (name) => plot(name).evaluate((p) => Math.round(p.getBoundingClientRect().height));
  check("sizes", [await plotHeight("Errors today: 18"), await plotHeight(week), await plotHeight(stacked)], [96, 160, 224]);
  const bare = figure("Answers per day, without legend or axis");
  check("bare", [await bare.locator("[data-chart-legend]").count(), await bare.locator("[data-chart-plot] > span").count(), await bare.locator(":scope > [aria-hidden=true]").count(), await bare.locator("[data-series] > rect").count()], [0, 0, 0, 14]);

  // Empty: a message, no bars, and not a tab stop.
  const empty = "Answers per day: no answers yet";
  check("empty", [await plot(empty).getAttribute("tabindex"), await figure(empty).locator("[data-series] > rect").count(), await figure(empty).getByText("No data", { exact: true }).isVisible(), await figure(empty).locator("[data-chart-slot]").count()], [null, 0, true, 0]);

  // Every string is an argument.
  const german = figure("Antworten pro Tag");
  check("strings", [await german.locator("summary").textContent().then((t) => t.trim()), await german.locator("[data-chart-slot]").nth(1).getAttribute("data-chart-value")], ["Daten zeigen", "Di: Antworten: Keine Daten"]);

  // Sizes to its container: narrower, the bars keep their shares.
  const narrowBefore = await figure(cpu).evaluate((f) => f.style.inlineSize);
  await figure(cpu).evaluate((f) => (f.style.inlineSize = "300px"));
  await page.waitForFunction(() => Math.round([...document.querySelectorAll("figure")][2].querySelector("svg").getBoundingClientRect().width) === 300);
  const [narrow] = await bars(cpu);
  check("narrowShares", near(narrow.map((b) => b.share), [0.42, 0.875, 1, 0.64], await areaHeight(cpu)));
  check("narrowWidths", narrow.every((b) => Math.abs(b.width * 300 - 60) <= 1));
  await figure(cpu).evaluate((f, v) => (f.style.inlineSize = v), narrowBefore);

  // Right to left: the time axis runs from the right, as the arrow keys do.
  await setDir("rtl");
  const [rtl] = await bars(week);
  check("rtlBars", rtl[0].centre > rtl[6].centre);
  const labels = await figure(week).locator(":scope > [aria-hidden=true] > span").evaluateAll((all) => all.map((s) => s.getBoundingClientRect().left));
  check("rtlAxis", labels[0] > labels[1]);
  const rtlSlots = await figure(week).locator("[data-chart-slot]").evaluateAll((all) => all.map((s) => s.getBoundingClientRect().left + s.getBoundingClientRect().width / 2));
  check("rtlSlotsOverBars", rtl.every((b, i) => Math.abs(b.centre - rtlSlots[i]) <= 1));
  await figure(week).locator("[data-chart-legend]").evaluate((l) => { l.tabIndex = -1; l.focus(); });
  await page.keyboard.press("Tab");
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent !== "");
  await page.keyboard.press("ArrowRight");
  check("rtlArrowRight", await readout(week).textContent(), "Sep 6: 32 Answers, 4 Escalations");
  await page.keyboard.press("ArrowLeft");
  check("rtlArrowLeft", await readout(week).textContent(), "Sep 7: 48 Answers, 10 Escalations");
  check("rtlPlaced", await over(week, 6));
};
