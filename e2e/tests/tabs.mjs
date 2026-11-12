export default async ({ page, check, focused, visible, setDir }) => {
  const selected = () => visible(page.locator("[role=tab][aria-selected=true]"));
  const panel = () => visible(page.locator("[role=tabpanel]:not([hidden])"));
  await page.getByRole("tab", { name: "Models" }).click();
  check("click", [await selected(), await panel()], [["Models"], ["Model settings."]]);
  await page.keyboard.press("ArrowRight");
  check("arrow", [await selected(), await focused()], [["Members"], "Members"]);
  await page.keyboard.press("ArrowRight");
  check("wraps", await selected(), ["General"]);
  await page.keyboard.press("End");
  check("end", await selected(), ["Members"]);
  check("roving", await visible(page.locator("[role=tab][tabindex='0']")), ["Members"]);
  await page.keyboard.press("Tab");
  check("tabToPanel", await focused(), "Member settings.");
  await setDir("rtl");
  await page.getByRole("tab", { name: "General" }).click();
  await page.keyboard.press("ArrowLeft");
  check("rtlArrows", await selected(), ["Models"]);
};
