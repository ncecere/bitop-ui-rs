export default async ({ page, check }) => {
  const list = page.getByRole("region", { name: "Episode 12 transcript" });
  const segments = list.getByRole("button");
  const current = (locator) => locator.evaluate((root) => [...root.querySelectorAll("[data-transcription-segment]")].map((s) => s.getAttribute("aria-current") === "true"));

  // Segments with text, each a button named by its time, speaker and text.
  check("segments", await segments.evaluateAll((all) => all.map((b) => b.textContent.replace(/\s+/g, " ").trim())), [
    "0:00 Ana Welcome back to the show.",
    "0:02 Ana Today: what changed in the leave policy.",
    "0:06 Ben Parental leave is now sixteen weeks.",
    "0:08 Ben And it starts on the first day of the job.",
  ]);
  check("ordered", await list.locator("ol > li").count(), 4);
  check("startActive", await current(list), [true, false, false, false]);

  // The media must load before it can seek. Every engine here plays WAV.
  const audio = page.locator("#episode-audio");
  await page.waitForFunction(() => document.getElementById("episode-audio").readyState >= 1);
  check("duration", await audio.evaluate((a) => Math.round(a.duration)), 12);

  // Enter on a segment seeks the audio, and the highlight follows.
  await page.evaluate(() => {
    window.seeks = [];
    document.addEventListener("bitop:transcription-seek", (e) => window.seeks.push(e.detail.time));
  });
  await segments.nth(2).focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => Math.abs(document.getElementById("episode-audio").currentTime - 6) < 0.01);
  await page.waitForFunction(() => document.querySelectorAll("[data-media=episode-audio] [aria-current=true]")[0]?.dataset.start === "6");
  check("seeked", [await page.evaluate(() => window.seeks), await current(list)], [[6], [false, false, true, false]]);
  check("past", await list.locator("[data-transcription-segment]").evaluateAll((all) => all.map((s) => s.hasAttribute("data-past"))), [true, true, false, false]);
  // The active segment does not rely on colour: it has a bar on its start side.
  check("bar", await segments.nth(2).evaluate((b) => getComputedStyle(b).boxShadow.includes("inset")), true);

  // Playback moves the highlight on.
  await audio.evaluate((a) => {
    a.muted = true;
    a.currentTime = 8.2;
    return a.play();
  });
  await page.waitForFunction(() => document.querySelector("[data-media=episode-audio] [data-start='8.5'][aria-current=true]"));
  check("follows", await current(list), [false, false, false, true]);
  await audio.evaluate((a) => a.pause());

  // Inline: the text flows; times are read but not shown. It follows data-current-time.
  const inline = page.locator("#inline-transcript");
  check("inlineLayout", [await inline.getAttribute("data-layout"), await inline.locator("li").first().evaluate((li) => getComputedStyle(li).display)], ["inline", "inline"]);
  check("inlineTimeHidden", await inline.locator("button span").first().evaluate((s) => [s.classList.contains("sr-only"), s.textContent]), [true, "0:00 "]);
  check("inlineStart", await current(inline), [false, true, false, false]);
  await page.evaluate(() => (document.getElementById("inline-transcript").dataset.currentTime = "7"));
  await page.waitForFunction(() => document.querySelector("#inline-transcript [data-start='6'][aria-current=true]"));
  check("inlineFollows", await current(inline), [false, false, true, false]);
  // A page with its own player cancels the seek.
  await page.evaluate(() => document.getElementById("inline-transcript").addEventListener("bitop:transcription-seek", (e) => e.preventDefault(), { once: true }));
  await inline.getByRole("button").first().click();
  check("canceled", [await inline.getAttribute("data-current-time"), await current(inline)], ["7", [false, false, true, false]]);
  await inline.getByRole("button").first().click();
  await page.waitForFunction(() => document.getElementById("inline-transcript").dataset.currentTime === "0");
  check("inlineSeek", await current(inline), [true, false, false, false]);

  // Plain text: no buttons, no visible times; it keeps the playing segment in view.
  const plain = page.locator("#plain-transcript");
  check("plain", [await plain.getByRole("button").count(), await plain.locator("[data-transcription-segment] > span").first().evaluate((s) => s.classList.contains("sr-only"))], [0, true]);
  check("noSpeakerSpace", await plain.locator("li").first().evaluate((li) => li.textContent.replace(/\s+/g, " ").trim()), "0:00 Ana Welcome back to the show.");
  const box = page.locator("[data-example=plain]");
  check("topFirst", await box.evaluate((b) => b.scrollTop), 0);
  await page.evaluate(() => (document.getElementById("plain-transcript").dataset.currentTime = "10"));
  // A smooth scroll: wait for the segment to be in view.
  await page.waitForFunction(() => {
    const b = document.querySelector("[data-example=plain]");
    const item = b.querySelector("[aria-current=true]")?.getBoundingClientRect();
    const view = b.getBoundingClientRect();
    return item && item.top >= view.top - 1 && item.bottom <= view.bottom + 1;
  });
  check("autoScroll", await box.evaluate((b) => b.scrollTop > 0), true);

  // Right to left: the bar moves to the right edge.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  check("rtlBar", await inline.locator("[aria-current=true]").evaluate(() => getComputedStyle(document.querySelector("[data-media=episode-audio] [aria-current=true]")).boxShadow.includes("-3px")), true);
};
