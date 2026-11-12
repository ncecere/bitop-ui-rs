# bitop-ui-rs

A component library for [Topcoat](https://github.com/tokio-rs/topcoat), the server-rendered Rust
web framework. It is the Rust port of [bitop-ui](https://github.com/ncecere/bitop-ui) and shares
its design tokens and themes, so a Rust site and a React site can look the same.

- **Parts, not monoliths.** A dialog is `dialog_trigger`, `dialog`, `dialog_title`, and so on.
  Each part is a Topcoat `#[component]` that renders one element.
- **Platform first.** Overlays use `<dialog>`, the Popover API, invoker commands and CSS anchor
  positioning. One small script adds what the platform lacks, such as arrow keys in menus.
- **CSS Modules and CSS variables.** No Tailwind, no Node build, no React. A misspelt class name
  is a Rust compile error.
- **Checked.** Every component has a gallery page and browser checks for keyboard behaviour,
  focus and accessibility (axe), run in Chromium, Firefox and WebKit.

Status: early. 79 of 120 components are built; see the [roadmap](docs/roadmap.md) for the rest.
Topcoat itself is experimental, and this library pins one exact version of it.

## See it

The gallery, with every component on its own page: **https://ncecere.github.io/bitop-ui-rs/**

It is a static copy, rebuilt on every push to `main` by `scripts/static-site.sh`. Everything that
runs in the browser works there. What needs a live server does not: a search typed into the
filter bar, the sidebar staying collapsed after a reload, and relative times, which show the
moment the copy was made. Run the gallery locally (below) for those.

## Layers

| Crate | Contents |
| --- | --- |
| `bitop-ui` | The base set: buttons, forms, overlays, navigation, layout |
| `bitop-ui-ai` | Planned. Chat and agent components built on the base set |
| `bitop-ui-build` | Build-script helper: CSS Modules to one stylesheet, component scripts to one file |
| `gallery` | A Topcoat app with one page per component |

## Use it

```toml
[dependencies]
bitop-ui = { git = "https://github.com/ncecere/bitop-ui-rs" }
topcoat = "=0.12.1"
```

Render `bitop_ui::head()` inside `<head>`, then use the parts:

```rust
use bitop_ui::components::dialog::{dialog, dialog_actions, dialog_close, dialog_title, dialog_trigger};

let id = element_id(cx);
let title = element_id(cx);
view! {
    dialog_trigger(target: &id, "Rename chat")
    dialog(id: &id, labelled_by: &title,
        dialog_title(id: &title, "Rename chat?")
        dialog_actions(dialog_close(target: &id, "Cancel"))
    )
}
```

Browsers: Chrome and Edge 135, Firefox 147, Safari 26.2, or newer.

## Develop

Needs Rust 1.99 (installed by `rust-toolchain.toml`), the Topcoat CLI
(`cargo install topcoat-cli --version 0.12.1 --locked`), and Docker for the browser checks.

```sh
topcoat dev -p gallery        # the gallery at http://127.0.0.1:3000, rebuilt on change
scripts/e2e.sh                # browser checks in three engines
scripts/e2e.sh button menu    # only these pages
```

- [How the library is built and how to add a component](docs/contributing.md)
- [Roadmap](docs/roadmap.md)

## Licence

MIT. The AI component set takes its component list and part names from Vercel's AI Elements
(Apache 2.0), as bitop-ui does; see bitop-ui's `NOTICE`.
