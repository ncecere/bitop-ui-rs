export default async ({ page, check, setDir }) => {
  const volume = page.getByRole("slider", { name: "Volume" });
  const output = (slider) => slider.evaluate((s) => s.closest("[data-slider]").querySelector("output")?.textContent.trim());
  const fill = (slider) => slider.evaluate((s) => Number(s.style.getPropertyValue("--slider-fill")));
  const formValue = () => page.evaluate(() => new FormData(document.getElementById("volume-form")).get("volume"));

  check("initial", [await volume.inputValue(), await output(volume), await fill(volume), await formValue()], ["40", "40", 0.4, "40"]);
  check("native", await volume.evaluate((s) => [s.tagName, s.type]), ["INPUT", "range"]);

  // The browser's slider pattern: arrows step, Home and End go to the ends.
  await volume.focus();
  await page.keyboard.press("ArrowRight");
  check("arrowRight", [await volume.inputValue(), await output(volume), await fill(volume)], ["41", "41", 0.41]);
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowLeft");
  check("arrowLeft", await volume.inputValue(), "39");
  await page.keyboard.press("ArrowUp");
  check("arrowUp", await volume.inputValue(), "40");
  await page.keyboard.press("PageUp");
  check("pageUp", Number(await volume.inputValue()) > 40);
  await page.keyboard.press("End");
  check("end", [await volume.inputValue(), await output(volume), await fill(volume), await formValue()], ["100", "100", 1, "100"]);
  await page.keyboard.press("Home");
  check("home", [await volume.inputValue(), await output(volume), await fill(volume)], ["0", "0", 0]);

  // A click on the track moves the thumb there.
  const box = await volume.boundingBox();
  await page.mouse.click(box.x + box.width * 0.75, box.y + box.height / 2);
  const clicked = Number(await volume.inputValue());
  check("pointer", clicked >= 72 && clicked <= 78);
  check("pointerUpdatesOutput", await output(volume), String(clicked));

  // The filled part of the track ends under the thumb.
  const gradient = await volume.evaluate((s) => getComputedStyle(s).getPropertyValue("--slider-gradient"));
  check("trackFill", gradient.includes("linear-gradient"));

  const zoom = page.getByRole("slider", { name: "Zoom" });
  check("valueText", [await zoom.getAttribute("aria-valuetext"), await output(zoom)], ["100 percent", "100%"]);
  await zoom.focus();
  await page.keyboard.press("ArrowRight");
  check("step", [await zoom.inputValue(), await zoom.getAttribute("aria-valuetext"), await output(zoom)], ["125", "125 percent", "125%"]);
  await page.keyboard.press("Home");
  check("min", [await zoom.inputValue(), await fill(zoom)], ["25", 0]);

  const opacity = page.getByRole("slider", { name: "Opacity" });
  await opacity.focus();
  await page.keyboard.press("ArrowRight");
  check("decimalStep", await opacity.inputValue(), "0.6");
  const height = (slider) => slider.evaluate((s) => s.getBoundingClientRect().height);
  check("sizes", [await height(opacity), await height(volume)], [14, 18]);

  // Upright: taller than wide, and up increases.
  const level = page.getByRole("slider", { name: "Level" });
  const upright = await level.boundingBox();
  check("vertical", [await level.getAttribute("aria-orientation"), upright.height > upright.width * 3], ["vertical", true]);
  await level.focus();
  await page.keyboard.press("ArrowUp");
  check("verticalArrowUp", await level.inputValue(), "31");
  await page.mouse.click(upright.x + upright.width / 2, upright.y + upright.height * 0.1);
  check("verticalTopIsMax", Number(await level.inputValue()) > 80);

  const disabled = page.getByRole("slider", { name: "Disabled" });
  check("disabled", [await disabled.isDisabled(), await output(disabled)], [true, "60"]);

  await setDir("rtl");
  await volume.focus();
  await page.keyboard.press("Home");
  const rtl = await volume.boundingBox();
  await page.mouse.click(rtl.x + rtl.width * 0.25, rtl.y + rtl.height / 2);
  const mirrored = Number(await volume.inputValue());
  check("rtlMirrors", mirrored >= 72 && mirrored <= 78);
  check("rtlFillDirection", await volume.evaluate((s) => getComputedStyle(s).getPropertyValue("--slider-direction").trim()), "to left");
};
