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

## Lists area audit notes (round 2)

- Wrapper gap kept local: `src/features/lists/calendar_view.rs` does NOT
  use `ui::pickers::calendar_of`, because that wrapper hardcodes its own
  `render_item` (it labels weekdays and months with bare numbers, so the
  month header would read "11" instead of "November 2026"), takes no
  `disabled_matcher` (the demo's weekends-off behavior is asserted by
  `tests/lists.rs`), and returns `impl IntoElement`, so neither
  `render_item` nor `disabled_matcher` can be chained by the caller. To
  adopt it, `ui::pickers::calendar_of` needs `disabled_matcher` and
  `render_item` passthrough parameters (or a `_with` variant).
- Retracted an earlier engine-quirk claim: "a row prepended by
  `on_load_more_at_start` never mounts, even after `scroll_to_index(0)`"
  was an artifact of the demo's old loader bookkeeping (a broken window
  fed duplicate ids into the list's keys). With the fixed window, the
  start-loader test asserts the arrival mounts after the Latest jump and
  passes; no upstream issue.
- Verified engine fact worth keeping: on an inverted list whose array is
  newest-first, a loader that prepends (array head) is a layout-tail
  append, so dropping `.keys(…)` produces no visible difference in these
  flows; keys stay in place per the coding guides, and the chat's loader
  test breaks on loader bookkeeping instead.

## Lists and data views

- Done: All nine deliverables in `src/features/lists/` — `Table` semantics
  parts over an order book plus a sectioned list with sticky headers
  (orders.rs), `DataTable` over 3,000 deterministic quotes with app-side
  sorting, a fixed symbol column and stripes on `ui::table::data_table_of`
  (quotes.rs), a controlled `Tree` with depth-indented, expansion-aware
  rows (tree.rs), a controlled `Calendar` with a weekends-off matcher and
  named month/weekday items (calendar_view.rs), a 2,000-card grid with
  viewability logging, jump controls and data-keyed stars (grid.rs), an
  inverted chat opened mid-history whose loaders produce gapless pages at
  both ends, with Receive/Latest controls and a `ui::field::text_field`
  draft (chat.rs), a `ui::carousel_of` cover carousel (carousel_view.rs),
  and `ui::pagination_of` over 500 generated entries (pager.rs); the
  audit round moved text, table, pagination and carousel surfaces onto the
  ui wrappers and landed the chat loader redesign (window 240..=280 of
  room 1..=300, page math in pure `older_page`/`newer_page` helpers).
- Verified: `cargo test --test lists` — 7/7 pass (contiguity unit test,
  sort orders, grid scroll keeps stars, chat start loader + no-jump, chat
  older loader + mounted-run span check, calendar selection + refused
  weekend, pager rows) on the manual clock, final run after the audit
  fixes; scoped `cargo clippy --all-targets` — no findings in
  `src/features/lists/**` or `tests/lists.rs`; scoped `cargo fmt --check`
  clean for the area's files; `tsc -p tsconfig.json` exit 0 covering
  `vision/lists.test.tsx`; two audit rounds — r1 six findings (the
  chat-loader HIGH window bug, missing loader tests, calendar/input
  wrapper swaps, in-repo break records, the Latest-mount re-check), r2
  clean with two comment nits fixed (the older page is 225..=239; the
  pager break fails at the Archive-21 wait, not the caption); seen-to-fail
  re-verified per test with the records in the `tests/lists.rs` header;
  the calendar wrapper-gap record and the retracted Latest-mount claim
  are in this file's "Lists area audit notes (round 2)" section.
- Not verified: pixels and real-platform runs — the vision phase has not
  run yet; `vision/lists.test.tsx` (8 views × light/dark × phone/desktop)
  is written and type-checks but has never executed, and it presses the
  shell's "Lists" nav entry, so it needs the shell and a real target;
  whole-tree gates (`cargo clippy/test --all-targets` over all areas,
  `nsh build`, the smoke suite) are left to the join/smoke phases —
  my scoped runs were blocked at times by other areas' in-flight files
  and re-run until green; two suspected engine quirks remain
  observations only, repro notes for `docs/upstream-issues.md`: (a) on a
  manual clock, an animated `scroll_to_index` can leave cell rebinds
  lagging the geometry (viewability reported rows 0–17 while the cells at
  those rows still showed their pre-jump texts, repro: grid demo, jump to
  card 1500 and back, read `screen().texts()`), and (b) on an inverted
  list, a row prepended by `on_load_more_at_start` while the viewport was
  anchored away from the start did not appear in the mounted pool after a
  later jump to the start in early probes — not reproducible after the
  loader fix (the Latest-mount assertion passes), so it is recorded as a
  first-half observation, not a claim.
