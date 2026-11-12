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
- A slot's `view!` takes what it uses by value, so a string used both in a slot and outside it
  is moved while borrowed. Give the slot its own: `let footer_id = id.clone();`.
- A part that sits in running text, such as a hover card after a link, must be a phrasing
  element (`<span>`), and so must its children. The parser closes a `<p>` at a `<div>`.

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
- Icons are the `icon` part, a CSS mask. To add one, add its Lucide path to
  `src/styles/icons.css` and a variant to `Icon`.
- An icon is an empty `<span>`, so `:has(> span:empty)` matches any element with an icon in
  it. Give the element you mean a `data-*` attribute and select on that.
- The parts of a native control have a different name in each engine
  (`::-webkit-slider-thumb`, `::-moz-range-thumb`). Write one rule for each: a selector list
  with a name the engine does not know drops the whole rule.
- axe measures contrast on any text, `aria-hidden` or not. Text dimmed with `opacity` beside
  a disabled control fails; only the control and its `<label>` are exempt. Dim the control,
  not its wrapper. For the same reason a gallery page cannot show a colour pair that fails on
  purpose: reach that state in the check instead.

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
- A script that rebuilds what the server rendered needs the scoped class names. Copy an
  element the server rendered (`cloneNode`), or have the server render a `<template>`.

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
- To check that something waits before it happens, record `performance.now()` in the page at
  both events and compare. A check made "straight after" from the runner fails on a slow
  machine.
