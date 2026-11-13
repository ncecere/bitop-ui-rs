// Side-by-side screenshots of the React bitop-ui docs and this gallery.
// node compare.mjs <ref base> <our base> <out dir> [slug ...]
// Each pair is "refSlug:ourSlug" or one slug used for both. For every pair it saves
// <out>/<slug>.png: the reference's examples on the left, ours on the right.
import { mkdirSync, readFileSync } from "node:fs";
import { chromium } from "playwright";

const [refBase, ourBase, out, ...pairs] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1100, height: 900 }, colorScheme: "light" });
const page = await context.newPage();

async function shots(url, selector) {
  await page.goto(url, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);
  const boxes = await page.locator(selector).all();
  const images = [];
  for (const box of boxes.slice(0, 4)) {
    await box.scrollIntoViewIfNeeded();
    images.push((await box.screenshot({ animations: "disabled" })).toString("base64"));
  }
  return images;
}

const composer = await context.newPage();
for (const pair of pairs) {
  const [refSlug, ourSlug = refSlug] = pair.split(":");
  try {
    const left = await shots(`${refBase}/components/${refSlug}`, '[role="group"][aria-labelledby]');
    const right = await shots(`${ourBase}/c/${ourSlug}`, "main section > h2 + div");
    const column = (title, images) =>
      `<div class="col"><h1>${title}</h1>${images.map((b) => `<img src="data:image/png;base64,${b}">`).join("")}</div>`;
    await composer.setContent(
      `<style>body{margin:0;font:14px sans-serif;display:flex;gap:16px;padding:12px;background:#ddd}
       .col{flex:1;display:flex;flex-direction:column;gap:8px;width:640px}img{max-width:640px;border:1px solid #999;background:#fff}
       h1{font-size:16px;margin:0}</style>` +
        column(`React bitop-ui: ${refSlug} (${left.length} shown)`, left) +
        column(`bitop-ui-rs: ${ourSlug} (${right.length} shown)`, right),
    );
    await composer.screenshot({ path: `${out}/${refSlug}.png`, fullPage: true });
    console.log(`ok ${pair}`);
  } catch (error) {
    console.log(`fail ${pair}: ${error.message.split("\n")[0]}`);
  }
}
await browser.close();
