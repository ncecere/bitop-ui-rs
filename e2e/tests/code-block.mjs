export default async ({ page, check, button }) => {
  const block = (name) => page.locator(`[data-example=${name}] [data-code-block]`).first();
  const header = (name) => block(name).locator(":scope > div").first().locator("span").first().textContent();
  const target = page.getByLabel("Paste target");
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };

  // The language's name in the header, the id on the root and on the <code>.
  const rust = block("language");
  check("label", await header("language"), "Rust");
  check("language", [await rust.getAttribute("data-language"), await rust.locator("code").getAttribute("class")], ["rs", "language-rs"]);
  check("finalNewlineDropped", await rust.locator("code").evaluate((c) => c.textContent.endsWith("}")), true);
  check("noGutter", await rust.locator("[aria-hidden=true]:not([data-icon])").count(), 0);

  // A file name replaces the language; the gutter numbers each line and is hidden.
  const numbered = block("numbers");
  check("filename", await header("numbers"), "src/main.rs");
  const gutter = numbered.locator("span[aria-hidden=true]:not([data-icon])");
  check("gutter", (await gutter.textContent()).trim().split("\n"), ["1", "2", "3", "4", "5", "6"]);
  const lines = await numbered.evaluate((b) => {
    const g = b.querySelector("span[aria-hidden=true]:not([data-icon])").getBoundingClientRect();
    const c = b.querySelector("pre").getBoundingClientRect();
    return [Math.round(g.height), Math.round(c.height)];
  });
  check("gutterLinesUp", Math.abs(lines[0] - lines[1]) <= 1, true);

  // The <pre> is a tab stop, and the arrow keys scroll a long line.
  const scroller = block("scroll").locator("pre");
  check("overflows", await scroller.evaluate((p) => p.scrollWidth > p.clientWidth), true);
  check("tabStop", await scroller.getAttribute("tabindex"), "0");
  await scroller.focus();
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(() => document.querySelector("[data-example=scroll] pre").scrollLeft > 0);
  check("keyboardScrolls", true);
  // Wrapping: no sideways scroll, and no line numbers.
  const wrapped = block("wrap");
  check("wraps", [await wrapped.getAttribute("data-wrap"), await wrapped.locator("pre").evaluate((p) => p.scrollWidth <= p.clientWidth + 1)], ["", true]);
  check("wrapHasNoGutter", await wrapped.locator("span[aria-hidden=true]:not([data-icon])").count(), 0);

  // A maximum height: the body stops at 12rem and scrolls.
  const tall = block("tall").locator(":scope > div").last();
  const size = await tall.evaluate((b) => [Math.round(b.getBoundingClientRect().height), b.scrollHeight > b.clientHeight]);
  check("maxHeight", [Math.abs(size[0] - 192) <= 1, size[1]], [true, true]);

  // Without a header the copy button floats; without a language it copies "code".
  const bare = block("bare");
  check("noHeader", await bare.locator("pre").evaluate((p) => p.parentElement.parentElement.children.length), 1);
  check("floatingCopy", await bare.getByRole("button").getAttribute("aria-label"), "Copy code");
  check("plainLabel", await header("plain"), "Code");

  // Copying: the state changes, the result is announced, and the raw code is copied.
  const copy = rust.getByRole("button", { name: "Copy Rust code" });
  await copy.click();
  await rust.locator("[data-state=copied]").waitFor();
  check("copied", [await button("Copied Rust code").count(), await rust.locator("[role=status]").textContent()], [1, "Copied Rust code to clipboard"]);
  check("clipboard", await pasted(), 'use bitop_ui_ai::components::code_block::code_block;\n\nfn main() {\n    let greeting = "Hello, world";\n    println!("{greeting}");\n}\n');
  // From the keyboard too.
  await bare.getByRole("button").focus();
  await page.keyboard.press("Enter");
  await bare.locator("[data-state=copied]").waitFor();
  check("keyboardCopy", await pasted(), "npm install bitop-ui");

  // Header actions come before the copy button, which can be left out.
  const actions = block("actions");
  check("actions", await actions.getByRole("button").evaluateAll((all) => all.map((b) => b.textContent.trim())), ["Run"]);

  // Every string can be replaced.
  const strings = page.locator("[data-strings]");
  check("strings", [await header("strings"), await strings.getByRole("button").getAttribute("aria-label")], ["Quelltext", "Quelltext kopieren"]);
};
