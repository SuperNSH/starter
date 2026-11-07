# Status

What is done, what was verified how (command + evidence path under
`docs/evidence/<area>/`), and what is NOT verified. A claim with no evidence
is unverified. Every area updates its own section as it lands work.

## Baseline (bootstrap, pre-workflow)

- Scaffolded the Rust app at the repo root with `nsh init --template rust`
  shape: `nsh.json` (`"rust"`, `"root"`, theme at `themes/theme.json`),
  `Cargo.toml` with `nsh-query`/`nsh-form` path deps, `rust-i18n`, and
  `nsh-engine`/`nsh-testing` 0.4.0 dev-deps; module tree in `src/` with
  placeholder pages for every feature area; `App.test.tsx` + `tsconfig.json`.
- Verified on the baseline only: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, `nsh check`,
  `nsh build` (no warnings), `nsh test` (1 passing test). No feature work
  exists yet; every satisfaction criterion is unverified.

## Project docs and tooling

- Done: The project area's full file set — `justfile` (12 recipes: check,
  test, test-tsx, smoke, run-macos/ios/android/web, package, rename, status),
  `scripts/rename.sh` (in-place rename of every name/id token form plus a
  `--roundtrip-test` mode that copies to a sibling, renames, runs the
  criterion-1 gate there, and always cleans the copy up), and the docs:
  README (clean clone to running app, page map aligned with the shell's
  `Route` enum), CONTRIBUTING (gates, manual-clock and seen-to-fail test
  rules, doc standards), AGENTS.md (PROMPT pointer kept, plus gates,
  ownership map, evidence conventions), CHANGELOG 0.1.0,
  `docs/architecture.md`, `docs/coverage.md` (every nsh_core, nsh_base,
  plot, nsh/mobile and nsh/desktop export as a row, named from
  `.nsh/types/nsh/`), `docs/gpui-starter-parity.md`, and
  `docs/upstream-issues.md`.
- Verified: scoped gate `bash -n scripts/rename.sh` pass and
  `test -x scripts/rename.sh` pass (re-run after every fix round);
  in-place rename functional test in a throwaway /tmp copy — all three
  token forms (`nsh-starter`, `nsh_starter`, `com.example.nshstarter`)
  renamed across 9 files, Cargo.lock correctly left to cargo, zero
  leftovers outside it, original untouched; roundtrip trap-fix repro in
  /tmp with the gate stubbed — pass path exits 0 with the copy deleted,
  FAIL path exits 1 with the copy deleted; 3 audit rounds (round 1: 5
  findings incl. 1 high EXIT-trap unbound-variable bug, round 2: 4/5
  confirmed fixed + 2 residual, round 3 fixes self-verified by checking
  every coverage row name against `.nsh/types` and confirmed by the
  orchestrator's spot-check); `just -n` parses all 12 recipes; every
  documented nsh flag checked against the installed nsh 0.4.0 help
  (`nsh --help`, `nsh test/run/package/keygen/publish/verify/revoke
  --help`); coverage rows and cross-references verified against
  `.nsh/types/nsh/*.d.ts`, the nsh-base/nsh-core 0.4.0 crate sources, and
  the `~/fo/nsh-query` / `~/fo/nsh-form` sources (`form.load` at
  nsh-form `form.rs:157`, nsh-query exports, `tracked.rs`).
- Not verified: the `--roundtrip-test` full criterion-1 gate inside a
  renamed copy (fmt + clippy + cargo test + nsh test compile the whole
  graph; it runs at the whole-tree smoke gate); docs/architecture.md and
  README factual claims about code other areas are still building (shell
  header/menus, overlays pages, platform surfaces, `smoke/`,
  `tests/<area>.rs`, `trace/`, `scripts/delivery.sh` in the remote-screen
  package — re-verify against the tree at the join);
  `docs/coverage.md` unused-export accounting (the *planned* rows need a
  final grep pass at the join, and anything still unused needs its
  stated reason); the `brew install nsh` formula name was inferred from
  the brief's `$(brew --prefix nsh)` and never run.
