const PROMPT = "Explain this stack trace: TypeError: cannot read properties of undefined (reading 'map') at render (list.tsx:12)";

export default async ({ page, check }) => {
  const items = (id) => page.locator(`#${id} [role=menuitem]`);
  const links = (id) => items(id).evaluateAll((all) => all.map((a) => [a.textContent.replace(/\s+/g, " ").trim(), a.getAttribute("href"), a.target, a.rel]));
  const form = (text) => new URLSearchParams({ q: text }).toString().slice(2);

  // The trigger opens a menu; the keyboard moves through the links.
  const trigger = page.locator("[data-example=default]").getByRole("button", { name: "Open in chat" });
  check("trigger", [await trigger.getAttribute("aria-haspopup"), await trigger.getAttribute("popovertarget")], ["menu", "open-default"]);
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.activeElement?.closest("#open-default"));
  check("focusFirst", (await page.evaluate(() => document.activeElement.textContent)).replace(/\s+/g, " ").trim(), "Open in ChatGPT (opens in a new tab)");
  check("defaults", await links("open-default"), [
    ["Open in ChatGPT (opens in a new tab)", `https://chatgpt.com/?hints=search&prompt=${form(PROMPT)}`, "_blank", "noopener noreferrer"],
    ["Open in Claude (opens in a new tab)", `https://claude.ai/new?q=${form(PROMPT)}`, "_blank", "noopener noreferrer"],
    ["Open in v0 (opens in a new tab)", `https://v0.app/?q=${form(PROMPT)}`, "_blank", "noopener noreferrer"],
  ]);
  check("names", await page.locator("#open-default").getByRole("menuitem", { name: "Open in Claude (opens in a new tab)" }).count(), 1);
  check("icons", await items("open-default").first().locator("[data-icon]").evaluateAll((all) => all.map((i) => [i.dataset.icon, i.getAttribute("aria-hidden")])), [["message-square", "true"], ["external-link", "true"]]);
  await page.keyboard.press("ArrowDown");
  check("arrowDown", (await page.evaluate(() => document.activeElement.textContent)).includes("Open in Claude"), true);
  await page.keyboard.press("Escape");
  await page.waitForFunction(() => !document.getElementById("open-default").matches(":popover-open"));
  check("escapeReturns", await page.evaluate(() => document.activeElement.getAttribute("popovertarget")), "open-default");

  // Every built-in provider, under a heading.
  check("group", await page.locator("#open-every [role=group][aria-label='Open this prompt in']").count(), 1);
  check("every", (await links("open-every")).map(([, href]) => new URL(href).host), ["chatgpt.com", "claude.ai", "v0.app", "cursor.com", "t3.chat", "scira.ai"]);
  check("cursorParam", new URL((await links("open-every"))[3][1]).searchParams.get("text"), PROMPT);

  // A provider of one's own (a URL component) and an extra item with its own prompt.
  check("custom", (await links("open-custom")).map(([text, href]) => [text, href]), [
    ["Open in Claude (opens in a new tab)", "https://claude.ai/new?q=Summarise+chapter+3"],
    ["Open in Campus AI (opens in a new tab)", "https://ai.example.edu/new?q=Summarise%20chapter%203"],
    ["Open in T3 Chat (opens in a new tab)", "https://t3.chat/new?q=Summarise+chapter+3+in+French"],
  ]);

  // Other strings.
  check("strings", [
    (await page.locator("[data-example=strings] [popovertarget]").textContent()).trim(),
    (await links("open-german")).map(([text, href]) => [text, href]),
  ], ["Im Chat öffnen", [
    ["In ChatGPT öffnen (öffnet in einem neuen Tab)", "https://chatgpt.com/?hints=search&prompt=Erkl%C3%A4re+diesen+Fehler"],
    ["In Claude öffnen (öffnet in einem neuen Tab)", "https://claude.ai/new?q=Erkl%C3%A4re+diesen+Fehler"],
  ]]);
};
