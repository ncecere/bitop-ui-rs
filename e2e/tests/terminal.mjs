export default async ({ page, check, button, setDir }) => {
  const target = page.getByLabel("Paste target");
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };
  const build = page.locator("#build");
  const span = (text) => build.locator("#build-output > span").filter({ hasText: text }).first();
  const attrs = (locator) => locator.evaluate((s) => Object.fromEntries([...s.attributes].filter((a) => a.name.startsWith("data-")).map((a) => [a.name.slice(5), a.value])));

  // The output is a region named by the title, left to right, in the tab order.
  const region = page.getByRole("region", { name: "cargo build" });
  check("region", [await region.getAttribute("tabindex"), await region.locator("pre").getAttribute("dir")], ["0", "ltr"]);

  // SGR codes become styled spans; the codes themselves are gone.
  const text = await page.locator("#build-output").textContent();
  check("noEscapes", text.includes("\u001b"), false);
  check("compiling", await attrs(span("Compiling")), { fg: "green", bold: "" });
  check("warning", await attrs(span("warning")), { fg: "yellow", bold: "" });
  check("error", await attrs(span("error[E0308]")), { fg: "red", bold: "" });
  check("arrow", await attrs(span("-->")), { fg: "blue" });
  check("dim", await attrs(span("note: expected")), { dim: "" });
  check("styles", [await attrs(span("italic")), await attrs(span("underlined")), await attrs(span("inverse")), await attrs(span("magenta")), await attrs(span("cyan")), await attrs(span("gray")), await attrs(span("red background"))], [{ italic: "" }, { underline: "" }, { inverse: "" }, { fg: "magenta" }, { fg: "cyan" }, { fg: "gray" }, { bg: "red" }]);
  // Colours are theme tokens: red is the danger text colour.
  const colours = await build.evaluate((root) => {
    const probe = document.createElement("span");
    probe.style.color = "var(--color-danger-text)";
    root.append(probe);
    const want = getComputedStyle(probe).color;
    probe.remove();
    const red = [...root.querySelectorAll("#build-output > span")].find((s) => s.dataset.fg === "red");
    return [getComputedStyle(red).color === want, getComputedStyle(red).fontWeight];
  });
  check("tokens", colours, [true, "700"]);
  // \r rewrites the line; other sequences and truecolour are dropped.
  check("carriageReturn", [text.includes("Downloading 100%"), text.includes("10%")], [true, false]);
  check("dropped", [text.includes("window title"), await attrs(span("truecolour is plain"))], [false, {}]);

  // Copy copies the text without escape codes.
  await build.getByRole("button", { name: "Copy output" }).click();
  await build.locator("[data-copy][data-state=copied]").waitFor();
  const copied = await pasted();
  check("copy", [copied.startsWith("   Compiling bitop-ui v0.1.0\nwarning: unused variable"), copied.includes("\u001b"), copied.includes("Downloading 100%")], [true, false, true]);

  // Clear empties the output, shows the placeholder, and can be cancelled.
  await page.evaluate(() => document.getElementById("build").addEventListener("bitop:terminal-clear", (e) => e.preventDefault(), { once: true }));
  const clear = build.getByRole("button", { name: "Clear output" });
  await clear.click();
  check("cancelled", (await page.locator("#build-output").textContent()).length > 0, true);
  await clear.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.getElementById("build-output").childNodes.length === 0);
  check("cleared", [await build.getByText("No output").isVisible(), await clear.locator("[data-icon]").getAttribute("data-icon")], [true, "eraser"]);

  // Streaming: the status, aria-busy and the cursor while it runs; it follows to the bottom.
  const stream = page.locator("#stream");
  const content = stream.locator("[data-terminal-content]");
  const status = stream.locator("[data-terminal-status]");
  check("idle", [await status.isVisible(), await content.getAttribute("aria-busy"), await stream.getByText("No output").isVisible()], [false, null, true]);
  await button("Run the tests").click();
  await page.waitForFunction(() => document.getElementById("stream").hasAttribute("data-streaming"));
  await page.waitForFunction(() => document.querySelector("#stream [data-terminal-content]").getAttribute("aria-busy") === "true");
  check("running", [await status.isVisible(), (await status.textContent()).trim(), await stream.locator("pre > span[aria-hidden=true]").isVisible(), await stream.getByText("No output").isVisible()], [true, "Running", true, false]);
  await page.waitForFunction(() => document.getElementById("stream").hasAttribute("data-streamed"));
  await page.waitForFunction(() => !document.querySelector("#stream [data-terminal-content]").hasAttribute("aria-busy"));
  const bottom = () => content.evaluate((c) => c.scrollHeight - c.scrollTop - c.clientHeight);
  check("followed", [await content.evaluate((c) => c.scrollHeight > c.clientHeight + 20), (await bottom()) <= 1, await status.isVisible()], [true, true, false]);
  check("failedLine", await stream.locator("#stream-output > span[data-fg=red]").first().textContent(), "FAILED");

  // Scrolled up, it stays put as output arrives; at the bottom again, it follows.
  const append = (line) =>
    page.evaluate((l) => {
      const s = document.createElement("span");
      s.textContent = l;
      document.getElementById("stream-output").append(s);
    }, line);
  await content.evaluate((c) => c.scrollTo({ top: 0, behavior: "instant" }));
  await page.waitForFunction(() => document.querySelector("#stream [data-terminal-content]").scrollTop === 0);
  // The scroll event that says the reader left the bottom comes a frame later.
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  await append("more output\n");
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  check("staysUp", await content.evaluate((c) => c.scrollTop), 0);
  await content.focus();
  await page.keyboard.press("End");
  await page.waitForFunction(() => {
    const c = document.querySelector("#stream [data-terminal-content]");
    return c.scrollHeight - c.scrollTop - c.clientHeight < 2;
  });
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  await append("and more\n".repeat(5));
  await page.waitForFunction(() => {
    const c = document.querySelector("#stream [data-terminal-content]");
    return c.scrollHeight - c.scrollTop - c.clientHeight <= 1;
  });
  check("followsAgain", (await bottom()) <= 1, true);

  // Running before any output: no placeholder; the status says so.
  const waiting = page.locator("#waiting");
  check("waiting", [await waiting.locator("[data-terminal-status]").isVisible(), await waiting.getByText("No output").isVisible(), await waiting.locator("[data-terminal-content]").getAttribute("aria-busy")], [true, false, "true"]);
  check("empty", [await page.locator("#empty").getByText("No output").isVisible(), await page.locator("#empty").getByRole("button", { name: "Clear output" }).count()], [true, 0]);

  // Composed: another icon, no clear button.
  const deploy = page.locator("#deploy");
  check("custom", [await deploy.locator("p [data-icon]").getAttribute("data-icon"), await deploy.locator("[data-terminal-clear]").count(), await page.getByRole("region", { name: "Deploy log" }).count()], ["globe", 0, 1]);

  const german = page.locator("#german");
  check("german", [await german.getByRole("button", { name: "Ausgabe kopieren" }).count(), await german.getByRole("button", { name: "Ausgabe leeren" }).count(), await german.getByText("Keine Ausgabe").isVisible()], [1, 1, true]);

  // Right to left: the actions go to the left; the output stays left to right.
  await setDir("rtl");
  const rtl = await page.locator("#deploy").evaluate((root) => {
    const title = root.querySelector("p").getBoundingClientRect();
    const copy = root.querySelector("[data-copy]").getBoundingClientRect();
    const pre = root.querySelector("pre");
    const first = pre.querySelector("span").getBoundingClientRect();
    return [copy.right <= title.left, getComputedStyle(pre).direction, first.left - pre.getBoundingClientRect().left < 40];
  });
  check("rtl", rtl, [true, "ltr", true]);
};
