# Architecture

nsh-starter is a [Native Shell](https://nsh.dev) (NSH) 0.7.0 application boilerplate. One Rust crate at the repo root is the app; `packages/remote-screen` is a TypeScript (TSX) feature delivered as a signed package. There is no server: `src/services/fake_api.rs` is the backend, with latency and failure switches, so the data screens run offline.

`AGENTS.md` is the map and the rules. This file explains how the parts work together.

## Entry points

`nsh.json` says `"rust": "."` and `"root": "App"`: the app is a Rust app whose root component is `App` in `src/lib.rs`.

- **macOS, Linux, Windows:** `src/main.rs` calls the platform crate's `run_app` with `Root::of::<App>()`.
- **iOS, Android, web:** `nsh run --target ios|android|web` boots the development host and mounts the same `App`. The host scaffolding is generated into `.nsh/` (git-ignored).
- **Packaged:** `nsh package --target macos|ios-simulator|android|web` builds an installable app from the prebuilt runtime.

`App` does two things: it calls `startup::install()`, then draws `shell::shell()`. `src/startup.rs` is the list of everything that runs once at launch, in order: keyboard shortcuts, the back order, the lifecycle watch, deep links, and the log line a config save leaves.

## Layers

```text
lib.rs, main.rs, startup.rs
        │
      shell            the frame: sidebar, tabs, headers, palette, back, links, error banner
        │
 pages, showcase       the screens
        │
      store            what the app remembers
        │
     services          what the app talks to
        
 ui ◀── native         below the screens: the component library, and platform views built on it
 routes.rs, error.rs   vocabulary every layer may name
```

Imports point down only, and a screen never imports another screen. `tests/architecture.rs` enforces both.

## Routes and pages

`src/routes.rs` holds two things. `Route` is a screen's address: what navigation pushes, what a deep link parses into, what undo snapshots. `Page` is what a screen declares about itself: key, title, summary, icons, how the shell frames it, an optional header button, and what it draws.

Each screen's folder has a `const PAGE: Page` in its `mod.rs`. `src/pages/mod.rs` joins addresses to records in one `match` (`Route::page`) and lists the sidebar's and the tab bar's screens in order.

The shell reads only records. `shell/mod.rs` draws the sidebar and the tab bar from `pages::sidebar()` and `pages::tabs()`; `shell/page.rs` frames a screen from `Page::frame` and `Page::action`; the palette, the application menu and the macOS command bar list `pages::sidebar()`. No file in `src/shell/` names a screen, so adding one changes nothing there.

The demo screens have their own enum, `showcase::Route`, wrapped by one variant, `Route::Showcase`. That is what lets `scripts/new-app.sh` remove them as a unit.

## Navigation

`store::nav` is the one stack: the pages pushed over Home. The sidebar, the tabs, the palette, system back, deep links and a page's own button all call `nav::open`, `nav::push` or `nav::pop`, and each change snapshots the stack for undo (`store::undo`).

A screen that opens a child (a post from the feed) calls `nav::push_child`. On phones the shell pushes it as a screen of its own; on desktop widths the call answers `false` and the screen presents the child itself. The screen does not know which shell is up.

Pages mount through `nav_stack`: a native stack on iOS and Android (swipe back, system header, predictive back), a stack with the app's own header on the desktops and the web. Android back runs through `shell::back`: modals first, then pushed pages, then the exit ask.

## State

Two layers, both signals.

**App state** (`src/store`): plain signals behind functions. Navigation, the typed config (persisted through NSH's `Storage`, with schema migrations), the UI language, the appearance, the event bus, the background task manager and the error queue. A store keeps its signal private and exports what reads and changes it. `global::<T>()` keeps one value per type, so every global has a type of its own. NSH's own reads (`dark()`, `locale()`, `font_scale()`) are already reactive and are not copied into the store.

**Async state** (`nsh-query`, in the Items page and the demo data screens): every server-shaped read is a `use_query*` with a typed key, a cache policy and a retry policy; writes are `use_mutation*` that invalidate keys on success. The fetchers call `services::fake_api`, never a real network.

**Forms** (`nsh-form`, in the Sign in page and the demo forms screens): `use_form` owns one value type per form; `field!` binds fields with rules; `form.submit` maps server failures to field errors; dirty state drives the back guard and draft autosave.

Two NSH constraints shape this code: `Signal::update` needs `T: Clone`, and a component's signals are already disposed when its `cleanup` runs.

## Errors

`error::AppError` is the one error type: a stable `code`, a `message`, an optional `detail`. Services return it. Anything raises one with `store::errors::report_error`, which logs it, records it in the monitoring sink, publishes it on the bus and queues it. `shell/error_surface.rs` shows the newest queued error in a banner over the app.

## UI

`themes/theme.json` defines the semantic tokens. `src/ui` holds one wrapper per `nsh_base` component in use, styled from those tokens, with the states (hover, focus ring, press, disabled). The app's pages take every control from `ui`. The showcase also imports `nsh_base` directly where it demonstrates a base component itself.

`src/native` holds the views UIKit and AppKit draw: the system's own controls, the soft scroll edges under the iOS header, the macOS command bar. Each has a fallback built from `ui` for every other platform, and each is registered from `src/lib.rs`. It is the only folder with `unsafe`; `src/lib.rs` forbids it everywhere else.

## Services

Each seam is a trait with a local implementation, so the real library replaces it by changing one file.

| Seam | Local implementation |
|---|---|
| `services/http.rs`: one small trait over NSH `fetch` | a thin `fetch` caller |
| `services/auth.rs`: the session provider | a mock user, no real account |
| `services/analytics.rs`: event sink | an in-memory sink |
| `services/monitoring.rs`: error and breadcrumb sink | an in-memory sink; `report_error` feeds it |

The Sign in page calls the auth seam through `store::session`; nothing calls the HTTP or analytics seams yet: they are the places a real app plugs in. `services/logging.rs` keeps a bounded in-memory ring; `init_file` adds a log file, and the app does not call it (add the call to `startup.rs` to turn it on).

## What is built but not switched on

- **The first-run flow** (`shell/first_run.rs`): the welcome page and its storage flag work and are tested (`tests/first_run.rs`), but `App` does not gate on it. `startup.rs` and `App` are where the gate goes.
- **File logging**, and the HTTP and analytics seams.

## i18n

`i18n!` in `src/lib.rs` loads `i18n/*.yml` at compile time. `store::locale` mirrors `rust-i18n`'s locale into a signal; `locale::text("key")` gives a closure that redraws when the language changes. The Items, Sign in and Settings pages and the demo Platform pages are translated; the other demo screens and the shell's labels are English literals.

## The signed package path

`packages/remote-screen` proves the delivery path a real app uses for code that changes independently of the binary:

1. **Develop** — TSX importing `nsh`, `nsh-query`, `nsh-form`. The TypeScript libraries reach the bundler through `node_modules` symlinks to the sibling checkouts (`mkdir -p node_modules && ln -s <path> node_modules/<name>`); `tsconfig.json` needs no change, and overriding `compilerOptions.paths` there breaks the `nsh` types.
2. **Gate** — `nsh check`, `nsh build` with no warnings, `nsh test` (the `test-tsx` recipe). `If` and `Each` are globals in TSX: importing them from `nsh` is a type error. `nsh check` needs `App.tsx` at the package root, or an entry passed to it.
3. **Sign** — `nsh keygen` makes a throwaway key pair at run time (never committed), `nsh certify` certifies it for this app id and a time window, `nsh publish` builds and signs the `.nshpkg`.
4. **Deliver** — the app verifies with `nsh verify` semantics against its trusted root and loads the package on demand. Bytecode loads only from the app bundle, the APK or a verified package. `nsh revoke` signs a revocation list; the app shows the revocation path.
5. **Fail closed** — host calls reject with `ERR_ACCESS_DENIED` when the capability is undeclared, `ERR_NSH_HOST_ERROR` when unhandled or thrown, and `ERR_FEATURE_UNAVAILABLE_ON_PLATFORM` with no host.

Capability keys declared in `nsh.json` come from the real 0.7.0 set (`KNOWN_CAPABILITIES` in `src/services/capabilities.rs`: sixteen keys, the 0.5.0 thirteen plus `application`, `homeDir` and `share`) — kept tight. Since 0.5.0 a typo is only a warning from `nsh build` (the key is not granted), so read the build output.

## Testing

- **Rust** (`tests/<screen>.rs`): headless engine tests with `nsh-testing` on a manual clock. `Headless::start` can return before the first frame, so tests `wait_for` something first.
- **The rules** (`tests/architecture.rs`): the layout's rules, read from the source as text.
- **TSX headless** (`tests/smoke/app.test.tsx`, `tests/smoke/pages.test.tsx`): `nsh test <file>` opens the app on an engine per test, after `cargo test`.
- **Per target** (`tests/smoke/`, `tests/vision/`): `nsh test --target macos|ios|android|web <file>` drives the real host and saves screenshots. `tests/vision/` needs a real target and someone to look at the captures; it fails headless by design.
- **Budgets** (`tests/trace/data.test.tsx`): `nsh test --trace` shows mount batches for a like tap and a feed scroll.

## A new app

`just new-app <dir> <name> <id>` copies the repo without the demo screens, the remote-screen package and this starter's build records, then renames it. The copy keeps Home, Items (with its detail), Sign in, Settings and Diagnostics, the fake API Items reads, and the `nsh-query` and `nsh-form` dependencies; its phone tabs are Home, Items and Settings.

## The template screens

- **Items** (`src/pages/items/`): the list-with-detail pattern. `queries.rs` holds the keys and the `use_query*` reads over `services::fake_api`; the list shows skeletons, the error with Retry, the empty case, then a row per item. A row calls `nav::push_child(Route::ItemsDetail(id))`: on phones the shell pushes the item's screen, elsewhere the list presents it over itself.
- **Sign in** (`src/pages/sign_in/`): an `nsh-form` form over `store::session`, which calls the auth seam's mock provider. Rules show field errors after the first submit; a refusal shows on the form; signed in, the page and Settings' Account card show the user and Sign out. Nothing in the app is gated on it.
