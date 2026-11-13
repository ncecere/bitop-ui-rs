export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const link = page.getByRole("link", { name: "@ada", exact: true });
  const card = page.locator("[data-hover-card-popup]", { hasText: "Mathematician and writer" });
  const open = (popup = card) => popup.evaluate((c) => c.matches(":popover-open"));
  // Waits, with the usual time limit, for a popover to be open or closed.
  const untilOpen = async (popup, want) =>
    page.waitForFunction(([el, w]) => el.matches(":popover-open") === w, [await popup.elementHandle(), want]);
  const centre = (box) => box.x + box.width / 2;

  check("closedAtFirst", [await open(), await link.getAttribute("aria-expanded")], [false, "false"]);
  check("controls", await link.evaluate((a) => document.getElementById(a.getAttribute("aria-controls")).getAttribute("aria-label")), "Ada Lovelace");

  // Keyboard focus opens it at once; Tab goes into it, because it follows the link.
  await button("Collapse sidebar").focus();
  for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
  check("onFocus", [await focused(), await open(), await link.getAttribute("aria-expanded")], ["@ada", true, "true"]);
  await settled(card);
  // The link is inline text, so its height is a fraction of a pixel that differs by font.
  const l = await link.boundingBox();
  const c = await card.boundingBox();
  check("anchored", { below: c.y >= l.y + l.height, gap: Math.abs(c.y - l.y - l.height - 8) <= 1, centred: Math.abs(centre(c) - centre(l)) < 2, width: Math.round(c.width) }, { below: true, gap: true, centred: true, width: 320 });
  check("axeOpen", await axe(), []);
  await page.keyboard.press("Tab");
  check("tabIntoCard", [await focused(), await open()], ["View profile", true]);
  await page.keyboard.press("Shift+Tab");
  check("tabBack", [await focused(), await open()], ["@ada", true]);
  await page.keyboard.press("Tab");
  await page.keyboard.press("Escape");
  check("escFromCard", [await open(), await focused(), await link.getAttribute("aria-expanded")], [false, "@ada", "false"]);
  // Focus leaving both closes it.
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Tab");
  check("reopensOnFocus", await open());
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  check("closesWhenFocusLeaves", [await open(), await focused()], [false, "analytical-engine"]);
  await page.keyboard.press("Escape");
  await page.evaluate(() => document.activeElement.blur());

  // Hover opens it after a delay, and the pointer can move into the card. The delays
  // are measured in the page, so that a slow machine does not change the result.
  await page.mouse.move(0, 0);
  await page.evaluate(() => {
    window.cardTimes = {};
    const mark = (name) => (window.cardTimes[name] = performance.now());
    document.addEventListener("pointerover", (event) => event.target.closest?.("[data-hover-card]") && mark("over"), true);
    document.addEventListener("pointerout", (event) => event.target.closest?.("[data-hover-card], [data-hover-card-popup]") && mark("out"), true);
    document.addEventListener("beforetoggle", (event) => mark(event.newState), true);
  });
  await link.hover();
  await untilOpen(card, true);
  check("onHover", await open());
  check("afterADelay", await page.evaluate(() => window.cardTimes.open - window.cardTimes.over >= 550));
  await settled(card);
  await card.getByRole("link", { name: "View profile" }).hover();
  // Longer than the closing delay.
  await page.waitForTimeout(450);
  check("staysWhileInCard", await open());
  await page.mouse.move(0, 0);
  await untilOpen(card, false);
  check("hidesOnLeave", [await open(), await link.getAttribute("aria-expanded")], [false, "false"]);
  check("closesAfterADelay", await page.evaluate(() => window.cardTimes.closed - window.cardTimes.out >= 250));

  // The other sides.
  const repo = page.locator("[data-hover-card-popup]", { hasText: "general-purpose" });
  const repoLink = page.getByRole("link", { name: "analytical-engine" });
  await repoLink.hover();
  await untilOpen(repo, true);
  await settled(repo);
  let t = await repoLink.boundingBox();
  let p = await repo.boundingBox();
  check("topStart", { above: p.y + p.height <= t.y, aligned: Math.abs(p.x - t.x) < 2 }, { above: true, aligned: true });
  const notes = page.locator("[data-hover-card-popup]", { hasText: "Menabrea" });
  const notesLink = page.getByRole("link", { name: "Notes" });
  await notesLink.hover();
  await untilOpen(notes, true);
  check("oneAtATime", await open(repo), false);
  await settled(notes);
  t = await notesLink.boundingBox();
  p = await notes.boundingBox();
  check("right", { after: p.x >= t.x + t.width, centred: Math.abs(p.y + p.height / 2 - (t.y + t.height / 2)) < 2 }, { after: true, centred: true });
  await page.mouse.move(0, 0);
  await untilOpen(notes, false);

  // Right to left: start alignment mirrors.
  await setDir("rtl");
  await repoLink.hover();
  await untilOpen(repo, true);
  await settled(repo);
  t = await repoLink.boundingBox();
  p = await repo.boundingBox();
  check("rtlStart", Math.abs(p.x + p.width - (t.x + t.width)) < 2);
  await page.mouse.move(0, 0);
  await untilOpen(repo, false);
};
