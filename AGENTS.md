# AGENTS.md

Read this first. It is the map of the repository and the rules every agent and subagent follows in it. The brief this starter was built to (goal, sandbox, hard rules, satisfaction criteria) is [docs/PROMPT.md](docs/PROMPT.md); its hard rules bind too. Progress is tracked in `docs/STATUS.md`.

## The map

```text
src/
  lib.rs        the crate root: module list, native registration, the App root component
  main.rs       the desktop entry (cargo run)
  routes.rs     every screen's address (Route) and the record each screen declares (Page)
  error.rs      the app's one error type (AppError)
  startup.rs    everything that runs once at launch, in order
  pages/        the app's screens, one folder each; mod.rs is the table of all screens
  showcase/     the demo screens, built like pages/; a new app leaves this folder behind
  shell/        the frame around the screens: sidebar, tabs, headers, palette, back, links, error banner
  store/        what the app remembers: navigation, config, locale, appearance, tasks, events, errors
  services/     what the app talks to: the fake API, auth, HTTP, analytics, monitoring, the log, nsh.json
  ui/           the component library: one control per file, styled from the theme
  native/       platform code (UIKit, AppKit); the only folder that may use unsafe
tests/          <screen>.rs for every screen folder, shell.rs, architecture.rs; TSX suites in smoke/, vision/, trace/
docs/recipes/   step lists for the common changes
i18n/  themes/  assets/  packages/remote-screen/  scripts/
```

Read in this order: `src/lib.rs`, `src/routes.rs`, `src/pages/mod.rs`, then the folder of the screen you are changing. [docs/architecture.md](docs/architecture.md) explains how the pieces work together.

## Where does it go?

| You are adding | It goes in |
|---|---|
| A screen | `src/pages/<name>/` ([recipe](docs/recipes/add-a-page.md)) |
| State two screens share, or that outlives a screen | `src/store/` ([recipe](docs/recipes/add-shared-state.md)) |
| A call to the outside: network, files, a system API with no view | `src/services/` ([recipe](docs/recipes/add-a-service.md)) |
| A control or layout piece more than one screen uses | `src/ui/` ([recipe](docs/recipes/add-a-control.md)) |
| A view UIKit or AppKit draws | `src/native/` |
| Something that must run at launch | `src/startup.rs` |
| A change to the sidebar, tabs, headers or palette for every screen | `src/shell/` |
| A string a person reads | `i18n/*.yml` ([recipe](docs/recipes/add-a-string.md)) |

## A screen

Every folder under `src/pages/` and `src/showcase/` has the same shape:

```text
pages/settings/
  mod.rs       what the screen is, and its PAGE record
  view.rs      its layout (a view/ folder once it outgrows one file)
  state.rs     its own signals and the functions that change them, if it has any
  queries.rs   its server reads and writes, if it has any
tests/settings.rs
```

A screen has one name. `settings` is the folder, the `Route::Settings` variant, the record's `key`, `tests/settings.rs`, and the ids the shell gives it (`page-settings`, `nav-settings`, `tab-settings`). New strings and test ids inside the screen start with it too (`settings.title`, `settings-save`); some older demo screens predate that prefix. A screen pushed from another lives in its parent's folder and starts with its name (`Route::DataItem` is `showcase/data/item.rs`).

The shell knows no screen by name. Its sidebar row, tab, header, card on Home and palette entry all come from the `PAGE` record, so a new screen changes nothing in `src/shell/`.

## Rules

`just rules` checks these in under a second and says how to fix a break ([tests/architecture.rs](tests/architecture.rs)).

1. **Imports point one way:** `shell → pages → store → services`. `ui` and `native` sit below the screens; `native` may use `ui`. A screen never imports another screen's folder or the shell: what two screens share lives in `store`.
2. **No source file is longer than 500 lines.** Split by responsibility: `x.rs` becomes `x/mod.rs` plus parts.
3. **Every `global::<T>()` uses a type declared for that one piece of state**, never `bool`, `String` or `Vec<..>`.
4. **`unsafe` appears only under `src/native/`.** The compiler enforces it too (`src/lib.rs`).
5. **Every screen folder has the shape above**, a record keyed by the folder's name, and `tests/<folder>.rs`.
6. **The app's pages take their controls from `ui`**, never from `nsh_base` (its `theme()` excepted). The showcase is exempt: it demonstrates the base components themselves.
7. **Every language file has every key.**
8. **This map names everything directly under `src/`.**

Also, from the brief: no raw hex, `rgb(` or `hsl(` outside `themes/theme.json`; comments are one or two lines and state a non-obvious constraint; dead code is deleted.

## Commands

| Command | What it does |
|---|---|
| `just check` | `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings` |
| `just test` | every Rust test |
| `just test settings` | one screen's tests (`tests/settings.rs`) |
| `just rules` | the architecture rules alone |
| `just smoke` | the headless TSX suites and the remote-screen package gates |
| `just run-macos`, `run-ios`, `run-android`, `run-web` | the app on that target |
| `just new-app ../acme acme com.acme.app` | a new app from this starter, without the demo screens |

## Gates

Before calling work done: `just check`, `just test`; for TSX, `nsh check`, `nsh build` (every warning fixed), `nsh test`. Tests go through the UI on a manual clock with no real network; every new test is seen to fail once by breaking the code on purpose. No check is weakened, skipped or deleted to get a pass.

## Ownership

When several agents work at once, file areas are disjoint. Find your area and stay in it.

- **A screen:** its folder under `src/pages/` or `src/showcase/`, `tests/<name>.rs`, `tests/vision/<name>.test.tsx`. The `data` screen also owns `src/services/fake_api.rs` and `tests/trace/`; `platform` also owns `i18n/`, the icon assets and `nsh.json`; `remote_screen` also owns `packages/remote-screen`.
- **shell:** `src/lib.rs`, `src/main.rs`, `src/startup.rs`, `src/routes.rs`, `src/pages/mod.rs`, `src/shell`, `tests/shell.rs`, `tests/showcase.rs`, `tests/first_run.rs`, `tests/smoke/`.
- **store and services:** `src/store`, `src/services` (except `fake_api.rs`), `src/error.rs`.
- **ui:** `src/ui`, `themes/theme.json`.
- **native:** `src/native`.
- **project:** `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `CHANGELOG.md`, `justfile`, `scripts/`, `tests/architecture.rs`, `docs/architecture.md`, `docs/recipes/`, `docs/coverage.md`, `docs/gpui-starter-parity.md`, `docs/upstream-issues.md`.

`docs/PROMPT.md` and `docs/STATUS.md` belong to the orchestrator. Adding a screen touches two shell files (`src/routes.rs`, `src/pages/mod.rs`), one line each: hand those lines to the shell's owner.

## Evidence

- Every satisfaction-criteria claim needs evidence: the command you ran and its output path under `docs/evidence/<area>/`.
- Look at every PNG you save. A screenshot nobody looked at proves nothing.
- Record numbers (trace mount counts, timings) in `docs/STATUS.md`.
- A claim with no evidence is reported as unverified, in your section and in the docs you write.

## Shared machine

Other agents build other repos on this Mac. Stay inside this checkout; reading `~/fo/.refs`, `~/fo/nsh-query` and `~/fo/nsh-form` is fine, writing to any other repository never is. Never run `shutdown-all`, `erase-all`, `kill-server` or `killall`; never kill a process or port holder you did not start. Make your own simulator/AVD names after this repo.

## No git machinery

Subagents do not commit, branch, stash, reset or check files out of git; the orchestrator commits. Edit the current checkout only. To undo an experiment, keep a copy of the file first and copy it back.

## Facts that bite

`docs/PROMPT.md` section "Facts that bite" overrides any docs, including upstream skills, when they disagree. Verify every nsh subcommand and flag against `nsh --help` and the source of the installed release (`~/.cargo/registry/src/*/nsh-core-0.6.0`) before documenting it.

Two more, from this layout:

- `rust-i18n`'s locale is one value for the whole process. Two tests in one file that switch the language read each other's: keep language switches in one test per file.
- `git mv` keeps the old content in the index. Checking a moved file out of git brings back what it held before the move.
