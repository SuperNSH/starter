# nsh-starter

A production-grade application boilerplate for [Native Shell](https://nsh.dev) (NSH): clone it, rename it, ship it on macOS, iOS, Android and the web. It is also the living reference for what NSH 0.7.1 can do — every element and base component ships in a real screen, all async state runs on [`nsh-query`](../nsh-query) and every form on [`nsh-form`](../nsh-form).

The app is Rust-first: one crate at the repo root is the UI, and `packages/remote-screen` is a TypeScript (TSX) feature published as a signed package and loaded on demand, proving the mixed path. There is no server and no account anywhere; a local fake API backs the data screens.

## What the app demonstrates

| Page | Area | What it shows |
|---|---|---|
| Home | shell | lifecycle state machine, typed routes with deep links, `nav_stack`, sidebar on desktop / tabs on mobile, command palette, keyboard shortcuts, undo and redo, status bar |
| UI gallery | ui | one themed wrapper per `nsh_base` component in use, in light and dark, with states, focus rings and press feedback |
| Forms | forms | a full sign-up (async username check with debounce, date, time, select, combobox, OTP, slider, color picker, rich editor, server errors, draft autosave, dirty guard) and a settings form loaded with `form.load` |
| Data, data item, query devtools | data | `virtual_list` on `use_infinite_query`, pull to refresh, optimistic like with rollback, invalidation, stale-while-revalidate, retry, offline banner, skeletons, and a devtools page listing cache entries |
| Lists | lists | `Table`, `DataTable`, `Tree`, `Calendar`, sticky headers, grids, a chat on an inverted list, viewability, carousel, pagination |
| Canvas | canvas | a drawing app with responder input and undo, animated canvas, `Plot` charts, a WGSL `shader`, SVG, images with placeholders, video, `web_view` |
| Motion | motion | transitions, layout animations, springs, swipe rows, drag and drop, pinch and rotate, a scroll-linked header, reduced-motion paths |
| Overlays | overlays | `Dialog`, `AlertDialog`, `Sheet`, `Popover`, `Menu`, `ContextMenu`, `HoverCard`, `Tooltip`, toasts, `DockArea`, resizable panels, `Editor`, `DiffView`, `TextView` |
| Platform, device | platform | safe areas, keyboard, clipboard, picker, speech, file dialogs, `nsh/mobile` `TabBar`, `nsh/desktop` window and menus, i18n (en, zh-CN) and accessibility |
| Remote screen | remote-screen | the signed TSX package: generated keys, `nsh publish`, `nsh verify`, loading on demand, revocation |
| Diagnostics | foundation | capability registry, event bus, background tasks, typed config with schema migrations, file logging, first-run state, the central error surface |

## Prerequisites

- **macOS** with Xcode and an iOS simulator runtime (tested against iOS 27).
- **nsh 0.7.1**, from the release's installer (Homebrew's tap still ships 0.4.0):
  ```sh
  curl -fsSL https://github.com/longbridge/nsh/releases/download/v0.7.1/install.sh | NSH_VERSION=v0.7.1 sh
  nsh component add android ios web
  ```
  It installs into `~/.nsh` and links `~/.local/bin/nsh`. `nsh --version` must print 0.7.1: if an older Homebrew `nsh` comes first on your `PATH`, put `~/.local/bin` before it. The Rust crates pin 0.7.1, and the CLI must match them. Platform runtimes are components since 0.5.0; a command that needs one also adds it on demand.
- **Rust stable** with the cross-compile targets:
  ```sh
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim \
      aarch64-linux-android armv7-linux-androideabi x86_64-linux-android \
      wasm32-unknown-unknown
  ```
- **Android SDK and NDK 29** (Android Studio's SDK is fine), with `ANDROID_HOME` pointing at the SDK. `/usr/bin/java` on macOS is a stub with no runtime behind it; export Android Studio's JDK before Android builds:
  ```sh
  export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
  ```
- **just** for the recipes (`brew install just`).
- The two sibling libraries, for the TSX package only. The Rust crate takes `nsh-query` and `nsh-form` 0.1.0 from crates.io; `packages/remote-screen` links their TypeScript packages from checkouts next to this repo:
  ```
  parent/
  ├── nsh-starter/   (this repo)
  ├── nsh-query/
  └── nsh-form/
  ```

The first build compiles the whole NSH crate graph and takes a while; builds after that are incremental.

## From a clean clone to a running app

```sh
git clone https://github.com/freeoxide/nsh-starter.git
cd nsh-starter

just check         # rustfmt + clippy -D warnings
just test          # cargo test
nsh check          # TSX type check (runs cargo check first)
nsh test           # cargo test, then the headless TSX tests
just run-macos     # AppKit dev host, hot reload
just run-ios       # iOS simulator
just run-android   # first device adb lists
just run-web       # the browser
```

`nsh run --target ios` needs a booted simulator (any iPhone; it picks the newest otherwise). `nsh run --target android` needs a running emulator or a plugged device. On Android, set `JAVA_HOME` as above — `just run-android` does it for you when it is unset.

## Commands

| Recipe | What it runs |
|---|---|
| `just check` | `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings` |
| `just test` | `cargo test`; `just test settings` runs one screen's file |
| `just rules` | the architecture rules alone ([AGENTS.md](AGENTS.md)) |
| `just new-app <dir> <name> <id>` | a new app from this starter, without the demo screens (see below) |
| `just test-tsx` | `nsh check`, `nsh build` (warnings denied) and `nsh test` inside `packages/remote-screen` |
| `just smoke` | `nsh test` at the root, then `just test-tsx` — the whole-tree headless gate |
| `just run-macos` / `run-ios` / `run-android` / `run-web` | `nsh run --target <t>` |
| `just package` | `nsh package --target macos`, `ios-simulator`, `android` and `web`, into `.nsh/out` |
| `just rename <name> <id>` | rename the app everywhere it lives (see below) |
| `just rename --roundtrip-test` | prove the rename: copy, rename, run the criterion-1 gate (fmt, clippy, `cargo test`, `nsh test`) in the copy, clean up |
| `just status` | print `docs/STATUS.md` — what is done and how it was verified |

Per-target smoke suites (open every page, screenshot) run as `nsh test --target macos|ios|android` — the web target passes only the features that exist on the web, and the rest are listed in `docs/STATUS.md`.

## Finding your way

[AGENTS.md](AGENTS.md) is the map: what each folder under `src/` is for, where a new thing goes, the shape every screen folder has, and the rules `just rules` checks. [docs/recipes/](docs/recipes/) has a step list for each common change, starting with [adding a page](docs/recipes/add-a-page.md).

## Starting a new app

```sh
just new-app ../acme-notes acme-notes com.acme.notes
```

This copies the repo to the directory without the demo screens (`src/showcase`), their tests, the remote-screen package and this starter's build records, then renames it. The copy keeps Home, Items (a list with a detail on `nsh-query`), Sign in (a form on `nsh-form`), Settings and Diagnostics, the shell, the store, the services with the fake API, the ui library and the native code, and passes `just check` and `just test` as made.

## Renaming the app

```sh
scripts/rename.sh acme-app dev.acme.app       # in place, in this checkout
scripts/rename.sh --roundtrip-test            # prove it on a throwaway copy
```

The script replaces the name and id everywhere they live: `nsh.json`, the Cargo package name and crate ident, the `#[native_module]` specifier, `src/main.rs` (title and app id), the i18n `app.name`, and every other text file that mentions them. `--roundtrip-test` copies the repo (without `target/`, `.nsh/`, `.git/`) to a sibling directory, renames the copy, runs the full gate there (`cargo fmt` check, clippy, `cargo test`, `nsh test`), reports pass/fail, and always deletes the copy. It exits 0 only on a pass.

## The signed TSX package

`packages/remote-screen` is a TSX feature that uses the TypeScript `nsh-query` and `nsh-form`. It is published as a signed package with `nsh publish` and loaded on demand, with `nsh verify` and a revocation path (`nsh revoke`). Keys are generated at run time and never committed. [docs/architecture.md](docs/architecture.md) walks the delivery path; the package's `scripts/delivery.sh` is the runnable version.

## Documentation

- [AGENTS.md](AGENTS.md) — the map of the source, the rules and the commands.
- [docs/architecture.md](docs/architecture.md) — how the layers, routes, pages, navigation, state and services work together.
- [docs/recipes/](docs/recipes/) — step lists: add a page, shared state, a service, a control, a string, move to a new NSH release.
- [docs/coverage.md](docs/coverage.md) — every `nsh_base` and `nsh_core` element export on 0.4.0, where it is used, why anything is not, and what 0.5.0 and 0.6.0 added.
- [docs/gpui-starter-parity.md](docs/gpui-starter-parity.md) — feature-by-feature against the GPUI starter this repo mirrors.
- [docs/upstream-issues.md](docs/upstream-issues.md) — known `nsh-query`/`nsh-form` bugs and their workarounds.
- [docs/STATUS.md](docs/STATUS.md) — what is done, what was verified how, what is not verified.
- [CONTRIBUTING.md](CONTRIBUTING.md) — the gates, the test rules, the doc standards.

## License

[MIT](LICENSE)
