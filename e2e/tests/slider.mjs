export default async ({ page, check, setDir }) => {
  const output = (slider) => slider.evaluate((s) => s.closest("[data-slider]").querySelector("output")?.textContent.trim());
  const fill = (slider) => slider.evaluate((s) => Number(s.style.getPropertyValue("--slider-fill")));
  const formValues = (id, name) => page.evaluate(([f, n]) => new FormData(document.getElementById(f)).getAll(n), [id, name]);

  // One thumb: a native range input, labelled by the visible label.
  const level = page.getByRole("slider", { name: "Level" });
  check("initial", [await level.inputValue(), await output(level), await fill(level), await formValues("level-form", "level")], ["40", "40", 0.4, ["40"]]);
  check("native", await level.evaluate((s) => [s.tagName, s.type]), ["INPUT", "range"]);

  // The browser's slider pattern: arrows step, Home and End go to the ends.
  await level.focus();
  await page.keyboard.press("ArrowRight");
  check("arrowRight", [await level.inputValue(), await output(level), await fill(level)], ["41", "41", 0.41]);
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowLeft");
  check("arrowLeft", await level.inputValue(), "39");
  await page.keyboard.press("PageUp");
  check("pageUp", Number(await level.inputValue()) > 39);
  await page.keyboard.press("End");
  check("end", [await level.inputValue(), await output(level), await fill(level), await formValues("level-form", "level")], ["100", "100", 1, ["100"]]);
  await page.keyboard.press("Home");
  check("home", [await level.inputValue(), await output(level), await fill(level)], ["0", "0", 0]);

  // A click on the track moves the thumb there.
  const box = await level.boundingBox();
  await page.mouse.click(box.x + box.width * 0.75, box.y + box.height / 2);
  const clicked = Number(await level.inputValue());
  check("pointer", clicked >= 72 && clicked <= 78);
  check("pointerUpdatesOutput", await output(level), String(clicked));

  // Decimal steps; a formatted readout with its own value text.
  const temperature = page.getByRole("slider", { name: "Temperature" });
  check("decimal", [await temperature.inputValue(), await output(temperature)], ["0.7", "0.7"]);
  await temperature.focus();
  await page.keyboard.press("ArrowRight");
  check("decimalStep", await temperature.inputValue(), "0.8");
  const opacity = page.getByRole("slider", { name: "Opacity" });
  check("valueText", [await opacity.getAttribute("aria-valuetext"), await output(opacity)], ["80 percent", "80%"]);
  await opacity.focus();
  await page.keyboard.press("ArrowRight");
  check("step", [await opacity.inputValue(), await opacity.getAttribute("aria-valuetext"), await output(opacity)], ["85", "85 percent", "85%"]);
  const height = (slider) => slider.evaluate((s) => s.getBoundingClientRect().height);
  check("sizes", [await height(opacity), await height(temperature)], [14, 18]);
  check("disabled", await page.getByRole("slider", { name: "Volume" }).isDisabled());

  // A range: two thumbs, each named, both submitted, the track filled between them.
  const price = page.getByRole("group", { name: "Price" });
  const low = page.getByRole("slider", { name: "Minimum price" });
  const high = page.getByRole("slider", { name: "Maximum price" });
  check("range", [await price.count(), await low.inputValue(), await high.inputValue(), await output(low)], [1, "40", "160", "$40 – $160"]);
  check("rangeSubmits", await formValues("price-form", "price"), ["40", "160"]);
  check("rangeFill", await low.evaluate((s) => [Number(s.style.getPropertyValue("--slider-from")), Number(s.style.getPropertyValue("--slider-fill"))]), [0.16, 0.64]);
  check("rangeStack", await low.evaluate((s) => {
    const other = s.parentElement.querySelector("[data-thumb=end]");
    const a = s.getBoundingClientRect();
    const b = other.getBoundingClientRect();
    return [a.x === b.x, a.y === b.y, a.width === b.width];
  }), [true, true, true]);
  await low.focus();
  await page.keyboard.press("ArrowRight");
  check("rangeKeys", [await low.inputValue(), await output(low), await formValues("price-form", "price")], ["45", "$45 – $160", ["45", "160"]]);
  // Each thumb is its own tab stop, in order.
  await page.keyboard.press("Tab");
  check("rangeTab", await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Maximum price");
  // The thumbs keep two steps (10) apart.
  await page.keyboard.press("Home");
  check("minStepsBetween", [await high.inputValue(), await output(low)], ["55", "$45 – $55"]);
  await low.focus();
  await page.keyboard.press("End");
  check("minStepsBetweenLow", await low.inputValue(), "45");
  await high.focus();
  await page.keyboard.press("End");
  check("rangeEnd", [await high.inputValue(), await formValues("price-form", "price")], ["250", ["45", "250"]]);

  // A press on the track moves the nearer thumb there, focuses it, and drags it.
  const track = await low.boundingBox();
  const at = (value) => track.x + 9 + (value / 250) * (track.width - 18);
  await page.mouse.click(at(200), track.y + track.height / 2);
  check("trackPressNearer", [await high.inputValue(), await low.inputValue(), await page.evaluate(() => document.activeElement.getAttribute("aria-label"))], ["200", "45", "Maximum price"]);
  await page.mouse.move(at(100), track.y + track.height / 2);
  await page.mouse.down();
  check("pressMovesLow", await low.inputValue(), "100");
  await page.mouse.move(at(150), track.y + track.height / 2, { steps: 5 });
  check("drags", await low.inputValue(), "150");
  await page.mouse.move(at(240), track.y + track.height / 2, { steps: 5 });
  check("dragStopsBeforeOther", await low.inputValue(), "190");
  await page.mouse.up();
  check("dragOutput", await output(low), "$190 – $200");
  // A thumb is grabbed with the browser's own drag.
  await page.mouse.move(at(200), track.y + track.height / 2);
  await page.mouse.down();
  await page.mouse.move(at(230), track.y + track.height / 2, { steps: 5 });
  await page.mouse.up();
  const dragged = Number(await high.inputValue());
  check("thumbDrag", dragged >= 225 && dragged <= 235);

  // Upright: taller than wide, and up increases.
  const bass = page.getByRole("slider", { name: "Bass" });
  const upright = await bass.boundingBox();
  check("vertical", [await bass.getAttribute("aria-orientation"), upright.height > upright.width * 3], ["vertical", true]);
  await bass.focus();
  await page.keyboard.press("ArrowUp");
  check("verticalArrowUp", await bass.inputValue(), "61");
  await page.mouse.click(upright.x + upright.width / 2, upright.y + upright.height * 0.1);
  check("verticalTopIsMax", Number(await bass.inputValue()) > 80);

  await setDir("rtl");
  await level.focus();
  await page.keyboard.press("Home");
  const rtl = await level.boundingBox();
  await page.mouse.click(rtl.x + rtl.width * 0.25, rtl.y + rtl.height / 2);
  const mirrored = Number(await level.inputValue());
  check("rtlMirrors", mirrored >= 72 && mirrored <= 78);
  check("rtlFillDirection", await level.evaluate((s) => getComputedStyle(s).getPropertyValue("--slider-direction").trim()), "to left");
  // A range in right-to-left: a press near the right end moves the low thumb.
  const mirroredTrack = await low.boundingBox();
  await page.mouse.click(mirroredTrack.x + mirroredTrack.width - 9 - (20 / 250) * (mirroredTrack.width - 18), mirroredTrack.y + mirroredTrack.height / 2);
  check("rtlRangePress", await low.inputValue(), "20");
};
