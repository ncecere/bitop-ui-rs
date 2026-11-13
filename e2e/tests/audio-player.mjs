export default async ({ page, check }) => {
  const player = page.getByRole("group", { name: "Episode 12" });
  const audio = page.locator("#episode-audio");
  const play = player.locator("[data-player-play]");
  const seek = player.getByRole("slider", { name: "Seek" });
  const volume = player.getByRole("slider", { name: "Volume" });
  const mute = player.getByRole("button", { name: "Mute" });
  const times = () => player.locator("[data-player-current], [data-player-duration]").allTextContents();

  // Every engine here loads WAV; the controls wait for the duration.
  await page.waitForFunction(() => document.getElementById("episode-audio").readyState >= 1);
  await page.waitForFunction(() => !document.querySelector("[data-player-seek]").disabled);
  check("loaded", [await times(), await seek.getAttribute("max"), await seek.getAttribute("aria-valuetext")], [["0:00", "0:20"], "20", "0:00 of 0:20"]);
  check("controls", await player.locator("button, input, select").evaluateAll((all) => all.map((c) => c.getAttribute("aria-label"))), ["Play", "Back 5 seconds", "Forward 5 seconds", "Seek", "Mute", "Volume", "Playback speed"]);
  check("speeds", await player.locator("select option").evaluateAll((all) => all.map((o) => [o.textContent, o.selected])), [["0.5×", false], ["0.75×", false], ["1×", true], ["1.25×", false], ["1.5×", false], ["2×", false]]);

  // Play and pause from the keyboard.
  await audio.evaluate((a) => (a.muted = true));
  await play.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => !document.getElementById("episode-audio").paused);
  await page.waitForFunction(() => document.querySelector("[data-audio-player]").hasAttribute("data-playing"));
  check("playing", [await play.getAttribute("aria-label"), await play.locator("[data-icon]").evaluateAll((all) => all.filter((i) => i.checkVisibility()).map((i) => i.dataset.icon))], ["Pause", ["pause"]]);
  await page.waitForFunction(() => document.getElementById("episode-audio").currentTime > 0.3);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector("[data-player-play]").getAttribute("aria-label") === "Play");
  check("paused", await audio.evaluate((a) => a.paused), true);

  // Seek with the keys: the time and the text follow.
  await seek.focus();
  await page.keyboard.press("End");
  await page.waitForFunction(() => document.getElementById("episode-audio").currentTime > 19);
  await page.keyboard.press("Home");
  await page.waitForFunction(() => document.getElementById("episode-audio").currentTime === 0);
  for (let i = 0; i < 7; i += 1) await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => Math.round(document.getElementById("episode-audio").currentTime) === 7);
  check("seekKeys", [await seek.getAttribute("aria-valuetext"), (await times())[0]], ["0:07 of 0:20", "0:07"]);

  // Skip forward and back.
  await player.getByRole("button", { name: "Forward 5 seconds" }).click();
  await page.waitForFunction(() => Math.round(document.getElementById("episode-audio").currentTime) === 12);
  await player.getByRole("button", { name: "Back 5 seconds" }).click();
  await player.getByRole("button", { name: "Back 5 seconds" }).click();
  await player.getByRole("button", { name: "Back 5 seconds" }).click();
  await page.waitForFunction(() => document.querySelector("[data-player-current]").textContent === "0:00");
  check("skipClamps", await audio.evaluate((a) => a.currentTime), 0);

  // A script that drives the element: the controls follow.
  await audio.evaluate((a) => (a.currentTime = 15));
  await page.waitForFunction(() => document.querySelector("[data-player-seek]").value === "15");
  check("follows", await seek.getAttribute("aria-valuetext"), "0:15 of 0:20");

  // Mute, and the volume slider.
  await audio.evaluate((a) => (a.muted = false));
  await page.waitForFunction(() => document.querySelector("[data-player-mute]").getAttribute("aria-pressed") === "false");
  await mute.focus();
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector("[data-player-mute]").getAttribute("aria-pressed") === "true");
  check("muted", [await mute.getAttribute("aria-pressed"), await mute.locator("[data-icon]").getAttribute("data-icon"), await volume.getAttribute("aria-valuetext")], ["true", "volume-x", "0%"]);
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector("[data-player-mute]").getAttribute("aria-pressed") === "false");
  await volume.focus();
  for (let i = 0; i < 12; i += 1) await page.keyboard.press("ArrowLeft");
  await page.waitForFunction(() => document.querySelector("[data-player-volume]").getAttribute("aria-valuetext") === "40%");
  check("volume", [await audio.evaluate((a) => Math.round(a.volume * 100)), await mute.locator("[data-icon]").getAttribute("data-icon")], [40, "volume-1"]);

  // Speed.
  await player.locator("select").selectOption("1.5");
  await page.waitForFunction(() => document.getElementById("episode-audio").playbackRate === 1.5);
  check("rate", await audio.evaluate((a) => a.playbackRate), 1.5);

  // Compact: no volume slider and no speeds; given as base64.
  const compact = page.getByRole("group", { name: "Generated speech" });
  check("compact", [await compact.getByRole("slider").count(), await compact.locator("select").count(), (await page.locator("#speech-audio").getAttribute("src")).startsWith("data:audio/mpeg;base64,data:") ], [1, 0, false]);
  check("noSkip", await compact.getByRole("button").count(), 2);

  // Nothing to play: play is disabled and the duration is unknown.
  const empty = page.getByRole("group", { name: "Pending speech" });
  check("empty", [await empty.locator("[data-player-play]").isDisabled(), await empty.locator("[data-player-duration]").textContent(), await empty.getByRole("slider", { name: "Seek" }).isDisabled(), await empty.getByRole("slider", { name: "Seek" }).getAttribute("aria-valuetext")], [true, "--:--", true, "0:00 of unknown duration"]);

  // Starting at another speed, with other strings.
  const german = page.getByRole("group", { name: "Folge 3" });
  await page.waitForFunction(() => document.getElementById("german-audio").readyState >= 1);
  check("germanRate", [await page.locator("#german-audio").evaluate((a) => a.playbackRate), await german.locator("select").inputValue()], [1.5, "1.5"]);
  check("germanText", await german.getByRole("slider", { name: "Position" }).getAttribute("aria-valuetext"), "0:00 von 0:03");

  // Right to left: play is at the right.
  await page.evaluate(() => (document.documentElement.dir = "rtl"));
  const right = await player.evaluate((p) => p.querySelector("[data-player-play]").getBoundingClientRect().right > p.querySelector("select").getBoundingClientRect().right);
  check("rtl", right, true);
};
