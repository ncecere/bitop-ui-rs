export default async ({ page, check, button, setDir }) => {
  const target = page.getByLabel("Paste target");
  const pasted = async () => {
    await target.fill("");
    await target.focus();
    await page.keyboard.press("ControlOrMeta+V");
    return target.inputValue();
  };
  const prod = page.getByRole("group", { name: "Production", exact: true });
  const row = (name) => prod.locator("li[data-env-var]").filter({ has: page.locator(`code:text-is("${name}")`) });
  const spoken = (locator) => locator.evaluate((e) => [...e.querySelectorAll(":scope > span:not([hidden])")].map((s) => s.textContent.replace(/\s+/g, " ").trim()).join(""));
  const shown = () => prod.locator("[data-env-reveal]").evaluateAll((all) => all.map((b) => b.getAttribute("aria-pressed")));

  // Masked by default: dots for the eye, "hidden" for assistive technology.
  const db = row("DATABASE_URL");
  const value = db.locator("[data-env-value]");
  check("masked", [await value.getAttribute("data-masked"), await value.locator("[aria-hidden=true]").textContent(), await spoken(value)], ["", "••••••••", "••••••••hidden"]);
  check("notInText", (await value.innerText()).includes("postgres"), false);
  check("allMasked", await shown(), ["false", "false", "false", "false"]);
  check("required", await db.locator("[data-env-required]").textContent(), "Required");
  check("description", await db.getByText("Used by the web server and the worker").count(), 1);

  // The row's toggle reveals its value, from the keyboard.
  const toggle = db.getByRole("button", { name: "Show DATABASE_URL" });
  check("toggleIcon", await toggle.locator("[data-icon]").getAttribute("data-icon"), "eye");
  await toggle.focus();
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector("#production [data-env-reveal]").getAttribute("aria-pressed") === "true");
  check("revealed", [await value.getAttribute("data-masked"), await value.innerText(), await toggle.locator("[data-icon]").getAttribute("data-icon"), await db.getAttribute("data-shown")], [null, "postgres://app:s3cret@db.internal:5432/app", "eye-off", ""]);
  check("onlyThatRow", await shown(), ["true", "false", "false", "false"]);
  // Its tooltip says what the next press does.
  check("tooltip", await page.locator("#database-url-tip").textContent(), "Hide value");
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector("#production [data-env-reveal]").getAttribute("aria-pressed") === "false");
  check("hiddenAgain", [await value.getAttribute("data-masked"), await page.locator("#database-url-tip").textContent()], ["", "Show value"]);

  // The switch reveals every row and resets their own choices.
  await row("API_KEY").getByRole("button", { name: "Show API_KEY" }).click();
  check("oneShown", await shown(), ["false", "true", "false", "false"]);
  const switchControl = prod.getByRole("switch", { name: "Show values" });
  await switchControl.focus();
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.getElementById("production").hasAttribute("data-show-values"));
  check("allShown", await shown(), ["true", "true", "true", "true"]);
  check("empty", await row("FEATURE_FLAGS").locator("[data-env-value]").innerText(), "(empty)");
  await row("GREETING").getByRole("button", { name: "Show GREETING" }).click();
  check("rowHidden", await shown(), ["true", "true", "false", "true"]);
  await switchControl.click();
  await page.waitForFunction(() => !document.getElementById("production").hasAttribute("data-show-values"));
  check("allHidden", await shown(), ["false", "false", "false", "false"]);

  // Copy copies the real value, masked or not, in the row's format.
  await db.getByRole("button", { name: "Copy DATABASE_URL" }).click();
  await db.locator("[data-copy][data-state=copied]").waitFor();
  check("copyValue", await pasted(), "postgres://app:s3cret@db.internal:5432/app");
  await row("API_KEY").getByRole("button", { name: "Copy API_KEY" }).click();
  await row("API_KEY").locator("[data-copy][data-state=copied]").waitFor();
  check("copyExport", await pasted(), 'export API_KEY="sk-live-\\$abc\\"123"');
  await row("GREETING").getByRole("button", { name: "Copy GREETING" }).click();
  await row("GREETING").locator("[data-copy][data-state=copied]").waitFor();
  check("copyDotenv", await pasted(), 'GREETING="hello world"');

  // Shown from the start.
  const dev = page.getByRole("group", { name: "Development" });
  check("startShown", [await dev.getByRole("switch").isChecked(), await dev.locator("[data-env-value]").first().innerText(), await dev.locator("[data-env-reveal]").first().getAttribute("aria-pressed")], [true, "debug", "true"]);
  await dev.getByRole("button", { name: "Copy PORT" }).click();
  await dev.locator("[data-copy][data-state=copied]").waitFor();
  check("copyName", await pasted(), "PORT");

  // A row composed from the parts: no toggle, four dots.
  const custom = page.locator("[data-example=custom]");
  check("custom", [await custom.locator("[data-env-reveal]").count(), await custom.locator("[data-env-value] [aria-hidden=true]").textContent()], [0, "••••"]);
  await button("Copy region").click();
  await custom.locator("[data-copy][data-state=copied]").waitFor();
  check("customCopy", await pasted(), "REGION=eu-west-1");

  const german = page.getByRole("group", { name: "Umgebungsvariablen" });
  check("german", [await german.getByRole("switch", { name: "Werte zeigen" }).count(), await german.getByRole("button", { name: "SECRET zeigen" }).count(), await german.getByRole("button", { name: "SECRET kopieren" }).count(), await spoken(german.locator("[data-env-value]")), await german.locator("[data-env-required]").textContent()], [1, 1, 1, "••••••••verborgen", "Pflicht"]);

  // Right to left: the value cell goes to the left; names and values stay left to right.
  await setDir("rtl");
  const rtl = await db.evaluate((li) => {
    const name = li.querySelector("code").getBoundingClientRect();
    const value = li.querySelector("[data-env-value]");
    return [value.getBoundingClientRect().right <= name.left, getComputedStyle(value).direction, getComputedStyle(li.querySelector("code")).direction];
  });
  check("rtl", rtl, [true, "ltr", "ltr"]);
};
