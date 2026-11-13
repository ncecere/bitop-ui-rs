# How the library is built

## A component is up to three files

In `crates/bitop-ui/src/components/`:

| File | Holds |
| --- | --- |
| `name.rs` | The parts, as Topcoat `#[component]` functions |
| `name.module.css` | The styles, as a CSS Module |
| `name.js` | Optional. Behaviour the platform does not provide |

`build.rs` compiles every `*.module.css` into one stylesheet and generates the scoped class
names as Rust constants in `bitop_ui::css::name`. It joins every `*.js` into one script, each
file in its own block.

Then add the module to `components.rs`, a page to the gallery, and a browser check.

## Two crates

- `crates/bitop-ui` is the base set. Its stylesheet carries the tokens, themes and base styles.
- `crates/bitop-ui-ai` is the AI set, built on the base set. It has its own `build.rs`,
  stylesheet and script, with no global styles: it reads the same tokens. Pages render
  `bitop_ui::head()` and then `bitop_ui_ai::head()`.

An AI component lives in `crates/bitop-ui-ai/src/components/` and is listed in that crate's
`components.rs`. It uses base parts by calling them (`bitop_ui::components::button::button`)
and may put base classes on its own elements (`bitop_ui::css::button::ROOT`) with the matching
`data-*` attributes. It shares script helpers through `window.bitopUi`, as the base set does;
the base script loads first. `bitop_ui::merge_style` joins a part's inline style with the
caller's. Its gallery pages go in `crates/gallery/src/app/c/` like any other, in the "AI: ..."
groups.

## Parts

```rust
#[component]
pub async fn badge(
    cx: &Cx,
    #[default] variant: BadgeVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(css::ROOT, attrs.remove("class"))) data-variant=(variant.as_str()) (attrs)>
            (child)
        </span>
    })
}
```

- One part renders one element and forwards `attrs` to it.
- Variants and state are `data-*` and ARIA attributes, never extra classes.
- A part that sets an inline style joins it with the caller's through `crate::merge_style`,
  so that the element gets one `style` attribute.
- Parts that refer to each other take IDs as arguments. Callers make IDs with `element_id(cx)`.
- Every user-visible string is an argument with an English default.
- Write a doc comment on every public item, with an example on the module.

## Porting from bitop-ui

The React source is at `registry/bitop/ui/<name>/` in bitop-ui. Keep the same part names,
variants and look. What changes:

| bitop-ui (Base UI) | Here |
| --- | --- |
| `Dialog` | `<dialog>` with `commandfor` and `command` |
| `Popover`, `Menu`, `Tooltip`, `Select` popups | Popover API with CSS anchor positioning |
| `[data-open]`, `[data-starting-style]`, `[data-ending-style]` | `:popover-open` or `[open]`, with `@starting-style` and `transition-behavior: allow-discrete` |
| `[data-checked]`, `[data-disabled]` | `:checked`, `:disabled`, or the ARIA attribute |
| `[data-highlighted]` | `:focus-visible` for menus, `[data-active]` for lists driven by a text input |
| `--anchor-width` | `anchor-size(inline)` |
| React state | Server-rendered attributes; a script only where the page must change without a request |

Prefer a native element (`<details>`, `<progress>`, `<input type="range">`, `<select>`) when it
gives the behaviour and can be styled.

## Styles

- Read semantic tokens only (`--color-*`, `--space-*`, `--radius-*`, `--font-size-*`), never
  `--palette-*` or literal colours. Tokens are in `src/styles/tokens.css`; colours are in
  `src/styles/theme-neutral.css`. Both are copied from bitop-ui and should stay in step with it.
- Use logical properties (`inline-size`, `padding-inline-start`) so right-to-left works.
- Component styles are not in a cascade layer; the reset and base styles are, so components
  always win over them.
- Module files are joined in alphabetical order. A rule that overrides another module's rule
  needs a more specific selector.
- `composes` is not supported; the build fails if a module uses it.
- Upstream's shared `popup.module.css` (the surface of menus and lists, their rows,
  separators, group labels and shortcut hints) is in the popover module as `SURFACE`,
  `ITEM`, `SEPARATOR`, `GROUP_LABEL` and `ITEM_SHORTCUT`. Its rules are under `:where()`,
  so a component's own rules win over them whatever order the modules load in. A
  surface does not take the `POPUP` class.

## Scripts

- Listen on `document` and find components by role and `data-*` attribute, so markup the server
  adds later works without registration.
- A file's names are private to it. To share something, put it on `window.bitopUi`.
- No dependencies, no build step.

## Gallery and checks

- Add an `entry!` to `GROUPS` in `crates/gallery/src/app.rs`, a `mod` line to `app/c.rs`, and a
  page at `app/c/name.rs` showing every variant and state.
- Add `e2e/tests/name.mjs` (the route name, so `scroll-area.mjs`). It exports one function that
  receives `page`, `check(label, got, want)` and helpers; see `e2e/run.mjs`.
- The runner also checks every page with axe in light and dark, and fails on console errors.
- Compare with bitop-ui: run its docs site (`npx vite --port 5317` in a bitop-ui checkout)
  and the gallery, then `node e2e/compare.mjs http://localhost:5317 http://127.0.0.1:3918
  /tmp/cmp button dialog` (or through the Playwright image, as `scripts/e2e.sh` does). It
  saves bitop-ui's examples beside ours for each name; `ref:ours` pairs a different name,
  such as `app-shell:sidebar`.

Before pushing: `cargo fmt --all`, `topcoat fmt crates`, `cargo clippy --workspace --all-targets
-- -D warnings`, `cargo test --workspace`, `scripts/e2e.sh`.

## Things that catch people out

Topcoat's `view!` macro:

- An attribute needs a value: write `popover="auto"`, `data-flush=""` and `disabled=(true)`, not
  a bare name.
- `if` can wrap attributes, but `if`/`else` between attributes with different names is safest
  written as one attribute with an `if` expression for its value.
- A part can take extra content by name. Declare `#[default] #[into] actions: Child<'_>` and
  callers write `alert(actions: view! { ... }, "message")`. A slot cannot be tested for being
  empty, so render its wrapper always and hide it in CSS with `:empty { display: none }`.
- A string built in a loop cannot be borrowed by a part in that loop. Build the strings before
  `view!`, or make them `&'static str`.
- `element_id(cx)` gives a different ID at each call site. Inside a loop, parts that call it
  need a keyed context; passing IDs in from the page avoids that.
- A gallery page function is called `show`, so that it does not clash with the component it
  imports. Routes use hyphens: `app/c/scroll_area.rs` is served at `/c/scroll-area`.
- A view is built lazily and owns what it uses. A closure that borrows an argument cannot be
  called inside `view!`; call it before and use the strings. A slice written in place
  (`selected: &[day]`) is a temporary that is gone by then; bind it with `let` first.
- A reference taken before `view!` to a local (`let pair = &series[..2];`) cannot be used in
  it: the view outlives the function's locals. Write `&series` inside the view, or bind an
  owned array.
- A slot's `view!` takes what it uses by value, so a string used both in a slot and outside it
  is moved while borrowed. Give the slot its own: `let footer_id = id.clone();`.
- A part that sits in running text, such as a hover card after a link, must be a phrasing
  element (`<span>`), and so must its children. The parser closes a `<p>` at a `<div>`.
- A view's type nests every part it renders. A page holding several large parts (response
  blocks with code blocks and citation cards, for example) overflowed the compiler's
  recursion limit while it checked that the page is `Send`. A large part returns
  `Ok(Child::new(view! { ... }))`, which boxes its view and ends the nesting there.
- A `class!(…)` value cannot be borrowed in a view (`&Class` is not an attribute value).
  For a class that several elements share, build a `String` and write
  `class=(name.as_str())`.
- Options a gallery page passes by reference (`&[QuestionChoice]`) must outlive the view.
  Give such structs `const fn` builders so the page can hold them in a `const`; a string
  that is computed, such as a `data:` URL, goes in a `static LazyLock<String>`.
- A part cannot render itself: the types of a component that calls itself never end.
  Nest through plain functions that return `Child::new(view! { ... })` and call each
  other, as `schema_display`'s property lists do. The caller's class goes in as
  `class!(css::ROWS, extra)` with `extra = attrs.remove("class")`; a `class!` value has
  no `to_string`.
- `#[component]` rewrites the lifetimes of `&str` in an argument's type, which breaks a
  function pointer argument (`fn(&str) -> bool`). Name the type with a `type` alias.
- Markup that is already sanitised goes into a view as `(Unescaped::new_unchecked(html))`.
  Only HTML built by code that escapes every piece of text may go there; see
  `bitop-ui-ai`'s `markdown.rs`.

Styles:

- The reset sets `margin: 0` on everything. Anything the browser places with a margin, such as
  a centred `<dialog>`, must set it again.
- A `display` value on a `<dialog>` or a `[popover]` shows it while it is closed. Set it under
  `[open]` or `:popover-open` only. The browser also caps a dialog's size and centres it; a
  dialog on an edge sets `inset`, `margin`, `max-inline-size` and `max-block-size` itself.
- A module that sorts before `popover` cannot change the popover's rules with one class. Write
  `.popup[popover]`.
- Animate a panel with `translate`, `scale` and `rotate`, not `transform`: the `settled` helper
  in the checks reads `translate`.
- WebKit does not run a transition of `translate` that starts from a `calc()` of a percentage
  and a length (`calc(100% + 2px)`) and ends at a value that comes from `var()`: the element
  stays where it started. Use a plain percentage.
- bitop-ui's stylesheets select Base UI's `data-*` state attributes. After copying one, change
  those selectors as the porting table says, and change `& svg` to `& [data-icon]`.
- Icons are the `icon` part, a CSS mask. To add Lucide icons, run
  `node scripts/add-icons.mjs <lucide-react's dist/esm/icons> pencil share-2`, which adds
  the masks to `src/styles/icons.css` and the variants to `Icon`.
- An icon is an empty `<span>`, so `:has(> span:empty)` matches any element with an icon in
  it. Give the element you mean a `data-*` attribute and select on that.
- The parts of a native control have a different name in each engine
  (`::-webkit-slider-thumb`, `::-moz-range-thumb`). Write one rule for each: a selector list
  with a name the engine does not know drops the whole rule.
- `position-area: inline-end` is the end of the page, not of the popup: the keywords follow
  the containing block, which for a popover is the viewport. A popup that must follow the
  direction of its own section, such as a submenu that ArrowLeft opens in a right-to-left
  section, uses `self-inline-end` and `span-self-block-end`.
- The button module turns off pointer events on `[aria-disabled="true"]`. A click on such a
  button falls through to what lies under it, which takes the focus. A button that lies over
  other content and must keep focus while disabled sets `pointer-events: auto` again.
- A `<details>` animates through `::details-content`, in a rule of its own, with
  `interpolate-size: allow-keywords` on an ancestor. Firefox has no `interpolate-size`, so the
  panel opens at once there; WebKit animates opening but hides the content at once on closing.
- Padding on a flex item is not part of what `flex-grow` shares out. Where sizes are flex
  weights, as in `resizable`, the padding goes on the content.
- axe wants the visible text of a control to be part of its `aria-label`
  (`label-content-name-mismatch`): "Go to next page" for "Next". Keep that when translating
  the two strings of a part.
- A gallery example's title is an `<h2>`, so a component that renders headings must start at
  `<h3>` there, or axe reports `heading-order`.
- axe measures contrast on any text, `aria-hidden` or not. Text dimmed with `opacity` beside
  a disabled control fails; only the control and its `<label>` are exempt. Dim the control,
  not its wrapper. For the same reason a gallery page cannot show a colour pair that fails on
  purpose: reach that state in the check instead.
- A panel hidden with `hidden` that fades out through `display … allow-discrete` is still
  shown until its transition ends. A check waits for it to be hidden
  (`waitFor({ state: "hidden" })`) rather than reading it at once.
- An SVG presentation attribute (`fill="url(#stripes)"`) loses to any stylesheet rule for
  the same property. Scope the rule so it does not match those elements.
- A drawing stretched with `viewBox` and `preserveAspectRatio="none"` stretches corner radii
  and dashes too. Bars that need round corners are drawn without a `viewBox`, in percentages
  (`x="12.5%"`), so that pixels stay pixels; lines use `vector-effect="non-scaling-stroke"`.
- A class that sets `display` wins over the browser's own rule for the `hidden`
  attribute, so `hidden` on such an element shows it anyway. A part that a script or a
  caller may hide with `hidden` adds `&[hidden] { display: none }`.
- An inline-flex element makes blocks of its children, and the accessible name then has
  spaces between them: `<span>Source</span> 1<span>:</span>` in a chip is read as
  "Source 1 :". Put the whole spoken text in one `sr-only` span and hide the visible
  number with `aria-hidden`.

Scripts:

- Setting an input's `value` in a focus handler drops the selection that Tab makes, so what
  is typed next is added to the text, not put in its place. Call `select()` after.
- Native keyboard behaviour is not the same everywhere: WebKit does not wrap the arrow keys
  in a radio group. Check the keyboard pattern in all three engines before leaving it to the
  browser.
- A control that shows its value in another form than it submits (grouping in a number) can
  keep one input: change the value in a `formdata` listener on `document`.
- To open a popover from a script, click its `popovertarget` button (`button.click()`). The
  browser then anchors the popup to the button and returns focus to it; `showPopover()` does
  neither. Focus the button first when another popover is open, or focus returns to an item of
  the popover that has just closed.
- `toggle` is sent after the popover has opened or closed, `beforetoggle` as it does. Set
  `aria-expanded` in `beforetoggle`; move focus in `toggle`.
- Some platforms send `contextmenu` when the mouse button goes down. A popover shown then is
  closed by the same click when the button comes up, as a click outside it. Wait for
  `pointerup`.
- Safari does not focus a button when it is clicked. A popover opened by a click then returns
  focus to the page, not to its button, when Escape closes it. Handle Escape and focus the
  button.
- Moving focus back to a trigger from a script fires `focusin` on it. A popup that opens on
  focus opens again unless the script remembers that it was dismissed.
- The scripts are joined in alphabetical order, so `menu.js` hears a key before `menubar.js`.
  The first one calls `preventDefault()` on what it handles, and the later one returns when
  `event.defaultPrevented` is set.
- A timer started by a pointer event acts on what was true when it started. The menu's
  hover timer once closed a submenu that the keyboard had opened while it ran; it now notes
  which submenus are open when the pointer arrives and closes only those.
- A popover that has just been shown is still scaled by its entry transition. A script
  that places it from its size reads `offsetWidth` and `offsetHeight`, not
  `getBoundingClientRect()`.
- `scroll` does not bubble. To hear it from every scroller, listen on `document` with
  `capture: true`.
- Media events (`timeupdate`, `play`, `volumechange`) do not bubble either. Listen on
  `document` with `capture: true`.
- `scrollBy({ behavior: "auto" })` follows the stylesheet's `scroll-behavior`, which may be
  `smooth`. To jump, say `"instant"`.
- A script that rebuilds what the server rendered needs the scoped class names. Copy an
  element the server rendered (`cloneNode`), or have the server render a `<template>`.
- Content of a `<details>` that has just been opened cannot take focus in Chromium until the
  next frame, while the box around it (`::details-content`) starts its transition. A script
  that opens one and focuses something inside tries again on the next frames.
- `querySelectorAll(":scope > …")` on a `<template>`'s content (a document fragment)
  matches nothing. Walk `content.children` instead.
- A `<details>`'s `toggle` event is queued too. A script that must know whether the reader
  opened or closed it (reasoning stops following the stream then) compares `open` with
  the state it last set itself, rather than waiting for the event: a change made by a
  server streaming into the page can arrive first.
- The gallery marks the current page in the sidebar only. A page's own navigation links to
  the same path with a query (`?page=2`) and marks its current item itself.

Checks:

- A role query in Playwright does not find hidden elements. Use a CSS locator for a popup that
  may be closed.
- Wait for an entry transition with the `settled` helper before measuring a popup or running
  axe on it; text that is still fading in fails the contrast rule. The `setTheme` helper waits
  for colour transitions for the same reason.
- Firefox lays out on fractions of a pixel. Round a measured size or position before comparing
  it.
- To read the clipboard, paste: focus a text box and press `ControlOrMeta+V`. To fill it, select
  text in a text box and press `ControlOrMeta+C`. Reading it with the Clipboard API needs a
  permission that only Chromium can grant, and a `ClipboardEvent` made in script carries no
  text in Firefox.
- In Playwright's Firefox, two role queries by name in a row crashed the page when a
  `<fieldset>` had `aria-labelledby` pointing at its own `<legend>`. The legend names the
  fieldset already; leave the attribute out.
- A loop on `requestAnimationFrame` inside `evaluate` has no time limit: on a popup that never
  opens, the run hangs. Wait with `page.waitForFunction`, which gives up after thirty seconds
  and says so. `settled` gives up too, and fails the page, when the element is not shown.
- Before a transition starts, the computed style already has its end values, and WebKit
  starts transitions late. `settled` therefore also waits for the element to stay in one place
  for several frames; a check that measures a moving element in its own way must do the same.
- Focus moves into a menu a moment after the key press that opens it, because it is moved in
  `toggle`. Wait for the state (`waitForFunction`), not for a time.
- Playwright does not click an element with `aria-disabled="true"` unless told to
  (`{ force: true }`), and a click on a `<label>` is also a click on the button it labels.
- A check that changes an element's inline style must put back only what it changed; the
  gallery pages lay out their examples with `style`.
- In Playwright's Firefox, a role query by name crashed the page at the next large
  `evaluate`, such as axe, when a button or a group on the page had `aria-labelledby` pointing
  at an element inside itself. This is the fieldset crash above in another form. Name such an
  element with `aria-label`. A dialog named by its own title does not crash. A crash in the
  container leaves a large `core.*` file in `e2e/`; delete it.
- WebKit moves the mouse faster than the other engines. A check of a gesture that depends on
  speed, such as a flick, sets the times itself: hold before releasing, or send pointer events
  with a `timeStamp`.
- WebKit tells the page about `emulateMedia` a moment after the call returns. Wait for what
  the page does with it.
- Firefox starts a transition on an element that is not rendered, such as a field in a closed
  `<details>`, and never runs it, so its `finished` promise never settles. `setTheme` leaves
  those out; a check that waits for transitions in its own way must do the same
  (`element.checkVisibility()`).
- Neither `getAnimations()` nor the transition events report a transition on
  `::details-content` in Chromium. To check that a panel opens gradually, read its height at
  every frame, and lengthen the transition for the check so that a slow machine sees it.
- In Playwright's Firefox, a mouse that moves outside the viewport during a drag reports
  `clientX` 0 and its release never arrives. Keep a drag inside the window.
- Firefox counts the stroke in `getBoundingClientRect()` of an SVG shape; the other engines
  do not. Measure a shape's geometry with `getBBox()`.
- In WebKit, after a click on a `<summary>`, Tab stays on it. Open a `<details>` from the
  keyboard before checking the tab order of what is inside.
- `CSS.supports()` can say yes where the feature only half works (WebKit and
  `interpolate-size` on closing). When no feature test tells engines apart, ask for the
  engine (`page.context().browser().browserType().name()`) and say why in a comment.
- To check that something waits before it happens, record `performance.now()` in the page at
  both events and compare. A check made "straight after" from the runner fails on a slow
  machine.
- The `toggle` event of a popover is queued, not sent at once. A listener that moves focus
  when a popup opens must first check that focus is not already inside it; otherwise, on a
  slow machine, it undoes keys pressed in the meantime. Menus, the menubar and the date
  picker had this bug.
- axe's `label-content-name-mismatch` joins the text of a control's block children without
  spaces before it looks for it in the `aria-label`: a card button showing "Travel policy"
  and "Uploaded file" in two blocks must have "Travel policyUploaded file" in its label.
  Keep an `aria-label` for a control whose visible text is one line.
- A plain `<button>` on a gallery page keeps the browser's colours, which fail axe's contrast
  rule in WebKit's dark theme. Use the base `button` part.
- A request that fails, such as an image from a host that does not exist, logs a console
  error and fails the page. Gallery pages load nothing from other hosts at load time. A check
  that needs a remote image routes the request (`page.route`) and fulfils it; an image that
  must not load before the route is set goes in a closed `<details>` with `loading="lazy"`.
  Fetching a revoked object URL logs an error too: to check that a URL was revoked, wrap
  `URL.revokeObjectURL` in the page and record the calls.
- A copy of an `<img>` (`cloneNode`) starts loading its `src` at once; Firefox reports that
  load even if the copy's `src` is changed straight after. Create a new `<img>` instead.
- A `ClipboardEvent` made in script carries no files in Firefox either, and a check cannot
  put files on the clipboard. Test whether the event kept its data and leave the paste out
  where it did not.
- Changing the page's direction resizes a log that follows its content, and its
  `ResizeObserver` pins it to the bottom a frame later. A check that then scrolls the log
  waits two frames first, or the pin undoes the scroll.
- The Firefox crash above happens for any element named by `aria-labelledby` pointing
  inside itself: a `<section>` (region) named by its title, a `<form>` named by its
  prompt, and a group named by its label all crashed the page. The agent, plan,
  confirmation, question and artifact cards are named with `aria-label` for that reason.
- In Firefox, a popover whose `position-try` moves it from a place off the screen (a
  card that opens above its trigger near the top of the page and flips below) stops its
  entry transition just short of the end, at an opacity of 0.999, so `settled` never
  returns. Gallery pages leave room above such a trigger.
- Firefox and WebKit leave the content of a `<details>` that has just opened out of the
  tab order until it is shown. Wait for an element inside to be visible before pressing
  Tab into it.
- In a search box that holds text, the first Escape empties it in Chromium and Firefox;
  a dialog around it closes on the second.
- A media element's state (`paused`, `muted`, `currentTime`) changes at once, but its
  events come a moment later, and the controls follow the events. Wait for what the
  controls show, not for the element.
- `run.mjs` launches Chromium with a fake microphone that is allowed without a prompt,
  and Firefox with fake streams and no permission prompt; WebKit has neither. A voice
  check compares the page with what the engine reports (`enumerateDevices`,
  `SpeechRecognition`), and runs the rest of the state machine on stand-ins installed
  with `addInitScript` in a page of its own (`page.context().newPage()`): the runner
  reloads the test's page for axe, and the stand-ins would stay on it.
- axe never finishes on a page with an `<iframe sandbox="">`: it cannot run in a frame
  whose scripts are off, and waits for it. Gallery frames keep `allow-scripts`.
- Reading `contentDocument` of a frame sandboxed without `allow-same-origin` logs a
  console error in WebKit, which fails the page. Check the `sandbox` attribute instead;
  read the frame's content with `page.frameLocator`, which works across origins.
- axe's `landmark-unique` fails two regions with the same name, and two unnamed `<main>`s,
  also when they are in different frames on the page. Give gallery examples of a
  component that names a region by its content (a stack trace by its error) different
  content, and name a frame page's `<main>`.
- `getComputedStyle` gives the value a transition has reached, not the end value. A
  check of a chevron's `rotate` waits for `getAnimations()` to finish first.
- A `scroll` event comes a frame after a script scrolls. A check that scrolls a log that
  follows its content, then adds to it, waits two frames between, or the log still
  thinks the reader is at the bottom.
- A `treeitem` named with `aria-label` fails `label-content-name-mismatch` once its folder
  is open, as axe counts the nested items' text as the item's. Name tree items by their
  row's text: the nested `group` is not part of a tree item's name.
- A `toggle` listener added with `{ once: true }` right after a popover closed can be used
  up by that close's queued `toggle` event. A check that times the next open records the
  first `newState === "open"` without `once`.
