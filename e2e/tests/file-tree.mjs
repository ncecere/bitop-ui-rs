export default async ({ page, check, setDir }) => {
  const tree = page.getByRole("tree", { name: "Project files" });
  const item = (path) => page.locator(`#project-tree [data-path="${path}"]`);
  const focused = () => page.evaluate(() => document.activeElement?.dataset.path ?? document.activeElement?.tagName);
  const press = async (key, path) => {
    await page.keyboard.press(key);
    if (path) await page.waitForFunction((p) => document.activeElement?.dataset.path === p, path);
  };
  const stops = () => page.locator("#project-tree [role=treeitem][tabindex='0']").evaluateAll((all) => all.map((i) => i.dataset.path));
  const event = () => page.locator("[data-tree-event]").textContent();

  // The tree's structure: items, groups, levels, states and names.
  check("roles", [await tree.count(), await page.locator("#project-tree [role=group]").count(), await page.locator("#project-tree [role=treeitem]").count()], [1, 4, 14]);
  check("levels", await page.locator("#project-tree [role=treeitem]").evaluateAll((all) => all.map((i) => i.getAttribute("aria-level")).join("")), "12332221212111");
  check("expanded", await page.locator("#project-tree [role=treeitem][data-kind=folder]").evaluateAll((all) => all.map((i) => i.getAttribute("aria-expanded"))), ["true", "false", "false", "false"]);
  check("selected", [await item("src/main.rs").getAttribute("aria-selected"), await page.locator("#project-tree [aria-selected=true]").count()], ["true", 1]);
  // An item is named by its row, not by the items in its group.
  check("names", [await page.getByRole("treeitem", { name: "main.rs M", exact: true }).count(), await page.getByRole("treeitem", { name: "src", exact: true }).count(), await page.getByRole("treeitem", { name: "Cargo.toml" }).count()], [1, 1, 1]);
  check("icons", await page.locator("#project-tree [data-tree-row] > [aria-hidden=true][data-kind] [data-icon]").evaluateAll((all) => all.map((i) => i.dataset.icon)), ["folder-open", "folder", "file-code", "file-code", "file-code", "file-code", "file-image", "folder", "file-code", "folder", "file-text", "file-braces", "file-text", "file"]);
  check("closedHidden", await page.locator("#project-tree [data-path='src/components'] > [role=group]").isHidden(), true);

  // One tab stop: the selected item.
  check("tabStop", await stops(), ["src/main.rs"]);
  await page.locator("main h1").click();
  await page.keyboard.press("Tab");
  check("tabIn", await focused(), "src/main.rs");

  // Up and Down move among visible items; Home and End go to the ends.
  await press("ArrowUp", "src/components");
  await press("ArrowDown", "src/main.rs");
  await press("ArrowDown", "src/lib.rs");
  check("rovingStop", await stops(), ["src/lib.rs"]);
  await press("Home", "src");
  await press("End", "LICENSE");
  await press("ArrowUp", "README.md");

  // Right opens a closed folder, then moves into it; Left closes it, then moves to the parent.
  await item("tests").focus();
  await press("ArrowRight");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="tests"]').getAttribute("aria-expanded") === "true");
  check("opened", [await focused(), await item("tests").locator("> [role=group]").isVisible(), await item("tests").locator("[data-folder-icon] [data-icon]").getAttribute("data-icon")], ["tests", true, "folder-open"]);
  check("toggleEvent", await event(), "Opened tests");
  await press("ArrowRight", "tests/smoke.rs");
  await press("ArrowLeft", "tests");
  await press("ArrowLeft");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="tests"]').getAttribute("aria-expanded") === "false");
  // The group hides when the folder's closing has run, which can be a frame later.
  await item("tests").locator("> [role=group]").waitFor({ state: "hidden" });
  check("closed", [await focused(), await item("tests").locator("> [role=group]").isHidden()], ["tests", true]);
  // Left on an item at the top level stays; on a child it goes to the folder.
  await item("src/lib.rs").focus();
  await press("ArrowLeft", "src");

  // Enter and Space select; a folder also opens or closes.
  await item("Cargo.toml").focus();
  await press("Enter");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="Cargo.toml"]').getAttribute("aria-selected") === "true");
  check("select", [await page.locator("#project-tree [aria-selected=true]").evaluateAll((all) => all.map((i) => i.dataset.path)), await event()], [["Cargo.toml"], "Selected Cargo.toml (file)"]);
  await item("docs").focus();
  await press(" ");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="docs"]').getAttribute("aria-expanded") === "true");
  check("spaceFolder", [await item("docs").getAttribute("aria-selected"), await item("docs").getAttribute("data-selected")], ["true", ""]);

  // * opens every folder beside the current one.
  await item("tests").focus();
  await press("*");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="tests"]').getAttribute("aria-expanded") === "true");
  check("star", await page.locator("#project-tree > [data-kind=folder]").evaluateAll((all) => all.map((i) => i.getAttribute("aria-expanded"))), ["true", "true", "true"]);

  // Typing moves to the next item whose name starts with the letters.
  await item("src").focus();
  await press("l", "src/lib.rs");
  await press("l", "src/logo.svg");
  await press("l", "LICENSE");
  await press("l", "src/lib.rs");
  await page.waitForTimeout(600);
  await page.keyboard.type("re");
  await page.waitForFunction(() => document.activeElement?.dataset.path === "README.md");
  check("typeahead", await focused(), "README.md");

  // Closing a folder that holds the tab stop moves it to the folder.
  await item("src/main.rs").focus();
  await item("src").locator("> [data-tree-row]").click();
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="src"]').getAttribute("aria-expanded") === "false");
  check("clickFolder", [await stops(), await focused(), await item("src").getAttribute("aria-selected")], [["src"], "src", "true"]);
  // A click on a file selects it.
  await item("README.md").locator("> [data-tree-row]").click();
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="README.md"]').getAttribute("aria-selected") === "true");
  check("clickFile", [await stops(), await event()], [["README.md"], "Selected README.md (file)"]);
  const selectedRow = await item("README.md").locator("> [data-tree-row]").evaluate((row) => getComputedStyle(row, "::before").content);
  check("selectionBar", selectedRow, '""');

  // Given levels and icons; with nothing selected, the first item is the tab stop.
  const uploads = page.locator("#uploads-tree");
  check("given", [await uploads.locator("[role=treeitem]").evaluateAll((all) => all.map((i) => i.getAttribute("aria-level"))), await uploads.locator("[role=treeitem][tabindex='0']").getAttribute("data-path"), await uploads.locator("[data-path=images] [data-tree-row] [data-icon]").nth(1).getAttribute("data-icon")], [["1", "2", "2", "1"], "images", "image"]);
  check("meta", await page.getByRole("treeitem", { name: "images 2 files", exact: true }).count(), 1);

  check("german", [await page.getByRole("tree", { name: "Projektdateien" }).count(), await page.getByRole("treeitem", { name: "quelle geändert", exact: true }).count()], [1, 1]);

  // Right to left: the arrows swap, the chevron points left, the guide line is on the right.
  await setDir("rtl");
  await item("tests").focus();
  await press("ArrowRight");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="tests"]').getAttribute("aria-expanded") === "false");
  await press("ArrowLeft");
  await page.waitForFunction(() => document.querySelector('#project-tree [data-path="tests"]').getAttribute("aria-expanded") === "true");
  await press("ArrowLeft", "tests/smoke.rs");
  const rtl = await item("tests").evaluate((folder) => {
    const group = folder.querySelector("[role=group]");
    const row = folder.querySelector("[data-tree-row]").getBoundingClientRect();
    const name = folder.querySelector("[data-tree-row] [dir=ltr]").getBoundingClientRect();
    return [getComputedStyle(group).borderRightWidth, name.right <= row.right && name.left > row.left + 20];
  });
  check("rtl", rtl, ["1px", true]);
};
