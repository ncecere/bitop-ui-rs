// Adds Lucide icons to the icon set: the mask in styles/icons.css and the variant in
// components/icon.rs.
//
//   node scripts/add-icons.mjs <lucide-react esm icons dir> pencil share-2 trash-2
//
// The directory is lucide-react's dist/esm/icons, from any checkout with lucide-react
// installed (`npm pack lucide-react` and unpack it if you have none). Icons already in
// the set are skipped.
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const [dir, ...names] = process.argv.slice(2);
if (!dir || !names.length) {
  console.error("usage: node scripts/add-icons.mjs <lucide esm icons dir> <name>...");
  process.exit(1);
}
const root = new URL("../crates/bitop-ui/src/", import.meta.url);
const cssPath = new URL("styles/icons.css", root);
const rsPath = new URL("components/icon.rs", root);
let css = readFileSync(cssPath, "utf8");
let rs = readFileSync(rsPath, "utf8");

const pascal = (name) => name.replace(/(^|-)([a-z0-9])/g, (_, __, c) => c.toUpperCase());
const attr = (value) => String(value).replace(/"/g, "'");

for (const name of names) {
  if (css.includes(`[data-icon="${name}"]`)) {
    console.log(`skip ${name}`);
    continue;
  }
  // Some names are aliases: a module that re-exports another icon.
  let file = join(dir, `${name}.mjs`);
  const alias = readFileSync(file, "utf8").match(/export \{ default \} from '\.\/([^']+)'/);
  if (alias) file = join(dir, alias[1]);
  const { __iconData } = await import(pathToFileURL(file).href);
  const body = __iconData.node
    .map(([tag, attrs]) => {
      const list = Object.entries(attrs)
        .filter(([key]) => key !== "key")
        .map(([key, value]) => `${key}="${attr(value)}"`)
        .join(" ");
      return `<${tag} ${list}/>`;
    })
    .join("");
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">${body}</svg>`;
  if (svg.includes("'")) throw new Error(`${name}: a quote in the drawing`);
  const variable = `  --icon-lucide-${name}: url('data:image/svg+xml,${svg.replace(/#/g, "%23")}');\n`;
  // The variables end with the :root block, the first "}" at the start of a line.
  const end = css.indexOf("\n}\n");
  css = `${css.slice(0, end + 1)}${variable}${css.slice(end + 1)}`;
  css = `${css.trimEnd()}\n\n[data-icon="${name}"] {\n  --icon: var(--icon-lucide-${name});\n}\n`;

  const variant = pascal(name);
  rs = rs.replace(/(pub enum Icon \{\n(?:.*\n)*?)(\})/, `$1    ${variant},\n$2`);
  rs = rs.replace(/(Self::\w+ => "[^"]+",\n)(\s+\}\n\s+\}\n\}\n)/, `$1            Self::${variant} => "${name}",\n$2`);
  console.log(`added ${name} as Icon::${variant}`);
}
writeFileSync(cssPath, css);
writeFileSync(rsPath, rs);
