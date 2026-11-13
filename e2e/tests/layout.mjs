export default async ({ page, check, setDir }) => {
  const demo = (name) => page.locator(`[data-demo="${name}"]`);
  const round = (b) => ({ x: Math.round(b.x), y: Math.round(b.y), w: Math.round(b.width), h: Math.round(b.height), r: Math.round(b.x + b.width), b: Math.round(b.y + b.height) });
  const box = async (locator) => round(await locator.boundingBox());
  const kids = (locator) =>
    locator.evaluate((el) =>
      [...el.children].map((child) => {
        const b = child.getBoundingClientRect();
        return { x: Math.round(b.x), y: Math.round(b.y), w: Math.round(b.width), h: Math.round(b.height), r: Math.round(b.right), b: Math.round(b.bottom) };
      }),
    );
  const style = (locator, property) => locator.evaluate((el, p) => getComputedStyle(el)[p], property);

  // Stack: a column whose children stretch, 16 pixels apart by default.
  let outer = await box(demo("stack"));
  let inner = await kids(demo("stack"));
  check("stackColumn", inner.every((k) => k.x === outer.x && k.w === outer.w));
  check("stackGap", [inner[1].y - inner[0].b, inner[2].y - inner[1].b], [16, 16]);
  check("stackNoGapAttribute", await demo("stack").getAttribute("data-gap"), null);
  outer = await box(demo("stack-wide"));
  inner = await kids(demo("stack-wide"));
  check("stackGapFromScale", [await demo("stack-wide").getAttribute("data-gap"), inner[1].y - inner[0].b], ["8", 32]);
  check("stackAlignStart", inner.every((k) => k.x === outer.x && k.w < outer.w));

  // As a list: a <ul> without markers or padding, here aligned to the end.
  const list = demo("stack-list");
  outer = await box(list);
  inner = await kids(list);
  check("stackList", [await list.evaluate((el) => el.tagName), await style(list, "listStyleType"), await style(list, "paddingInlineStart")], ["UL", "none", "0px"]);
  check("stackListItems", await list.getByRole("listitem").count(), 2);
  check("stackAlignEnd", inner.every((k) => k.r === outer.r && k.w < outer.w));
  check("stackGapOne", inner[1].y - inner[0].b, 4);

  // Inline: a row, 8 pixels apart, that wraps, with its children centred across it.
  inner = await kids(demo("inline"));
  check("inlineRow", [inner[1].y === inner[0].y, inner[1].x - inner[0].r], [true, 8]);
  check("inlineWraps", new Set(inner.map((k) => k.y)).size > 1);
  outer = await box(demo("inline-between"));
  inner = await kids(demo("inline-between"));
  check("inlineBetween", [inner[0].x === outer.x, inner[1].r === outer.r, await style(demo("inline-between"), "flexWrap")], [true, true, "nowrap"]);
  check("inlineCentred", Math.abs(inner[0].y + inner[0].h / 2 - (inner[1].y + inner[1].h / 2)) <= 1);
  const nested = await kids(demo("inline-between").locator("> div"));
  check("inlineGapFromScale", nested[1].x - nested[0].r, 12);
  outer = await box(demo("inline-baseline"));
  inner = await kids(demo("inline-baseline"));
  check("inlineBaseline", [await style(demo("inline-baseline"), "alignItems"), inner[0].y < inner[1].y, inner[1].x - inner[0].r], ["baseline", true, 16]);
  check("inlineJustifyCentre", Math.abs(inner[0].x - outer.x - (outer.r - inner[1].r)) <= 1);
  outer = await box(demo("inline-end"));
  inner = await kids(demo("inline-end"));
  check("inlineEndNoGap", [inner[1].r === outer.r, inner[1].x - inner[0].r], [true, 0]);

  // Container: a centred column with a greatest width and side padding.
  const containers = demo("containers").locator("[data-size]");
  check("containerSizes", await containers.evaluateAll((all) => all.map((c) => `${c.dataset.size}:${getComputedStyle(c).maxInlineSize}`)), ["md:896px", "lg:1200px", "full:none"]);
  check("containerPadding", await style(containers.first(), "paddingInlineStart"), "24px");
  const frame = await box(demo("containers").locator("> div").first());
  const column = await box(containers.first());
  // The frame's border is a pixel and a half, which engines round differently.
  check("containerFillsNarrowParent", [column.x - frame.x <= 2, column.x - frame.x === frame.r - column.r], [true, true]);
  await page.setViewportSize({ width: 1100, height: 900 });
  check("containerPaddingWide", await style(containers.first(), "paddingInlineStart"), "32px");
  await containers.first().evaluate((el) => (el.style.maxInlineSize = "10rem"));
  const narrow = await box(containers.first());
  const around = await box(demo("containers").locator("> div").first());
  check("containerCentred", Math.abs(narrow.x - around.x - (around.r - narrow.r)) <= 1);
  await containers.first().evaluate((el) => (el.style.maxInlineSize = ""));
  await page.setViewportSize({ width: 1000, height: 900 });

  // Right to left: a row starts on the right, and "end" is the left.
  await setDir("rtl");
  outer = await box(demo("inline-between"));
  inner = await kids(demo("inline-between"));
  check("rtlRowStartsRight", [inner[0].r === outer.r, inner[1].x === outer.x], [true, true]);
  outer = await box(list);
  inner = await kids(list);
  check("rtlEndIsLeft", inner.every((k) => k.x === outer.x));
  outer = await box(demo("inline-end"));
  inner = await kids(demo("inline-end"));
  check("rtlJustifyEnd", inner[1].x === outer.x);
};
