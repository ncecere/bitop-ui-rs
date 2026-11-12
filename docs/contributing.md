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

Styles:

- The reset sets `margin: 0` on everything. Anything the browser places with a margin, such as
  a centred `<dialog>`, must set it again.
- bitop-ui's stylesheets select Base UI's `data-*` state attributes. After copying one, change
  those selectors as the porting table says, and change `& svg` to `& [data-icon]`.
- Icons are the `icon` part, a CSS mask. To add one, add its Lucide path to
  `src/styles/icons.css` and a variant to `Icon`.

Checks:

- A role query in Playwright does not find hidden elements. Use a CSS locator for a popup that
  may be closed.
- Wait for an entry transition with the `settled` helper before measuring a popup or running
  axe on it; text that is still fading in fails the contrast rule.
