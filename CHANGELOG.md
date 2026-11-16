# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Changed

- The to-do page's strings render from the i18n catalogs (`en`, `zh-CN`):
  placeholder, filters, remaining count, clear actions, the summary rail,
  the empty states, and the route title.
- Accessibility: row controls name their task ("Edit task: <title>"), the
  add and edit fields carry labels, section titles are header roles, and
  the light theme's focus ring meets 3:1 (`#818cf8` → `#6366f1`).
- README: a run note for the shipped to-do app on desktop, web, iOS and
  Android, and the clone URL corrected to this repository.

### Added

- `tests/todo_i18n.rs`: the page rendered under `zh-CN`, waiting on catalog
  strings. A keyboard smoke test proves the desktop auto-focus flow on
  real targets and skips where focus is withheld by design.

## [0.1.0] - 2026-11-07

### Added

- Rust app scaffolded with the `nsh init` shape: `nsh.json` (`"rust"`, `"root"`),
  `Cargo.toml` with `nsh-query` and `nsh-form` path dependencies, theme at
  `themes/theme.json`, and the module tree (`app`, `shell`, `foundation`,
  `state`, `services`, `ui`, `platform`, `features/{forms,data,lists,canvas,
  motion,overlays,remote_screen}`) with placeholder pages per feature area.
- `App.test.tsx` headless smoke test; `nsh check`, `nsh build` and `nsh test`
  pass on the scaffold.
- `justfile`: `check`, `test`, `test-tsx`, `smoke`, `run-macos`, `run-ios`,
  `run-android`, `run-web`, `package`, `rename`, `status`.
- `scripts/rename.sh`: renames the app's name and id everywhere they live,
  with a `--roundtrip-test` mode that proves the renamed copy passes the full
  gate (fmt, clippy, `cargo test`, `nsh test`) and always cleans the copy up.
- Project docs: this changelog, `CONTRIBUTING.md` (gates, test rules, doc
  standards), `AGENTS.md` (ownership map, evidence conventions), `README.md`
  (clean clone to running app, page map), `docs/architecture.md`,
  `docs/coverage.md`, `docs/gpui-starter-parity.md`, `docs/upstream-issues.md`.
- i18n scaffold with `en` and `zh-CN` locales.
- Shell: typed route enum with deep-link parsing, a nav stack every entry
  point drives, the lifecycle state machine, undo snapshots on navigation and
  the back guard (`src/shell/`).
- Foundation primitives: ids, time, validation, typed config, and the
  capability registry (`src/foundation/`); state modules for the event bus,
  background tasks and undo history (`src/state/`).
- Data feature: the infinite feed with optimistic likes and retry, the post
  detail, and the query devtools, backed by the local fake API with latency,
  failure and offline switches (`src/features/data/`,
  `src/services/fake_api.rs`).
- Lists feature: virtualized orders, the quotes `DataTable`, and the tree
  demo (`src/features/lists/`).
- Remote-screen package: the TSX feature over the TypeScript `nsh-query` and
  `nsh-form`, with its manifest, theme and `node_modules` symlinks to the
  sibling checkouts (`packages/remote-screen/`).
