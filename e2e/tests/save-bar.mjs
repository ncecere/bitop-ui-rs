export default async ({ page, check, axe, button, focused, settled, setDir }) => {
  const scroller = page.locator('[data-demo="scroller"]');
  const form = page.locator('[data-demo="form"]');
  const bar = form.locator("[data-save-bar]");
  const status = bar.getByRole("status");
  const name = page.getByLabel("Name");
  const box = async (locator) => {
    const b = await locator.boundingBox();
    return { x: Math.round(b.x), y: Math.round(b.y), r: Math.round(b.x + b.width), b: Math.round(b.y + b.height), h: Math.round(b.height) };
  };
  await page.evaluate(() => {
    window.barEvents = [];
    document.addEventListener("bitop:toggle", (event) => window.barEvents.push(event.detail.open));
  });

  // Closed: no room taken, nothing to announce, the actions hidden.
  check("closed", await bar.evaluate((el) => [el.hasAttribute("data-open"), el.getBoundingClientRect().height, getComputedStyle(el).position]), [false, 0, "absolute"]);
  check("closedStatus", [await status.count(), await status.textContent()], [1, ""]);
  check("closedActions", await form.getByRole("button", { name: "Save" }).count(), 0);
  const before = await form.evaluate((el) => el.getBoundingClientRect().height);

  // Typing in a field opens it: the message is put into the live region, and the bar
  // sticks to the bottom of the scrolling area.
  await name.focus();
  await page.keyboard.press("End");
  await page.keyboard.type(" L");
  await page.waitForFunction(() => document.querySelector('[data-demo="form"] [data-save-bar]').hasAttribute("data-open"));
  await settled(bar);
  check("open", [await status.textContent(), await form.getByRole("button", { name: "Save" }).isVisible(), await form.getByRole("button", { name: "Discard" }).isVisible()], ["Unsaved changes", true, true]);
  check("events", await page.evaluate(() => window.barEvents), [true]);
  let s = await box(scroller);
  let b = await box(bar);
  // 16 pixels above the scrolling area's padding (16) and border (1).
  check("sticksToBottom", [await bar.evaluate((el) => getComputedStyle(el).position), s.b - b.b], ["sticky", 33]);
  check("overflows", await scroller.evaluate((el) => el.scrollHeight > el.clientHeight + 40));
  check("inFlowWhenOpen", (await form.evaluate((el) => el.getBoundingClientRect().height)) > before);
  check("axeOpen", await axe(), []);
  await scroller.evaluate((el) => el.scrollTo(0, 60));
  b = await box(bar);
  check("staysWhileScrolling", s.b - b.b, 33);
  // At the end of the form it rests after the last field, still clear of the edge.
  await scroller.evaluate((el) => el.scrollTo(0, el.scrollHeight));
  const about = await box(page.getByLabel("About"));
  b = await box(bar);
  check("restsAtEnd", [b.y - about.b, s.b - b.b], [16, 33]);

  // Typed back to what it was, the form is no longer dirty and the bar closes; typed
  // again, it opens.
  await name.focus();
  await page.keyboard.press("End");
  await page.keyboard.press("Backspace");
  await page.keyboard.press("Backspace");
  await page.waitForFunction(() => !document.querySelector('[data-demo="form"] [data-save-bar]').hasAttribute("data-open"));
  check("cleanCloses", [await name.inputValue(), await status.textContent(), await form.getByRole("button", { name: "Save" }).count()], ["Ada", "", 0]);
  await page.keyboard.type("!");
  await page.waitForFunction(() => document.querySelector('[data-demo="form"] [data-save-bar]').hasAttribute("data-open"));
  check("eventsDirtyAgain", await page.evaluate(() => window.barEvents), [true, false, true]);
  await settled(bar);
  await scroller.evaluate((el) => el.scrollTo(0, el.scrollHeight));

  // The actions are in the tab order after the fields.
  await page.getByLabel("About").focus();
  await page.keyboard.press("Tab");
  check("tabToDiscard", await focused(), "Discard");
  await page.keyboard.press("Tab");
  check("tabToSave", await focused(), "Save");

  // Discard resets the form, which closes the bar and empties the live region.
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Enter");
  check("reset", [await name.inputValue(), await bar.evaluate((el) => el.hasAttribute("data-open")), await status.textContent(), await form.getByRole("button", { name: "Save" }).count()], ["Ada", false, "", 0]);
  check("eventsAfterReset", await page.evaluate(() => window.barEvents), [true, false, true, false]);

  // A change event that leaves the form as it was does not open it; one that changes a
  // field (a select, a checkbox) does, and so can a script.
  await name.evaluate((el) => el.dispatchEvent(new Event("change", { bubbles: true })));
  check("cleanChangeStaysClosed", await bar.evaluate((el) => el.hasAttribute("data-open")), false);
  await name.evaluate((el) => { el.value = "Grace"; el.dispatchEvent(new Event("change", { bubbles: true })); });
  check("changeOpens", await bar.evaluate((el) => el.hasAttribute("data-open")));
  await form.evaluate((el) => el.reset());
  await bar.evaluate((el) => window.bitopUi.setSaveBar(el, false));
  check("scriptCloses", [await bar.evaluate((el) => el.hasAttribute("data-open")), await status.textContent()], [false, ""]);
  await bar.evaluate((el) => window.bitopUi.setSaveBar(el, true));
  check("scriptOpens", await status.textContent(), "Unsaved changes");
  await bar.evaluate((el) => window.bitopUi.setSaveBar(el, false));

  // Rendered open, with its own message; it is not automatic, so a reset leaves it.
  const open = page.locator('[data-demo="open"] [data-save-bar]');
  check("renderedOpen", [await open.getByRole("status").textContent(), await open.getAttribute("data-save-bar"), await open.getByRole("button", { name: "Save changes" }).isVisible()], ["2 unsaved changes", "", true]);
  await open.getByRole("button", { name: "Discard" }).click();
  check("notAutomatic", await open.evaluate((el) => el.hasAttribute("data-open")));
  const ob = await box(open);
  const message = await box(open.getByRole("status"));
  const save = await box(open.getByRole("button", { name: "Save changes" }));
  check("messageThenActions", [message.x - ob.x, ob.r - save.r], [16, 16]);

  // Right to left: the message is on the right and the actions on the left.
  await setDir("rtl");
  const rb = await box(open);
  const rm = await box(open.getByRole("status"));
  const rs = await box(open.getByRole("button", { name: "Save changes" }));
  check("rtl", [rb.r - rm.r, rs.x - rb.x], [16, 16]);
};
