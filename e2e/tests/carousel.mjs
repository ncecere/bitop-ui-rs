export default async ({ page, check, focused, setDir }) => {
  const demo = (name) => page.locator(`[data-demo="${name}"]`);
  const track = (root) => root.locator("[data-carousel-track]");
  const index = (root) => root.evaluate((el) => window.bitopUi.carousel.index(el));
  const status = (root) => root.locator("> [role=status]").textContent();
  const disabled = (root) => root.locator("[data-carousel-step]").evaluateAll((all) => all.map((b) => b.getAttribute("aria-disabled")));
  const currentDot = (root) => root.locator("[data-carousel-indicator][aria-current=true]").getAttribute("aria-label");
  // Waits until slide `i` is the current one and has come to rest at the start edge.
  const arrives = async (root, i) =>
    page.waitForFunction(
      ([el, want]) => {
        const scroller = el.querySelector("[data-carousel-track]");
        const slide = scroller.children[want].getBoundingClientRect();
        const box = scroller.getBoundingClientRect();
        const rtl = getComputedStyle(scroller).direction === "rtl";
        const distance = el.dataset.orientation === "vertical" ? slide.top - box.top : rtl ? box.right - slide.right : slide.left - box.left;
        return Math.abs(distance) < 1.5 && window.bitopUi.carousel.index(el) === want && scroller.children[want].hasAttribute("data-current");
      },
      [await root.elementHandle(), i],
    );

  // The carousel pattern: a named region, slides that are labelled groups, a live
  // region, and a track that can be focused.
  const basic = demo("basic");
  check("region", [await page.getByRole("region", { name: "Featured templates" }).count(), await basic.getAttribute("aria-roledescription")], [1, "carousel"]);
  check("slides", await track(basic).locator("> *").evaluateAll((all) => all.map((s) => [s.getAttribute("role"), s.getAttribute("aria-roledescription"), s.getAttribute("aria-label")].join(" "))), [
    "group slide 1 of 5", "group slide 2 of 5", "group slide 3 of 5", "group slide 4 of 5", "group slide 5 of 5",
  ]);
  check("live", await basic.locator("> [role=status]").evaluate((el) => [el.getAttribute("aria-live"), el.getAttribute("aria-atomic"), el.textContent]), ["polite", "true", ""]);
  check("trackFocusable", [await track(basic).getAttribute("tabindex"), await track(basic).evaluate((el) => getComputedStyle(el).scrollSnapType)], ["0", "x mandatory"]);
  check("initial", [await index(basic), await disabled(basic), await currentDot(basic)], [0, ["true", null], "Go to slide 1"]);
  const previous = basic.getByRole("button", { name: "Previous slide" });
  const next = basic.getByRole("button", { name: "Next slide" });
  check("controls", [await next.getAttribute("aria-controls"), await track(basic).getAttribute("id")].every((id, _, all) => id === all[0]));
  const t = await track(basic).boundingBox();
  const p = await previous.boundingBox();
  const n = await next.boundingBox();
  check("buttonsOverTrack", [Math.round(p.x - t.x), Math.round(t.x + t.width - n.x - n.width), Math.abs(p.y + p.height / 2 - (t.y + t.height / 2)) <= 1], [8, 8, true]);
  const dots = basic.getByRole("group", { name: "Choose a slide" }).getByRole("button");
  const dot = await dots.first().boundingBox();
  check("dots", [await dots.count(), Math.round(dot.width), Math.round(dot.height), dot.y >= t.y + t.height], [5, 24, 24, true]);

  // The next button glides to the second slide and announces it.
  await page.evaluate(() => {
    window.slideChanges = [];
    document.addEventListener("bitop:change", (event) => window.slideChanges.push(event.detail.index));
  });
  await next.click();
  await arrives(basic, 1);
  check("next", [await status(basic), await disabled(basic), await currentDot(basic)], ["Slide 2 of 5", [null, null], "Go to slide 2"]);
  check("changeEvent", await page.evaluate(() => window.slideChanges), [1]);

  // The arrow keys, from a button or from the track itself.
  await next.focus();
  await page.keyboard.press("ArrowRight");
  await arrives(basic, 2);
  check("arrowRight", await status(basic), "Slide 3 of 5");
  await track(basic).focus();
  await page.keyboard.press("ArrowLeft");
  await arrives(basic, 1);
  check("arrowLeftOnTrack", [await status(basic), await page.evaluate(() => document.activeElement.hasAttribute("data-carousel-track"))], ["Slide 2 of 5", true]);
  await page.keyboard.press("ArrowDown");
  check("otherAxisIgnored", await index(basic), 1);

  // A dot goes to its slide. At the end the next button is disabled, but keeps focus.
  await dots.nth(4).focus();
  await page.keyboard.press("Enter");
  await arrives(basic, 4);
  check("dot", [await status(basic), await disabled(basic), await currentDot(basic)], ["Slide 5 of 5", [null, "true"], "Go to slide 5"]);
  await next.focus();
  await next.click({ force: true });
  await page.keyboard.press("ArrowRight");
  check("endHolds", [await index(basic), await page.evaluate(() => document.activeElement.getAttribute("aria-label")), await status(basic)], [4, "Next slide", "Slide 5 of 5"]);

  // Scrolling the track by hand (a swipe, a wheel) is followed, and not announced.
  await track(basic).evaluate((el) => el.scrollTo({ left: 0, behavior: "instant" }));
  await arrives(basic, 0);
  check("followsScroll", [await disabled(basic), await currentDot(basic), await status(basic)], [["true", null], "Go to slide 1", "Slide 5 of 5"]);

  // With reduced motion the track jumps: it is there before the click returns.
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.waitForFunction(() => matchMedia("(prefers-reduced-motion: reduce)").matches);
  const jumped = await basic.evaluate((el) => {
    const scroller = el.querySelector("[data-carousel-track]");
    el.querySelector('[data-carousel-step="next"]').click();
    return Math.round(scroller.children[1].getBoundingClientRect().left - scroller.getBoundingClientRect().left);
  });
  check("reducedMotionJumps", jumped, 0);
  await arrives(basic, 1);
  await page.emulateMedia({ reducedMotion: "no-preference" });
  await page.waitForFunction(() => !matchMedia("(prefers-reduced-motion: reduce)").matches);

  // Three at a time, starting at the third slide: the last position is the fourth.
  const several = demo("several");
  await arrives(several, 2);
  const widths = await track(several).evaluate((el) => [Math.round(el.children[0].getBoundingClientRect().width), Math.round((el.getBoundingClientRect().width - 32) / 3)]);
  check("perView", [widths[0] === widths[1], await disabled(several), await track(several).locator("> *").nth(5).getAttribute("aria-label")], [true, [null, null], "6 of 6"]);
  await several.getByRole("button", { name: "Next slide" }).click();
  await arrives(several, 3);
  check("lastPosition", [await disabled(several), await status(several)], [[null, "true"], "Slide 4 of 6"]);
  await several.getByRole("button", { name: "Previous slide" }).click();
  await arrives(several, 2);

  // Slides with their own size, their own texts, and controls inside.
  const sized = demo("sized");
  check("basis", await track(sized).locator("> *").first().evaluate((el) => Math.round(el.getBoundingClientRect().width)), 224);
  check("labels", await track(sized).locator("> *").evaluateAll((all) => all.map((s) => s.getAttribute("aria-label"))), ["Project 1 of 4", "Project 2 of 4", "Project 3 of 4", "The last project"]);
  await sized.getByLabel("Rename").focus();
  await page.keyboard.press("ArrowRight");
  check("arrowsInAFieldAreTheFields", await index(sized), 0);
  await sized.getByRole("button", { name: "Open API" }).focus();
  await page.keyboard.press("ArrowRight");
  await arrives(sized, 1);
  check("ownAnnouncement", [await status(sized), await sized.getByRole("button", { name: "Next project" }).count()], ["Project 2 of 4", 1]);

  // Vertical: up and down, with the buttons above and below.
  const vertical = demo("vertical");
  check("verticalTrack", await track(vertical).evaluate((el) => [getComputedStyle(el).scrollSnapType, getComputedStyle(el).flexDirection, Math.round(el.getBoundingClientRect().height)]), ["y mandatory", "column", 192]);
  const vt = await track(vertical).boundingBox();
  const vp = await vertical.getByRole("button", { name: "Previous slide" }).boundingBox();
  const vn = await vertical.getByRole("button", { name: "Next slide" }).boundingBox();
  check("verticalButtons", [Math.round(vp.y - vt.y), Math.round(vt.y + vt.height - vn.y - vn.height), Math.abs(vp.x + vp.width / 2 - (vt.x + vt.width / 2)) <= 1], [8, 8, true]);
  check("verticalChevrons", await vertical.locator("[data-icon]").first().evaluate((el) => getComputedStyle(el).rotate), "90deg");
  await vertical.getByRole("button", { name: "Next slide" }).click();
  await arrives(vertical, 1);
  await page.keyboard.press("ArrowRight");
  check("verticalIgnoresSideways", await index(vertical), 1);
  await page.keyboard.press("ArrowDown");
  await arrives(vertical, 2);
  check("verticalEnd", [await status(vertical), await disabled(vertical)], ["Slide 3 of 3", [null, "true"]]);
  await page.keyboard.press("ArrowUp");
  await arrives(vertical, 1);

  // Right to left: the first slide is on the right, next is on the left, and the
  // arrow keys follow what is seen.
  await setDir("rtl");
  await basic.evaluate((el) => window.bitopUi.carousel.scrollTo(el, 0));
  await arrives(basic, 0);
  const rt = await track(basic).boundingBox();
  const first = await track(basic).locator("> *").first().boundingBox();
  const rn = await next.boundingBox();
  const rp = await previous.boundingBox();
  check("rtlLayout", [Math.abs(first.x + first.width - rt.x - rt.width) < 1.5, rn.x < rp.x, await next.locator("[data-icon]").evaluate((el) => getComputedStyle(el).scale)], [true, true, "-1 1"]);
  await next.click();
  await arrives(basic, 1);
  check("rtlNext", await status(basic), "Slide 2 of 5");
  await page.keyboard.press("ArrowLeft");
  await arrives(basic, 2);
  await page.keyboard.press("ArrowRight");
  await arrives(basic, 1);
  check("rtlArrows", [await index(basic), await disabled(basic)], [1, [null, null]]);
};
