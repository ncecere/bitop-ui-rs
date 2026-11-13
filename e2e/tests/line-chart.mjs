export default async ({ page, check, setDir }) => {
  const plot = (name) => page.getByRole("img", { name, exact: true });
  const figure = (name) => page.locator("figure").filter({ has: plot(name) });
  const readout = (name) => figure(name).locator("[data-chart-readout]");
  const slot = (name, i) => figure(name).locator("[data-chart-slot]").nth(i);
  // Each series' line: its pattern, dash array, colour and the commands of its path.
  const lines = (name) =>
    figure(name).locator("[data-series]").evaluateAll((all) =>
      all.map((g) => {
        const line = g.querySelector("path[data-pattern]");
        const commands = line.getAttribute("d").match(/[ML]/g).join("");
        return { tone: g.dataset.tone, pattern: line.dataset.pattern, dash: getComputedStyle(line).strokeDasharray, colour: getComputedStyle(line).stroke, commands, fill: g.querySelectorAll("path").length > 1 };
      }),
    );
  // Where a point is drawn, as fractions of the plot's drawing area from the top left.
  const pointsOf = (name) =>
    figure(name).locator("svg").evaluate((svg) => {
      const area = svg.getBoundingClientRect();
      return [...svg.querySelectorAll("path[data-pattern]")].map((line) => {
        const m = svg.getScreenCTM();
        return line.getAttribute("d").match(/-?[\d.]+,-?[\d.]+/g).map((pair) => {
          const [x, y] = pair.split(",").map(Number);
          const p = new DOMPoint(x, y).matrixTransform(m);
          return [(p.x - area.left) / area.width, (p.y - area.top) / area.height, area.width, area.height];
        });
      });
    });
  const near = (got, want) => got.every(([x, y, w, h], i) => Math.abs(x * w - want[i][0] * w) <= 1 && Math.abs(y * h - want[i][1] * h) <= 1);
  const over = async (name, i) => { const r = await readout(name).boundingBox(); const s = await slot(name, i).boundingBox(); const f = await readout(name).locator("xpath=..").boundingBox(); const c = s.x + s.width / 2; return r.x <= c && c <= r.x + r.width && r.x >= f.x - 1 && r.x + r.width <= f.x + f.width + 1; };

  // Three series: a path each through every point, told apart by pattern.
  const traffic = "Traffic per day, Sep 1 to 7: answers peaked at 96 on Sep 4";
  check("name", await plot(traffic).count(), 1);
  const three = await lines(traffic);
  check("paths", three.map((l) => [l.pattern, l.commands, l.fill]), [["solid", "MLLLLLL", false], ["dashed", "MLLLLLL", false], ["dotted", "MLLLLLL", false]]);
  check("dashesDiffer", new Set(three.map((l) => l.dash)).size, 3);
  check("solidHasNoDash", three[0].dash, "none");
  check("tonesDiffer", new Set(three.map((l) => l.colour)).size, 3);
  check("legend", await figure(traffic).locator("[data-chart-legend] span").evaluateAll((all) => all.map((s) => [s.dataset.shape, s.dataset.pattern, getComputedStyle(s).borderTopStyle])), [["line", "solid", "solid"], ["line", "dashed", "dashed"], ["line", "dotted", "dotted"]]);
  // The points sit at their values on a scale from 0 to the peak, evenly spaced.
  const answers = [61, 74, 50, 96, 80, 32, 48];
  check("positions", near((await pointsOf(traffic))[0], answers.map((v, i) => [i / 6, 1 - v / 96])));
  check("peak", await figure(traffic).locator("[data-chart-plot] > span").first().textContent(), "96");
  check("axis", await figure(traffic).locator(":scope > [aria-hidden=true] > span").allTextContents(), ["Sep 1", "Sep 7"]);
  check("noDots", await figure(traffic).locator("[data-point]").count(), 0);

  // The readout: on hover, and from the keyboard.
  await slot(traffic, 3).hover();
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent !== "");
  check("hover", await readout(traffic).textContent(), "Sep 4: 96 Answers, 60 Conversations, 20 Escalations");
  check("hoverPlaced", await over(traffic, 3));
  check("hoverHighlight", await slot(traffic, 3).evaluate((s) => [s.hasAttribute("data-active"), getComputedStyle(s).backgroundColor !== "rgba(0, 0, 0, 0)"]), [true, true]);
  await page.mouse.move(5, 890);
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent === "");
  check("leaveHides", await readout(traffic).isVisible(), false);
  await figure(traffic).locator("[data-chart-legend]").evaluate((l) => { l.tabIndex = -1; l.focus(); });
  await page.keyboard.press("Tab");
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent !== "");
  check("focus", await readout(traffic).textContent(), "Sep 7: 48 Answers, 44 Conversations, 9 Escalations");
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowLeft");
  check("arrows", await readout(traffic).textContent(), "Sep 5: 80 Answers, 66 Conversations, 14 Escalations");
  check("arrowsPlaced", await over(traffic, 4));
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowRight");
  check("homeArrow", await readout(traffic).textContent(), "Sep 2: 74 Answers, 52 Conversations, 12 Escalations");
  // The pointer passing over does not move the keyboard's place.
  await slot(traffic, 5).hover();
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent.startsWith("Sep 6"));
  await page.mouse.move(5, 890);
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent.startsWith("Sep 2"));
  check("keyboardPlaceKept", await readout(traffic).textContent(), "Sep 2: 74 Answers, 52 Conversations, 12 Escalations");
  await page.keyboard.press("Tab");
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent === "");
  check("blurHides", await readout(traffic).isVisible(), false);
  await figure(traffic).locator("[data-chart-legend]").evaluate((l) => l.removeAttribute("tabindex"));

  // The data table has every number.
  await figure(traffic).locator("summary").click();
  const table = page.getByRole("table", { name: "Traffic per day", exact: true });
  await table.waitFor();
  check("table", await table.locator("tr").nth(4).evaluate((tr) => [...tr.cells].map((c) => c.textContent.trim())), ["Sep 4", "96", "60", "20"]);
  check("tableRows", await table.locator("tbody tr").count(), 7);

  // Area: each line over a fill closed to the bottom; the legend keeps the line patterns.
  const area = "Answers and conversations per day, Sep 1 to 7, as areas";
  check("area", (await lines(area)).map((l) => [l.pattern, l.fill]), [["solid", true], ["dashed", true]]);
  check("areaClosed", await figure(area).locator("path:not([data-pattern])").evaluateAll((all) => all.map((p) => /^M[\d.]+,100\.00 .* Z$/.test(p.getAttribute("d")))), [true, true]);
  check("areaLegend", await figure(area).locator("[data-chart-legend] span").evaluateAll((all) => all.map((s) => s.dataset.shape)), ["line", "line"]);

  // Gaps: a missing value splits the path; a lone point between gaps gets a dot.
  const latency = "Latency in milliseconds, 12:00 to 12:30: no data at 12:10, p95 peaked at 420";
  check("gaps", (await lines(latency)).map((l) => l.commands), ["MLMML", "MLMLLL"]);
  const lone = await figure(latency).locator("[data-point]").evaluateAll((all) => all.map((d) => [d.dataset.tone, d.getAttribute("aria-hidden")]));
  check("loneDot", lone, [["success", "true"]]);
  const dot = await figure(latency).locator("[data-point]").boundingBox();
  const lat = await figure(latency).locator("svg").boundingBox();
  check("loneDotPlace", [Math.abs(dot.x + dot.width / 2 - (lat.x + lat.width / 2)) <= 1, Math.abs(dot.y + dot.height / 2 - (lat.y + lat.height * (1 - 130 / 420))) <= 1], [true, true]);
  await slot(latency, 2).hover();
  await page.waitForFunction(() => [...document.querySelectorAll("[data-chart-readout]")].some((r) => r.textContent.startsWith("12:10")));
  check("gapReadout", await readout(latency).textContent(), "12:10: p50: No data, p95: No data");
  await page.mouse.move(5, 890);

  // A domain: values on a scale from 90 to 100, the floor and a tick labelled.
  const uptime = "Uptime per day, Mon to Fri: lowest on Thu (92%)";
  check("domain", near((await pointsOf(uptime))[0], [99.2, 97.5, 99.9, 92, 98.4].map((v, i) => [i / 4, 1 - (v - 90) / 10])));
  check("domainLabels", [await figure(uptime).locator("[data-chart-plot] > span").first().textContent(), await figure(uptime).getByText("90%", { exact: true }).isVisible(), await figure(uptime).locator("[data-tick]").allTextContents()], ["100%", true, ["95%"]]);
  const tick = await figure(uptime).locator("[data-tick]").evaluate((t) => { const area = t.parentElement.getBoundingClientRect(); return Math.round(t.getBoundingClientRect().bottom - area.top) === Math.round(area.height / 2); });
  check("tickPlace", tick);
  // points: a dot on every point, on the line.
  const dots = await figure(uptime).locator("[data-point]").evaluateAll((all) => all.map((d) => { const a = d.parentElement.getBoundingClientRect(); const b = d.getBoundingClientRect(); return [(b.left + b.width / 2 - a.left) / a.width, (b.top + b.height / 2 - a.top) / a.height, a.width, a.height]; }));
  check("dots", dots.length === 5 && near(dots, [99.2, 97.5, 99.9, 92, 98.4].map((v, i) => [i / 4, 1 - (v - 90) / 10])));
  check("uptimeTable", await page.getByRole("table", { name: "Uptime per day", exact: true }).locator("tbody td").allTextContents(), ["99.2%", "97.5%", "99.9%", "92%", "98.4%"]);

  // Ticks from a fixed top; a negative value is drawn at 0, two equal values level.
  const score = "Score per run: from 40 to 55, with -10 on run 2";
  check("ticks", await figure(score).locator("[data-tick]").evaluateAll((all) => all.map((t) => [t.textContent.trim(), t.hasAttribute("data-floor")])), [["0", true], ["25", false], ["50", false], ["75", false]]);
  check("negativeAtZero", near((await pointsOf(score))[0], [[0, 0.6], [1 / 3, 1], [2 / 3, 0.45], [1, 0.45]]));
  await figure(score).locator("summary").click();
  check("negativeInTable", await page.getByRole("table", { name: "Score per run", exact: true }).locator("tbody td").nth(1).textContent(), "-10");

  // One point: no line, a dot per series in the middle.
  const one = "Traffic today: 42 answers, 30 conversations, 5 escalations";
  const single = await figure(one).locator("[data-point]").evaluateAll((all) => all.map((d) => { const a = d.parentElement.getBoundingClientRect(); const b = d.getBoundingClientRect(); return Math.abs(b.left + b.width / 2 - (a.left + a.width / 2)) <= 1; }));
  check("single", [await figure(one).locator("path").count(), single], [0, [true, true, true]]);
  check("singleAxis", await figure(one).locator(":scope > [aria-hidden=true] > span").allTextContents(), ["Today"]);
  check("small", await plot(one).evaluate((p) => Math.round(p.getBoundingClientRect().height)), 96);
  check("large", await plot(area).evaluate((p) => Math.round(p.getBoundingClientRect().height)), 224);

  // Empty: a message, no lines, and not a tab stop.
  const empty = "Traffic per day: nothing yet";
  check("empty", [await plot(empty).getAttribute("tabindex"), await figure(empty).locator("path").count(), await figure(empty).getByText("No data", { exact: true }).isVisible()], [null, 0, true]);

  // Every string is an argument.
  const german = figure("Antworten pro Tag");
  check("strings", [await german.locator("summary").textContent().then((t) => t.trim()), await german.locator("[data-chart-slot]").nth(1).getAttribute("data-chart-value")], ["Daten zeigen", "Di: Antworten: Keine Daten"]);

  // Sizes to its container: narrower, the points keep their places.
  const before = await figure(uptime).evaluate((f) => f.style.inlineSize);
  await figure(uptime).evaluate((f) => (f.style.inlineSize = "320px"));
  await page.waitForFunction((n) => [...document.querySelectorAll("figure")].find((f) => f.querySelector(`[aria-label="${n}"]`)).querySelector("svg").getBoundingClientRect().width === 320, uptime);
  const narrow = (await pointsOf(uptime))[0];
  check("narrow", narrow[0][2] === 320 && near(narrow, [99.2, 97.5, 99.9, 92, 98.4].map((v, i) => [i / 4, 1 - (v - 90) / 10])));
  // Strokes do not stretch with the plot.
  check("strokeKept", await figure(uptime).locator("path[data-pattern]").evaluate((p) => [p.getAttribute("vector-effect"), getComputedStyle(p).strokeWidth]), ["non-scaling-stroke", "2px"]);
  await figure(uptime).evaluate((f, v) => (f.style.inlineSize = v), before);

  // Right to left: the time axis still runs from the left, and so do the arrow keys.
  await setDir("rtl");
  check("rtlLine", near((await pointsOf(traffic))[0], answers.map((v, i) => [i / 6, 1 - v / 96])));
  const labels = await figure(traffic).locator(":scope > [aria-hidden=true] > span").evaluateAll((all) => all.map((s) => s.getBoundingClientRect().left));
  check("rtlAxis", labels[0] < labels[1]);
  const slots = await figure(traffic).locator("[data-chart-slot]").evaluateAll((all) => all.map((s) => s.getBoundingClientRect().left));
  check("rtlSlots", slots[0] < slots[6]);
  await figure(traffic).locator("[data-chart-legend]").evaluate((l) => { l.tabIndex = -1; l.focus(); });
  await page.keyboard.press("Tab");
  await page.waitForFunction(() => document.querySelector("[data-chart-readout]").textContent !== "");
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowRight");
  check("rtlArrowRight", await readout(traffic).textContent(), "Sep 2: 74 Answers, 52 Conversations, 12 Escalations");
  check("rtlPlaced", await over(traffic, 1));
  const uptimeDots = await figure(uptime).locator("[data-point]").evaluateAll((all) => all.map((d) => d.getBoundingClientRect().left));
  check("rtlDots", uptimeDots[0] < uptimeDots[4]);
};
