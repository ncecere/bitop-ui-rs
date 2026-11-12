export default async ({ page, check, button }) => {
  const field = page.getByRole("group", { name: "API key" });
  check("group", await field.count(), 1);
  check(
    "described",
    await field.evaluate((g) => document.getElementById(g.getAttribute("aria-describedby")).textContent.trim()),
    "Shown once. Store it somewhere safe.",
  );
  const value = field.locator("code");
  check("value", [await value.textContent(), await value.evaluate((v) => getComputedStyle(v).userSelect || getComputedStyle(v).webkitUserSelect)], ["sk-live-4f9a8c2e71b04d6f", "all"]);
  check("mono", [await value.getAttribute("data-mono"), await page.getByRole("group", { name: "Workspace name" }).locator("code").getAttribute("data-mono")], ["", null]);

  // The clipboard is read by pasting into a text box made for the purpose.
  const pasted = async () => {
    await page.evaluate(() => {
      document.getElementById("paste-target")?.remove();
      const area = document.createElement("textarea");
      area.id = "paste-target";
      area.setAttribute("aria-label", "Paste target");
      document.querySelector("main").append(area);
      area.focus();
    });
    await page.keyboard.press("ControlOrMeta+V");
    const text = await page.locator("#paste-target").inputValue();
    await page.evaluate(() => document.getElementById("paste-target").remove());
    return text;
  };

  const copy = button("Copy API key");
  check("idle", [(await copy.textContent()).trim(), await copy.getAttribute("data-state")], ["Copy", "idle"]);
  await copy.focus();
  await page.keyboard.press("Enter");
  await field.locator("[data-state=copied]").waitFor();
  const copied = button("Copied API key");
  check("copied", [await copied.count(), (await copied.textContent()).trim(), await copied.locator("[data-icon]:visible").getAttribute("data-icon")], [1, "Copied", "check"]);
  check("announced", await field.locator("[role=status]").textContent(), "API key copied to clipboard");
  check("clipboard", await pasted(), "sk-live-4f9a8c2e71b04d6f");

  // A multiline value keeps its line breaks, on the page and in the clipboard.
  const request = page.getByRole("group", { name: "Example request" });
  const lines = await request.locator("pre").evaluate((pre) => [pre.textContent.split("\n").length, getComputedStyle(pre).whiteSpace.startsWith("pre")]);
  check("multiline", lines, [3, true]);
  await button("Copy request").click();
  await request.locator("[data-state=copied]").waitFor();
  check("namedByName", [await button("Copied request").count(), await request.locator("[role=status]").textContent()], [1, "request copied to clipboard"]);
  const text = await pasted();
  check("clipboardMultiline", [text.split("\n").length, text.startsWith("curl https://api.example.com/v1/chat \\"), text.endsWith(`-d '{"message": "Hello"}'`)], [3, true, true]);

  // The button sits inside the box, at its end.
  const box = await field.locator("code").locator("xpath=..").boundingBox();
  const at = await copied.boundingBox();
  check("buttonInside", at.x + at.width <= box.x + box.width && at.y >= box.y && at.y + at.height <= box.y + box.height);
};
