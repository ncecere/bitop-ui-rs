export default async ({ page, check, axe, button, focused, settled }) => {
  const region = page.getByRole("region", { name: "Notifications" });
  const toasts = page.locator("[data-toast-region] > [data-toast]");
  const live = page.locator("[data-toast-region] > [data-toast]:not([data-closing])");
  const titles = () => live.evaluateAll((all) => all.map((t) => t.querySelector("[data-toast-title]").textContent));
  const away = () => page.mouse.move(5, 5);
  const gone = (text) => page.waitForFunction((t) => ![...document.querySelectorAll("[data-toast]")].some((el) => el.textContent.includes(t)), text);

  // Tones, from the buttons.
  await button("Success").click();
  await away();
  const first = toasts.first();
  check("success", [await first.getAttribute("role"), await first.getAttribute("data-tone"), await first.locator("[data-icon=circle-check]").count()], ["status", "success", 1]);
  await button("Info").click();
  await button("Warning").click();
  await button("Error").click();
  await away();
  check("newestFirst", await titles(), ["Deployment failed", "Slow build", "Build queued", "Deployment ready"]);
  check("dangerIsAlert", await toasts.first().getAttribute("role"), "alert");
  check("icons", await live.evaluateAll((all) => all.map((t) => t.querySelector("[data-icon]:not([data-icon=x])")?.dataset.icon)), ["circle-alert", "triangle-alert", "info", "circle-check"]);
  check("noColouredEdge", await toasts.first().evaluate((t) => getComputedStyle(t).borderInlineStartWidth), "0px");
  // Collapsed: the newest is in front; the ones behind are smaller and show only their edge.
  const stack = await live.evaluateAll((all) =>
    all.map((t) => ({ index: t.style.getPropertyValue("--toast-index"), limited: t.hasAttribute("data-limited"), behind: t.hasAttribute("data-behind") })),
  );
  check("stack", stack.map((s) => [s.index, s.limited, s.behind]), [["0", false, false], ["1", false, true], ["2", false, true], ["3", true, true]]);
  check("limitInert", await live.nth(3).evaluate((t) => t.inert), true);
  await settled(live.first());
  const front = await live.first().boundingBox();
  const second = await live.nth(1).boundingBox();
  check("peeks", [Math.round(front.y - second.y) > 0, second.width < front.width], [true, true]);
  check("axeOpen", await axe(), []);

  // Hovering spreads the stack out and pauses the timers.
  await live.first().hover();
  await page.waitForFunction(() => document.querySelector("[data-toast-region]").hasAttribute("data-expanded"));
  await page.waitForTimeout(500);
  const spread = await live.evaluateAll((all) => all.slice(0, 3).map((t) => t.getBoundingClientRect()));
  check("expanded", [spread[0].y > spread[1].bottom, spread[1].y > spread[2].bottom, Math.round(spread[1].width) === Math.round(spread[0].width)], [true, true, true]);
  await away();
  await page.waitForFunction(() => !document.querySelector("[data-toast-region]").hasAttribute("data-expanded"));
  await page.evaluate(() => window.bitopUi.toast.close());
  await page.waitForFunction(() => !document.querySelector("[data-toast]"));

  // An action button, here Undo.
  await button("With action").click();
  await away();
  const undo = toasts.first().getByRole("button", { name: "Undo" });
  check("action", await undo.isVisible());
  await undo.click();
  await away();
  check("actionRuns", (await titles())[0], "Token restored");
  await page.evaluate(() => window.bitopUi.toast.close());
  await page.waitForFunction(() => !document.querySelector("[data-toast]"));

  // Timers: a toast times out, and pauses while the pointer is on it.
  await page.evaluate(() => window.bitopUi.toast.add({ title: "Short", timeout: 200 }));
  await gone("Short");
  check("timesOut", await toasts.count(), 0);
  await page.evaluate(() => window.bitopUi.toast({ title: "Held", timeout: 1200 }));
  await toasts.first().hover();
  await page.waitForTimeout(1600);
  check("pausesOnHover", await live.count(), 1);
  await away();
  await gone("Held");
  check("resumes", await toasts.count(), 0);

  // close(id), update by ID, and promise.
  const id = await page.evaluate(() => window.bitopUi.toast.add({ title: "Uploading", timeout: 0 }));
  check("returnsId", typeof id, "string");
  await page.evaluate((i) => window.bitopUi.toast.add({ id: i, title: "Uploaded", tone: "success", timeout: 0 }), id);
  check("updatesById", [await live.count(), (await titles())[0], await live.first().getAttribute("data-tone")], [1, "Uploaded", "success"]);
  await page.evaluate((i) => window.bitopUi.toast.close(i), id);
  await page.waitForFunction(() => !document.querySelector("[data-toast]"));
  check("closeById", await toasts.count(), 0);
  await page.evaluate(() => {
    window.resolveSave = undefined;
    window.bitopUi.toast.promise(new Promise((resolve) => (window.resolveSave = resolve)), {
      loading: "Saving…",
      success: (name) => ({ title: "Saved", description: name }),
      error: "Could not save",
    });
  });
  check("promiseLoading", [(await titles())[0], await live.first().getAttribute("data-tone")], ["Saving…", "loading"]);
  await page.evaluate(() => window.resolveSave("draft.md"));
  await page.waitForFunction(() => document.querySelector("[data-toast]")?.dataset.tone === "success");
  check("promiseSuccess", (await live.first().textContent()).trim(), "Saveddraft.md");
  await page.evaluate(() => window.bitopUi.toast.promise(Promise.reject(new Error("x")), { loading: "Saving…", success: "Saved", error: "Could not save" }).catch(() => {}));
  await page.waitForFunction(() => document.querySelector("[data-toast]")?.dataset.tone === "danger");
  check("promiseError", [(await titles())[0], await live.first().getAttribute("role")], ["Could not save", "alert"]);
  await page.evaluate(() => window.bitopUi.toast.close());
  await page.waitForFunction(() => !document.querySelector("[data-toast]"));

  // F6 moves focus to the region and pauses; Tab goes through the toasts; Escape closes
  // the focused one; focus then returns where it was.
  await page.evaluate(() => {
    window.bitopUi.toast.add({ title: "Older", timeout: 1000 });
    window.bitopUi.toast.add({ title: "Newer", timeout: 1000 });
  });
  await button("Info").focus();
  await page.keyboard.press("F6");
  check("f6", await page.evaluate(() => document.activeElement.matches("[data-toast-region]")));
  await page.waitForTimeout(1400);
  check("f6Pauses", await titles(), ["Newer", "Older"]);
  check("f6Expands", await region.evaluate((r) => r.hasAttribute("data-expanded")));
  await page.keyboard.press("Tab");
  check("tabToNewest", await page.evaluate(() => document.activeElement.querySelector("[data-toast-title]")?.textContent), "Newer");
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => document.activeElement.querySelector?.("[data-toast-title]")?.textContent === "Older");
  check("escMovesToNext", await titles(), ["Older"]);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.activeElement.closest("[data-toast-region]"));
  check("focusRestored", await focused(), "Info");
  await page.waitForFunction(() => !document.querySelector("[data-toast]"));
};
