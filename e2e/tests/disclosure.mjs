export default async ({ page, check, focused, setDir }) => {
  const demo = (name) => page.locator(`[data-demo="${name}"]`);
  const advanced = demo("advanced");
  const summary = advanced.locator("summary");
  const rotate = (scope) => scope.locator("summary [data-icon]").evaluate((el) => getComputedStyle(el).rotate);
  // Waits for the chevron and the panel to finish moving.
  const settledDetails = (details) =>
    details.evaluate(
      (el) =>
        new Promise((resolve, reject) => {
          const started = performance.now();
          let last = "";
          let frames = 0;
          const poll = () => {
            const now = `${el.getBoundingClientRect().height} ${getComputedStyle(el.querySelector("summary [data-icon]")).rotate}`;
            frames = now === last ? frames + 1 : 0;
            last = now;
            if (frames >= 6) resolve();
            else if (performance.now() - started > 30_000) reject(new Error("the disclosure did not settle"));
            else requestAnimationFrame(poll);
          };
          poll();
        }),
    );

  // Native <details>: closed, with the title and a summary of what is inside.
  check("closed", [await advanced.evaluate((el) => el.tagName), await advanced.evaluate((el) => el.open), await page.getByLabel("Depth").isVisible()], ["DETAILS", false, false]);
  check("texts", await summary.locator("span:not([data-icon])").allTextContents(), ["Advanced options", "Depth 2 · 200 pages"]);
  check("summaryTitle", await summary.locator("span").last().getAttribute("title"), "Depth 2 · 200 pages");
  const closed = await advanced.boundingBox();
  check("closedHeight", Math.round(closed.height), 40);
  check("card", await advanced.evaluate((el) => getComputedStyle(el).boxShadow !== "none"));

  // The keyboard: Enter and Space open and close; the fields are reachable only open.
  await summary.focus();
  await page.keyboard.press("Tab");
  check("closedFieldsSkipped", await page.evaluate(() => document.activeElement.closest("[data-demo]")?.dataset.demo), "open");
  await summary.focus();
  await page.keyboard.press("Enter");
  check("enterOpens", await advanced.evaluate((el) => el.open));
  await settledDetails(advanced);
  check("fieldsShow", await page.getByLabel("Depth").isVisible());
  check("chevronDown", await rotate(advanced), "90deg");
  check("openHeight", (await advanced.boundingBox()).height > 150);
  check("openCorners", await summary.evaluate((el) => getComputedStyle(el).borderEndStartRadius), "0px");
  await page.keyboard.press("Tab");
  check("tabIntoPanel", await page.evaluate(() => document.activeElement.id), "disclosure-depth");
  await summary.focus();
  await page.keyboard.press("Space");
  check("spaceCloses", await advanced.evaluate((el) => el.open), false);
  await settledDetails(advanced);
  check("chevronBack", [await rotate(advanced), Math.round((await advanced.boundingBox()).height)], ["none", 40]);
  await summary.click();
  check("clickOpens", await advanced.evaluate((el) => el.open));
  await settledDetails(advanced);

  // Open from the start.
  check("renderedOpen", [await demo("open").evaluate((el) => el.open), await demo("open").getByText("Accept: text/html").isVisible(), await rotate(demo("open"))], [true, true, "90deg"]);
  check("noSummary", await demo("open").locator("summary span").last().evaluate((el) => getComputedStyle(el).display), "none");

  // A long summary is cut with an ellipsis, and the trigger stays one line high.
  const long = demo("long").locator("summary span").last();
  check("longCut", [await long.evaluate((el) => el.scrollWidth > el.clientWidth), await long.evaluate((el) => getComputedStyle(el).textOverflow), Math.round((await demo("long").boundingBox()).height)], [true, "ellipsis", 40]);
  check("longTitle", (await long.getAttribute("title")).startsWith("Status is live or paused"));

  // Sharing a name: opening one closes the other.
  const shipping = page.locator("details", { hasText: "Shipping" });
  const payment = page.locator("details", { hasText: "Payment" });
  await payment.locator("summary").focus();
  await page.keyboard.press("Enter");
  check("exclusive", [await shipping.evaluate((el) => el.open), await payment.evaluate((el) => el.open)], [false, true]);

  // Right to left: the chevron is on the right, points left, and still turns down.
  await setDir("rtl");
  const title = await summary.locator("span").nth(1).boundingBox();
  const chevron = await summary.locator("[data-icon]").boundingBox();
  check("rtlChevronOnTheRight", chevron.x > title.x + title.width - 1);
  await settledDetails(advanced);
  check("rtlChevron", [await summary.locator("[data-icon]").evaluate((el) => getComputedStyle(el).scale), await rotate(advanced)], ["-1 1", "-90deg"]);
  const s = await summary.locator("span").last().boundingBox();
  const box = await advanced.boundingBox();
  check("rtlSummaryOnTheLeft", Math.round(s.x - box.x), 12);
};
