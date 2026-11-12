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
