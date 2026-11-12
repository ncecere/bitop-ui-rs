export default async ({ page, check, button, setDir }) => {
  const group = (name) => page.getByRole("group", { name, exact: true });
  const pressed = (name) => group(name).locator("[aria-pressed=true]").allTextContents().then((all) => all.map((t) => t.trim()));
  const stops = (name) => group(name).locator("[tabindex='0']").allTextContents().then((all) => all.map((t) => t.trim()));
  const focused = () => page.evaluate(() => document.activeElement.textContent.trim() || document.activeElement.getAttribute("aria-label"));
  const formValue = (id, name) => page.evaluate(([i, n]) => new FormData(document.getElementById(i)).getAll(n), [id, name]);

  // One tab stop per group: the pressed item.
  check("tabStop", await stops("Text alignment"), ["Left"]);
  check("tabStopMultiple", await stops("Text style"), ["Italic"]);
  check("formInitial", await formValue("align-form", "align"), ["left"]);

  await button("Centre").click();
  check("single", await pressed("Text alignment"), ["Centre"]);
  check("formSingle", await formValue("align-form", "align"), ["center"]);
  check("tabStopFollows", await stops("Text alignment"), ["Centre"]);

  // Arrow keys move focus without pressing; they wrap and skip a disabled item.
  await button("Centre").focus();
  await page.keyboard.press("ArrowRight");
  check("arrowMovesFocus", [await focused(), await pressed("Text alignment")], ["Right", ["Centre"]]);
  await page.keyboard.press("ArrowRight");
  check("wrapsPastDisabled", await focused(), "Left");
  await page.keyboard.press("End");
  check("end", await focused(), "Right");
  await page.keyboard.press("Home");
  check("home", await focused(), "Left");
  await page.keyboard.press("ArrowLeft");
  check("arrowBack", await focused(), "Right");
  await page.keyboard.press("Space");
  check("spacePresses", await pressed("Text alignment"), ["Right"]);
  await page.keyboard.press("Enter");
  check("enterReleases", [await pressed("Text alignment"), await formValue("align-form", "align")], [[], []]);
  await page.keyboard.press("Enter");

  // Tab leaves the group in one step, and the tab stop goes back to the pressed item.
  await page.keyboard.press("ArrowLeft");
  check("movedOffPressed", await focused(), "Centre");
  await page.keyboard.press("Tab");
  check("tabLeaves", await focused(), "Italic");
  check("tabStopResets", await stops("Text alignment"), ["Right"]);
  await page.keyboard.press("Shift+Tab");
  check("shiftTabReturnsToPressed", await focused(), "Right");

  await button("Justify").click({ force: true });
  check("disabledItem", [await button("Justify").isDisabled(), await pressed("Text alignment")], [true, ["Right"]]);

  await button("Bold").click();
  check("multiple", await pressed("Text style"), ["Bold", "Italic", "Underline"]);
  await button("Italic").click();
  check("multipleReleases", await pressed("Text style"), ["Bold", "Underline"]);
  check("formMultiple", await formValue("style-form", "style"), ["bold", "underline"]);

  // An item takes its variant and size from the group.
  const outline = await button("Bold").evaluate((b) => getComputedStyle(b).getPropertyValue("--toggle-ring"));
  const ghost = await button("Left").evaluate((b) => getComputedStyle(b).getPropertyValue("--toggle-ring"));
  check("variantFromGroup", outline !== ghost);
  const add = await button("Add").boundingBox();
  check("sizeFromGroup", [add.width, add.height], [32, 32]);

  const track = await group("Period").boundingBox();
  const week = await group("Period").getByText("Week").boundingBox();
  check("joinedTrack", [track.height, week.height].map(Math.round), [36, 32]);
  const small = await group("Period, small").boundingBox();
  check("joinedSmall", Math.round(small.height), 32);

  const range = group("Range");
  check("scrolls", await range.evaluate((g) => g.scrollWidth > g.clientWidth && getComputedStyle(g).overflowX === "auto"));
  await button("Last hour").focus();
  await page.keyboard.press("End");
  check("scrollsFocusedIntoView", await range.evaluate((g) => g.scrollLeft > 0));

  await button("Search").focus();
  await page.keyboard.press("ArrowDown");
  check("verticalArrow", await focused(), "Add");
  await page.keyboard.press("ArrowRight");
  check("verticalIgnoresSideArrows", await focused(), "Add");
  await page.keyboard.press("ArrowUp");
  check("verticalArrowBack", await focused(), "Search");

  await setDir("rtl");
  await group("Period").getByText("Week").focus();
  await page.keyboard.press("ArrowLeft");
  check("rtlArrows", await focused(), "Month");
};
