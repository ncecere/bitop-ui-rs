export default async ({ page, check, setDir }) => {
  const text = (locator) => locator.evaluate((e) => e.textContent.replace(/\s+/g, " ").trim());

  // The card is an article named by the package; the change is spelled out.
  const major = page.getByRole("article", { name: "react", exact: true });
  check("article", await major.count(), 1);
  const badge = major.locator("[data-change]");
  check("badge", [await badge.getAttribute("data-change"), await text(badge), await badge.locator("[data-icon]").getAttribute("data-icon")], ["major", "Major", "arrow-right"]);
  // The versions are read as "from … to …"; the arrow is decorative.
  const version = major.locator("p[dir=ltr]");
  check("versionSpoken", await text(version), "from 18.3.1 to 19.0.0");
  check("versionShown", await version.locator(":scope > span:not(.sr-only), :scope > [data-icon]").evaluateAll((all) => all.map((e) => e.dataset.icon ?? e.textContent)), ["18.3.1", "arrow-right", "19.0.0"]);
  check("description", await text(major.locator("p").nth(2)), "React 19 removes legacy APIs such as string refs and propTypes.");
  // The dependencies are a named list.
  const deps = page.getByRole("list", { name: "Dependencies" });
  check("deps", await deps.locator("li").evaluateAll((all) => all.map((li) => li.textContent.replace(/\s+/g, " ").trim())), ["scheduler ^0.25.0", "loose-envify ^1.1.0"]);
  check("caps", await major.locator("p[aria-hidden=true]").evaluate((p) => getComputedStyle(p).textTransform), "uppercase");

  // Every kind has its own word, icon and colour.
  const kinds = page.locator("[data-example=kinds] [data-change]");
  check("kinds", await kinds.evaluateAll((all) => all.map((b) => [b.dataset.change, b.textContent.trim(), b.querySelector("[data-icon]").dataset.icon])), [
    ["minor", "Minor", "arrow-right"],
    ["patch", "Patch", "arrow-right"],
    ["added", "Added", "plus"],
    ["removed", "Removed", "minus"],
  ]);
  const colours = await page.locator("main [data-change]").evaluateAll((all) => new Set(all.map((b) => getComputedStyle(b).backgroundColor)).size);
  check("colours", colours >= 5, true);
  // One version only: no arrow and no "from".
  const added = page.getByRole("article", { name: "zod", exact: true });
  check("addedVersion", [await text(added.locator("p[dir=ltr]")), await added.locator("p[dir=ltr] [data-icon]").count()], ["3.24.1", 0]);
  check("removedVersion", await text(page.getByRole("article", { name: "moment", exact: true }).locator("p[dir=ltr]")), "2.30.1");

  // A header composed from the parts.
  const custom = page.getByRole("article", { name: "serde", exact: true });
  check("custom", [await custom.locator("[data-icon=file-braces]").count(), await text(custom.locator("[data-change]")), await custom.locator("p").count()], [1, "Feature", 2]);

  check("german", [await text(page.locator("#german-package [data-change]")), await text(page.locator("#german-package p[dir=ltr]")), await page.getByRole("list", { name: "Abhängigkeiten" }).count()], ["Hauptversion", "von 5.4.0 auf 6.0.0", 1]);

  // Right to left: the badge goes to the left; the versions stay left to right.
  await setDir("rtl");
  const rtl = await major.evaluate((card) => {
    const name = card.querySelector("code").getBoundingClientRect();
    const badge = card.querySelector("[data-change]").getBoundingClientRect();
    const current = card.querySelector("[data-version=current]").getBoundingClientRect();
    const next = card.querySelector("[data-version=new]").getBoundingClientRect();
    return [badge.right <= name.left, current.right <= next.left];
  });
  check("rtl", rtl, [true, true]);
};
