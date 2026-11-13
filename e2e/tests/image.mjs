export default async ({ page, check }) => {
  const frame = (example, n = 0) => page.locator(`[data-example=${example}] [data-image-frame]`).nth(n);
  const settledState = (locator) => locator.evaluate((f) => new Promise((resolve) => {
    const poll = () => (f.dataset.state === "loading" ? requestAnimationFrame(poll) : resolve(f.dataset.state));
    poll();
  }));

  // A URL: loaded, no longer busy, with its alt text and the reserved shape.
  const url = frame("url");
  check("loaded", [await settledState(url), await url.getAttribute("aria-busy")], ["loaded", null]);
  check("alt", await url.getByRole("img").getAttribute("alt"), "A lake below a mountain at dusk");
  check("aspect", await url.evaluate((f) => { const b = f.getBoundingClientRect(); return Math.abs(b.width / b.height - 4 / 3) < 0.02; }), true);
  check("framed", await url.getAttribute("data-framed"), "");
  check("fallbackHidden", await url.locator("span").first().isVisible(), false);

  // Bytes become a data URL; base64 too, here square and without a frame.
  const bytes = frame("bytes");
  check("bytes", [await settledState(bytes), (await bytes.locator("img").getAttribute("src")).startsWith("data:image/png;base64,iVBORw0KGgo"), await bytes.locator("img").evaluate((i) => i.naturalWidth)], ["loaded", true, 4]);
  const b64 = frame("bytes", 1);
  check("base64", [await settledState(b64), await b64.getAttribute("data-framed"), await b64.evaluate((f) => { const b = f.getBoundingClientRect(); return Math.round(b.width) === Math.round(b.height); })], ["loaded", null, true]);

  // A failed image: the fallback names the problem and repeats the alt text.
  const failed = frame("error");
  check("error", [await settledState(failed), await failed.locator("img").isVisible()], ["error", false]);
  check("fallback", (await failed.locator(":scope > span").innerText()).split("\n").map((t) => t.trim()).filter(Boolean), ["Image unavailable", "A chart of monthly sign-ups"]);
  check("fallbackIcon", await failed.locator("[data-icon=image-off]").getAttribute("aria-hidden"), "true");
  // No image at all is an error from the start; without alt text only the label shows.
  const missing = page.locator("[data-missing]");
  check("missing", [await missing.getAttribute("data-state"), await missing.locator("img").count(), (await missing.innerText()).trim()], ["error", 0, "Image unavailable"]);
  check("strings", (await frame("error", 2).innerText()).split("\n").map((t) => t.trim()).filter(Boolean), ["Bild nicht verfügbar", "Ein Diagramm"]);

  // Loading: a shimmering placeholder and a hidden image, until the image arrives.
  let release;
  const arrived = new Promise((resolve) => (release = resolve));
  await page.route("**/slow-image.png", async (route) => {
    await arrived;
    await route.fulfill({ contentType: "image/png", body: await frame("bytes").locator("img").evaluate((i) => i.src).then((src) => Buffer.from(src.split(",")[1], "base64")) });
  });
  await page.evaluate(() => {
    const copy = document.querySelector("[data-example=url] [data-image-frame]").cloneNode(true);
    copy.dataset.state = "loading";
    copy.setAttribute("aria-busy", "true");
    copy.id = "slow";
    // A new <img>: a copy would start loading the original picture at once.
    const old = copy.querySelector("img");
    const img = document.createElement("img");
    for (const { name, value } of old.attributes) if (name !== "src") img.setAttribute(name, value);
    img.src = "/slow-image.png";
    old.replaceWith(img);
    document.querySelector("[data-example=url]").append(copy);
  });
  const slow = page.locator("#slow");
  await page.waitForTimeout(100);
  check("loading", [await slow.getAttribute("data-state"), await slow.getAttribute("aria-busy"), await slow.locator("img").evaluate((i) => getComputedStyle(i).opacity)], ["loading", "true", "0"]);
  check("shimmer", await slow.evaluate((f) => getComputedStyle(f).backgroundImage.includes("gradient")), true);
  release();
  check("arrives", await settledState(slow), "loaded");
  check("notBusy", await slow.getAttribute("aria-busy"), null);
};
