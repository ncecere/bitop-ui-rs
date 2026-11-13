export default async ({ page, check, setDir }) => {
  const meter = (name) => page.getByRole("meter", { name, exact: true });
  const aria = (name) => meter(name).evaluate((m) => [m.getAttribute("aria-valuenow"), m.getAttribute("aria-valuemin"), m.getAttribute("aria-valuemax"), m.getAttribute("aria-valuetext"), m.dataset.level]);

  // The levels: the value text says the level in words.
  check("normal", await aria("Documents"), ["1200", "0", "5000", "1,200 of 5,000", "normal"]);
  check("warning", await aria("Storage"), ["8.6", "0", "10", "8.6 GB of 10 GB, near limit", "warning"]);
  check("critical", await aria("Seats"), ["20", "0", "20", "20 of 20, at limit", "critical"]);
  // Over the limit: the text keeps the real number, the value stops at the limit.
  check("over", await aria("Tokens"), ["5000", "0", "5000", "5,400 of 5,000, over limit", "over"]);

  // The fill is the share of the range, and stops at full.
  const fill = (name) => meter(name).evaluate((m) => { const t = m.querySelector("[style^=width]").parentElement; return Math.round(t.firstElementChild.getBoundingClientRect().width / t.getBoundingClientRect().width * 1000) / 10; });
  check("fill", [await fill("Documents"), await fill("Storage"), await fill("Tokens"), await fill("Temperature")], [24, 86, 100, 80]);
  check("range", await aria("Temperature"), ["180", "100", "200", "180 of 200, near limit", "warning"]);

  // The level is never colour alone: a word and an icon come with it.
  const status = (name) => meter(name).evaluate((m) => { const i = m.querySelector("[data-icon]"); return i ? [i.dataset.icon, i.parentElement.textContent.trim()] : null; });
  check("status", [await status("Documents"), await status("Storage"), await status("Seats"), await status("Tokens")], [null, ["triangle-alert", "Near limit"], ["circle-alert", "At limit"], ["circle-alert", "Over limit"]]);
  const colour = (name) => meter(name).evaluate((m) => getComputedStyle(m.querySelector("[style^=width]")).backgroundColor);
  const colours = [await colour("Documents"), await colour("Storage"), await colour("Seats"), await colour("Tokens")];
  check("colours", [new Set(colours).size, colours[2] === colours[3]], [3, true]);
  check("noStatus", [await status("Uploads"), (await aria("Uploads"))[3]], [null, "95 of 100, near limit"]);
  // The visible text is hidden from assistive technology, which has aria-valuetext.
  check("visibleTextHidden", await meter("Storage").getByText("8.6 GB of 10 GB").evaluate((e) => e.closest("[aria-hidden=true]") !== null));

  // Thresholds are arguments.
  check("warningAt", (await aria("Queries today"))[4], "normal");
  check("criticalAbove", await aria("Budget"), ["100", "0", "100", "110% of the monthly budget, near limit", "warning"]);

  // The marker: a tick on the track, described under the bar.
  const marked = meter("Queries today");
  const tick = await marked.evaluate((m) => { const k = m.querySelector("[style^='--meter-marker']"); const t = k.parentElement.getBoundingClientRect(); const b = k.getBoundingClientRect(); return [k.getAttribute("aria-hidden"), Math.round((b.x + b.width / 2 - t.x) / t.width * 100), b.height > t.height]; });
  check("marker", tick, ["true", 80, true]);
  const described = await marked.evaluate((m) => document.getElementById(m.getAttribute("aria-describedby")).textContent.replace(/\s+/g, " ").trim());
  check("described", described, "Team limit: 4,000 · Resets at midnight UTC");
  check("descriptionOnly", await meter("Builds").evaluate((m) => document.getElementById(m.getAttribute("aria-describedby")).textContent.trim()), "Resets on the first of the month");
  check("noDescription", await meter("Documents").getAttribute("aria-describedby"), null);

  // Small, and a hidden label that still names the meter.
  const height = (name) => meter(name).evaluate((m) => m.querySelector("[style^=width]").parentElement.getBoundingClientRect().height);
  check("sizes", [await height("Memory"), await height("Documents")], [6, 8]);
  const label = await meter("Disk").getByText("Disk", { exact: true }).boundingBox();
  check("hiddenLabel", [Math.round(label.width), await meter("Disk").count()], [1, 1]);

  // No limit: the usage in text, no meter and no bar.
  const unlimited = page.locator("[data-meter]").filter({ hasText: "API calls" });
  check("unlimited", [await unlimited.getAttribute("role"), await unlimited.locator("span").nth(1).textContent().then((t) => t.replace(/\s+/g, " ").trim()), await unlimited.locator("[style]").count(), await unlimited.getByText("Your plan has no limit.").count()], [null, "58,210 · No limit", 0, 1]);

  // Strings.
  check("strings", [(await aria("Speicher"))[3], await status("Speicher")], ["9.5 GB von 10 GB, fast voll", ["triangle-alert", "Fast voll"]]);

  // Right to left: the bar fills from the right, and the marker is measured from there.
  await setDir("rtl");
  const rtl = await marked.evaluate((m) => { const k = m.querySelector("[style^='--meter-marker']"); const t = k.parentElement.getBoundingClientRect(); const b = k.getBoundingClientRect(); const f = m.querySelector("[style^=width]").getBoundingClientRect(); return [Math.round((t.right - (b.x + b.width / 2)) / t.width * 100), Math.round(t.right - f.right)]; });
  check("rtl", rtl, [80, 0]);
};
