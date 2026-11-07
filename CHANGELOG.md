# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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
