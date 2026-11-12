// Runs the browser checks in tests/, one file per gallery page.
// Usage: node run.mjs http://localhost:3000 [chromium|firefox|webkit] [name...]
//
// A test file is named after its page (tests/button.mjs checks /c/button) and exports
// one async function. The runner opens the page, calls the function, then checks the
// page with axe in light and dark and fails on any console error.
import { readdirSync } from "node:fs";
import * as playwright from "playwright";
import { AxeBuilder } from "@axe-core/playwright";

const [base, engine = "chromium", ...only] = process.argv.slice(2);
const names = readdirSync(new URL("./tests/", import.meta.url))
  .filter((file) => file.endsWith(".mjs"))
  .map((file) => file.slice(0, -4))
  .filter((name) => only.length === 0 || only.includes(name))
  .sort();

const browser = await playwright[engine].launch();
let total = 0;
const failures = [];

for (const name of names) {
  const { default: test } = await import(`./tests/${name}.mjs`);
  const context = await browser.newContext({ viewport: { width: 1000, height: 900 } });
  const page = await context.newPage();
  const errors = [];
  page.on("console", (message) => message.type() === "error" && errors.push(message.text()));
  page.on("pageerror", (error) => errors.push(String(error)));

  // Records one check: the value seen and the value wanted.
  const check = (label, got, want = true) => {
    total += 1;
    if (JSON.stringify(got) !== JSON.stringify(want)) {
      failures.push(`${name}.${label}: expected ${JSON.stringify(want)}, got ${JSON.stringify(got)}`);
    }
  };
  const axe = async () =>
    (await new AxeBuilder({ page }).analyze()).violations.map(
      (v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(", ")}`,
    );
  const helpers = {
    page,
    base,
    check,
    axe,
    button: (label) => page.getByRole("button", { name: label, exact: true }),
    focused: () => page.evaluate(() => document.activeElement?.textContent?.trim()),
    // Waits for an entry transition to end. Headless WebKit starts transitions late,
    // so a fixed wait is not reliable.
    settled: (locator) =>
      locator.evaluate(
        (element) =>
          new Promise((resolve) => {
            const poll = () => {
              const style = getComputedStyle(element);
              const still = ["none", "0px", "0px 0px"].includes(style.translate);
              if (style.opacity === "1" && still) resolve();
              else requestAnimationFrame(poll);
            };
            poll();
          }),
      ),
    // Colours change through transitions on some components; the promise resolves when
    // they have ended, so that axe does not measure a colour half way.
    setTheme: (mode) =>
      page.evaluate((m) => {
        window.bitopUi.setColorMode(m);
        const running = document.getAnimations().filter((a) => a instanceof CSSTransition);
        return Promise.allSettled(running.map((a) => a.finished)).then(() => undefined);
      }, mode),
    setDir: (dir) => page.evaluate((d) => (document.documentElement.dir = d), dir),
    visible: (locator) => locator.allTextContents().then((texts) => texts.map((t) => t.trim())),
  };

  try {
    await page.goto(`${base}/c/${name}`);
    await test(helpers);
    await page.goto(`${base}/c/${name}`);
    await helpers.setTheme("light");
    check("axeLight", await axe(), []);
    await helpers.setTheme("dark");
    check("axeDark", await axe(), []);
    check("errors", errors, []);
  } catch (error) {
    total += 1;
    failures.push(`${name}: ${String(error).split("\n").slice(0, 6).join("\n    ")}`);
  }
  await context.close();
}

const version = browser.version();
await browser.close();
for (const failure of failures) console.error(`FAIL ${failure}`);
console.log(`${engine} ${version}: ${total - failures.length}/${total} checks passed in ${names.length} pages`);
process.exit(failures.length ? 1 : 0);
