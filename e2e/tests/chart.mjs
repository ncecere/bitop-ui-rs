export default async ({ page, check }) => {
  const legend = (name) => page.getByRole("list", { name, exact: true });
  const swatches = (name) =>
    legend(name).locator("li").evaluateAll((items) =>
      items.map((li) => {
        const s = li.querySelector("span");
        const style = getComputedStyle(s);
        return { text: li.textContent.trim(), hidden: s.getAttribute("aria-hidden"), tone: s.dataset.tone, pattern: s.dataset.pattern, colour: style.backgroundColor, ring: style.boxShadow.match(/rgba?\([^)]*\)/)?.[0] ?? null, image: style.backgroundImage, line: style.borderTopStyle, width: Math.round(s.getBoundingClientRect().width) };
      }),
    );

  // Tones by position, each its own colour from the chart tokens.
  const tones = await swatches("Tones");
  check("toneNames", tones.map((s) => [s.text, s.tone]), [["Primary", "primary"], ["Info", "info"], ["Success", "success"], ["Warning", "warning"], ["Danger", "danger"], ["Neutral", "neutral"], ["Accent", "accent"]]);
  const tokens = await page.evaluate(() => {
    const probe = document.createElement("span");
    document.body.append(probe);
    const out = ["primary", "info", "success", "warning", "danger", "neutral", "accent"].map((t) => { probe.style.color = `var(--color-chart-${t})`; return getComputedStyle(probe).color; });
    probe.remove();
    return out;
  });
  // A striped or dotted square also draws its colour as an inset ring (the pattern is a gradient).
  check("toneColours", tones.map((s) => (s.pattern === "solid" ? s.colour : s.ring)), tokens);
  check("toneColoursDiffer", new Set(tokens).size, 7);
  check("swatchesHidden", tones.every((s) => s.hidden === "true"));

  // Patterns by position: lines solid, dashed, dotted; squares solid, striped, dotted.
  const lines = await swatches("Line patterns");
  check("linePatterns", lines.map((s) => [s.text, s.pattern, s.line, s.width]), [["Answers", "solid", "solid", 20], ["Conversations", "dashed", "dashed", 20], ["Escalations", "dotted", "dotted", 20]]);
  const squares = await swatches("Bar patterns");
  check("squarePatterns", squares.map((s) => [s.pattern, s.image === "none", s.width]), [["solid", true, 12], ["dashed", false, 12], ["dotted", false, 12]]);
  check("squaresDiffer", new Set(squares.map((s) => s.image)).size, 3);
  // A series may set its own tone and pattern.
  check("ownTone", (await swatches("Own tones")).map((s) => [s.tone, s.pattern, s.line]), [["neutral", "dotted", "dotted"], ["danger", "solid", "solid"]]);

  // The data table: a caption, a header per series, and every value; missing values say so.
  const table = page.getByRole("table", { name: "Answers per day", exact: true });
  check("tableOpen", await table.isVisible());
  check("headers", await table.getByRole("columnheader").allTextContents(), ["Date", "Answers", "Conversations", "Escalations"]);
  const rows = await table.locator("tbody tr").evaluateAll((all) => all.map((tr) => [...tr.cells].map((c) => [c.textContent.trim(), c.tagName])));
  check("rows", rows, [
    [["Sep 1", "TH"], ["61", "TD"], ["20", "TD"], ["3", "TD"]],
    [["Sep 2", "TH"], ["74", "TD"], ["No data", "TD"], ["5", "TD"]],
    // A point with fewer values than series: the rest count as 0.
    [["Sep 3", "TH"], ["1,204.5", "TD"], ["31", "TD"], ["0", "TD"]],
  ]);
  check("rowHeaders", await table.getByRole("rowheader").allTextContents(), ["Sep 1", "Sep 2", "Sep 3"]);
  check("numeric", await table.locator("tbody td").first().evaluate((td) => getComputedStyle(td).textAlign), "end");

  // Closed by default; every string is an argument.
  const german = page.locator("details").filter({ hasText: "Daten zeigen" });
  check("closed", await german.evaluate((d) => d.open), false);
  await german.locator("summary").click();
  const tabelle = page.getByRole("table", { name: "Antworten pro Tag", exact: true });
  await tabelle.waitFor();
  check("strings", [await tabelle.getByRole("columnheader").allTextContents(), await tabelle.locator("tbody tr").nth(1).locator("td").first().textContent()], [["Tag", "Antworten", "Gespräche"], "Keine Daten"]);
};
