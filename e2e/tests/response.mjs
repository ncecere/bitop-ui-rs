// A 1 by 1 PNG, served for every image on a made-up host, so that nothing leaves the
// machine and no failed request is logged.
const PNG = Buffer.from("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==", "base64");

export default async ({ page, check, base }) => {
  const requested = [];
  await page.route(/^https:\/\/[^/]*example\.(com|net|de)\//, (route) => {
    requested.push(new URL(route.request().url()).host);
    return route.fulfill({ contentType: "image/png", body: PNG });
  });
  await page.goto(`${base}/c/response`);
  const answer = page.locator("#answer");

  // Headings are demoted: "#" is an <h3>, so an answer never competes with the page.
  check("heading", await answer.locator("h3").evaluate((h) => [h.textContent, h.dataset.level]), ["Parental leave", "1"]);
  check("noHighHeadings", await answer.locator("h1, h2").count(), 0);
  check("inline", await answer.locator("p").first().evaluate((p) => [p.querySelector("strong").textContent, p.querySelector("em").textContent, p.querySelector("del").textContent]), ["2025 policy", "16 weeks", "unpaid"]);

  // A table: an empty header cell is a plain cell, alignment is kept, <br> breaks a
  // cell, and the scrolling wrapper takes focus.
  const table = answer.locator("table");
  check("tableHead", await table.locator("thead tr > *").evaluateAll((all) => all.map((c) => [c.tagName, c.textContent, c.dataset.align ?? null])), [["TD", "", null], ["TH", "Details", "right"]]);
  check("cellBreak", await table.locator("tbody tr").nth(1).locator("td").nth(1).evaluate((td) => td.querySelectorAll("br").length), 1);
  check("tableWrap", await table.evaluate((t) => t.parentElement.tabIndex), 0);

  // Lists, task lists in words, a quote, inline code and a code block with a copy button.
  check("ordered", await answer.locator("ol > li").count(), 2);
  check("tasks", await answer.locator("li:has(> span[aria-hidden=true]:first-child)").evaluateAll((all) => all.map((li) => li.textContent.trim())), ["Done: Read the policy", "To do: Book the leave"]);
  check("noCheckboxes", await answer.locator("input").count(), 0);
  check("quote", (await answer.locator("blockquote").textContent()).trim(), "Requests need 8 weeks' notice.");
  check("inlineCode", await answer.locator("p code").first().textContent(), "leave --request");
  const code = answer.locator("[data-code-block]");
  check("codeBlock", [await code.getAttribute("data-language"), await code.locator("pre code").textContent(), await code.getByRole("button").getAttribute("aria-label")], ["bash", "hr leave request --weeks 16", "Copy Bash code"]);

  // Links: another site opens in a new tab and says so; mail does not.
  const portal = answer.getByRole("link", { name: "HR portal (opens in a new tab)" });
  check("external", [await portal.getAttribute("href"), await portal.getAttribute("target"), await portal.getAttribute("rel")], ["https://hr.example.com/portal", "_blank", "noreferrer noopener"]);
  check("mail", [await answer.getByRole("link", { name: "hr@example.com" }).getAttribute("href"), await answer.getByRole("link", { name: "hr@example.com" }).getAttribute("target")], ["mailto:hr@example.com", null]);

  // Citations: [1] and [2, 3] are chips; [7] has no source and a[1] is an index, so both stay text.
  check("chips", await answer.locator("[data-citation-trigger]").evaluateAll((all) => all.map((c) => c.getAttribute("aria-label"))), [
    "Source 1: Parental leave policy 2025",
    "Sources 2, 3: Benefits FAQ and 1 more",
    "Source 2: Benefits FAQ",
  ]);
  check("markersAsText", (await answer.locator("p").last().textContent()).includes("unknown marker [7] stays text, and so does a[1]."), true);
  await answer.locator("[data-citation-trigger]").first().click();
  await page.waitForFunction(() => document.querySelector("#answer [data-citation-card]:popover-open"));
  check("citationCard", (await answer.locator("[data-citation-card]:popover-open a").textContent()).trim(), "Parental leave policy 2025 (opens in a new tab)");
  await page.keyboard.press("Escape");

  // Streaming: half-written syntax renders as it will once complete.
  const snapshot = (n) => page.locator("[data-snapshot]").nth(n).locator("[data-response]");
  check("streamingFlag", await snapshot(0).getAttribute("data-streaming"), "");
  check("openBold", [await snapshot(0).locator("strong").last().textContent(), (await snapshot(0).textContent()).includes("**")], ["2025 pol", false]);
  check("partialMarkerHidden", (await snapshot(1).textContent()).trimEnd().endsWith("fully paid leave"), true);
  check("partialLinkIsText", [await snapshot(2).locator("ol a").count(), (await snapshot(2).locator("ol li").last().textContent()).trim()], [0, "Submit the request in the HR portal"]);
  check("openCode", await snapshot(3).locator("p code").last().textContent(), "leave --req");
  check("openFence", await snapshot(4).locator("[data-code-block] pre code").textContent(), "hr leave request --we");

  // Block by block: the replay patches only changed blocks and ends where the whole
  // answer does.
  const replay = page.locator("#replay");
  check("replayEmpty", await replay.locator("[data-block]").count(), 0);
  await page.getByRole("button", { name: "Replay the answer" }).click();
  await page.waitForFunction(() => document.getElementById("replay").hasAttribute("data-replayed"));
  const text = (locator) => locator.evaluate((r) => r.innerText.replace(/\s+/g, " ").trim());
  check("replayMatches", await text(replay), await text(answer));
  check("replayBlocks", await replay.evaluate((r) => [...r.children].map((c) => Number(c.dataset.block))), [...Array(await answer.evaluate((a) => a.children.length)).keys()]);
  check("replayDone", await replay.getAttribute("data-streaming"), null);

  // Untrusted output: no script, no unsafe link, no image from another host until asked.
  const untrusted = page.locator("[data-example=untrusted] [data-response]");
  check("noScript", await untrusted.locator("script").count(), 0);
  check("scriptAsText", (await untrusted.locator("p").first().textContent()).trim(), '<script>alert("injected")</script>');
  check("rawHtmlAsText", await untrusted.locator("b, img[onerror]").count(), 0);
  check("onlySafeLinks", await untrusted.locator("a[href]").evaluateAll((all) => all.map((a) => a.getAttribute("href"))), ["https://example.com/docs"]);
  check("noJavascriptHref", await page.locator("[href^='javascript:' i], [src^='javascript:' i], [href^='data:' i]").count(), 0);
  check("svgRefused", await untrusted.locator("img[src^='data:image/svg']").count(), 0);
  check("dataPngShown", await untrusted.locator("img[alt='A dot']").evaluate((i) => i.src.startsWith("data:image/png")), true);
  check("notFetched", [await untrusted.locator("img[src*=tracker]").count(), requested.includes("tracker.example.net")], [0, false]);
  const load = untrusted.getByRole("button", { name: /^Load image from tracker\.example\.net\s*:\s*A tracking pixel$/ });
  await load.focus();
  await page.keyboard.press("Enter");
  const loaded = untrusted.locator("img[src*=tracker]");
  await loaded.waitFor();
  check("loadedOnRequest", [await loaded.getAttribute("alt"), await page.evaluate(() => document.activeElement.firstElementChild?.getAttribute("alt"))], ["A tracking pixel", "A tracking pixel"]);
  await page.waitForFunction(() => document.querySelector("[data-example=untrusted] img[src*=tracker]").complete);
  check("fetchedAfterPress", requested.includes("tracker.example.net"), true);

  // Shown at once, never shown, and HTML dropped.
  check("showWaitsClosed", requested.includes("images.example.com"), false);
  await page.getByText("An image from another host, shown at once").click();
  await page.waitForFunction(() => document.querySelector("[data-mode=show] img").complete);
  check("show", [await page.locator("[data-mode=show] img").getAttribute("src"), requested.includes("images.example.com")], ["https://images.example.com/dot.png", true]);
  check("alt", (await page.locator("[data-mode=alt]").textContent()).trim(), "[Image: A chart] and [Image] are text.");
  check("skip", (await page.locator("[data-mode=skip]").textContent()).trim(), "Raw HTML is dropped.");
  check("oneLevel", await page.locator("[data-example=headings] h3").allTextContents(), ["Policy", "Scope"]);

  // Other strings.
  const german = page.locator("[data-example=strings] [data-response]");
  check("strings", [
    await german.locator("li").allTextContents(),
    await german.getByRole("link").textContent(),
    (await german.getByRole("button", { name: /Bild von bilder.example.de laden/ }).count()),
    await german.locator("[data-code-block]").getByRole("button").getAttribute("aria-label"),
  ], [["Erledigt: Antrag gestellt", "Offen: Urlaub gebucht"], "das Portal (öffnet in einem neuen Tab)", 1, "Quelltext kopieren"]);
};
