export default async ({ page, check, base }) => {
  // A labelled group of real buttons named by their text.
  const wrap = page.getByRole("group", { name: "Suggestions" }).first();
  check("group", [await wrap.getAttribute("data-layout"), await wrap.getAttribute("data-align")], ["wrap", "center"]);
  check("names", (await wrap.getByRole("button").allTextContents()).slice(0, 3).map((t) => t.trim()), [
    "What is our parental leave policy?",
    "How do I request time off?",
    "Summarise the travel policy",
  ]);
  check("notSubmit", await wrap.getByRole("button").first().getAttribute("type"), "button");

  // A long suggestion wraps inside its chip, never wider than the group.
  const long = wrap.getByRole("button").last();
  const fit = await long.evaluate((b) => {
    const chip = b.getBoundingClientRect();
    const group = b.parentElement.getBoundingClientRect();
    return [Math.round(chip.width) <= Math.round(group.width) + 1, chip.height > 40];
  });
  check("longWraps", fit, [true, true]);

  // The start layout, with icons, a shorter visible text and a disabled chip.
  const ideas = page.getByRole("group", { name: "Ideas" });
  check("start", await ideas.getAttribute("data-align"), "start");
  check("icon", await ideas.getByRole("button").first().locator("[data-icon]").getAttribute("aria-hidden"), "true");
  check("shortText", await ideas.getByRole("button", { name: "Expense rules" }).getAttribute("data-suggestion"), "Explain the expense report rules for contractors");
  check("disabled", await ideas.getByRole("button", { name: "Not available yet" }).isDisabled(), true);

  // The scroll layout keeps one row and scrolls it.
  const row = page.locator("[data-example=scroll] [role=group]");
  check("oneRow", await row.evaluate((g) => {
    const tops = [...g.children].map((c) => Math.round(c.getBoundingClientRect().top));
    return new Set(tops).size === 1 && g.scrollWidth > g.clientWidth;
  }), true);

  // Every press is announced to the page.
  await page.evaluate(() => {
    window.picked = [];
    document.addEventListener("bitop:suggestion", (event) => window.picked.push(event.detail.suggestion));
  });
  await wrap.getByRole("button").nth(1).click();
  check("event", await page.evaluate(() => window.picked), ["How do I request time off?"]);

  // With a target: the text goes into the box, which takes focus, from the keyboard too.
  const box = page.getByRole("textbox", { name: "Prompt" });
  const fill = page.getByRole("button", { name: "Translate this into French" });
  await fill.focus();
  await page.keyboard.press("Enter");
  check("filled", [await box.inputValue(), await box.evaluate((b) => b === document.activeElement)], ["Translate this into French", true]);
  await page.keyboard.type("!");
  check("caretAtEnd", await box.inputValue(), "Translate this into French!");

  // With send: the form is submitted with the text.
  await Promise.all([page.waitForURL("**/c/suggestion?prompt=*"), page.getByRole("button", { name: "Make it shorter" }).click()]);
  check("sent", new URL(page.url()).searchParams.get("prompt"), "Make it shorter");

  // With a name: a plain submit button, which works without the script.
  await page.goto(`${base}/c/suggestion`);
  const joke = page.getByRole("button", { name: "Tell me a joke" });
  check("submit", [await joke.getAttribute("type"), await joke.getAttribute("name"), await joke.getAttribute("value")], ["submit", "message", "Tell me a joke"]);
  await Promise.all([page.waitForURL("**/c/suggestion?message=*"), joke.click()]);
  check("submitted", new URL(page.url()).searchParams.get("message"), "Tell me a joke");
};
