export default async ({ page, check, focused, setDir }) => {
  const single = page.locator('[data-demo="single"]');
  const items = single.locator("details");
  const trigger = (scope, name) => scope.locator("summary", { hasText: name });
  const open = (scope) => scope.locator("details").evaluateAll((all) => all.map((d) => d.open));
  // An item has finished opening or closing when its height is that of its trigger,
  // plus that of its panel while open.
  const settledItem = (item) =>
    item.evaluate(
      (details) =>
        new Promise((resolve, reject) => {
          const started = performance.now();
          let last = -1;
          let frames = 0;
          const poll = () => {
            const height = details.getBoundingClientRect().height;
            const want = details.querySelector("summary").getBoundingClientRect().height + (details.open ? details.lastElementChild.getBoundingClientRect().height : 0);
            frames = height === last ? frames + 1 : 0;
            last = height;
            if (Math.abs(height - want) < 1.5 && frames >= 6) resolve();
            else if (performance.now() - started > 30_000) reject(new Error(`the item did not settle: ${height} of ${want}`));
            else requestAnimationFrame(poll);
          };
          poll();
        }),
    );

  // Native <details>: a <summary> with the heading inside it, one item open.
  check("structure", await items.evaluateAll((all) => all.map((d) => [d.firstElementChild.tagName, d.querySelector("summary > :first-child").tagName, d.getAttribute("name")].join(" "))), [
    "SUMMARY H3 faq", "SUMMARY H3 faq", "SUMMARY H3 faq", "SUMMARY H3 faq",
  ]);
  check("initial", await open(single), [true, false, false, false]);
  check("meta", await trigger(single, "Members").locator("span").nth(1).textContent(), "3");
  check("noMarker", await trigger(single, "Billing").evaluate((el) => getComputedStyle(el).display), "flex");
  // What shows of an item's panel: the item without its trigger and its border.
  const panelHeight = (index) => items.nth(index).evaluate((d) => Math.round(d.clientHeight - d.querySelector("summary").getBoundingClientRect().height));
  check("openPanelShows", [(await panelHeight(0)) > 40, await panelHeight(1)], [true, 0]);
  check("closedPanelHidden", await single.getByText("Owners can invite").isVisible(), false);

  // The keyboard: Tab moves between triggers and skips a disabled one; Enter and
  // Space open and close.
  await trigger(single, "Billing").focus();
  await page.keyboard.press("Tab");
  check("tabToNextTrigger", await page.evaluate(() => document.activeElement.querySelector("h3").textContent), "Members");
  await page.keyboard.press("Enter");
  check("enterOpensAndClosesOther", await open(single), [false, true, false, false]);
  await settledItem(items.nth(1));
  await settledItem(items.nth(0));
  check("panelOpened", [(await panelHeight(1)) > 20, await panelHeight(0), await single.getByText("Owners can invite").isVisible()], [true, 0, true]);
  check("chevronTurns", [await trigger(single, "Members").locator("[data-icon]").evaluate((el) => getComputedStyle(el).rotate), await trigger(single, "Billing").locator("[data-icon]").evaluate((el) => getComputedStyle(el).rotate)], ["180deg", "none"]);
  await page.keyboard.press("Space");
  check("spaceCloses", await open(single), [false, false, false, false]);
  await page.keyboard.press("Tab");
  check("disabledSkipped", await page.evaluate(() => document.activeElement.querySelector("h3")?.textContent), "Security");
  // A link in an open panel is in the tab order. (From the keyboard: after a click on
  // a <summary>, WebKit's Tab stays on it.)
  await page.keyboard.press("Enter");
  await settledItem(items.nth(3));
  await page.keyboard.press("Tab");
  check("tabIntoPanel", await focused(), "Read the policy");
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Enter");
  check("enterClosesAgain", await open(single), [false, false, false, false]);
  await settledItem(items.nth(3));
  const disabled = trigger(single, "Audit log");
  check("disabledTrigger", [await disabled.getAttribute("aria-disabled"), await disabled.getAttribute("tabindex"), await disabled.evaluate((el) => getComputedStyle(el).pointerEvents)], ["true", "-1", "none"]);
  await disabled.click({ force: true });
  check("disabledStaysClosed", await open(single), [false, false, false, false]);

  // A click opens, and closes the other items of the same name.
  await trigger(single, "Security").click();
  check("clickOpens", await open(single), [false, false, false, true]);
  await settledItem(items.nth(3));
  await trigger(single, "Billing").click();
  check("exclusive", await open(single), [true, false, false, false]);
  await settledItem(items.nth(0));
  await settledItem(items.nth(3));

  // The panel's height is animated where the browser can do it, and the panel opens
  // at once where it cannot: Firefox has ::details-content but not interpolate-size,
  // so a height cannot run to auto there. The item's height is read at every frame;
  // a height between closed and open means that it ran. The transition is made
  // longer here so that a slow machine sees it too.
  const loose = page.locator('[data-demo="multiple"]');
  const limits = loose.locator("details").nth(2);
  await loose.evaluate((el) => el.style.setProperty("--motion-duration-normal", "600ms"));
  const toggle = (item) =>
    item.evaluate(async (details) => {
      const heights = [Math.round(details.getBoundingClientRect().height)];
      details.querySelector("summary").click();
      const started = performance.now();
      while (performance.now() - started < 1200) {
        heights.push(Math.round(details.getBoundingClientRect().height));
        await new Promise((resolve) => requestAnimationFrame(resolve));
      }
      const low = Math.min(...heights);
      const high = Math.max(...heights);
      return { open: details.open, gradual: heights.some((h) => h > low + 2 && h < high - 2), grew: high - low > 20 };
    });
  const animates = await page.evaluate(() => CSS.supports("selector(::details-content)") && CSS.supports("interpolate-size", "allow-keywords"));
  check("opens", await toggle(limits), { open: true, gradual: animates, grew: true });
  // Closing: Chromium runs the height back to nothing. WebKit hides the content at
  // once, because it does not hold content-visibility for the length of a transition,
  // and no feature test tells the two apart, so this asks for the engine.
  const holdsContent = animates && page.context().browser().browserType().name() !== "webkit";
  check("closes", await toggle(limits), { open: false, gradual: holdsContent, grew: true });
  // With reduced motion nothing runs.
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.waitForFunction(() => matchMedia("(prefers-reduced-motion: reduce)").matches);
  check("reducedMotionOpens", await toggle(limits), { open: true, gradual: false, grew: true });
  check("reducedMotionCloses", await toggle(limits), { open: false, gradual: false, grew: true });
  await page.emulateMedia({ reducedMotion: "no-preference" });
  await page.waitForFunction(() => !matchMedia("(prefers-reduced-motion: reduce)").matches);
  await loose.evaluate((el) => el.style.removeProperty("--motion-duration-normal"));

  // Outline, without a name: any number open at once, on a card.
  const multiple = page.locator('[data-demo="multiple"]');
  check("outline", [await multiple.getAttribute("data-variant"), await multiple.evaluate((el) => getComputedStyle(el).boxShadow !== "none"), await multiple.locator("summary > h4").count()], ["outline", true, 3]);
  check("multipleInitial", await open(multiple), [true, true, false]);
  await trigger(multiple, "Limits").focus();
  await page.keyboard.press("Enter");
  check("multipleStayOpen", await open(multiple), [true, true, true]);
  const pad = await trigger(multiple, "Limits").evaluate((el) => getComputedStyle(el).paddingInlineStart);
  check("outlinePadding", [pad, await trigger(single, "Billing").evaluate((el) => getComputedStyle(el).paddingInlineStart)], ["16px", "4px"]);
  check("plainLastBorder", [await items.last().evaluate((el) => getComputedStyle(el).borderBlockEndWidth), await multiple.locator("details").last().evaluate((el) => getComputedStyle(el).borderBlockEndWidth)], ["1px", "0px"]);

  // Right to left: the title starts on the right and the chevron is on the left.
  await setDir("rtl");
  const t = await trigger(single, "Billing").locator("h3").boundingBox();
  const c = await trigger(single, "Billing").locator("[data-icon]").boundingBox();
  check("rtl", c.x < t.x);
};
