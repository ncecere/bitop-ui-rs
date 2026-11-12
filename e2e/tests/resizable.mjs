export default async ({ page, base, check, focused, setDir }) => {
  const demo = (name) => page.locator(`[data-demo="${name}"]`);
  const handle = (name) => page.getByRole("separator", { name, exact: true });
  const aria = (locator) =>
    locator.evaluate((el) => ["aria-valuenow", "aria-valuemin", "aria-valuemax"].map((name) => Number(el.getAttribute(name))));
  // A panel's share of the two panels around a handle, in percent.
  const share = (locator) =>
    locator.evaluate((el) => {
      const horizontal = el.parentElement.dataset.orientation !== "vertical";
      const size = (panel) => panel.getBoundingClientRect()[horizontal ? "width" : "height"];
      const before = size(el.previousElementSibling);
      return Math.round((100 * before) / (before + size(el.nextElementSibling)));
    });
  await page.evaluate(() => {
    window.layouts = [];
    document.addEventListener("bitop:layoutchange", (event) => window.layouts.push(event.detail.layout.join(" ")));
  });

  // The script lays the group out from the default sizes and gives each handle the
  // window splitter's ARIA: the first panel's size, its limits, and what it controls.
  const side = handle("Resize sidebar");
  check("splitter", await side.evaluate((el) => ["aria-orientation", "aria-controls", "tabindex"].map((name) => el.getAttribute(name))), ["vertical", "resize-sidebar", "0"]);
  check("values", await aria(side), [30, 20, 50]);
  check("layout", [await share(side), await demo("horizontal").locator("[data-size-pending]").count()], [30, 0]);
  const frame = await demo("horizontal").boundingBox();
  const line = await side.boundingBox();
  check("handleLine", [Math.round(line.width), Math.round(line.height) === Math.round(frame.height), await side.locator("[data-icon=grip-vertical]").count()], [1, true, 1]);

  // The keyboard: arrows move by five percent, Home and End go to the limits.
  await side.focus();
  await page.keyboard.press("ArrowRight");
  check("arrowRight", [await aria(side), await share(side)], [[35, 20, 50], 35]);
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowLeft");
  check("arrowLeft", [await aria(side), await share(side)], [[25, 20, 50], 25]);
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowLeft");
  check("stopsAtMin", [await aria(side), await share(side)], [[20, 20, 50], 20]);
  await page.keyboard.press("End");
  check("end", [await aria(side), await share(side)], [[50, 20, 50], 50]);
  await page.keyboard.press("ArrowRight");
  check("stopsAtMax", await share(side), 50);
  await page.keyboard.press("Home");
  check("home", [await aria(side), await share(side)], [[20, 20, 50], 20]);
  await page.keyboard.press("ArrowDown");
  check("otherAxisIgnored", await share(side), 20);
  check("layoutEvents", await page.evaluate(() => window.layouts.slice(0, 2).concat(window.layouts.at(-1))), ["35 65", "30 70", "20 80"]);

  // The pointer: dragging moves the handle with it, within the limits.
  let box = await side.boundingBox();
  const width = await demo("horizontal").evaluate((el) => el.getBoundingClientRect().width - 1);
  await page.mouse.move(box.x + 0.5, box.y + box.height / 2);
  await page.mouse.down();
  await page.mouse.move(box.x + 0.5 + width / 10, box.y + box.height / 2, { steps: 4 });
  check("dragging", [await side.evaluate((el) => el.hasAttribute("data-dragging")), await share(side)], [true, 30]);
  await page.mouse.up();
  check("dragged", [await side.evaluate((el) => el.hasAttribute("data-dragging")), await aria(side), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], [false, [30, 20, 50], "Resize sidebar"]);
  // Firefox and WebKit place the handle on a fraction of a pixel.
  check("dragEvent", await page.evaluate(() => window.layouts.at(-1).split(" ").map((size) => Math.round(Number(size)))), [30, 70]);
  box = await side.boundingBox();
  await page.mouse.move(box.x + 0.5, box.y + 20);
  await page.mouse.down();
  // Far past the limit, but inside the window.
  await page.mouse.move(box.x + 300, box.y + 20, { steps: 4 });
  await page.mouse.up();
  check("dragStopsAtMax", [await aria(side), await share(side)], [[50, 20, 50], 50]);

  // Stacked: the splitter is horizontal, and up and down move it, here by ten.
  const output = handle("Resize output");
  check("vertical", [await output.getAttribute("aria-orientation"), await output.getAttribute("aria-controls"), await aria(output), await share(output)], ["horizontal", "resize-code", [60, 0, 80], 60]);
  const row = await output.boundingBox();
  check("verticalLine", [Math.round(row.height), row.width > 100], [1, true]);
  await output.focus();
  await page.keyboard.press("ArrowDown");
  check("arrowDown", [await aria(output), await share(output)], [[70, 0, 80], 70]);
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("ArrowRight");
  check("arrowUp", await share(output), 60);
  await page.keyboard.press("End");
  check("verticalEnd", await share(output), 80);
  box = await output.boundingBox();
  const height = await demo("vertical").evaluate((el) => el.getBoundingClientRect().height - 1);
  await page.mouse.move(box.x + 50, box.y + 0.5);
  await page.mouse.down();
  await page.mouse.move(box.x + 50, box.y + 0.5 - height * 0.3, { steps: 4 });
  await page.mouse.up();
  check("verticalDrag", [await aria(output), await share(output)], [[50, 0, 80], 50]);

  // Three panels with a group inside the middle one; a handle's limits take both of
  // its panels into account, and a disabled handle cannot be focused or moved.
  const files = handle("Resize files");
  const preview = handle("Resize preview");
  const terminal = handle("Resize terminal");
  check("three", [await aria(files), await aria(preview), await aria(terminal)], [[25, 10, 45], [50, 30, 75], [60, 0, 100]]);
  check("nestedOrientation", [await files.getAttribute("aria-orientation"), await terminal.getAttribute("aria-orientation")], ["vertical", "horizontal"]);
  check("disabled", [await preview.getAttribute("aria-disabled"), await preview.getAttribute("tabindex")], ["true", null]);
  box = await preview.boundingBox();
  await page.mouse.move(box.x + 0.5, box.y + 30);
  await page.mouse.down();
  await page.mouse.move(box.x - 80, box.y + 30, { steps: 3 });
  await page.mouse.up();
  check("disabledDoesNotMove", await aria(preview), [50, 30, 75]);
  await files.focus();
  await page.keyboard.press("Tab");
  check("tabToInnerHandle", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Resize terminal");
  await page.keyboard.press("ArrowUp");
  check("innerMoves", [await aria(terminal), await aria(files)], [[55, 0, 100], [25, 10, 45]]);
  await files.focus();
  await page.keyboard.press("End");
  check("limitedByNeighbour", [await aria(files), await aria(preview)], [[45, 10, 45], [30, 30, 55]]);

  // Kept in this browser: the layout is stored, and comes back after a reload.
  check("stored", await page.evaluate(() => localStorage.getItem("bitop-gallery-resizable")), "[45,30,25]");
  await page.goto(`${base}/c/resizable`);
  check("restored", [await aria(handle("Resize files")), await share(handle("Resize files")), await aria(handle("Resize sidebar"))], [[45, 10, 45], 60, [30, 20, 50]]);

  // Right to left: the first panel is on the right, and the arrows follow what is seen.
  await setDir("rtl");
  const rtl = handle("Resize sidebar");
  const sidebar = await page.locator("#resize-sidebar").boundingBox();
  const editor = await page.locator("#resize-editor").boundingBox();
  check("rtlOrder", sidebar.x > editor.x);
  await rtl.focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlLeftGrows", await aria(rtl), [35, 20, 50]);
  await page.keyboard.press("ArrowRight");
  check("rtlRightShrinks", await aria(rtl), [30, 20, 50]);
  box = await rtl.boundingBox();
  const rtlWidth = await demo("horizontal").evaluate((el) => el.getBoundingClientRect().width - 1);
  await page.mouse.move(box.x + 0.5, box.y + 20);
  await page.mouse.down();
  await page.mouse.move(box.x + 0.5 - rtlWidth / 10, box.y + 20, { steps: 4 });
  await page.mouse.up();
  check("rtlDragLeftGrows", [await aria(rtl), await share(rtl)], [[40, 20, 50], 40]);
};
