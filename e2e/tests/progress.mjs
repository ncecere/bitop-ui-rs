export default async ({ page, check }) => {
  const bar = (name) => page.getByRole("progressbar", { name });
  check("value", await bar("Uploading").evaluate((b) => [b.getAttribute("aria-valuenow"), b.getAttribute("aria-valuemax"), b.getAttribute("aria-valuetext")]), ["40", "100", "40%"]);
  check("shownValue", await bar("Uploading").getByText("40%").isVisible());
  check("indeterminate", await bar("Indexing").evaluate((b) => [b.hasAttribute("aria-valuenow"), !!b.querySelector("[data-indeterminate]")]), [false, true]);
  check("customMax", await bar("Done").getAttribute("aria-valuetext"), "100%");
  const fill = await bar("Uploading").evaluate((b) => { const [track, ind] = [b.querySelector("div:last-child"), b.querySelector("div:last-child > span")]; return ind.getBoundingClientRect().width / track.getBoundingClientRect().width; });
  check("fill", Math.abs(fill - 0.4) < 0.01);
  check("hiddenLabelStillNames", await bar("Storage").count(), 1);
};
