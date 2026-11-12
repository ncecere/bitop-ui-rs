export default async ({ page, check, setDir }) => {
  const demo = (name) => page.locator(`[data-demo="${name}"]`);
  const box = async (locator) => {
    const b = await locator.boundingBox();
    return { x: Math.round(b.x), y: Math.round(b.y), w: Math.round(b.width), h: Math.round(b.height), r: Math.round(b.x + b.width), b: Math.round(b.y + b.height) };
  };

  // Everything: breadcrumbs above, status beside the title, description and facts
  // under it, actions at the end of the title's row.
  const full = demo("full");
  const title = full.getByRole("heading", { level: 2, name: "Deployments" });
  check("title", [await title.count(), await title.getAttribute("id")], [1, "deployments-title"]);
  const header = await box(full);
  const t = await box(title);
  const crumbs = await box(full.getByRole("navigation", { name: "Deployments breadcrumb" }));
  const badge = await box(full.getByText("Live", { exact: true }));
  const description = await box(full.locator("p"));
  const facts = await box(full.getByText("42 deployments"));
  const actions = await box(full.getByRole("button", { name: "Deploy" }));
  const settings = await box(full.getByRole("button", { name: "Settings" }));
  check("breadcrumbsAbove", crumbs.b <= t.y);
  check("metaBesideTitle", [badge.x >= t.r, Math.abs(badge.y + badge.h / 2 - (t.y + t.h / 2)) <= 2], [true, true]);
  check("descriptionUnderTitle", [description.y >= t.b, description.x === t.x], [true, true]);
  check("factsUnderDescription", facts.y >= description.b);
  check("actionsAtEnd", [actions.r === header.r, settings.r < actions.x, actions.x > description.r - 1], [true, true, true]);
  check("actionsLevelWithTitle", Math.abs(actions.y + actions.h / 2 - (t.y + t.h / 2)) <= 4);
  check("descriptionMuted", await full.locator("p").evaluate((el) => getComputedStyle(el).color !== getComputedStyle(el.closest("[data-page-header]")).color));

  // A title alone: the empty slots take no room, and there is no description.
  const plain = demo("plain");
  check("plainHeading", await plain.getByRole("heading", { level: 2, name: "Members" }).count(), 1);
  check("emptySlotsHidden", await plain.locator("div:empty").evaluateAll((all) => all.map((el) => getComputedStyle(el).display)), ["none", "none", "none", "none"]);
  check("noDescription", await plain.locator("p").count(), 0);
  const p = await box(plain);
  const pt = await box(plain.getByRole("heading"));
  check("plainHeight", p.h === pt.h);

  // A long name without spaces wraps instead of widening the header.
  const long = demo("long");
  const lt = long.getByRole("heading", { level: 3 });
  const lh = await box(long);
  const ltb = await box(lt);
  check("longWraps", [lh.w, ltb.r <= lh.r, ltb.h > 40, await lt.evaluate((el) => el.scrollWidth <= el.clientWidth)], [384, true, true, true]);
  const del = await box(long.getByRole("button", { name: "Delete" }));
  check("actionsWrapUnder", [del.y >= ltb.b, del.r <= lh.r], [true, true]);

  // Right to left: the title starts on the right and the actions end on the left.
  await setDir("rtl");
  const rh = await box(full);
  const rt = await box(title);
  const ra = await box(full.getByRole("button", { name: "Deploy" }));
  check("rtl", [rt.r === rh.r, ra.x === rh.x], [true, true]);
};
