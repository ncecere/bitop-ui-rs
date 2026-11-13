export default async ({ page, check, setDir }) => {
  const spark = (name) => page.getByRole("img", { name, exact: true });
  // The line's commands, and each point as a fraction of the drawing box from the top left.
  const shape = (locator) =>
    locator.evaluate((s) => {
      const svg = s.querySelector("svg");
      const box = svg.getBoundingClientRect();
      const m = svg.getScreenCTM();
      const line = svg.querySelector("path:last-of-type");
      const d = line?.getAttribute("d") ?? "";
      const points = (d.match(/-?[\d.]+,-?[\d.]+/g) ?? []).map((pair) => {
        const [x, y] = pair.split(",").map(Number);
        const p = new DOMPoint(x, y).matrixTransform(m);
        return [(p.x - box.left) / box.width, (p.y - box.top) / box.height, box.width, box.height];
      });
      const last = s.querySelector(":scope > span");
      const dot = last && last.getBoundingClientRect();
      return {
        commands: (d.match(/[MLZ]/g) ?? []).join(""),
        paths: svg.querySelectorAll("path").length,
        points,
        last: dot && [(dot.left + dot.width / 2 - box.left) / box.width, (dot.top + dot.height / 2 - box.top) / box.height, box.width, box.height],
        height: Math.round(s.getBoundingClientRect().height),
        stroke: line ? getComputedStyle(line).stroke : null,
      };
    });
  const near = (got, want) => got.length === want.length && got.every(([x, y, w, h], i) => Math.abs(x * w - want[i][0] * w) <= 1 && Math.abs(y * h - want[i][1] * h) <= 1);
  const rising = [61, 70, 66, 74, 72, 88, 96];
  const risingPoints = rising.map((v, i) => [i / 6, 1 - v / 96]);

  // An image named by its label; the line through every value, from 0, and the last point marked.
  const primary = await shape(spark("Primary: rising from 61 to 96"));
  check("line", [primary.commands, primary.paths, primary.height], ["MLLLLLL", 1, 40]);
  check("points", near(primary.points, risingPoints));
  check("last", near([primary.last], [[1, 0]]));
  check("lastHidden", await spark("Primary: rising from 61 to 96").locator(":scope > span").evaluate((s) => s.closest("[aria-hidden=true]") === null && s.textContent === ""));

  // Seven tones, each a chart colour.
  const names = ["Primary", "Info", "Success", "Warning", "Danger", "Neutral", "Accent"];
  const strokes = [];
  for (const name of names) strokes.push((await shape(spark(`${name}: rising from 61 to 96`))).stroke);
  const tokens = await page.evaluate((all) => {
    const probe = document.createElement("span");
    document.body.append(probe);
    const out = all.map((t) => { probe.style.color = `var(--color-chart-${t.toLowerCase()})`; return getComputedStyle(probe).color; });
    probe.remove();
    return out;
  }, names);
  check("tones", strokes, tokens);
  check("tonesDiffer", new Set(strokes).size, 7);

  // Area, small, and no last point.
  const area = await shape(spark("Answers per day: rising from 61 to 96"));
  check("area", [area.paths, await spark("Answers per day: rising from 61 to 96").locator("path").first().evaluate((p) => /^M0\.00,100\.00 .* Z$/.test(p.getAttribute("d")))], [2, true]);
  check("small", (await shape(spark("Small: rising from 61 to 96"))).height, 24);
  check("noLast", (await shape(spark("No last point: rising"))).last, null);

  // Gaps: the path splits at missing values; a missing last value is not marked.
  const gaps = await shape(spark("Builds per day, with three days missing"));
  check("gaps", [gaps.commands, gaps.last], ["MLMML", null]);
  check("gapPoints", near(gaps.points, [[0, 1 - 12 / 18], [1 / 7, 0], [3 / 7, 1 - 15 / 18], [5 / 7, 1 - 9 / 18], [6 / 7, 1 - 14 / 18]]));
  // Negative values: the scale runs from the smallest value to the largest.
  const signed = [4, -2, -6, 1, 5, -3, 2];
  check("negative", near((await shape(spark("Net change per day: between -6 and 5"))).points, signed.map((v, i) => [i / 6, 1 - (v + 6) / 11])));
  // From zero, small changes are flat; from the minimum they fill the height.
  const narrow = [96, 97, 96.5, 98, 97.2, 99];
  check("fromZero", near((await shape(spark("Uptime from zero: 96 to 99"))).points, narrow.map((v, i) => [i / 5, 1 - v / 99])));
  check("fromMin", near((await shape(spark("Uptime from the minimum: 96 to 99"))).points, narrow.map((v, i) => [i / 5, 1 - (v - 96) / 3])));

  // All equal, one value, none.
  check("flat", near((await shape(spark("Flat at 5"))).points, [[0, 0], [1 / 3, 0], [2 / 3, 0], [1, 0]]));
  const one = await shape(spark("One value: 42"));
  check("one", [one.paths, near([one.last], [[1, 0]])], [0, true]);
  const none = await shape(spark("No values yet"));
  check("none", [none.paths, none.last], [0, null]);

  // Decorative: hidden from assistive technology, still drawn.
  const decorative = page.locator("[data-sparkline][aria-hidden=true]");
  check("decorative", [await decorative.count(), await decorative.getAttribute("role"), await decorative.getAttribute("aria-label"), (await decorative.boundingBox()).width > 100], [1, null, null, true]);
  check("labelled", await page.locator("[data-sparkline][role=img]").count(), 18);

  // In a stat card, as wide as the card.
  const card = spark("Answers per day, last 7 days: rising from 61 to 96");
  const inCard = await card.evaluate((s) => [Math.round(s.getBoundingClientRect().width), Math.round(s.closest("dd").getBoundingClientRect().width)]);
  check("inCard", inCard[0], inCard[1]);

  // Sizes to its container.
  const box = spark("Primary: rising from 61 to 96");
  const before = await box.evaluate((s) => s.style.width);
  await box.evaluate((s) => (s.style.width = "200px"));
  await page.waitForFunction(() => document.querySelector("[data-sparkline]").getBoundingClientRect().width === 200);
  const resized = await shape(box);
  check("resized", resized.points[0][2] === 194 && near(resized.points, risingPoints) && near([resized.last], [[1, 0]]));
  await box.evaluate((s, v) => (s.style.width = v), before);

  // Right to left: the trend still runs from the left; the last point is on the right.
  await setDir("rtl");
  const rtl = await shape(spark("Primary: rising from 61 to 96"));
  check("rtl", near(rtl.points, risingPoints) && near([rtl.last], [[1, 0]]));
};
