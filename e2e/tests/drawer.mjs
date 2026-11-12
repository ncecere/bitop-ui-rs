export default async ({ page, check, axe, button, focused, settled }) => {
  const view = page.viewportSize();
  const drawer = (text) => page.locator("dialog[data-drawer]", { hasText: text });
  const box = async (dialog) => {
    const b = await dialog.boundingBox();
    return { x: Math.round(b.x), y: Math.round(b.y), width: Math.round(b.width), height: Math.round(b.height) };
  };
  const closed = (dialog) => dialog.evaluate((d) => !d.open);
  // Drags from the middle of `grip` by (dx, dy) in small steps, as a hand does.
  const drag = async (grip, dx, dy) => {
    const g = await grip.boundingBox();
    const x = g.x + g.width / 2;
    const y = g.y + g.height / 2;
    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x + dx, y + dy, { steps: 8 });
    return () => page.mouse.up();
  };

  const goal = drawer("Set your daily activity goal.");
  await button("Set goal").click();
  check("open", await goal.evaluate((d) => d.open && d.matches(":modal")));
  await settled(goal);
  const g = await box(goal);
  check("bottom", [g.x, g.width, g.y + g.height], [0, view.width, view.height]);
  check("name", await page.getByRole("dialog", { name: "Move goal" }).count(), 1);
  check("described", await goal.evaluate((d) => document.getElementById(d.getAttribute("aria-describedby")).textContent.trim()), "Set your daily activity goal.");
  check("focusFirstField", await page.evaluate(() => document.activeElement.name), "steps");
  check("handle", await goal.locator("[data-drawer-handle]").evaluate((h) => [h.getAttribute("aria-hidden"), getComputedStyle(h).cursor]), ["true", "grab"]);
  check("axeOpen", await axe(), []);
  for (let i = 0; i < 5; i++) await page.keyboard.press("Tab");
  check("focusTrapped", await page.evaluate(() => !!document.activeElement.closest("dialog[data-drawer]")));
  await page.keyboard.press("Escape");
  check("closedByEsc", await closed(goal));
  check("focusReturned", await focused(), "Set goal");
  await button("Set goal").click();
  await page.mouse.click(5, 5);
  check("closedByBackdrop", await closed(goal));
  await button("Set goal").click();
  await goal.getByRole("button", { name: "Cancel" }).click();
  check("closedByButton", await closed(goal));

  // A short drag follows the pointer and springs back.
  await button("Set goal").click();
  await settled(goal);
  const handle = goal.locator("[data-drawer-handle]");
  let release = await drag(handle, 0, 40);
  check("followsPointer", [await goal.evaluate((d) => d.hasAttribute("data-swiping")), (await box(goal)).y - g.y], [true, 40]);
  check("backdropFades", await goal.evaluate((d) => Number(d.style.getPropertyValue("--drawer-swipe-progress")) > 0));
  // A quick short drag is a flick, which dismisses. This one is held, so that it is slow
  // however fast the engine moves the pointer.
  await page.waitForTimeout(400);
  await release();
  const stayed = !(await closed(goal));
  check("shortDragStaysOpen", [stayed, await goal.evaluate((d) => d.hasAttribute("data-swiping"))], [true, false]);
  // `settled` would wait for ever on a closed drawer.
  if (stayed) await settled(goal);
  check("springsBack", (await box(goal)).y, g.y);
  // Dragging away from the edge does nothing.
  release = await drag(handle, 0, -60);
  check("noDragAwayFromEdge", (await box(goal)).y, g.y);
  await release();
  // A quick short drag, a flick, dismisses it.
  await goal.evaluate((d) => {
    const handle = d.querySelector("[data-drawer-handle]");
    const at = (type, y, timeStamp) => {
      const event = new PointerEvent(type, { bubbles: true, pointerId: 9, pointerType: "mouse", button: 0, clientX: 500, clientY: y });
      Object.defineProperty(event, "timeStamp", { value: timeStamp });
      handle.dispatchEvent(event);
    };
    at("pointerdown", 600, 1000);
    at("pointermove", 620, 1020);
    at("pointermove", 640, 1040);
    at("pointerup", 640, 1050);
  });
  check("flickCloses", await closed(goal));
  // A long drag dismisses it, however slow.
  await button("Set goal").click();
  await settled(goal);
  release = await drag(handle, 0, Math.round(g.height * 0.6));
  await page.waitForTimeout(400);
  await release();
  check("longDragCloses", await closed(goal));

  // The other edges.
  const right = drawer("A drawer on the right edge");
  await button("Open right").click();
  await settled(right);
  const r = await box(right);
  check("right", [r.x + r.width, r.y, r.height, r.width], [view.width, 0, view.height, 384]);
  check("noHandleOnSides", await right.locator("[data-drawer-handle]").count(), 0);
  // The × is last, so focus starts on it only because nothing else can take it.
  await right.getByRole("button", { name: "Close" }).click();
  check("closedByX", await closed(right));

  const left = drawer("A drawer on the left edge");
  await button("Open left").click();
  await settled(left);
  check("left", (await box(left)).x, 0);
  release = await drag(left.locator("[data-drawer-handle]"), -250, 0);
  await release();
  check("leftSwipeCloses", await closed(left));

  const top = drawer("A drawer on the top edge.");
  await button("Open top").click();
  await settled(top);
  check("top", [(await box(top)).y, (await box(top)).width], [0, view.width]);
  // A touch can drag by the header too; a mouse cannot, so that text can be selected.
  const header = top.locator("[data-drawer-header]");
  release = await drag(header, 0, -200);
  await release();
  check("mouseOnHeaderDoesNotSwipe", await closed(top), false);
  await header.evaluate((h) => {
    const at = (type, y) =>
      h.dispatchEvent(new PointerEvent(type, { bubbles: true, pointerId: 7, pointerType: "touch", button: 0, clientX: 300, clientY: y }));
    at("pointerdown", 60);
    at("pointermove", 30);
    at("pointermove", -140);
    at("pointerup", -140);
  });
  check("touchOnHeaderSwipes", await closed(top));
};
